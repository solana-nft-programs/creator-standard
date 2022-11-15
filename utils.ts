import { utils, Wallet } from "@project-serum/anchor";
import {
  createAssociatedTokenAccountInstruction,
  createInitializeMint2Instruction,
  createMintToInstruction,
  getAssociatedTokenAddressSync,
  getMinimumBalanceForRentExemptMint,
  MINT_SIZE,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import type { SendTransactionError, Signer } from "@solana/web3.js";
import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  sendAndConfirmRawTransaction,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";

import { PROGRAM_ADDRESS } from "./sdk/generated";

export async function newAccountWithLamports(
  connection: Connection,
  lamports = LAMPORTS_PER_SOL,
  keypair = Keypair.generate()
): Promise<Keypair> {
  const account = keypair;
  const signature = await connection.requestAirdrop(
    account.publicKey,
    lamports
  );
  await connection.confirmTransaction(signature, "confirmed");
  return account;
}

export function getConnection(): Connection {
  const url = "http://localhost:8899";
  return new Connection(url, "confirmed");
}

export async function executeTransaction(
  connection: Connection,
  tx: Transaction,
  wallet: Wallet,
  signers?: Signer[]
): Promise<string> {
  tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
  tx.feePayer = wallet.publicKey;
  await wallet.signTransaction(tx);
  if (signers) {
    tx.partialSign(...signers);
  }
  try {
    const txid = await sendAndConfirmRawTransaction(connection, tx.serialize());
    return txid;
  } catch (e) {
    handleError(e);
    throw e;
  }
}

export type CardinalProvider = {
  connection: Connection;
  wallet: Wallet;
  keypair: Keypair;
};

export async function getProvider(): Promise<CardinalProvider> {
  const connection = getConnection();
  const keypair = await newAccountWithLamports(
    connection,
    LAMPORTS_PER_SOL,
    keypairFrom(process.env.TEST_KEY ?? "./tests/test-keypairs/test-key.json")
  );
  const wallet = new Wallet(keypair);
  return {
    connection,
    wallet,
    keypair,
  };
}

export const TEST_PROGRAM_ID = process.env.TEST_PROGRAM_ID
  ? new PublicKey(process.env.TEST_PROGRAM_ID)
  : PROGRAM_ADDRESS;

export const keypairFrom = (s: string, n?: string): Keypair => {
  try {
    if (s.includes("[")) {
      return Keypair.fromSecretKey(
        Buffer.from(
          s
            .replace("[", "")
            .replace("]", "")
            .split(",")
            .map((c) => parseInt(c))
        )
      );
    } else {
      return Keypair.fromSecretKey(utils.bytes.bs58.decode(s));
    }
  } catch (e) {
    try {
      return Keypair.fromSecretKey(
        Buffer.from(
          JSON.parse(
            require("fs").readFileSync(s, {
              encoding: "utf-8",
            })
          )
        )
      );
    } catch (e) {
      process.stdout.write(`${n ?? "keypair"} is not valid keypair`);
      process.exit(1);
    }
  }
};

export const handleError = (e: any) => {
  const message = (e as SendTransactionError).message ?? "";
  const logs = (e as SendTransactionError).logs;
  if (logs) {
    console.log(logs);
    // const parsed = parseProgramLogs(logs, message);
    // const fmt = formatInstructionLogsForConsole(parsed);
    // console.log(fmt);
  } else {
    console.log(e, message);
  }
};

const networkURLs: { [key: string]: { primary: string; secondary?: string } } =
  {
    ["mainnet-beta"]: {
      primary:
        process.env.MAINNET_PRIMARY || "https://solana-api.projectserum.com",
      secondary: "https://solana-api.projectserum.com",
    },
    mainnet: {
      primary:
        process.env.MAINNET_PRIMARY || "https://solana-api.projectserum.com",
      secondary: "https://solana-api.projectserum.com",
    },
    devnet: { primary: "https://api.devnet.solana.com/" },
    testnet: { primary: "https://api.testnet.solana.com/" },
    localnet: { primary: "http://localhost:8899/" },
  };

export const connectionFor = (
  cluster: string | null,
  defaultCluster = "mainnet"
) => {
  return new Connection(
    process.env.RPC_URL || networkURLs[cluster || defaultCluster]!.primary,
    "recent"
  );
};

export const secondaryConnectionFor = (
  cluster: string | null,
  defaultCluster = "mainnet"
) => {
  return new Connection(
    process.env.RPC_URL ||
      networkURLs[cluster || defaultCluster]?.secondary ||
      networkURLs[cluster || defaultCluster]!.primary,
    "recent"
  );
};

export const createMintTx = async (
  connection: Connection,
  mint: PublicKey,
  authority: PublicKey,
  target = authority
) => {
  const ata = getAssociatedTokenAddressSync(mint, target);
  return new Transaction().add(
    SystemProgram.createAccount({
      fromPubkey: authority,
      newAccountPubkey: mint,
      space: MINT_SIZE,
      lamports: await getMinimumBalanceForRentExemptMint(connection),
      programId: TOKEN_PROGRAM_ID,
    }),
    createInitializeMint2Instruction(mint, 0, authority, authority),
    createAssociatedTokenAccountInstruction(authority, ata, target, mint),
    createMintToInstruction(mint, ata, authority, 1)
  );
};

type AccountFn<T> = () => Promise<T>;
export async function tryGetAccount<T>(fn: AccountFn<T>) {
  try {
    return await fn();
  } catch {
    return null;
  }
}

import { utils } from "@project-serum/anchor";
import type {
  Wallet as IWallet,
  Wallet,
} from "@project-serum/anchor/dist/cjs/provider";
import {
  createAssociatedTokenAccountInstruction,
  createInitializeMint2Instruction,
  createMintToInstruction,
  getAssociatedTokenAddressSync,
  getMinimumBalanceForRentExemptMint,
  MINT_SIZE,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import type {
  Cluster,
  ConfirmOptions,
  SendTransactionError,
  Signer,
} from "@solana/web3.js";
import {
  Connection,
  Keypair,
  PublicKey,
  sendAndConfirmRawTransaction,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";
import * as dotenv from "dotenv";

dotenv.config();

export const createMintTx = async (
  connection: Connection,
  mint: PublicKey,
  authority: PublicKey,
  amount?: number,
) => {
  const ata = getAssociatedTokenAddressSync(mint, authority);
  return new Transaction().add(
    SystemProgram.createAccount({
      fromPubkey: authority,
      newAccountPubkey: mint,
      space: MINT_SIZE,
      lamports: await getMinimumBalanceForRentExemptMint(connection),
      programId: TOKEN_PROGRAM_ID,
    }),
    createInitializeMint2Instruction(mint, 0, authority, authority),
    createAssociatedTokenAccountInstruction(authority, ata, authority, mint),
    createMintToInstruction(mint, ata, authority, amount ?? 1),
  );
};

const networkURLs: { [key in Cluster | "mainnet" | "localnet"]: string } = {
  ["mainnet-beta"]:
    process.env.MAINNET_PRIMARY_URL ?? "https://solana-api.projectserum.com",
  mainnet:
    process.env.MAINNET_PRIMARY_URL ?? "https://solana-api.projectserum.com",
  devnet: "https://api.devnet.solana.com/",
  testnet: "https://api.testnet.solana.com/",
  localnet: "http://localhost:8899/",
};

export const connectionFor = (
  cluster: Cluster | "mainnet" | "localnet",
  defaultCluster = "mainnet",
) => {
  return new Connection(
    process.env.RPC_URL || networkURLs[cluster || defaultCluster],
    "recent",
  );
};

export const keypairFrom = (s: string, n?: string): Keypair => {
  try {
    if (s.includes("[")) {
      return Keypair.fromSecretKey(
        Buffer.from(
          s
            .replace("[", "")
            .replace("]", "")
            .split(",")
            .map((c) => parseInt(c)),
        ),
      );
    } else {
      return Keypair.fromSecretKey(utils.bytes.bs58.decode(s));
    }
  } catch (e) {
    try {
      return Keypair.fromSecretKey(
        Buffer.from(
          // eslint-disable-next-line @typescript-eslint/no-unsafe-argument
          JSON.parse(
            // eslint-disable-next-line @typescript-eslint/no-unsafe-argument, @typescript-eslint/no-unsafe-member-access, @typescript-eslint/no-unsafe-call, @typescript-eslint/no-var-requires
            require("fs").readFileSync(s, {
              encoding: "utf-8",
            }),
          ),
        ),
      );
    } catch (e) {
      process.stdout.write(`${n ?? "keypair"} is not valid keypair`);
      process.exit(1);
    }
  }
};

export async function executeTransaction(
  connection: Connection,
  tx: Transaction,
  wallet: Wallet,
  signers?: Signer[],
  silent?: boolean,
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
    if (!silent) {
      handleError(e);
    }
    throw e;
  }
}

export const publicKeyFrom = (s: string, n?: string): PublicKey => {
  try {
    return new PublicKey(s);
  } catch (e) {
    process.stdout.write(`${n ?? "publicKey"} is not valid publicKey`);
    process.exit(1);
  }
};

export async function executeTransactions(
  connection: Connection,
  txs: Transaction[],
  wallet: Wallet,
  signers?: Signer[],
): Promise<string[]> {
  const latestBlockhash = (await connection.getLatestBlockhash()).blockhash;
  const signedTxs = await wallet.signAllTransactions(
    txs.map((tx) => {
      tx.recentBlockhash = latestBlockhash;
      tx.feePayer = wallet.publicKey;
      if (signers) {
        tx.partialSign(...signers);
      }
      return tx;
    }),
  );
  const txids = await Promise.all(
    signedTxs.map(async (tx) => {
      try {
        const txid = await sendAndConfirmRawTransaction(
          connection,
          tx.serialize(),
        );
        return txid;
      } catch (e) {
        handleError(e);
        throw e;
      }
    }),
  );
  return txids;
}

export const handleError = (e: unknown) => {
  const message = (e as SendTransactionError).message ?? "";
  const logs = (e as SendTransactionError).logs;
  if (logs) {
    console.log(logs);
  } else {
    console.log(e, message);
  }
};

export const clusterFrom = (s: string): Cluster => {
  switch (s) {
    case "mainnet":
      return "mainnet-beta";
    case "mainnet-beta":
      return "mainnet-beta";
    case "devnet":
      return "devnet";
    case "testnet":
      return "testnet";
    default:
      return "devnet";
  }
};

export async function executeTransactionBatches<T = null>(
  connection: Connection,
  txs: Transaction[],
  wallet: IWallet,
  config?: {
    signers?: Keypair[][];
    batchSize?: number;
    successHandler?: (
      txid: string,
      ix: { i: number; j: number; it: number; jt: number },
    ) => void;
    errorHandler?: (
      e: unknown,
      ix: { i: number; j: number; it: number; jt: number },
    ) => T;
    confirmOptions?: ConfirmOptions;
  },
): Promise<(string | null | T)[]> {
  const batchLength = config?.batchSize ?? txs.length;
  const batchedTxs = chunkArray(txs, batchLength);
  const txids: (string | T | null)[] = [];
  for (let i = 0; i < batchedTxs.length; i++) {
    const batch = batchedTxs[i];
    if (batch) {
      const latestBlockhash = (await connection.getLatestBlockhash()).blockhash;
      const batchSignedTxs = await wallet.signAllTransactions(
        batch.map((tx, j) => {
          tx.recentBlockhash = latestBlockhash;
          tx.feePayer = wallet.publicKey;
          if (config?.signers?.at(i * batchLength + j)) {
            tx.partialSign(...(config?.signers.at(i * batchLength + j) ?? []));
          }
          return tx;
        }),
      );
      const batchTxids = await Promise.all(
        batchSignedTxs.map(async (tx, j) => {
          try {
            const txid = await sendAndConfirmRawTransaction(
              connection,
              tx.serialize(),
              config?.confirmOptions,
            );
            if (config?.successHandler) {
              config?.successHandler(txid, {
                i,
                it: batchedTxs.length,
                j,
                jt: batchSignedTxs.length,
              });
            }
            return txid;
          } catch (e) {
            if (config?.errorHandler) {
              return config?.errorHandler(e, {
                i,
                it: batchedTxs.length,
                j,
                jt: batchSignedTxs.length,
              });
            }
            return null;
          }
        }),
      );
      txids.push(...batchTxids);
    }
  }
  return txids;
}

export const chunkArray = <T>(arr: T[], size: number): T[][] =>
  arr.length > size
    ? [arr.slice(0, size), ...chunkArray(arr.slice(size), size)]
    : [arr];

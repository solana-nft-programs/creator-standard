import {
  Connection,
  Transaction,
  sendAndConfirmRawTransaction,
} from "@solana/web3.js";
import { PublicKey, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { PROGRAM_ADDRESS } from "./src/generated";
import { utils, Wallet } from "@project-serum/anchor";

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
  await connection.confirmTransaction(signature);
  return account;
}

export async function getConnection(): Promise<Connection> {
  const url = "http://localhost:8899";
  const connection = new Connection(url, "confirmed");
  await connection.getVersion();
  return connection;
}

export async function executeTransaction(
  connection: Connection,
  tx: Transaction,
  wallet: Wallet
): Promise<String> {
  tx.recentBlockhash = await (await connection.getLatestBlockhash()).blockhash;
  tx.feePayer = wallet.publicKey;
  await wallet.signTransaction(tx);
  return sendAndConfirmRawTransaction(connection, tx.serialize());
}

type CardinalProvider = {
  connection: Connection;
  wallet: Wallet;
  keypair: Keypair;
};

export async function getProvider(): Promise<CardinalProvider> {
  const connection = await getConnection();
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

import { Connection } from "@solana/web3.js";
import { SignerWallet } from "@saberhq/solana-contrib";
import { PublicKey, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { PROGRAM_ADDRESS } from "./src/generated";
import { utils } from "@project-serum/anchor";

export async function newAccountWithLamports(
  connection: Connection,
  lamports = LAMPORTS_PER_SOL
): Promise<Keypair> {
  const account = Keypair.fromSecretKey(
    utils.bytes.bs58.decode(process.env.TEST_SECRET_KEY || "")
  );
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

type CardinalProvider = {
  connection: Connection;
  wallet: SignerWallet;
  keypair: Keypair;
};
export async function getProvider(): Promise<CardinalProvider> {
  const connection = await getConnection();
  const keypair = await newAccountWithLamports(connection);
  const wallet = new SignerWallet(keypair);
  return {
    connection,
    wallet,
    keypair,
  };
}

export const TEST_PROGRAM_ID = process.env.TEST_PROGRAM_ID
  ? new PublicKey(process.env.TEST_PROGRAM_ID)
  : PROGRAM_ADDRESS;

import { test, beforeAll } from "@jest/globals";
import { getConnection, newAccountWithLamports } from "./utils";
import type { Connection, PublicKey, Signer } from "@solana/web3.js";
import {
  Keypair,
  Transaction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import { createInitInstruction } from "../src/generated";
import { createMint } from "@solana/spl-token";

let connection: Connection;
let authority: Signer;
let mint: PublicKey;

beforeAll(async () => {
  connection = await getConnection();
  authority = await newAccountWithLamports(connection);
  const mintKeypair = Keypair.generate();
  mint = await createMint(
    connection,
    authority,
    authority.publicKey,
    authority.publicKey,
    0,
    mintKeypair
  );
});

test("Init", async () => {
  const tx = new Transaction();
  tx.add(
    createInitInstruction({
      mint: mint,
      authority: authority.publicKey,
      standard: Keypair.generate().publicKey,
    })
  );
  await sendAndConfirmTransaction(connection, tx, [authority]);
});

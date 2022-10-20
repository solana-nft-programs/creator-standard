import { test, beforeAll } from "@jest/globals";
import { getConnection, getProvider, newAccountWithLamports } from "./utils";
import type { Connection, PublicKey, Signer } from "@solana/web3.js";
import {
  Keypair,
  Transaction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import { createInitInstruction } from "../src/generated";
import { createMint } from "@solana/spl-token";

let mint: PublicKey;

beforeAll(async () => {
  const provider = await getProvider();
  const mintKeypair = Keypair.generate();
  mint = await createMint(
    provider.connection,
    provider.keypair,
    provider.wallet.publicKey,
    provider.wallet.publicKey,
    0,
    mintKeypair
  );
});

test("Init", async () => {
  const provider = await getProvider();
  const tx = new Transaction();
  tx.add(
    createInitInstruction({
      mint: mint,
      authority: provider.wallet.publicKey,
      standard: Keypair.generate().publicKey,
    })
  );
  await sendAndConfirmTransaction(provider.connection, tx, [provider.keypair]);
});

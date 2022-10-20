import { test, beforeAll, expect } from "@jest/globals";
import { getProvider } from "../utils";
import type { PublicKey } from "@solana/web3.js";
import {
  Keypair,
  Transaction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import {
  createInitMintManagerInstruction,
  MintManager,
  MintManagerV2,
} from "../src/generated";
import { createMint } from "@solana/spl-token";
import { findMintManagerId } from "../src/pda";

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
    createInitMintManagerInstruction({
      mint: mint,
      mintManager: findMintManagerId(mint),
      authority: provider.wallet.publicKey,
      payer: provider.wallet.publicKey,
      ruleset: Keypair.generate().publicKey,
    })
  );
  await sendAndConfirmTransaction(provider.connection, tx, [provider.keypair]);

  const mintManager = await MintManager.fromAccountAddress(
    provider.connection,
    findMintManagerId(mint)
  );
  expect(mintManager.mint.toString()).toBe(mint.toString());

  const mintManager2 = await MintManagerV2.fromAccountAddress(
    provider.connection,
    findMintManagerId(mint)
  );
  console.log("-----", mintManager2);
  expect(mintManager2.mint.toString()).toBe(mint.toString());
});

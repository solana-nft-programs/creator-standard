import { test, beforeAll, expect } from "@jest/globals";
import { executeTransaction, getProvider } from "../utils";
import type { PublicKey } from "@solana/web3.js";
import { Keypair, Transaction } from "@solana/web3.js";
import {
  createInitMintManagerInstruction,
  MintManager,
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
      payer: provider.wallet.payer.publicKey,
      ruleset: Keypair.generate().publicKey,
    })
  );
  await executeTransaction(provider.connection, tx, provider.wallet);

  const mintManager = await MintManager.fromAccountAddress(
    provider.connection,
    findMintManagerId(mint)
  );
  expect(mintManager.mint.toString()).toBe(mint.toString());
});

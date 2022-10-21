import { test, beforeAll, expect } from "@jest/globals";
import { executeTransaction, getProvider } from "../utils";
import type { PublicKey } from "@solana/web3.js";
import { Keypair, Transaction } from "@solana/web3.js";
import { createMint } from "@solana/spl-token";
import {
  findMintManagerId,
  createInitMintManagerInstruction,
  MintManager,
  createInitStandardInstruction,
  findStandardId,
  Standard,
} from "../src";
let mint: PublicKey;

const STANDARD_NAME = "global";
const STANDARD_ID = findStandardId(STANDARD_NAME);

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

test("Create standard", async () => {
  const provider = await getProvider();
  const tx = new Transaction();
  tx.add(
    createInitStandardInstruction(
      {
        standard: STANDARD_ID,
        authority: provider.wallet.publicKey,
        payer: provider.wallet.publicKey,
      },
      {
        ix: {
          standardName: STANDARD_NAME,
          checkSellerFeeBasisPoints: true,
          disallowedPrograms: [],
          allowedPrograms: [],
        },
      }
    )
  );
  await executeTransaction(provider.connection, tx, provider.wallet);
  const standard = await Standard.fromAccountAddress(
    provider.connection,
    STANDARD_ID
  );
  expect(standard.authority.toString()).toBe(
    provider.wallet.publicKey.toString()
  );
  expect(standard.checkSellerFeeBasisPoints).toBe(true);
  expect(standard.disallowedPrograms).toBe([]);
  expect(standard.allowedPrograms).toBe([]);
});

test("Init", async () => {
  const provider = await getProvider();
  const mintManagerId = findMintManagerId(mint);
  const tx = new Transaction();
  tx.add(
    createInitMintManagerInstruction({
      mint: mint,
      mintManager: mintManagerId,
      authority: provider.wallet.publicKey,
      payer: provider.wallet.publicKey,
      collector: provider.wallet.publicKey,
      standard: Keypair.generate().publicKey,
    })
  );
  await executeTransaction(provider.connection, tx, provider.wallet);

  const mintManager = await MintManager.fromAccountAddress(
    provider.connection,
    mintManagerId
  );
  expect(mintManager.mint.toString()).toBe(mint.toString());
  expect(mintManager.authority.toString()).toBe(
    provider.wallet.publicKey.toString()
  );
  expect(mintManager.standard.toString()).toBe(
    findStandardId(STANDARD_NAME).toString()
  );
});

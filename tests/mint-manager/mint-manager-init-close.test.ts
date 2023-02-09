import { beforeAll, expect, test } from "@jest/globals";
import { getAssociatedTokenAddressSync, getMint } from "@solana/spl-token";
import { Keypair, Transaction } from "@solana/web3.js";

import {
  createCloseMintManagerInstruction,
  createInitMintManagerInstruction,
} from "../../sdk";
import { MintManager } from "../../sdk/generated/accounts/MintManager";
import {
  findMintManagerId,
  findMintMetadataId,
  findRulesetId,
} from "../../sdk/pda";
import type { CardinalProvider } from "../../utils";
import { createMintTx, executeTransaction, getProvider } from "../../utils";

const mintKeypair = Keypair.generate();
const RULESET_ID_1 = findRulesetId();
let provider: CardinalProvider;

beforeAll(async () => {
  provider = await getProvider();
  const splMintIx = await createMintTx(
    provider.connection,
    mintKeypair.publicKey,
    provider.wallet.publicKey
  );
  await executeTransaction(provider.connection, splMintIx, provider.wallet, [
    mintKeypair,
  ]);
});

test("Init mint manager", async () => {
  const mintManagerId = findMintManagerId(mintKeypair.publicKey);
  const mintMetadataId = findMintMetadataId(mintKeypair.publicKey);
  const tx = new Transaction();

  const ata = getAssociatedTokenAddressSync(
    mintKeypair.publicKey,
    provider.wallet.publicKey
  );
  tx.add(
    createInitMintManagerInstruction({
      mintManager: mintManagerId,
      mint: mintKeypair.publicKey,
      mintMetadata: mintMetadataId,
      ruleset: RULESET_ID_1,
      holderTokenAccount: ata,
      tokenAuthority: provider.wallet.publicKey,
      authority: provider.wallet.publicKey,
      payer: provider.wallet.publicKey,
    })
  );
  await executeTransaction(provider.connection, tx, provider.wallet);

  // check mint manager
  const mintManager = await MintManager.fromAccountAddress(
    provider.connection,
    mintManagerId
  );
  expect(mintManager.mint.toString()).toBe(mintKeypair.publicKey.toString());
  expect(mintManager.authority.toString()).toBe(
    provider.wallet.publicKey.toString()
  );
  expect(mintManager.ruleset.toString()).toBe(RULESET_ID_1.toString());
});

test("Close mint manager", async () => {
  const newAuthority = Keypair.generate();
  const mintManagerId = findMintManagerId(mintKeypair.publicKey);

  const ata = getAssociatedTokenAddressSync(
    mintKeypair.publicKey,
    provider.wallet.publicKey
  );

  const tx = new Transaction();
  tx.add(
    createCloseMintManagerInstruction({
      mintManager: mintManagerId,
      mint: mintKeypair.publicKey,
      holderTokenAccount: ata,
      newTokenAuthority: newAuthority.publicKey,
      authority: provider.wallet.publicKey,
      payer: provider.wallet.publicKey,
    })
  );
  await executeTransaction(provider.connection, tx, provider.wallet);

  // check mint manager
  await expect(
    MintManager.fromAccountAddress(provider.connection, mintManagerId)
  ).rejects.toThrow();
  const mintData = await getMint(provider.connection, mintKeypair.publicKey);
  expect(mintData.freezeAuthority?.toString()).toBe(
    newAuthority.publicKey.toString()
  );
  expect(mintData.mintAuthority?.toString()).toBe(
    newAuthority.publicKey.toString()
  );
});

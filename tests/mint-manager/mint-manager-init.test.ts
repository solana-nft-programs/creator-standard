import { beforeAll, expect, test } from "@jest/globals";
import { getAssociatedTokenAddressSync } from "@solana/spl-token";
import { Keypair, Transaction } from "@solana/web3.js";

import { createInitMintManagerInstruction } from "../../sdk";
import { MintManager } from "../../sdk/generated/accounts/MintManager";
import {
  findMintManagerId,
  findMintMetadataId,
  findRulesetId,
} from "../../sdk/pda";
import type { CardinalProvider } from "../../utils";
import { createMintTx, executeTransaction, getProvider } from "../../utils";

const mintKeypair = Keypair.generate();

const RULESET_NAME = "ruleset-no-checks";
const RULESET_ID = findRulesetId(RULESET_NAME);

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

test("Init", async () => {
  const mintManagerId = findMintManagerId(mintKeypair.publicKey);
  const tx = new Transaction();

  const ata = getAssociatedTokenAddressSync(
    mintKeypair.publicKey,
    provider.wallet.publicKey
  );
  tx.add(
    createInitMintManagerInstruction({
      mintManager: mintManagerId,
      mint: mintKeypair.publicKey,
      mintMetadata: findMintMetadataId(mintKeypair.publicKey),
      ruleset: RULESET_ID,
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
  expect(mintManager.ruleset.toString()).toBe(
    findRulesetId(RULESET_NAME).toString()
  );
});

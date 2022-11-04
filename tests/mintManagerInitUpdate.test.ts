import { beforeAll, expect, test } from "@jest/globals";
import { getAssociatedTokenAddressSync } from "@solana/spl-token";
import { Keypair, Transaction } from "@solana/web3.js";

import {
  createInitMintManagerInstruction,
  createUpdateMintManagerInstruction,
  findMintManagerId,
  findRulesetId,
  MintManager,
  Ruleset,
} from "../sdk";
import type { CardinalProvider } from "../utils";
import { createMintTx, executeTransaction, getProvider } from "../utils";

const mintKeypair = Keypair.generate();
const RULESET_NAME = "cardinal-no-check";
const RULESET_NAME_2 = "cardinal-no-check-2";
const RULESET_ID = findRulesetId(RULESET_NAME);
let provider: CardinalProvider;

beforeAll(async () => {
  provider = await getProvider();
  await executeTransaction(
    provider.connection,
    await createMintTx(
      provider.connection,
      mintKeypair.publicKey,
      provider.wallet.publicKey
    ),
    provider.wallet,
    [mintKeypair]
  );
});

test("Init mint manager", async () => {
  const mintManagerId = findMintManagerId(mintKeypair.publicKey);
  const tx = new Transaction();
  const ruleset = await Ruleset.fromAccountAddress(
    provider.connection,
    RULESET_ID
  );

  const ata = getAssociatedTokenAddressSync(
    mintKeypair.publicKey,
    provider.wallet.publicKey
  );
  tx.add(
    createInitMintManagerInstruction({
      mintManager: mintManagerId,
      mint: mintKeypair.publicKey,
      ruleset: RULESET_ID,
      holderTokenAccount: ata,
      rulesetCollector: ruleset.collector,
      collector: ruleset.collector,
      authority: provider.wallet.publicKey,
      payer: provider.wallet.publicKey,
    })
  );
  await executeTransaction(provider.connection, tx, provider.wallet);

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

test("Update mint manager", async () => {
  const newAuthority = Keypair.generate();
  const mintManagerId = findMintManagerId(mintKeypair.publicKey);
  const tx = new Transaction();
  const ruleset = await Ruleset.fromAccountAddress(
    provider.connection,
    RULESET_ID
  );

  tx.add(
    createUpdateMintManagerInstruction(
      {
        mintManager: mintManagerId,
        authority: provider.wallet.publicKey,
        payer: provider.wallet.publicKey,
        collector: ruleset.collector,
        ruleset: findRulesetId(RULESET_NAME_2),
      },
      { ix: { authority: newAuthority.publicKey } }
    )
  );
  await executeTransaction(provider.connection, tx, provider.wallet);

  const mintManager = await MintManager.fromAccountAddress(
    provider.connection,
    mintManagerId
  );
  expect(mintManager.mint.toString()).toBe(mintKeypair.publicKey.toString());
  expect(mintManager.authority.toString()).toBe(
    newAuthority.publicKey.toString()
  );
  expect(mintManager.ruleset.toString()).toBe(
    findRulesetId(RULESET_NAME_2).toString()
  );
});

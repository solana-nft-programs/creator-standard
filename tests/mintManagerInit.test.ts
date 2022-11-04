import { test, beforeAll, expect } from "@jest/globals";
import { CardinalProvider, executeTransaction, getProvider } from "../utils";
import { PublicKey } from "@solana/web3.js";
import { Keypair, Transaction } from "@solana/web3.js";

import {
  findMintManagerId,
  createInitMintManagerInstruction,
  MintManager,
  findRulesetId,
  Ruleset,
} from "../sdk";
import { createMintTx } from "./mint";
import { getAssociatedTokenAddressSync } from "@solana/spl-token";

const mintKeypair = Keypair.generate();

const RULESET_NAME = "cardinal-no-check";
const RULESET_ID = findRulesetId(RULESET_NAME);
const RULESET_COLLECTOR = new PublicKey(
  "gmdS6fDgVbeCCYwwvTPJRKM9bFbAgSZh6MTDUT2DcgV"
);

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

test("Init", async () => {
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
      rulesetCollector: RULESET_COLLECTOR,
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

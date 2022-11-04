import { test, expect } from "@jest/globals";
import { CardinalProvider, executeTransaction, getProvider } from "../../utils";
import { Keypair, PublicKey, Transaction } from "@solana/web3.js";

import {
  findMintManagerId,
  MintManager,
  findRulesetId,
  Ruleset,
  createInitializeMintInstruction,
} from "../../sdk";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddressSync,
} from "@solana/spl-token";

const RULESET_NAME = "cardinal-no-check";
const RULESET_ID = findRulesetId(RULESET_NAME);
const RULESET_COLLECTOR = new PublicKey(
  "gmdS6fDgVbeCCYwwvTPJRKM9bFbAgSZh6MTDUT2DcgV"
);

let provider: CardinalProvider;

beforeAll(async () => {
  provider = await getProvider();
});

test("Init", async () => {
  const mintKeypair = Keypair.generate();
  const mint = mintKeypair.publicKey;
  const mintManagerId = findMintManagerId(mint);
  const ruleset = await Ruleset.fromAccountAddress(
    provider.connection,
    RULESET_ID
  );
  console.log(ruleset.collector.toString());

  const tx = new Transaction();
  tx.add(
    createInitializeMintInstruction({
      mintManager: mintManagerId,
      mint: mint,
      ruleset: RULESET_ID,
      targetTokenAccount: getAssociatedTokenAddressSync(
        mintKeypair.publicKey,
        provider.wallet.publicKey
      ),
      target: provider.wallet.publicKey,
      rulesetCollector: ruleset.collector,
      authority: provider.wallet.publicKey,
      payer: provider.wallet.publicKey,
      collector: ruleset.collector,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    })
  );
  await executeTransaction(provider.connection, tx, provider.wallet, [
    mintKeypair,
  ]);

  const mintManager = await MintManager.fromAccountAddress(
    provider.connection,
    mintManagerId
  );
  expect(mintManager.mint.toString()).toBe(mint.toString());
  expect(mintManager.authority.toString()).toBe(
    provider.wallet.publicKey.toString()
  );
  expect(mintManager.ruleset.toString()).toBe(
    findRulesetId(RULESET_NAME).toString()
  );
});

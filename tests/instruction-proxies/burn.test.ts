import { expect, test } from "@jest/globals";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAccount,
  getAssociatedTokenAddressSync,
} from "@solana/spl-token";
import { Keypair, PublicKey, Transaction } from "@solana/web3.js";

import {
  createBurnInstruction,
  createInitializeMintInstruction,
  findMintManagerId,
  findRulesetId,
  MintManager,
  Ruleset,
} from "../../sdk";
import type { CardinalProvider } from "../../utils";
import { executeTransaction, getProvider } from "../../utils";

const mintKeypair = Keypair.generate();
const mint = mintKeypair.publicKey;

const RULESET_NAME = "cardinal-no-check";
const RULESET_ID = findRulesetId(RULESET_NAME);
const RULESET_COLLECTOR = new PublicKey(
  "gmdS6fDgVbeCCYwwvTPJRKM9bFbAgSZh6MTDUT2DcgV"
);

let provider: CardinalProvider;

beforeAll(async () => {
  provider = await getProvider();
});

test("Initialize mint", async () => {
  const mintManagerId = findMintManagerId(mint);
  const ruleset = await Ruleset.fromAccountAddress(
    provider.connection,
    RULESET_ID
  );

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
      rulesetCollector: RULESET_COLLECTOR,
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

test("Burn mint", async () => {
  const mintManagerId = findMintManagerId(mint);
  const tx = new Transaction();
  const holderAtaId = getAssociatedTokenAddressSync(
    mint,
    provider.wallet.publicKey
  );
  const holderAta = await getAccount(provider.connection, holderAtaId);
  expect(holderAta.isFrozen).toBe(true);
  expect(holderAta.mint.toString()).toBe(mint.toString());
  expect(holderAta.amount.toString()).toBe("1");

  tx.add(
    createBurnInstruction({
      mintManager: mintManagerId,
      mint: mintKeypair.publicKey,
      holderTokenAccount: holderAtaId,
      holder: provider.wallet.publicKey,
    })
  );
  await executeTransaction(provider.connection, tx, provider.wallet);

  // const fromAtaCheck = await getAccount(provider.connection, fromAtaId);
  // expect(fromAtaCheck.isFrozen).toBe(true);
  // expect(fromAtaCheck.mint.toString()).toBe(mint.toString());
  // expect(fromAtaCheck.amount.toString()).toBe("1");
  // expect(fromAtaCheck.delegate?.toString()).toBe(delegate.publicKey.toString());
  // expect(fromAtaCheck.delegatedAmount.toString()).toBe("1");
});

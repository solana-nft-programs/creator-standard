import { expect, test } from "@jest/globals";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountInstruction,
  getAccount,
  getAssociatedTokenAddressSync,
  getMint,
} from "@solana/spl-token";
import { Keypair, PublicKey, Transaction } from "@solana/web3.js";

import { Ruleset } from "../../sdk";
import { MintManager } from "../../sdk/generated/accounts/MintManager";
import { createInitializeMintInstruction } from "../../sdk/generated/instructions/InitializeMint";
import { findMintManagerId, findRulesetId } from "../../sdk/pda";
import type { CardinalProvider } from "../../utils";
import { executeTransaction, getProvider, tryGetAccount } from "../../utils";

const mintKeypair = Keypair.generate();

const RULESET_NAME = "ruleset-no-checks";
const RULESET_ID = findRulesetId(RULESET_NAME);
const RULESET_COLLECTOR = new PublicKey(
  "gmdS6fDgVbeCCYwwvTPJRKM9bFbAgSZh6MTDUT2DcgV"
);
const user = Keypair.generate();

let provider: CardinalProvider;

beforeAll(async () => {
  provider = await getProvider();
});

test("Initialize mint", async () => {
  const mintManagerId = findMintManagerId(mintKeypair.publicKey);
  const ruleset = await Ruleset.fromAccountAddress(
    provider.connection,
    RULESET_ID
  );
  const targetTokenAccount = getAssociatedTokenAddressSync(
    mintKeypair.publicKey,
    provider.wallet.publicKey
  );

  const tx = new Transaction();
  tx.add(
    createInitializeMintInstruction({
      mintManager: mintManagerId,
      mint: mintKeypair.publicKey,
      ruleset: RULESET_ID,
      targetTokenAccount: targetTokenAccount,
      target: provider.wallet.publicKey,
      rulesetCollector: ruleset.collector,
      authority: provider.wallet.publicKey,
      payer: provider.wallet.publicKey,
      collector: RULESET_COLLECTOR,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    })
  );
  await executeTransaction(provider.connection, tx, provider.wallet, [
    mintKeypair,
  ]);

  // check mint
  const mintInfo = await tryGetAccount(() =>
    getMint(provider.connection, mintKeypair.publicKey)
  );
  expect(mintInfo).not.toBeNull();
  expect(mintInfo?.isInitialized).toBeTruthy();
  expect(mintInfo?.supply.toString()).toBe("1");
  expect(mintInfo?.decimals.toString()).toBe("0");
  expect(mintInfo?.freezeAuthority?.toString()).toBe(mintManagerId.toString());
  expect(mintInfo?.mintAuthority?.toString()).toBe(mintManagerId.toString());

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

test("Create token account for a user", async () => {
  const tx = new Transaction();

  const userAtaId = getAssociatedTokenAddressSync(
    mintKeypair.publicKey,
    user.publicKey
  );

  tx.add(
    createAssociatedTokenAccountInstruction(
      provider.wallet.publicKey,
      userAtaId,
      user.publicKey,
      mintKeypair.publicKey
    )
  );
  await executeTransaction(provider.connection, tx, provider.wallet);

  const closedTokenAccount = await tryGetAccount(() =>
    getAccount(provider.connection, userAtaId)
  );
  expect(closedTokenAccount?.isInitialized).toBeTruthy();
  expect(closedTokenAccount?.mint.toString()).toBe(
    mintKeypair.publicKey.toString()
  );
  expect(closedTokenAccount?.isFrozen).toBeFalsy();
  expect(closedTokenAccount?.owner.toString()).toBe(user.publicKey.toString());
});

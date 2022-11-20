import { expect, test } from "@jest/globals";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAccount,
  getAssociatedTokenAddressSync,
  getMint,
} from "@solana/spl-token";
import { Keypair, Transaction } from "@solana/web3.js";

import { handleRemainingAccountsForRuleset, Ruleset } from "../../sdk";
import { MintManager } from "../../sdk/generated/accounts/MintManager";
import { createApproveInstruction } from "../../sdk/generated/instructions/Approve";
import { createInitializeMintInstruction } from "../../sdk/generated/instructions/InitializeMint";
import {
  DEFAULT_COLLECTOR,
  findMintManagerId,
  findRulesetId,
} from "../../sdk/pda";
import type { CardinalProvider } from "../../utils";
import {
  executeTransaction,
  getProvider,
  newAccountWithLamports,
  tryGetAccount,
} from "../../utils";

const mintKeypair = Keypair.generate();

const RULESET_NAME = "ruleset-no-checks";
const RULESET_ID = findRulesetId(RULESET_NAME);

let provider: CardinalProvider;
let delegate: Keypair;

beforeAll(async () => {
  provider = await getProvider();
  delegate = await newAccountWithLamports(provider.connection);
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
      collector: DEFAULT_COLLECTOR,
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

test("Delegate", async () => {
  const rulesetData = await Ruleset.fromAccountAddress(
    provider.connection,
    RULESET_ID
  );
  const mintManagerId = findMintManagerId(mintKeypair.publicKey);
  const tx = new Transaction();
  const holderAtaId = getAssociatedTokenAddressSync(
    mintKeypair.publicKey,
    provider.wallet.publicKey
  );
  const holderAta = await getAccount(provider.connection, holderAtaId);
  expect(holderAta.isFrozen).toBe(true);
  expect(holderAta.mint.toString()).toBe(mintKeypair.publicKey.toString());
  expect(holderAta.amount.toString()).toBe("1");

  const ix = createApproveInstruction(
    {
      mintManager: mintManagerId,
      ruleset: RULESET_ID,
      mint: mintKeypair.publicKey,
      holderTokenAccount: holderAtaId,
      holder: provider.wallet.publicKey,
      delegate: delegate.publicKey,
    },
    { approveIx: { amount: 1 } }
  );
  await handleRemainingAccountsForRuleset(ix, rulesetData);
  tx.add(ix);
  await executeTransaction(provider.connection, tx, provider.wallet);

  const holderAtaCheck = await getAccount(provider.connection, holderAtaId);
  expect(holderAtaCheck.isFrozen).toBe(true);
  expect(holderAtaCheck.mint.toString()).toBe(mintKeypair.publicKey.toString());
  expect(holderAtaCheck.amount.toString()).toBe("1");
  expect(holderAtaCheck.delegate?.toString()).toBe(
    delegate.publicKey.toString()
  );
  expect(holderAtaCheck.delegatedAmount.toString()).toBe("1");
});

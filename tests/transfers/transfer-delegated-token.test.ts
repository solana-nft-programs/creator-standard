import { expect, test } from "@jest/globals";
import { Wallet } from "@project-serum/anchor";
import {
  createAssociatedTokenAccountInstruction,
  getAccount,
  getAssociatedTokenAddressSync,
  getMint,
} from "@solana/spl-token";
import {
  Keypair,
  SYSVAR_INSTRUCTIONS_PUBKEY,
  Transaction,
} from "@solana/web3.js";

import { handleRemainingAccountsForRuleset, Ruleset } from "../../sdk";
import { MintManager } from "../../sdk/generated/accounts/MintManager";
import { createApproveInstruction } from "../../sdk/generated/instructions/Approve";
import { createTransferInstruction } from "../../sdk/generated/instructions/Transfer";
import {
  findMintManagerId,
  findMintMetadataId,
  findRulesetId,
} from "../../sdk/pda";
import type { CardinalProvider } from "../../utils";
import {
  createCCSMintTx,
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

  const tx = await createCCSMintTx(
    provider.connection,
    mintKeypair.publicKey,
    provider.wallet.publicKey,
    RULESET_ID
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
  const fromAtaId = getAssociatedTokenAddressSync(
    mintKeypair.publicKey,
    provider.wallet.publicKey
  );
  const fromAta = await getAccount(provider.connection, fromAtaId);
  expect(fromAta.isFrozen).toBe(true);
  expect(fromAta.mint.toString()).toBe(mintKeypair.publicKey.toString());
  expect(fromAta.amount.toString()).toBe("1");

  const ix = createApproveInstruction(
    {
      mintManager: mintManagerId,
      ruleset: RULESET_ID,
      mint: mintKeypair.publicKey,
      holderTokenAccount: fromAtaId,
      holder: provider.wallet.publicKey,
      delegate: delegate.publicKey,
    },
    { approveIx: { amount: 1 } }
  );
  handleRemainingAccountsForRuleset(ix, rulesetData);
  tx.add(ix);
  await executeTransaction(provider.connection, tx, provider.wallet);

  const fromAtaCheck = await getAccount(provider.connection, fromAtaId);
  expect(fromAtaCheck.isFrozen).toBe(true);
  expect(fromAtaCheck.mint.toString()).toBe(mintKeypair.publicKey.toString());
  expect(fromAtaCheck.amount.toString()).toBe("1");
  expect(fromAtaCheck.delegate?.toString()).toBe(delegate.publicKey.toString());
  expect(fromAtaCheck.delegatedAmount.toString()).toBe("1");
});

test("Transfer", async () => {
  const rulesetData = await Ruleset.fromAccountAddress(
    provider.connection,
    RULESET_ID
  );
  const mintManagerId = findMintManagerId(mintKeypair.publicKey);
  const mintMetadataId = findMintMetadataId(mintKeypair.publicKey);
  const tx = new Transaction();
  const recipient = Keypair.generate();
  const fromAtaId = getAssociatedTokenAddressSync(
    mintKeypair.publicKey,
    provider.wallet.publicKey
  );
  const toAtaId = getAssociatedTokenAddressSync(
    mintKeypair.publicKey,
    recipient.publicKey
  );
  const fromAta = await getAccount(provider.connection, fromAtaId);

  expect(fromAta.isFrozen).toBe(true);
  expect(fromAta.mint.toString()).toBe(mintKeypair.publicKey.toString());
  expect(fromAta.amount.toString()).toBe("1");

  const ix = createTransferInstruction({
    mintManager: mintManagerId,
    mintMetadata: mintMetadataId,
    ruleset: RULESET_ID,
    mint: mintKeypair.publicKey,
    from: fromAtaId,
    to: toAtaId,
    authority: delegate.publicKey,
    instructions: SYSVAR_INSTRUCTIONS_PUBKEY,
  });
  tx.add(
    createAssociatedTokenAccountInstruction(
      delegate.publicKey,
      toAtaId,
      recipient.publicKey,
      mintKeypair.publicKey
    )
  );
  handleRemainingAccountsForRuleset(ix, rulesetData);
  tx.add(ix);
  await executeTransaction(provider.connection, tx, new Wallet(delegate));

  const fromAtaCheck = await getAccount(provider.connection, fromAtaId);
  expect(fromAtaCheck.isFrozen).toBe(false);
  expect(fromAtaCheck.mint.toString()).toBe(mintKeypair.publicKey.toString());
  expect(fromAtaCheck.amount.toString()).toBe("0");

  const toAtaCheck = await getAccount(provider.connection, toAtaId);
  expect(toAtaCheck.owner.toString()).toBe(recipient.publicKey.toString());
  expect(toAtaCheck.isFrozen).toBe(true);
  expect(toAtaCheck.mint.toString()).toBe(mintKeypair.publicKey.toString());
  expect(toAtaCheck.amount.toString()).toBe("1");
  expect(toAtaCheck.delegate).toBeNull();
});

import { expect, test } from "@jest/globals";
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
import { createSetInUseByInstruction } from "../../sdk/generated/instructions/SetInUseBy";
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
  tryGetAccount,
} from "../../utils";

const mintKeypair = Keypair.generate();

const RULESET_NAME = "ruleset-no-checks";
const RULESET_ID = findRulesetId(RULESET_NAME);
const inUseByAddress = Keypair.generate();

let provider: CardinalProvider;

beforeAll(async () => {
  provider = await getProvider();
});

test("Init", async () => {
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

test("Set in use by", async () => {
  const rulesetData = await Ruleset.fromAccountAddress(
    provider.connection,
    RULESET_ID
  );
  const mintManagerId = findMintManagerId(mintKeypair.publicKey);
  const holderAtaId = getAssociatedTokenAddressSync(
    mintKeypair.publicKey,
    provider.wallet.publicKey
  );

  const tx = new Transaction();
  const ix = createSetInUseByInstruction({
    mintManager: mintManagerId,
    ruleset: RULESET_ID,
    inUseByAddress: inUseByAddress.publicKey,
    holder: provider.wallet.publicKey,
    holderTokenAccount: holderAtaId,
  });
  handleRemainingAccountsForRuleset(ix, rulesetData);
  tx.add(ix);
  await executeTransaction(provider.connection, tx, provider.wallet);

  // check mint manager
  const mintManager = await MintManager.fromAccountAddress(
    provider.connection,
    mintManagerId
  );
  expect(mintManager.mint.toString()).toBe(mintKeypair.publicKey.toString());
  expect(mintManager.inUseBy?.toString()).toBe(
    inUseByAddress.publicKey.toString()
  );
  expect(mintManager.authority.toString()).toBe(
    provider.wallet.publicKey.toString()
  );
  expect(mintManager.ruleset.toString()).toBe(
    findRulesetId(RULESET_NAME).toString()
  );
});

test("Transfer", async () => {
  const rulesetData = await Ruleset.fromAccountAddress(
    provider.connection,
    RULESET_ID
  );
  const mintManagerId = findMintManagerId(mintKeypair.publicKey);
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
  tx.add(
    createAssociatedTokenAccountInstruction(
      provider.wallet.publicKey,
      toAtaId,
      recipient.publicKey,
      mintKeypair.publicKey
    )
  );
  const ix = createTransferInstruction({
    mintManager: mintManagerId,
    mintMetadata: findMintMetadataId(mintKeypair.publicKey),
    ruleset: RULESET_ID,
    mint: mintKeypair.publicKey,
    from: fromAtaId,
    to: toAtaId,
    authority: provider.wallet.publicKey,
    instructions: SYSVAR_INSTRUCTIONS_PUBKEY,
  });
  handleRemainingAccountsForRuleset(ix, rulesetData);
  tx.add(ix);
  await expect(
    executeTransaction(provider.connection, tx, provider.wallet)
  ).rejects.toThrow();
});

import { beforeAll, expect, test } from "@jest/globals";
import { Wallet } from "@project-serum/anchor";
import { getAssociatedTokenAddressSync, getMint } from "@solana/spl-token";
import { Keypair, LAMPORTS_PER_SOL, Transaction } from "@solana/web3.js";

import { handleRemainingAccountsForRuleset, Ruleset } from "../../sdk";
import { MintManager } from "../../sdk/generated/accounts/MintManager";
import { createRemoveInUseByInstruction } from "../../sdk/generated/instructions/RemoveInUseBy";
import { createSetInUseByInstruction } from "../../sdk/generated/instructions/SetInUseBy";
import { findMintManagerId, findRulesetId } from "../../sdk/pda";
import type { SolanaProvider } from "../../utils";
import {
  createCCSMintTx,
  executeTransaction,
  getProvider,
  tryGetAccount,
} from "../../utils";

const mintKeypair = Keypair.generate();

const RULESET_ID = findRulesetId();
const inUseByAddress = Keypair.generate();

let provider: SolanaProvider;

beforeAll(async () => {
  provider = await getProvider();
  const signature = await provider.connection.requestAirdrop(
    inUseByAddress.publicKey,
    LAMPORTS_PER_SOL,
  );
  await provider.connection.confirmTransaction(signature, "confirmed");
});

test("Initialize mint", async () => {
  const mintManagerId = findMintManagerId(mintKeypair.publicKey);
  const tx = await createCCSMintTx(
    provider.connection,
    mintKeypair.publicKey,
    provider.wallet.publicKey,
    RULESET_ID,
  );
  await executeTransaction(provider.connection, tx, provider.wallet, [
    mintKeypair,
  ]);

  // check mint
  const mintInfo = await tryGetAccount(() =>
    getMint(provider.connection, mintKeypair.publicKey),
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
    mintManagerId,
  );
  expect(mintManager.mint.toString()).toBe(mintKeypair.publicKey.toString());
  expect(mintManager.authority.toString()).toBe(
    provider.wallet.publicKey.toString(),
  );
  expect(mintManager.ruleset.toString()).toBe(RULESET_ID.toString());
});

test("Set in use by", async () => {
  const rulesetData = await Ruleset.fromAccountAddress(
    provider.connection,
    RULESET_ID,
  );
  const mintManagerId = findMintManagerId(mintKeypair.publicKey);
  const holderAtaId = getAssociatedTokenAddressSync(
    mintKeypair.publicKey,
    provider.wallet.publicKey,
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
    mintManagerId,
  );
  expect(mintManager.mint.toString()).toBe(mintKeypair.publicKey.toString());
  expect(mintManager.inUseBy?.toString()).toBe(
    inUseByAddress.publicKey.toString(),
  );
  expect(mintManager.authority.toString()).toBe(
    provider.wallet.publicKey.toString(),
  );
  expect(mintManager.ruleset.toString()).toBe(RULESET_ID.toString());
});

test("Remove in use by", async () => {
  const mintManagerId = findMintManagerId(mintKeypair.publicKey);

  const tx = new Transaction();
  tx.add(
    createRemoveInUseByInstruction({
      mintManager: mintManagerId,
      user: inUseByAddress.publicKey,
    }),
  );
  await executeTransaction(provider.connection, tx, new Wallet(inUseByAddress));

  // check mint manager
  const mintManager = await MintManager.fromAccountAddress(
    provider.connection,
    mintManagerId,
  );
  expect(mintManager.mint.toString()).toBe(mintKeypair.publicKey.toString());
  expect(mintManager.inUseBy).toBeNull();
  expect(mintManager.authority.toString()).toBe(
    provider.wallet.publicKey.toString(),
  );
  expect(mintManager.ruleset.toString()).toBe(RULESET_ID.toString());
});

import { expect, test } from "@jest/globals";
import { Wallet } from "@project-serum/anchor";
import {
  createAssociatedTokenAccountInstruction,
  getAccount,
  getAssociatedTokenAddressSync,
  getMint,
} from "@solana/spl-token";
import { Keypair, LAMPORTS_PER_SOL, Transaction } from "@solana/web3.js";

import { MintManager } from "../../sdk/generated/accounts/MintManager";
import { createCloseInstruction } from "../../sdk/generated/instructions/Close";
import { findMintManagerId, findRulesetId } from "../../sdk/pda";
import type { CardinalProvider } from "../../utils";
import {
  createCCSMintTx,
  executeTransaction,
  getProvider,
  tryGetAccount,
} from "../../utils";

const mintKeypair = Keypair.generate();

const RULESET_ID = findRulesetId();
const user = Keypair.generate();

let provider: CardinalProvider;

beforeAll(async () => {
  provider = await getProvider();
  const signature = await provider.connection.requestAirdrop(
    user.publicKey,
    LAMPORTS_PER_SOL
  );
  await provider.connection.confirmTransaction(signature, "confirmed");
});

test("Init mint manager", async () => {
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
  expect(mintManager.ruleset.toString()).toBe(RULESET_ID.toString());
});

test("Attempt to close but account is non-empty", async () => {
  const mintManagerId = findMintManagerId(mintKeypair.publicKey);
  const tx = new Transaction();
  const tokenAtaId = getAssociatedTokenAddressSync(
    mintKeypair.publicKey,
    provider.wallet.publicKey
  );
  const tokenAta = await getAccount(provider.connection, tokenAtaId);
  expect(tokenAta.isFrozen).toBe(true);
  expect(tokenAta.mint.toString()).toBe(mintKeypair.publicKey.toString());
  expect(tokenAta.amount.toString()).toBe("1");

  tx.add(
    createCloseInstruction({
      mintManager: mintManagerId,
      mint: mintKeypair.publicKey,
      tokenAccount: tokenAtaId,
      owner: provider.wallet.publicKey,
    })
  );

  await expect(
    executeTransaction(provider.connection, tx, provider.wallet)
  ).rejects.toThrow();
});

test("Create empty mint token account for a user", async () => {
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

  const createdTokenAccount = await tryGetAccount(() =>
    getAccount(provider.connection, userAtaId)
  );
  expect(createdTokenAccount?.isInitialized).toBeTruthy();
  expect(createdTokenAccount?.mint.toString()).toBe(
    mintKeypair.publicKey.toString()
  );
  expect(createdTokenAccount?.isFrozen).toBeFalsy();
  expect(createdTokenAccount?.owner.toString()).toBe(user.publicKey.toString());
});

test("Close token account", async () => {
  const tx = new Transaction();
  const mintManagerId = findMintManagerId(mintKeypair.publicKey);

  const userAtaId = getAssociatedTokenAddressSync(
    mintKeypair.publicKey,
    user.publicKey
  );
  const holderAta = await getAccount(provider.connection, userAtaId);
  expect(holderAta.isFrozen).toBe(false);
  expect(holderAta.mint.toString()).toBe(mintKeypair.publicKey.toString());
  expect(holderAta.amount.toString()).toBe("0");

  // close empty account
  tx.add(
    createCloseInstruction({
      mintManager: mintManagerId,
      mint: mintKeypair.publicKey,
      tokenAccount: userAtaId,
      owner: user.publicKey,
    })
  );
  await executeTransaction(provider.connection, tx, new Wallet(user));

  const closedTokenAccount = await tryGetAccount(() =>
    getAccount(provider.connection, userAtaId)
  );
  expect(closedTokenAccount).toBeNull();
});

import { expect, test } from "@jest/globals";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountInstruction,
  getAccount,
  getAssociatedTokenAddressSync,
  getMint,
} from "@solana/spl-token";
import {
  Keypair,
  PublicKey,
  SYSVAR_INSTRUCTIONS_PUBKEY,
  Transaction,
} from "@solana/web3.js";

import {
  createInitializeMintInstruction,
  createSetInUseByInstruction,
  createTransferInstruction,
  Ruleset,
} from "../../sdk";
import { MintManager } from "../../sdk/generated/accounts/MintManager";
import { findMintManagerId, findRulesetId } from "../../sdk/pda";
import type { CardinalProvider } from "../../utils";
import { executeTransaction, getProvider, tryGetAccount } from "../../utils";

const mintKeypair = Keypair.generate();

const RULESET_NAME = "ruleset-no-checks";
const RULESET_ID = findRulesetId(RULESET_NAME);
const RULESET_COLLECTOR = new PublicKey(
  "gmdS6fDgVbeCCYwwvTPJRKM9bFbAgSZh6MTDUT2DcgV"
);
const IN_USE_BY_AUTHORITY = Keypair.generate();

let provider: CardinalProvider;

beforeAll(async () => {
  provider = await getProvider();
});

test("Init", async () => {
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
  const mintManagerId = findMintManagerId(mintKeypair.publicKey);
  const holderAtaId = getAssociatedTokenAddressSync(
    mintKeypair.publicKey,
    provider.wallet.publicKey
  );

  const tx = new Transaction();
  tx.add(
    createSetInUseByInstruction(
      {
        mintManager: mintManagerId,
        holder: provider.wallet.publicKey,
        holderTokenAccount: holderAtaId,
      },
      {
        ix: {
          inUseBy: IN_USE_BY_AUTHORITY.publicKey,
        },
      }
    )
  );
  await executeTransaction(provider.connection, tx, provider.wallet);

  // check mint manager
  const mintManager = await MintManager.fromAccountAddress(
    provider.connection,
    mintManagerId
  );
  expect(mintManager.mint.toString()).toBe(mintKeypair.publicKey.toString());
  expect(mintManager.inUseBy?.toString()).toBe(
    IN_USE_BY_AUTHORITY.publicKey.toString()
  );
  expect(mintManager.authority.toString()).toBe(
    provider.wallet.publicKey.toString()
  );
  expect(mintManager.ruleset.toString()).toBe(
    findRulesetId(RULESET_NAME).toString()
  );
});

test("Transfer", async () => {
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
    ),
    createTransferInstruction({
      mintManager: mintManagerId,
      ruleset: RULESET_ID,
      mint: mintKeypair.publicKey,
      from: fromAtaId,
      to: toAtaId,
      authority: provider.wallet.publicKey,
      instructions: SYSVAR_INSTRUCTIONS_PUBKEY,
    })
  );
  await expect(
    executeTransaction(provider.connection, tx, provider.wallet)
  ).rejects.toThrow();
});

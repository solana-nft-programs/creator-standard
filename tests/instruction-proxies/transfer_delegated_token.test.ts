import { test, expect } from "@jest/globals";
import {
  CardinalProvider,
  executeTransaction,
  getProvider,
  newAccountWithLamports,
} from "../../utils";
import {
  Keypair,
  PublicKey,
  SYSVAR_INSTRUCTIONS_PUBKEY,
  Transaction,
} from "@solana/web3.js";

import {
  findMintManagerId,
  MintManager,
  findRulesetId,
  Ruleset,
  createApproveInstruction,
  createInitializeMintInstruction,
  createTransferInstruction,
} from "../../sdk";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAccount,
  getAssociatedTokenAddressSync,
  createAssociatedTokenAccountInstruction,
} from "@solana/spl-token";
import { Wallet } from "@project-serum/anchor";

const mintKeypair = Keypair.generate();
const mint = mintKeypair.publicKey;

const RULESET_NAME = "cardinal-no-check";
const RULESET_ID = findRulesetId(RULESET_NAME);
const RULESET_COLLECTOR = new PublicKey(
  "gmdS6fDgVbeCCYwwvTPJRKM9bFbAgSZh6MTDUT2DcgV"
);

let provider: CardinalProvider;
let delegate: Keypair;

beforeAll(async () => {
  provider = await getProvider();
  delegate = await newAccountWithLamports(provider.connection);
});

test("Init mint manager", async () => {
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

test("Delegate", async () => {
  const mintManagerId = findMintManagerId(mint);
  const tx = new Transaction();
  const fromAtaId = getAssociatedTokenAddressSync(
    mint,
    provider.wallet.publicKey
  );
  const fromAta = await getAccount(provider.connection, fromAtaId);
  expect(fromAta.isFrozen).toBe(true);
  expect(fromAta.mint.toString()).toBe(mint.toString());
  expect(fromAta.amount.toString()).toBe("1");

  tx.add(
    createApproveInstruction({
      mintManager: mintManagerId,
      mint: mint,
      holderTokenAccount: fromAtaId,
      holder: provider.wallet.publicKey,
      delegate: delegate.publicKey,
    })
  );
  await executeTransaction(provider.connection, tx, provider.wallet);

  const fromAtaCheck = await getAccount(provider.connection, fromAtaId);
  expect(fromAtaCheck.isFrozen).toBe(true);
  expect(fromAtaCheck.mint.toString()).toBe(mint.toString());
  expect(fromAtaCheck.amount.toString()).toBe("1");
  expect(fromAtaCheck.delegate?.toString()).toBe(delegate.publicKey.toString());
  expect(fromAtaCheck.delegatedAmount.toString()).toBe("1");
});

test("Transfer", async () => {
  const mintManagerId = findMintManagerId(mint);
  const tx = new Transaction();
  const recipient = Keypair.generate();
  const fromAtaId = getAssociatedTokenAddressSync(
    mint,
    provider.wallet.publicKey
  );
  const toAtaId = getAssociatedTokenAddressSync(mint, recipient.publicKey);
  const fromAta = await getAccount(provider.connection, fromAtaId);

  expect(fromAta.isFrozen).toBe(true);
  expect(fromAta.mint.toString()).toBe(mint.toString());
  expect(fromAta.amount.toString()).toBe("1");

  tx.add(
    createAssociatedTokenAccountInstruction(
      delegate.publicKey,
      toAtaId,
      recipient.publicKey,
      mint
    ),
    createTransferInstruction({
      mintManager: mintManagerId,
      ruleset: RULESET_ID,
      mint: mint,
      from: fromAtaId,
      to: toAtaId,
      authority: delegate.publicKey,
      instructions: SYSVAR_INSTRUCTIONS_PUBKEY,
    })
  );
  await executeTransaction(provider.connection, tx, new Wallet(delegate));

  const fromAtaCheck = await getAccount(provider.connection, fromAtaId);
  expect(fromAtaCheck.isFrozen).toBe(false);
  expect(fromAtaCheck.mint.toString()).toBe(mint.toString());
  expect(fromAtaCheck.amount.toString()).toBe("0");

  const toAtaCheck = await getAccount(provider.connection, toAtaId);
  expect(toAtaCheck.isFrozen).toBe(true);
  expect(toAtaCheck.mint.toString()).toBe(mint.toString());
  expect(toAtaCheck.amount.toString()).toBe("1");
});

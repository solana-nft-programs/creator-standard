import { test, expect } from "@jest/globals";
import { CardinalProvider, executeTransaction, getProvider } from "../../utils";
import { PublicKey, SYSVAR_INSTRUCTIONS_PUBKEY } from "@solana/web3.js";
import { Keypair, Transaction } from "@solana/web3.js";

import {
  findMintManagerId,
  MintManager,
  createInitRulesetInstruction,
  findRulesetId,
  Ruleset,
  createTransferInstruction,
  createInitializeMintInstruction,
} from "../../sdk";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountInstruction,
  getAccount,
  getAssociatedTokenAddressSync,
} from "@solana/spl-token";

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
});

test("Init", async () => {
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
      provider.wallet.publicKey,
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
      authority: provider.wallet.publicKey,
      instructions: SYSVAR_INSTRUCTIONS_PUBKEY,
    })
  );
  await executeTransaction(provider.connection, tx, provider.wallet);
  await expect(async () => {
    await getAccount(provider.connection, fromAtaId);
  }).rejects.toThrow();

  const toAtaCheck = await getAccount(provider.connection, toAtaId);
  expect(toAtaCheck.isFrozen).toBe(true);
  expect(toAtaCheck.mint.toString()).toBe(mint.toString());
  expect(toAtaCheck.amount.toString()).toBe("1");
});

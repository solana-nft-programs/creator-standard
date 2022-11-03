import { test, expect } from "@jest/globals";
import { CardinalProvider, executeTransaction, getProvider } from "./utils";
import { SYSVAR_INSTRUCTIONS_PUBKEY } from "@solana/web3.js";
import { Keypair, Transaction } from "@solana/web3.js";

import {
  findMintManagerId,
  MintManager,
  createInitRulesetInstruction,
  findRulesetId,
  Ruleset,
  createTransferInstruction,
  createInitMintInstruction,
} from "../sdk";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountInstruction,
  getAccount,
  getAssociatedTokenAddressSync,
} from "@solana/spl-token";
const mintKeypair = Keypair.generate();
const mint = mintKeypair.publicKey;

const RULESET_NAME = `global-${Math.random()}`;
const RULESET_ID = findRulesetId(RULESET_NAME);
let provider: CardinalProvider;

beforeAll(async () => {
  provider = await getProvider();
});

test("Create ruleset", async () => {
  const tx = new Transaction();
  tx.add(
    createInitRulesetInstruction(
      {
        ruleset: RULESET_ID,
        authority: provider.wallet.publicKey,
        payer: provider.wallet.publicKey,
      },
      {
        ix: {
          name: RULESET_NAME,
          collector: provider.wallet.publicKey,
          checkSellerFeeBasisPoints: false,
          disallowedAddresses: [],
          allowedPrograms: [],
        },
      }
    )
  );
  await executeTransaction(provider.connection, tx, provider.wallet);
  const ruleset = await Ruleset.fromAccountAddress(
    provider.connection,
    RULESET_ID
  );
  expect(ruleset.authority.toString()).toBe(
    provider.wallet.publicKey.toString()
  );
  expect(ruleset.checkSellerFeeBasisPoints).toBe(false);
  expect(ruleset.disallowedAddresses.length).toBe(0);
  expect(ruleset.allowedPrograms.length).toBe(0);
});

test("Init", async () => {
  const mintManagerId = findMintManagerId(mint);
  const ruleset = await Ruleset.fromAccountAddress(
    provider.connection,
    RULESET_ID
  );

  const tx = new Transaction();
  tx.add(
    createInitMintInstruction({
      mintManager: mintManagerId,
      mint: mint,
      ruleset: RULESET_ID,
      targetTokenAccount: getAssociatedTokenAddressSync(
        mintKeypair.publicKey,
        provider.wallet.publicKey
      ),
      target: provider.wallet.publicKey,
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
  const fromAtaCheck = await getAccount(provider.connection, fromAtaId);
  expect(fromAtaCheck.isFrozen).toBe(false);
  expect(fromAtaCheck.mint.toString()).toBe(mint.toString());
  expect(fromAtaCheck.amount.toString()).toBe("0");

  const toAtaCheck = await getAccount(provider.connection, toAtaId);
  expect(toAtaCheck.isFrozen).toBe(true);
  expect(toAtaCheck.mint.toString()).toBe(mint.toString());
  expect(toAtaCheck.amount.toString()).toBe("1");
});

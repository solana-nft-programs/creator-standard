import { test, beforeAll, expect } from "@jest/globals";
import {
  CardinalProvider,
  executeTransaction,
  getProvider,
} from "../src/utils";
import { PublicKey } from "@solana/web3.js";
import { Keypair, Transaction } from "@solana/web3.js";

import {
  createInitRulesetInstruction,
  findRulesetId,
  Ruleset,
  createUpdateRulesetInstruction,
} from "../src";
import { createMintTx } from "./utils";
let mint: PublicKey;

const RULESET_NAME = `global-${Math.random()}`;
const RULESET_ID = findRulesetId(RULESET_NAME);
let provider: CardinalProvider;

beforeAll(async () => {
  provider = await getProvider();
  const mintKeypair = Keypair.generate();
  mint = mintKeypair.publicKey;
  executeTransaction(
    provider.connection,
    await createMintTx(provider.connection, mint, provider.wallet.publicKey),
    provider.wallet,
    [mintKeypair]
  );
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
          checkSellerFeeBasisPoints: true,
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
  expect(ruleset.checkSellerFeeBasisPoints).toBe(true);
  expect(ruleset.disallowedAddresses.length).toBe(0);
  expect(ruleset.allowedPrograms.length).toBe(0);
});

test("Update ruleset", async () => {
  const tx = new Transaction();
  tx.add(
    createUpdateRulesetInstruction(
      {
        ruleset: RULESET_ID,
        authority: provider.wallet.publicKey,
      },
      {
        ix: {
          authority: provider.wallet.publicKey,
          collector: provider.wallet.publicKey,
          checkSellerFeeBasisPoints: true,
          disallowedAddresses: [provider.wallet.publicKey],
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
  expect(ruleset.checkSellerFeeBasisPoints).toBe(true);
  expect(ruleset.disallowedAddresses.length).toBe(1);
  expect(ruleset.allowedPrograms.length).toBe(0);
});

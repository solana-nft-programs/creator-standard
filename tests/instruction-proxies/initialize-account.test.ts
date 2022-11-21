import { expect, test } from "@jest/globals";
import {
  createAssociatedTokenAccountInstruction,
  getAccount,
  getAssociatedTokenAddressSync,
  getMint,
} from "@solana/spl-token";
import { Keypair, Transaction } from "@solana/web3.js";

import type { CardinalProvider } from "../../utils";
import {
  createMintTx,
  executeTransaction,
  getProvider,
  tryGetAccount,
} from "../../utils";

const mintKeypair = Keypair.generate();
const user = Keypair.generate();

let provider: CardinalProvider;

beforeAll(async () => {
  provider = await getProvider();
});

test("Initialize mint", async () => {
  const tx = await createMintTx(
    provider.connection,
    mintKeypair.publicKey,
    provider.wallet.publicKey
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
  expect(mintInfo?.freezeAuthority?.toString()).toBe(
    provider.wallet.publicKey.toString()
  );
  expect(mintInfo?.mintAuthority?.toString()).toBe(
    provider.wallet.publicKey.toString()
  );
});

test("Create token account for a user", async () => {
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

  const closedTokenAccount = await tryGetAccount(() =>
    getAccount(provider.connection, userAtaId)
  );
  expect(closedTokenAccount?.isInitialized).toBeTruthy();
  expect(closedTokenAccount?.mint.toString()).toBe(
    mintKeypair.publicKey.toString()
  );
  expect(closedTokenAccount?.isFrozen).toBeFalsy();
  expect(closedTokenAccount?.owner.toString()).toBe(user.publicKey.toString());
});

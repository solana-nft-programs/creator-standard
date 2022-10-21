import { test, beforeAll, expect } from "@jest/globals";
import { getProvider } from "../utils";
import type { PublicKey } from "@solana/web3.js";
import { Keypair } from "@solana/web3.js";
import { createMint } from "@solana/spl-token";
import { CreatorStandard } from "../src";

let mint: PublicKey;

beforeAll(async () => {
  const provider = await getProvider();
  const mintKeypair = Keypair.generate();
  mint = await createMint(
    provider.connection,
    provider.keypair,
    provider.wallet.publicKey,
    provider.wallet.publicKey,
    0,
    mintKeypair
  );
});

test("Init", async () => {
  await CreatorStandard.methods
    .initMintManager()
    .accounts({
      mint: mint,
    })
    .rpc();
});

import { findAta } from "@cardinal/common";
import {
  CreateMetadataV2,
  Creator,
  DataV2,
  Metadata,
} from "@metaplex-foundation/mpl-token-metadata";
import * as anchor from "@project-serum/anchor";
import { Keypair, PublicKey } from "@solana/web3.js";
import dotenv from "dotenv";
import fetch from "node-fetch";

import {
  createInitMintManagerInstruction,
  DEFAULT_MINIMUM_CREATOR_SHARE,
  DEFAULT_REQUIRED_CREATOR,
} from "../sdk";
import {
  findMintManagerId,
  findMintMetadataId,
  findRulesetId,
} from "../sdk/pda";
import { connectionFor, createMintTx, executeTransaction } from "../utils";

dotenv.config();

const DEFAULT_RULESET = "";

export type CreateCSSTokenParams = {
  rulesetName?: string;
  target?: PublicKey;
};

const wallet = Keypair.fromSecretKey(
  anchor.utils.bytes.bs58.decode(process.env.TEST_WALLET || "")
); // your wallet's secret key // your wallet's secret key

const main = async (params: CreateCSSTokenParams, cluster = "devnet") => {
  const connection = connectionFor(cluster);

  const mintKeypair = Keypair.generate();
  const rulesetId = findRulesetId(params.rulesetName ?? DEFAULT_RULESET);
  const mintManagerId = findMintManagerId(mintKeypair.publicKey);
  const holdetAta = await findAta(
    mintKeypair.publicKey,
    params.target || wallet.publicKey,
    true
  );

  // init mint
  const tx = await createMintTx(
    connection,
    mintKeypair.publicKey,
    wallet.publicKey,
    params.target
  );

  // metadata
  const metadataName = `TEST - ${new Date()
    .toLocaleString("en-gb", {
      day: "2-digit",
      month: "2-digit",
      year: "2-digit",
      hour: "2-digit",
      minute: "2-digit",
    })
    .toUpperCase()}`;
  const response = await fetch("https://picsum.photos/400", {
    method: "GET",
    redirect: "follow",
  });
  const metadataUri = `https://nft.cardinal.so/metadata?img=${
    response.url
  }&name=${encodeURIComponent(metadataName)}`;
  const metadataId = await Metadata.getPDA(mintKeypair.publicKey);
  tx.instructions.push(
    ...new CreateMetadataV2(
      { feePayer: wallet.publicKey },
      {
        metadata: metadataId,
        metadataData: new DataV2({
          name: metadataName,
          symbol: "TST",
          uri: metadataUri,
          sellerFeeBasisPoints: 500,
          creators: [
            new Creator({
              address: wallet.publicKey.toString(),
              verified: false,
              share: 100 - DEFAULT_MINIMUM_CREATOR_SHARE,
            }),
            new Creator({
              address: DEFAULT_REQUIRED_CREATOR,
              verified: false,
              share: DEFAULT_MINIMUM_CREATOR_SHARE,
            }),
          ],
          collection: null,
          uses: null,
        }),
        updateAuthority: wallet.publicKey,
        mint: mintKeypair.publicKey,
        mintAuthority: wallet.publicKey,
      }
    ).instructions
  );

  // init mint manager
  tx.add(
    createInitMintManagerInstruction({
      mintManager: mintManagerId,
      mintMetadata: findMintMetadataId(mintKeypair.publicKey),
      mint: mintKeypair.publicKey,
      ruleset: rulesetId,
      holderTokenAccount: holdetAta,
      tokenAuthority: wallet.publicKey,
      authority: wallet.publicKey,
      payer: wallet.publicKey,
    })
  );

  let txid = "";
  try {
    txid = await executeTransaction(connection, tx, new anchor.Wallet(wallet), [
      mintKeypair,
    ]);
    console.log(
      `Created CCS token https://explorer.solana.com/tx/${txid}?cluster=${cluster}.`
    );
  } catch (e) {
    // eslint-disable-next-line @typescript-eslint/restrict-template-expressions
    console.log(`Transactionn failed: ${e}`);
  }
};

const params: CreateCSSTokenParams = {
  target: new PublicKey("6twkNYKJuv2642Hr99fDBoFxPM3yZt38GJRw7qBj9bem"),
};
main(params).catch((e) => console.log(e));

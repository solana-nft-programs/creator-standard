import { findAta } from "@cardinal/common";
import * as anchor from "@project-serum/anchor";
import { Keypair, PublicKey, Transaction } from "@solana/web3.js";
import dotenv from "dotenv";

import { createInitMintManagerInstruction, Ruleset } from "../sdk";
import { findMintManagerId, findRulesetId } from "../sdk/pda";
import { connectionFor, createMintTx, executeTransaction } from "../utils";

dotenv.config();

const DEFAULT_RULESET = "ruleset-no-checks";
const COLLECTOR = new PublicKey("gmdS6fDgVbeCCYwwvTPJRKM9bFbAgSZh6MTDUT2DcgV");

export type CreateCSSTokenParams = {
  target: PublicKey;
  rulesetName?: string;
};

const wallet = Keypair.fromSecretKey(
  anchor.utils.bytes.bs58.decode(process.env.RULESET_AUTHORITY || "")
); // your wallet's secret key // your wallet's secret key

const main = async (params: CreateCSSTokenParams, cluster = "devnet") => {
  const connection = connectionFor(cluster);
  const transaction = new Transaction();

  const mintKeypair = Keypair.generate();
  const tx = await createMintTx(
    connection,
    mintKeypair.publicKey,
    wallet.publicKey
  );
  const rulesetId = findRulesetId(params.rulesetName ?? DEFAULT_RULESET);
  const rulesetData = await Ruleset.fromAccountAddress(connection, rulesetId);
  const mintManagerId = findMintManagerId(mintKeypair.publicKey);
  const holdetAta = await findAta(
    mintKeypair.publicKey,
    wallet.publicKey,
    true
  );
  transaction.add(
    createInitMintManagerInstruction({
      mintManager: mintManagerId,
      mint: mintKeypair.publicKey,
      ruleset: rulesetId,
      holderTokenAccount: holdetAta,
      tokenAuthority: wallet.publicKey,
      rulesetCollector: rulesetData.collector,
      collector: COLLECTOR,
      authority: wallet.publicKey,
      payer: wallet.publicKey,
    })
  );

  let txid = "";
  try {
    txid = await executeTransaction(
      connection,
      transaction,
      new anchor.Wallet(wallet)
    );
  } catch (e) {
    // eslint-disable-next-line @typescript-eslint/restrict-template-expressions
    console.log(`Transactionn failed: ${e}`);
  }

  try {
    await Ruleset.fromAccountAddress(connection, rulesetId);
    console.log(
      `Initialized ruleset successfully https://explorer.solana.com/tx/${txid}?cluster=${cluster}.`
    );
  } catch (e) {
    console.log("Could not initialize ruleset successfully.");
  }
};

const params: CreateCSSTokenParams = {
  target: new PublicKey(""),
};
main(params).catch((e) => console.log(e));

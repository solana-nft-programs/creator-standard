import * as anchor from "@project-serum/anchor";
import { Keypair, PublicKey, Transaction } from "@solana/web3.js";
import dotenv from "dotenv";

import { createCloseRulesetInstruction } from "../sdk";
import { findRulesetId } from "../sdk/pda";
import { connectionFor, executeTransaction } from "../utils";

dotenv.config();

export type RulesetParams = {
  pubkey?: PublicKey;
  name?: string;
};

const wallet = Keypair.fromSecretKey(
  anchor.utils.bytes.bs58.decode(process.env.RULESET_AUTHORITY || "")
); // your wallet's secret key // your wallet's secret key

const main = async (params: RulesetParams, cluster = "devnet") => {
  const connection = connectionFor(cluster);
  const transaction = new Transaction();

  if (!params.pubkey && !params.name)
    throw new Error("No name or pubkey provided");

  const rulesetId = params.pubkey || findRulesetId(params.name!);
  transaction.add(
    createCloseRulesetInstruction({
      ruleset: rulesetId,
      authority: wallet.publicKey,
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

  console.log(
    `Closed ruleset successfully https://explorer.solana.com/tx/${txid}?cluster=${cluster}.`
  );
};

const params: RulesetParams = {
  pubkey: new PublicKey("EBs1boZXeJHZpNxi6WJeWN3sBtZH1fMe5x8owCg2Z4Z7"),
};
main(params).catch((e) => console.log(e));

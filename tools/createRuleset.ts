import * as anchor from "@project-serum/anchor";
import { Keypair, Transaction } from "@solana/web3.js";
import dotenv from "dotenv";

import { Ruleset } from "../sdk";
import { createInitRulesetInstruction } from "../sdk/generated/instructions/InitRuleset";
import { findRulesetId } from "../sdk/pda";
import { connectionFor, executeTransaction } from "../utils";

dotenv.config();

export type RulesetParams = {
  name: string;
};

const wallet = Keypair.fromSecretKey(
  anchor.utils.bytes.bs58.decode(process.env.RULESET_AUTHORITY || "")
); // your wallet's secret key

const main = async (params: RulesetParams, cluster = "devnet") => {
  const connection = connectionFor(cluster);
  const transaction = new Transaction();

  const rulesetId = findRulesetId(params.name);
  transaction.add(
    createInitRulesetInstruction(
      {
        ruleset: rulesetId,
        authority: wallet.publicKey,
        payer: wallet.publicKey,
      },
      {
        initRulesetIx: {
          name: params.name,
          disallowedAddresses: [],
          allowedPrograms: [],
          extensions: [],
        },
      }
    )
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

const params: RulesetParams = {
  name: "",
};
main(params).catch((e) => console.log(e));

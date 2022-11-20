import { TransactionInstruction } from "@solana/web3.js";

import { Ruleset } from "./generated";

// Needed for transfer, approve, set_in_use_by, approve_and_set_in_use_by instructions
export const handleRemainingAccountsForRuleset = async (
  ix: TransactionInstruction,
  rulesetData: Ruleset
): Promise<TransactionInstruction> => {
  for (const extension of rulesetData.extensions) {
    ix.keys.push({
      pubkey: extension,
      isWritable: false,
      isSigner: false,
    });
  }
  return ix;
};

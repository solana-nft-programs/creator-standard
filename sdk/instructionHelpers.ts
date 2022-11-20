import type { TransactionInstruction } from "@solana/web3.js";

import type { Ruleset } from "./generated";

// Needed for transfer, approve, set_in_use_by, approve_and_set_in_use_by instructions
export const handleRemainingAccountsForRuleset = (
  ix: TransactionInstruction,
  rulesetData: Ruleset
): TransactionInstruction => {
  for (const extension of rulesetData.extensions) {
    ix.keys.push({
      pubkey: extension,
      isWritable: false,
      isSigner: false,
    });
  }
  return ix;
};

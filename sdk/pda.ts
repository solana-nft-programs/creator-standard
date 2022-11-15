import { utils } from "@project-serum/anchor";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { PublicKey } from "@solana/web3.js";

import { PROGRAM_ID } from "./generated";

export const DEFAULT_COLLECTOR = new PublicKey(
  "gmdS6fDgVbeCCYwwvTPJRKM9bFbAgSZh6MTDUT2DcgV"
);

export const findRulesetId = (name: string): PublicKey => {
  return findProgramAddressSync(
    [utils.bytes.utf8.encode("ruleset"), utils.bytes.utf8.encode(name)],
    PROGRAM_ID
  )[0];
};

export const findMintManagerId = (mintId: PublicKey): PublicKey => {
  return findProgramAddressSync(
    [utils.bytes.utf8.encode("mint-manager"), mintId.toBuffer()],
    PROGRAM_ID
  )[0];
};

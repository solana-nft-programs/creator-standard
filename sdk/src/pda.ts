import { utils } from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";
import { PROGRAM_ID } from "./generated";

/**
 * Finds the mint manager id.
 * @returns
 */
export const findMintManagerId = (mintId: PublicKey): PublicKey => {
  return PublicKey.findProgramAddressSync(
    [utils.bytes.utf8.encode("mint-manager"), mintId.toBuffer()],
    PROGRAM_ID
  )[0];
};

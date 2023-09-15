import { getMint } from "@solana/spl-token";
import type { Connection, PublicKey } from "@solana/web3.js";

import { findMintManagerId } from "./pda";

export const identifyCCSToken = async (
  connection: Connection,
  mintId: PublicKey,
): Promise<boolean> => {
  const mintManagerId = findMintManagerId(mintId);
  const mintData = await getMint(connection, mintId);
  return (
    mintData.freezeAuthority?.toString() === mintManagerId.toString() &&
    mintData.mintAuthority?.toString() === mintManagerId.toString()
  );
};

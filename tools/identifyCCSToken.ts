import { getMint } from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";
import dotenv from "dotenv";

import { findMintManagerId } from "../sdk/pda";
import { connectionFor } from "../utils";

dotenv.config();

const main = async (mintId: PublicKey, cluster = "devnet") => {
  const connection = connectionFor(cluster);

  const mintManagerId = findMintManagerId(mintId);
  const mintInfo = await getMint(connection, mintId);
  if (
    mintInfo.freezeAuthority?.toString() === mintManagerId.toString() &&
    mintInfo.mintAuthority?.toString() === mintManagerId.toString()
  ) {
    console.log("Found CCS token.");
  } else {
    console.log(`Token with mint id ${mintId.toString()} is not a CCS token.`);
  }
};

const mintId = new PublicKey("");
main(mintId).catch((e) => console.log(e));

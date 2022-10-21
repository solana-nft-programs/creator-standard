import { Program } from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";
import { CardinalCreatorStandard, IDL } from "./idl/cardinal_creator_standard";

export const CREATOR_STANDARD_ID = new PublicKey(
  "creatS3mfzrTGjwuLD1Pa2HXJ1gmq6WXb4ssnwUbJez"
);

export const CreatorStandard = new Program<CardinalCreatorStandard>(
  IDL,
  CREATOR_STANDARD_ID
);

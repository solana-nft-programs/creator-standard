import type { PublicKey } from "@solana/web3.js";

export type RulesetData = {
  accountType: number;
  version: number;
  authority: PublicKey;
  collector: PublicKey;
  checkSellerFeeBasisPoint: boolean;
  name: string;
  allowedPrograms: PublicKey[];
  disallowedAddresses: PublicKey[];
};

export type MintManagerData = {
  accountType: number;
  version: number;
  mint: PublicKey;
  authority: PublicKey;
  ruleset: PublicKey;
  inUseBy: PublicKey | null;
};

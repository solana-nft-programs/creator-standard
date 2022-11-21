import type { PublicKey } from "@solana/web3.js";

export type RulesetData = {
  accountType: number;
  version: number;
  authority: PublicKey;
  name: string;
  allowedPrograms: PublicKey[];
  disallowedAddresses: PublicKey[];
  extensions: PublicKey[] | null;
};

export type MintManagerData = {
  accountType: number;
  version: number;
  mint: PublicKey;
  authority: PublicKey;
  ruleset: PublicKey;
  inUseBy: PublicKey | null;
};

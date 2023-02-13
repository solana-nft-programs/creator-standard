import * as anchor from "@project-serum/anchor";
import { BorshAccountsCoder } from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";
import dotenv from "dotenv";

import { MintManager, PROGRAM_ADDRESS } from "../sdk";
import { connectionFor } from "../utils";

dotenv.config();

export type RulesetParams = {
  name: string;
};

const main = async (cluster = "devnet") => {
  const connection = connectionFor(cluster);

  const programAccounts = await connection.getProgramAccounts(
    new PublicKey(PROGRAM_ADDRESS),
    {
      filters: [
        {
          memcmp: {
            offset: 0,
            bytes: anchor.utils.bytes.bs58.encode(
              BorshAccountsCoder.accountDiscriminator("mint-manager")
            ),
          },
        },
      ],
    }
  );
  const uniqueMintManagerAuthorities: { [key: string]: string[] } = {};
  programAccounts.forEach((account) => {
    const mintManager = MintManager.fromAccountInfo(account.account)[0];
    if (
      !Object.keys(uniqueMintManagerAuthorities).includes(
        mintManager.authority.toString()
      )
    ) {
      uniqueMintManagerAuthorities[mintManager.authority.toString()] = [
        `https://explorer.solana.com/account/${mintManager.mint.toString()}?cluster=${cluster}`,
      ];
    } else {
      uniqueMintManagerAuthorities[mintManager.authority.toString()]?.push(
        `https://explorer.solana.com/account/${mintManager.mint.toString()}?cluster=${cluster}`
      );
    }
  });
  console.log(uniqueMintManagerAuthorities);
};

main("mainnet").catch((e) => console.log(e));

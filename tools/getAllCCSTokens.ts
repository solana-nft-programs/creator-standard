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
  );
  const mintManagers: [PublicKey, MintManager][] = [];
  // console.log(programAccounts.map((acc) => acc.pubkey.toString()));
  programAccounts.forEach((account) => {
    mintManagers.push([
      account.pubkey,
      MintManager.fromAccountInfo(account.account)[0],
    ]);
  });
  const uniqueMints = mintManagers.reduce(
    (acc, mintManager) => {
      const authority = mintManager[1].authority.toString();
      if (!acc[authority]) {
        acc[authority] = [
          `https://explorer.solana.com/address/${mintManager[1].mint.toString()}`,
          "1",
        ];
      } else {
        acc[authority] = [
          acc[authority]![0]!,
          (Number(acc[authority]![1]) + 1).toString(),
        ];
      }
      return acc;
    },
    {} as { [key: string]: string[] },
  );
  console.log(uniqueMints);
};

main("mainnet").catch((e) => console.log(e));

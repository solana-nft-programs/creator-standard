import * as anchor from "@project-serum/anchor";
import { BorshAccountsCoder } from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";
import dotenv from "dotenv";

import { PROGRAM_ADDRESS, Ruleset } from "../sdk";
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
              BorshAccountsCoder.accountDiscriminator("ruleset")
            ),
          },
        },
      ],
    }
  );
  const rulesets: [PublicKey, Ruleset][] = [];
  programAccounts.forEach((account) => {
    rulesets.push([
      account.pubkey,
      Ruleset.fromAccountInfo(account.account)[0],
    ]);
  });
  console.log(
    rulesets.map((ruleset) => [
      ruleset[0].toString(),
      ruleset[1].name,
      ruleset[1].accountType.toString(),
      ruleset[1].allowedPrograms,
    ])
  );
};

main().catch((e) => console.log(e));

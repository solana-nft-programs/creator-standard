import dotenv from "dotenv";

import { findRulesetId, Ruleset } from "../sdk";
import { connectionFor } from "../utils";

dotenv.config();

export type RulesetParams = {
  name: string;
};

const main = async (cluster = "devnet") => {
  const connection = connectionFor(cluster);

  const rulesetId = findRulesetId();
  const ruleset = await Ruleset.fromAccountAddress(connection, rulesetId);
  console.log(ruleset.name);
  console.log(ruleset.allowedPrograms);
};

main().catch((e) => console.log(e));

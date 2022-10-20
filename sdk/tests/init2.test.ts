import { test, describe, beforeAll } from "@jest/globals";
import { getConnection, newAccountWithLamports } from "./utils";
import type { Connection, PublicKey, Signer } from "@solana/web3.js";

let connection: Connection;
let authority: Signer;

beforeAll(async () => {
  connection = await getConnection();
  authority = await newAccountWithLamports(connection);
});

test("Init", () => {
  console.log("test");
});

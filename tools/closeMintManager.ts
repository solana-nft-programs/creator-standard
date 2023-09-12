import * as anchor from "@project-serum/anchor";
import { Keypair, PublicKey, Transaction } from "@solana/web3.js";
import dotenv from "dotenv";

import { createCloseMintManagerInstruction, MintManager } from "../sdk";
import { findMintEditionId, findMintManagerId } from "../sdk/pda";
import { connectionFor, executeTransaction } from "../utils";

dotenv.config();

export type Params = {
  mintManagerId: PublicKey;
};

const wallet = Keypair.fromSecretKey(
  anchor.utils.bytes.bs58.decode(process.env.WALLET || "")
); // your wallet's secret key

const main = async (params: Params, cluster = "devnet") => {
  const connection = connectionFor(cluster);
  const transaction = new Transaction();

  const mintManager = await MintManager.fromAccountAddress(
    connection,
    params.mintManagerId
  );
  const tkas = await connection.getTokenLargestAccounts(mintManager.mint);
  const tokenAccount = tkas.value.find((tk) => Number(tk.amount) === 1);

  if (!tokenAccount) {
    throw "None";
  }

  transaction.add(
    createCloseMintManagerInstruction({
      mintManager: findMintManagerId(mintManager.mint),
      mint: mintManager.mint,
      holderTokenAccount: tokenAccount.address,
      newTokenAuthority: findMintEditionId(mintManager.mint),
      authority: wallet.publicKey,
      payer: wallet.publicKey,
    })
  );

  try {
    const txid = await executeTransaction(
      connection,
      transaction,
      new anchor.Wallet(wallet)
    );
    console.log(`https://explorer.solana.com/address/${txid}`);
  } catch (e) {
    // eslint-disable-next-line @typescript-eslint/restrict-template-expressions
    console.log(`Transactionn failed: ${e}`);
  }
};

main(
  {
    mintManagerId: new PublicKey("address-here"),
  },
  "mainnet-beta"
).catch((e) => console.log(e));

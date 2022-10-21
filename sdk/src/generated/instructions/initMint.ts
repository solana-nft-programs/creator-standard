/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as splToken from '@solana/spl-token'
import * as beet from '@metaplex-foundation/beet'
import * as web3 from '@solana/web3.js'

/**
 * @category Instructions
 * @category InitMint
 * @category generated
 */
export const initMintStruct = new beet.BeetArgsStruct<{
  instructionDiscriminator: number[] /* size: 8 */
}>(
  [['instructionDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)]],
  'InitMintInstructionArgs'
)
/**
 * Accounts required by the _initMint_ instruction
 *
 * @property [_writable_] mintManager
 * @property [_writable_] mint
 * @property [] standard
 * @property [_writable_] targetTokenAccount
 * @property [**signer**] target
 * @property [_writable_] collector
 * @property [**signer**] authority
 * @property [_writable_, **signer**] payer
 * @property [] associatedTokenProgram
 * @category Instructions
 * @category InitMint
 * @category generated
 */
export type InitMintInstructionAccounts = {
  mintManager: web3.PublicKey
  mint: web3.PublicKey
  standard: web3.PublicKey
  targetTokenAccount: web3.PublicKey
  target: web3.PublicKey
  collector: web3.PublicKey
  authority: web3.PublicKey
  payer: web3.PublicKey
  rent?: web3.PublicKey
  tokenProgram?: web3.PublicKey
  associatedTokenProgram: web3.PublicKey
  systemProgram?: web3.PublicKey
}

export const initMintInstructionDiscriminator = [
  126, 176, 233, 16, 66, 117, 209, 125,
]

/**
 * Creates a _InitMint_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @category Instructions
 * @category InitMint
 * @category generated
 */
export function createInitMintInstruction(
  accounts: InitMintInstructionAccounts,
  programId = new web3.PublicKey('creatS3mfzrTGjwuLD1Pa2HXJ1gmq6WXb4ssnwUbJez')
) {
  const [data] = initMintStruct.serialize({
    instructionDiscriminator: initMintInstructionDiscriminator,
  })
  const keys: web3.AccountMeta[] = [
    {
      pubkey: accounts.mintManager,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.mint,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.standard,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.targetTokenAccount,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.target,
      isWritable: false,
      isSigner: true,
    },
    {
      pubkey: accounts.collector,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.authority,
      isWritable: false,
      isSigner: true,
    },
    {
      pubkey: accounts.payer,
      isWritable: true,
      isSigner: true,
    },
    {
      pubkey: accounts.rent ?? web3.SYSVAR_RENT_PUBKEY,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.tokenProgram ?? splToken.TOKEN_PROGRAM_ID,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.associatedTokenProgram,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.systemProgram ?? web3.SystemProgram.programId,
      isWritable: false,
      isSigner: false,
    },
  ]

  const ix = new web3.TransactionInstruction({
    programId,
    keys,
    data,
  })
  return ix
}

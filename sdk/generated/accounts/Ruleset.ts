/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as web3 from '@solana/web3.js'
import * as beet from '@metaplex-foundation/beet'
import * as beetSolana from '@metaplex-foundation/beet-solana'

/**
 * Arguments used to create {@link Ruleset}
 * @category Accounts
 * @category generated
 */
export type RulesetArgs = {
  bump: number
  version: number
  authority: web3.PublicKey
  collector: web3.PublicKey
  checkSellerFeeBasisPoints: boolean
  name: string
  allowedPrograms: web3.PublicKey[]
  disallowedAddresses: web3.PublicKey[]
}

export const rulesetDiscriminator = [123, 92, 136, 166, 160, 236, 248, 180]
/**
 * Holds the data for the {@link Ruleset} Account and provides de/serialization
 * functionality for that data
 *
 * @category Accounts
 * @category generated
 */
export class Ruleset implements RulesetArgs {
  private constructor(
    readonly bump: number,
    readonly version: number,
    readonly authority: web3.PublicKey,
    readonly collector: web3.PublicKey,
    readonly checkSellerFeeBasisPoints: boolean,
    readonly name: string,
    readonly allowedPrograms: web3.PublicKey[],
    readonly disallowedAddresses: web3.PublicKey[]
  ) {}

  /**
   * Creates a {@link Ruleset} instance from the provided args.
   */
  static fromArgs(args: RulesetArgs) {
    return new Ruleset(
      args.bump,
      args.version,
      args.authority,
      args.collector,
      args.checkSellerFeeBasisPoints,
      args.name,
      args.allowedPrograms,
      args.disallowedAddresses
    )
  }

  /**
   * Deserializes the {@link Ruleset} from the data of the provided {@link web3.AccountInfo}.
   * @returns a tuple of the account data and the offset up to which the buffer was read to obtain it.
   */
  static fromAccountInfo(
    accountInfo: web3.AccountInfo<Buffer>,
    offset = 0
  ): [Ruleset, number] {
    return Ruleset.deserialize(accountInfo.data, offset)
  }

  /**
   * Retrieves the account info from the provided address and deserializes
   * the {@link Ruleset} from its data.
   *
   * @throws Error if no account info is found at the address or if deserialization fails
   */
  static async fromAccountAddress(
    connection: web3.Connection,
    address: web3.PublicKey
  ): Promise<Ruleset> {
    const accountInfo = await connection.getAccountInfo(address)
    if (accountInfo == null) {
      throw new Error(`Unable to find Ruleset account at ${address}`)
    }
    return Ruleset.fromAccountInfo(accountInfo, 0)[0]
  }

  /**
   * Provides a {@link web3.Connection.getProgramAccounts} config builder,
   * to fetch accounts matching filters that can be specified via that builder.
   *
   * @param programId - the program that owns the accounts we are filtering
   */
  static gpaBuilder(
    programId: web3.PublicKey = new web3.PublicKey(
      'creatS3mfzrTGjwuLD1Pa2HXJ1gmq6WXb4ssnwUbJez'
    )
  ) {
    return beetSolana.GpaBuilder.fromStruct(programId, rulesetBeet)
  }

  /**
   * Deserializes the {@link Ruleset} from the provided data Buffer.
   * @returns a tuple of the account data and the offset up to which the buffer was read to obtain it.
   */
  static deserialize(buf: Buffer, offset = 0): [Ruleset, number] {
    return rulesetBeet.deserialize(buf, offset)
  }

  /**
   * Serializes the {@link Ruleset} into a Buffer.
   * @returns a tuple of the created Buffer and the offset up to which the buffer was written to store it.
   */
  serialize(): [Buffer, number] {
    return rulesetBeet.serialize({
      accountDiscriminator: rulesetDiscriminator,
      ...this,
    })
  }

  /**
   * Returns the byteSize of a {@link Buffer} holding the serialized data of
   * {@link Ruleset} for the provided args.
   *
   * @param args need to be provided since the byte size for this account
   * depends on them
   */
  static byteSize(args: RulesetArgs) {
    const instance = Ruleset.fromArgs(args)
    return rulesetBeet.toFixedFromValue({
      accountDiscriminator: rulesetDiscriminator,
      ...instance,
    }).byteSize
  }

  /**
   * Fetches the minimum balance needed to exempt an account holding
   * {@link Ruleset} data from rent
   *
   * @param args need to be provided since the byte size for this account
   * depends on them
   * @param connection used to retrieve the rent exemption information
   */
  static async getMinimumBalanceForRentExemption(
    args: RulesetArgs,
    connection: web3.Connection,
    commitment?: web3.Commitment
  ): Promise<number> {
    return connection.getMinimumBalanceForRentExemption(
      Ruleset.byteSize(args),
      commitment
    )
  }

  /**
   * Returns a readable version of {@link Ruleset} properties
   * and can be used to convert to JSON and/or logging
   */
  pretty() {
    return {
      bump: this.bump,
      version: this.version,
      authority: this.authority.toBase58(),
      collector: this.collector.toBase58(),
      checkSellerFeeBasisPoints: this.checkSellerFeeBasisPoints,
      name: this.name,
      allowedPrograms: this.allowedPrograms,
      disallowedAddresses: this.disallowedAddresses,
    }
  }
}

/**
 * @category Accounts
 * @category generated
 */
export const rulesetBeet = new beet.FixableBeetStruct<
  Ruleset,
  RulesetArgs & {
    accountDiscriminator: number[] /* size: 8 */
  }
>(
  [
    ['accountDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)],
    ['bump', beet.u8],
    ['version', beet.u8],
    ['authority', beetSolana.publicKey],
    ['collector', beetSolana.publicKey],
    ['checkSellerFeeBasisPoints', beet.bool],
    ['name', beet.utf8String],
    ['allowedPrograms', beet.array(beetSolana.publicKey)],
    ['disallowedAddresses', beet.array(beetSolana.publicKey)],
  ],
  Ruleset.fromArgs,
  'Ruleset'
)
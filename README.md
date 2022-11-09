# Cardinal Creator Standard

<div style="text-align: center; width: 100%;">
  <img style="width: full" src="./doc-assets/banner.png" />
</div>

## Spec outlining [details](https://cardinal-labs.notion.site/The-Creator-Standard-v1-5fce56a1d8bb440f849205e9fb5befc2)

> Link to: High level non-techincal proposal that includes background, motivation and proposed solution

# Background

The goal of this standard is not to replace the Token program or the Metadata program (base layer). The standard aims to introduce a layer on top of a token’s base layer that permissions transfers according to a predefined set of rules specified by the token’s creator.

We bounced a lot back and forth on the phrasing of the proposed solution between “standard” vs “solution”. We concluded to the “standard” phrasing because this approach will not be supporting already minted Master Edition tokens. A “solution” would allow this on-top layer to be introduced to all already existing existing tokens, however the current proposed approach implements permissioned transfers for all but Master Edition tokens. Our standard utilizes a token’s freeze authority which for a Master Edition is owned by Metaplex and cannot be proxied.

# Basic NFT

<div style="text-align: center; width: 100%;">
  <img style="width: full" src="./doc-assets/nft.png" />
</div>

# NFT with Attached Ruleset

<div style="text-align: center; width: 100%;">
  <img style="width: full" src="./doc-assets/nft-ruleset.png" />
</div>

# NFT Ruleset Program Interaction

<div style="text-align: center; width: 100%;">
  <img style="width: full" src="./doc-assets/nft-ruleset-programs.png" />
</div>

# Rulesets Can Be 1:Many with NFTs

<div style="text-align: center; width: 100%;">
  <img style="width: full" src="./doc-assets/ruleset-1-to-many.png" />
</div>

# Rulesets Can Extend Other Rulesets

<div style="text-align: center; width: 100%;">
  <img style="width: full" src="./doc-assets/rulesets-extends.png" />
</div>

# Program State

The program utilizes two types of program derived accounts.

### Ruleset

<aside>
📖 Set of rules that permission the transfer of a token specified by the token’s creator

</aside>

```rust
let mut seeds = ['ruleset'.as_bytes(), name.as_bytes()];
pub struct Ruleset {
    // account discriminator
    pub account_type: u8,
    // version for potential future versioning
    pub version: u8,
    // authority who can update this ruleset
    pub authority: Pubkey,
    // collector who collects anything from this ruleset
    pub collector: Pubkey,
    // whether to check the seller_fee_basis_points using post-transfer
    pub check_seller_fee_basis_points: bool,
    // name for this ruleset (also used to derive ruleset address)
    pub name: String,
    // vector of allowed programs that can interact with this nft
    pub allowed_programs: Vec<Pubkey>,
    // vector of disallowed addresses that cannot interact with this nft
    pub disallowed_addresses: Vec<Pubkey>,
}
```

### Mint Manager

<aside>
📖 One-to-one relationship with a token’s mint. A token minted implementing CCS has a respective mint manager that holds its mint and freeze authority. An already existing toke can implement CSS by surrendering its aforementioned authorities.

</aside>

```rust
let mut seeds = ['mint-manager'.as_bytes(), mint.as_ref()];
pub struct MintManager {
  // account discriminator
  pub account_type: u8,
  // version for potential future versioning
  pub version: u8,
  // mint address of this NFT that this mint-manager corresponds to
  pub mint: Pubkey,
  // authority who can update the ruleset of this mint-manager
  pub authority: Pubkey,
  // ruleset to be checked when transferring
  pub ruleset: Pubkey,
  // optional address currently using this NFT meaning it cannot be transferred while in-use
  pub in_use_by: Option<Pubkey>,
}
```

When a mint manager gets created for a mint, the mint manager freezes the mint perpetually. Each mint manager is associated with a ruleset, with it specifying specific a set of rules

Most fields are self-explanatory with the exception of `in_use_by` field which needs some underlying context and will be discussed below.

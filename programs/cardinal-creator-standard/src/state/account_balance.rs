use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, ShankAccount)]
pub struct AccountBalance {
    pub account_type: u8,
    pub address: Pubkey,
    pub mint: Pubkey,
    pub size: u64,
    pub balance: u64,
}

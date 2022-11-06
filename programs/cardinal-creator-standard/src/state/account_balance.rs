use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

use super::AccountType;
use super::CreatorStandardAccount;

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

impl CreatorStandardAccount for AccountBalance {
    fn account_type() -> AccountType {
        AccountType::AccountBalance
    }

    fn set_account_type(&mut self) -> () {
        self.account_type = AccountType::AccountBalance as u8
    }
}

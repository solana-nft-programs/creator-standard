use crate::state::shared::AccountType;
use crate::state::shared::CreatorStandardAccount;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

pub const ACCOUNT_BALANCES_SEED: &str = "account-balances";

#[repr(C)]
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)] 
pub struct AccountBalance {
    pub address: Pubkey,
    pub mint: Pubkey,
    pub size: u64,
    pub balance: u64,
}

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, ShankAccount)]
pub struct AccountBalances {
    pub account_type: u8,
    pub balances: Vec<AccountBalance>,
}

impl CreatorStandardAccount for AccountBalances {
    fn account_type() -> AccountType {
        AccountType::AccountBalances
    }

    fn set_account_type(&mut self) -> () {
        self.account_type = AccountType::AccountBalances as u8
    }
}

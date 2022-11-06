use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use shank::ShankAccount;

use super::AccountBalance;
use super::AccountType;
use super::CreatorStandardAccount;

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

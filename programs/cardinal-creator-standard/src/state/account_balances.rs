use super::account_balance::AccountBalance;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use shank::ShankAccount;

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, ShankAccount)]
pub struct AccountBalances {
    pub account_type: u8,
    pub balances: Vec<AccountBalance>,
}

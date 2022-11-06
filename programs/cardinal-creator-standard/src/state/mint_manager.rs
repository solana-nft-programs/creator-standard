use crate::state::shared::AccountType;
use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

use crate::utils::assert_with_msg;

use super::CreatorStandardAccount;

pub const MINT_MANAGER_SEED: &str = "mint-manager";
pub const MINT_MANAGER_SIZE: usize = std::mem::size_of::<MintManager>() + 64;

#[inline]
pub fn mint_manager_seeds(mint_id: &Pubkey) -> (Pubkey, Vec<Vec<u8>>) {
    let mut seeds = vec![
        MINT_MANAGER_SEED.as_bytes().to_vec(),
        mint_id.as_ref().to_vec(),
    ];
    let (key, bump) = Pubkey::find_program_address(
        &seeds.iter().map(|s| s.as_slice()).collect::<Vec<&[u8]>>(),
        &crate::id(),
    );
    seeds.push(vec![bump]);
    (key, seeds)
}

#[inline]
pub fn assert_mint_manager_seeds(
    mint_id: &Pubkey,
    expected_key: &Pubkey,
) -> Result<Vec<Vec<u8>>, ProgramError> {
    let (key, seeds) = mint_manager_seeds(mint_id);
    assert_with_msg(
        expected_key == &key,
        ProgramError::InvalidInstructionData,
        "Invalid mint manager seeds",
    )?;
    Ok(seeds)
}

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(Clone, BorshSerialize, BorshDeserialize, Debug, PartialEq, ShankAccount)]
pub struct MintManager {
    pub account_type: u8,
    pub version: u8,
    pub mint: Pubkey,
    pub authority: Pubkey,
    pub ruleset: Pubkey,
    pub in_use_by: Option<Pubkey>,
}

impl CreatorStandardAccount for MintManager {
    fn account_type() -> AccountType {
        AccountType::MintManager
    }

    fn set_account_type(&mut self) -> () {
        self.account_type = AccountType::MintManager as u8
    }
}

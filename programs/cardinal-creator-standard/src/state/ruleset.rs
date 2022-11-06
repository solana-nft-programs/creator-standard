use crate::state::shared::AccountType;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use shank::ShankAccount;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

use crate::utils::assert_with_msg;

use super::CreatorStandardAccount;

pub const RULESET_SEED: &str = "ruleset";
pub const RULESET_SIZE: usize = std::mem::size_of::<Ruleset>() + 64;

#[inline]
pub fn ruleset_seeds(mint_id: &Pubkey) -> (Pubkey, Vec<Vec<u8>>) {
    let mut seeds = vec![RULESET_SEED.as_bytes().to_vec(), mint_id.as_ref().to_vec()];
    let (key, bump) = Pubkey::find_program_address(
        &seeds.iter().map(|s| s.as_slice()).collect::<Vec<&[u8]>>(),
        &crate::id(),
    );
    seeds.push(vec![bump]);
    (key, seeds)
}

#[inline]
pub fn assert_ruleset_seeds(
    mint_id: &Pubkey,
    expected_key: &Pubkey,
) -> Result<Vec<Vec<u8>>, ProgramError> {
    let (key, seeds) = ruleset_seeds(mint_id);
    assert_with_msg(
        expected_key == &key,
        ProgramError::InvalidInstructionData,
        "Invalid ruleset seeds",
    )?;
    Ok(seeds)
}

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(Clone, BorshSerialize, BorshDeserialize, Debug, PartialEq, ShankAccount)]
pub struct Ruleset {
    pub account_type: u8,
    pub version: u8,
    pub authority: Pubkey,
    pub collector: Pubkey,
    pub check_seller_fee_basis_points: bool,
    pub name: String,
    pub allowed_programs: Vec<Pubkey>,
    pub disallowed_addresses: Vec<Pubkey>,
}

impl CreatorStandardAccount for Ruleset {
    fn account_type() -> AccountType {
        AccountType::Ruleset
    }
    fn set_account_type(&mut self) -> () {
        self.account_type = AccountType::Ruleset as u8
    }
}

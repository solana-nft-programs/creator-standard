use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use mpl_token_metadata::pda::find_metadata_account;
use mpl_token_metadata::state::Metadata;
use mpl_token_metadata::state::TokenMetadataAccount;
use shank::ShankAccount;
use solana_program::entrypoint::ProgramResult;
use solana_program::hash::hash;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

use crate::utils::assert_with_msg;

use std::collections::HashSet;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::io::ErrorKind;
use std::slice::Iter;
use std::u8;

use borsh::maybestd::io::Error as BorshError;
use solana_program::account_info::AccountInfo;

use crate::errors::ErrorCode;
use crate::id;
use crate::utils::assert_owner;

///////////// CONSTANTS /////////////
pub const COLLECTOR: &str = "gmdS6fDgVbeCCYwwvTPJRKM9bFbAgSZh6MTDUT2DcgV";
pub const RULESET_AUTHORITY: &str = "gmdS6fDgVbeCCYwwvTPJRKM9bFbAgSZh6MTDUT2DcgV";
pub const DEFAULT_REQUIRED_CREATOR: &str = "gmdS6fDgVbeCCYwwvTPJRKM9bFbAgSZh6MTDUT2DcgV";
pub const DEFAULT_MINIMUM_CREATOR_SHARE: u8 = 5;
pub const DEFAULT_PROGRAMS: [&str; 2] = [
    "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
    "mccsLbWK9m7pbFotPmPGBhN37WnsfHG6SRsmeRTJSiP",
];

pub fn is_default_program(program_id: &Pubkey) -> bool {
    DEFAULT_PROGRAMS.contains(&&program_id.to_string()[..])
}
///////////// CONSTANTS /////////////

///////////// ACCOUNT TYPE /////////////
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum AccountType {
    Ruleset = 0,
    MintManager = 1,
    Unrecognized = 2,
}

impl From<u8> for AccountType {
    fn from(orig: u8) -> Self {
        match orig {
            0 => AccountType::Ruleset,
            1 => AccountType::MintManager,
            _ => AccountType::Unrecognized,
        }
    }
}

impl Display for AccountType {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            AccountType::Ruleset => write!(f, "Ruleset"),
            AccountType::MintManager => write!(f, "MintManager"),
            AccountType::Unrecognized => write!(f, "Unrecognized"),
        }
    }
}
///////////// ACCOUNT TYPE /////////////

///////////// CREATOR STANDARD ACCOUNT /////////////
pub trait CreatorStandardAccount {
    fn account_type() -> AccountType;
    fn save(&self, account: &AccountInfo) -> ProgramResult;
    fn new() -> Self;
    fn hash() -> [u8; 8];

    fn safe_deserialize<T: BorshDeserialize>(mut data: &[u8]) -> Result<T, BorshError> {
        if !is_correct_account_type(data, Self::hash()) {
            return Err(BorshError::new(ErrorKind::Other, "InvalidAccountType"));
        }

        let result: Result<T, std::io::Error> = T::deserialize(&mut data);
        if result.is_err() {
            return Err(BorshError::new(ErrorKind::Other, "FailToDeserialize"));
        }

        Ok(result.unwrap())
    }

    fn from_account_info<T: BorshDeserialize>(account: &AccountInfo) -> Result<T, ProgramError> {
        // check that account belongs in the program`
        assert_owner(account, &id(), "account")?;

        let account: T = Self::safe_deserialize(&account.data.borrow_mut())
            .map_err(|_| ErrorCode::DataTypeMismatch)?;

        Ok(account)
    }
}

pub fn is_correct_account_type(data: &[u8], discriminator: [u8; 8]) -> bool {
    data[..8] == discriminator
}
///////////// CREATOR STANDARD ACCOUNT /////////////

///////////// MINT MANAGER /////////////
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
        "Invalid mint manager seeds".to_string(),
    )?;
    Ok(seeds)
}

pub const MINT_MANAGER_SEED: &str = "mint-manager";
pub const MINT_MANAGER_SIZE: usize = std::mem::size_of::<MintManager>() + 64;

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(Clone, BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, ShankAccount)]
pub struct MintManager {
    pub account_type: [u8; 8], // account discriminator
    pub version: u8,           // for potential future verisioning
    pub mint: Pubkey,
    pub authority: Pubkey,
    pub ruleset: Pubkey,
    pub in_use_by: Option<Pubkey>,
}

impl CreatorStandardAccount for MintManager {
    fn hash() -> [u8; 8] {
        let discriminator_preimage = format!("account:{}", "MintManager");
        let mut discriminator = [0u8; 8];
        discriminator.copy_from_slice(&hash(discriminator_preimage.as_bytes()).to_bytes()[..8]);
        discriminator
    }

    fn new() -> Self {
        MintManager {
            account_type: MintManager::hash(),
            version: 0,
            mint: Pubkey::default(),
            authority: Pubkey::default(),
            ruleset: Pubkey::default(),
            in_use_by: None,
        }
    }

    fn account_type() -> AccountType {
        AccountType::MintManager
    }

    fn save(&self, account: &AccountInfo) -> ProgramResult {
        BorshSerialize::serialize(self, &mut *account.data.borrow_mut())?;
        Ok(())
    }

    fn safe_deserialize<T: BorshDeserialize>(mut data: &[u8]) -> Result<T, BorshError> {
        if !is_correct_account_type(data, Self::hash()) {
            return Err(BorshError::new(ErrorKind::Other, "InvalidAccountType"));
        }

        let result: Result<T, std::io::Error> = T::deserialize(&mut data);
        if result.is_err() {
            return Err(BorshError::new(ErrorKind::Other, "FailToDeserialize"));
        }

        Ok(result.unwrap())
    }

    fn from_account_info<T: BorshDeserialize>(account: &AccountInfo) -> Result<T, ProgramError> {
        // check that account belongs in the program`
        assert_owner(account, &id(), "account")?;

        let account: T = Self::safe_deserialize(&account.data.borrow_mut())
            .map_err(|_| ErrorCode::DataTypeMismatch)?;

        Ok(account)
    }
}
///////////// MINT MANAGER /////////////

///////////// RULESET /////////////
pub const RULESET_SEED: &str = "ruleset";
#[inline]
pub fn ruleset_seeds(name: &String) -> (Pubkey, Vec<Vec<u8>>) {
    let mut seeds = vec![RULESET_SEED.as_bytes().to_vec(), name.as_bytes().to_vec()];
    let (key, bump) = Pubkey::find_program_address(
        &seeds.iter().map(|s| s.as_slice()).collect::<Vec<&[u8]>>(),
        &crate::id(),
    );
    seeds.push(vec![bump]);
    (key, seeds)
}

#[inline]
pub fn assert_ruleset_seeds(
    name: &String,
    expected_key: &Pubkey,
) -> Result<Vec<Vec<u8>>, ProgramError> {
    let (key, seeds) = ruleset_seeds(name);
    assert_with_msg(
        expected_key == &key,
        ProgramError::InvalidInstructionData,
        "Invalid ruleset seeds".to_string(),
    )?;
    Ok(seeds)
}

#[inline]
pub fn calculate_ruleset_size(
    allowed_programs: &Vec<Pubkey>,
    disallowed_addresses: &Vec<Pubkey>,
    extensions: &Vec<Pubkey>,
) -> usize {
    std::mem::size_of::<Ruleset>()
        + allowed_programs.len()
        + disallowed_addresses.len()
        + extensions.len()
        + 64
}

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone, ShankAccount)]
pub struct Ruleset {
    pub account_type: [u8; 8], // account discriminator
    pub version: u8,           // for potential future verisioning
    pub authority: Pubkey,
    pub collector: Pubkey,
    pub check_seller_fee_basis_points: bool,
    pub name: String,
    pub allowed_programs: Vec<Pubkey>,
    pub disallowed_addresses: Vec<Pubkey>,
    pub extensions: Vec<Pubkey>,
}

impl CreatorStandardAccount for Ruleset {
    fn hash() -> [u8; 8] {
        let discriminator_preimage = format!("account:{}", "Ruleset");
        let mut discriminator = [0u8; 8];
        discriminator.copy_from_slice(&hash(discriminator_preimage.as_bytes()).to_bytes()[..8]);
        discriminator
    }

    fn new() -> Self {
        Ruleset {
            account_type: Ruleset::hash(),
            version: 0,
            authority: Pubkey::default(),
            collector: Pubkey::default(),
            check_seller_fee_basis_points: true,
            name: String::from(""),
            allowed_programs: Vec::new(),
            disallowed_addresses: Vec::new(),
            extensions: Vec::new(),
        }
    }

    fn account_type() -> AccountType {
        AccountType::Ruleset
    }

    fn save(&self, account: &AccountInfo) -> ProgramResult {
        BorshSerialize::serialize(self, &mut *account.data.borrow_mut())?;
        Ok(())
    }
}

///////////// RULESET /////////////

///////////// UTILS /////////////
pub fn allowlist_disallowlist<'info>(
    ruleset: &Ruleset,
    remaining_accounts: &mut Iter<&AccountInfo<'info>>,
) -> Result<[HashSet<String>; 2], ProgramError> {
    let mut allowed_programs = HashSet::new();
    for program_id in &ruleset.allowed_programs {
        allowed_programs.insert(program_id.to_string());
    }

    let mut disallowed_addresses = HashSet::new();
    for program_id in &ruleset.disallowed_addresses {
        disallowed_addresses.insert(program_id.to_string());
    }

    for ruleset_pubkey in &ruleset.extensions {
        let extension_ruleset_info = remaining_accounts
            .next()
            .ok_or(ProgramError::NotEnoughAccountKeys)?;
        if extension_ruleset_info.key != ruleset_pubkey {
            return Err(ProgramError::from(ErrorCode::InvalidRuleset));
        }
        let extension_ruleset: Ruleset = Ruleset::from_account_info(extension_ruleset_info)
            .expect("Invalid ruleset remaining account");

        for program_id in extension_ruleset.allowed_programs {
            allowed_programs.insert(program_id.to_string().clone());
        }

        for program_id in extension_ruleset.disallowed_addresses {
            disallowed_addresses.insert(program_id.to_string().clone());
        }
    }

    return Ok([allowed_programs, disallowed_addresses]);
}

pub fn check_allowlist_disallowlist<'info>(
    account_id: &Pubkey,
    ruleset: &Ruleset,
    remaining_accounts: &mut Iter<&AccountInfo<'info>>,
) -> Result<bool, ProgramError> {
    let [allowed_programs, disallowed_addresses] =
        allowlist_disallowlist(ruleset, remaining_accounts)?;

    if !allowed_programs.is_empty()
        && !is_default_program(account_id)
        && !allowed_programs.contains(&account_id.to_string())
    {
        return Err(ProgramError::from(ErrorCode::ProgramNotAllowed));
    }

    if !disallowed_addresses.is_empty() && disallowed_addresses.contains(&account_id.to_string()) {
        return Err(ProgramError::from(ErrorCode::AddressDisallowed));
    }
    Ok(true)
}

pub fn check_creators<'info>(
    mint: &Pubkey,
    _ruleset: &Ruleset,
    mint_metadata_account_info: &AccountInfo<'info>,
) -> Result<bool, ProgramError> {
    let mint_metadata_id = find_metadata_account(mint).0;
    assert_with_msg(
        mint_metadata_account_info.key == &mint_metadata_id,
        ErrorCode::InvalidMintMetadata,
        "Invalid mint metadata address",
    )?;
    if !mint_metadata_account_info.data_is_empty() {
        let mint_metadata = Metadata::from_account_info(mint_metadata_account_info)?;
        if let Some(creators) = mint_metadata.data.creators {
            let mut allowed = false;
            for creator in creators {
                if creator.address.to_string() == DEFAULT_REQUIRED_CREATOR
                    && creator.share >= DEFAULT_MINIMUM_CREATOR_SHARE
                {
                    allowed = true;
                }
            }
            if !allowed {
                return Err(ProgramError::from(
                    ErrorCode::InusufficientMinimumCreatorShare,
                ));
            }
        }
    }
    Ok(true)
}
///////////// UTILS /////////////

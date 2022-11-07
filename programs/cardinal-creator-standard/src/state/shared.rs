use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::io::ErrorKind;
use std::u8;

use borsh::maybestd::io::Error as BorshError;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_program::account_info::AccountInfo;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

use crate::errors::ErrorCode;
use crate::id;
use crate::utils::assert_owner;

pub const CREATION_LAMPORTS: u64 = 10_000_000;
pub const UPDATE_LAMPORTS: u64 = 5_000_000;
pub const COLLECTOR_SHARE: u64 = 50;
pub const COLLECTOR: &str = "gmdS6fDgVbeCCYwwvTPJRKM9bFbAgSZh6MTDUT2DcgV";
pub const RULESET_AUTHORITY: &str = "gmdS6fDgVbeCCYwwvTPJRKM9bFbAgSZh6MTDUT2DcgV";
pub const DEFAULT_PROGRAMS: [&str; 1] = ["TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"];
// pub const PRE_TRANSFER_DISCRIMINATOR: [u8; 8] = [158, 85, 53, 202, 155, 118, 19, 228];
// pub const POST_TRANSFER_DISCRIMINATOR: [u8; 8] = [195, 252, 43, 202, 149, 119, 175, 84];

pub fn is_default_program(program_id: Pubkey) -> bool {
    DEFAULT_PROGRAMS.contains(&&program_id.to_string()[..])
}

#[repr(C)]
#[derive(Clone, Copy, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum AccountType {
    Ruleset = 0,
    MintManager = 1,
    AccountBalances = 3,
    Unrecognized = 4,
}

impl From<u8> for AccountType {
    fn from(orig: u8) -> Self {
        match orig {
            0 => return AccountType::Ruleset,
            1 => return AccountType::MintManager,
            3 => return AccountType::AccountBalances,
            _ => return AccountType::Unrecognized,
        };
    }
}

impl Display for AccountType {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            AccountType::Ruleset => write!(f, "Ruleset"),
            AccountType::MintManager => write!(f, "MintManager"),
            AccountType::AccountBalances => write!(f, "AccountBalances"),
            AccountType::Unrecognized => write!(f, "Unrecognized"),
        }
    }
}

// pub struct AccountData<'info, T> {
//     pub account: T,
//     pub info: AccountInfo<'info>,
// }

// CreatorStandardAccount
pub trait CreatorStandardAccount {
    fn account_type() -> AccountType;
    fn set_account_type(&mut self) -> ();

    fn safe_deserialize<T: BorshDeserialize>(mut data: &[u8]) -> Result<T, BorshError> {
        if !is_correct_account_type(data, Self::account_type()) {
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
    // fn dyn_from_account_info<T: BorshDeserialize>(
    //     &self,
    //     a: &AccountInfo,
    // ) -> Result<T, ProgramError> {
    //     let data = &a.data.borrow_mut();
    //     let account_type = AccountType::from(data[0] as u8);
    //     match account_type {
    //         AccountType::Ruleset => return Ruleset::from_account_info(a),
    //         AccountType::MintManager => return MintManager::from_account_info(a),
    //         AccountType::Unrecognized => {
    //             return Err(ProgramError::from(ErrorCode::InvalidAccountType))
    //         }
    //     }
    // }
}

pub fn is_correct_account_type(data: &[u8], data_type: AccountType) -> bool {
    data[0] == data_type as u8
}

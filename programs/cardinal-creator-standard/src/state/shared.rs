use std::{io::ErrorKind, u8};

use borsh::{maybestd::io::Error as BorshError, BorshDeserialize, BorshSerialize};
use solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

use crate::{errors::ErrorCode, id, utils::assert_owner};

pub const CREATION_LAMPORTS: u64 = 10_000_000;
pub const COLLECTOR_SHARE: u64 = 50;
pub const COLLECTOR: &str = "gmdS6fDgVbeCCYwwvTPJRKM9bFbAgSZh6MTDUT2DcgV";
pub const DEFAULT_PROGRAMS: [&str; 1] = ["TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"];
pub fn is_default_program(program_id: Pubkey) -> bool {
    DEFAULT_PROGRAMS.contains(&&program_id.to_string()[..])
}

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum AccountType {
    Ruleset = 0,
    MintManager = 1,
    AccountBalance = 2,
    AccountBalances = 3,
    Unrecognized = 4,
}

impl From<u8> for AccountType {
    fn from(orig: u8) -> Self {
        match orig {
            0 => return AccountType::Ruleset,
            1 => return AccountType::MintManager,
            2 => return AccountType::AccountBalance,
            3 => return AccountType::AccountBalances,
            _ => return AccountType::Unrecognized,
        };
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

    fn from_account_info<T: BorshDeserialize>(a: &AccountInfo) -> Result<T, ProgramError> {
        // check that account belongs in the program`
        assert_owner(a, &id(), "account")?;

        let account: T = Self::safe_deserialize(&a.data.borrow_mut())
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

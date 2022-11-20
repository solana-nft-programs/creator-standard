use std::fmt::Display;

use lazy_format::lazy_format;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program_error::ProgramError;
use solana_program::program_pack::Pack;
use solana_program::pubkey::Pubkey;
use spl_token::state::Account as TokenAccount;
use spl_token::state::Mint;

use crate::errors::ErrorCode;
use crate::id;
use crate::state::is_correct_account_type;

#[inline(always)]
pub fn assert_with_msg(v: bool, err: impl Into<ProgramError>, msg: impl Display) -> ProgramResult {
    if v {
        Ok(())
    } else {
        let caller = std::panic::Location::caller();
        msg!("{} {}", msg, caller);
        Err(err.into())
    }
}

#[inline(always)]
pub fn assert_mut(account: &AccountInfo, name: &str) -> ProgramResult {
    assert_with_msg(
        account.is_writable,
        ProgramError::InvalidInstructionData,
        lazy_format!("{} must be mutable", name),
    )
}

#[inline(always)]
pub fn assert_signer(account: &AccountInfo, name: &str) -> ProgramResult {
    assert_with_msg(
        account.is_signer,
        ProgramError::InvalidInstructionData,
        lazy_format!("{} must be signer", name),
    )
}

#[inline(always)]
pub fn assert_owner(account: &AccountInfo, owner: &Pubkey, name: &str) -> ProgramResult {
    assert_with_msg(
        account.owner == owner,
        ProgramError::IllegalOwner,
        lazy_format!("{} must be owned by {}", name, owner),
    )
}

#[inline(always)]
pub fn assert_amount(amount_one: &str, amount_two: &str, name: &str) -> ProgramResult {
    assert_with_msg(
        amount_one == amount_two,
        ProgramError::from(ErrorCode::InvalidAmount),
        lazy_format!("Invalid amount for {}", name),
    )
}

#[inline(always)]
pub fn assert_empty(account: &AccountInfo, name: &str) -> ProgramResult {
    assert_with_msg(
        account.data_is_empty(),
        ProgramError::InvalidInstructionData,
        lazy_format!("{} must be empty", name),
    )
}

#[inline(always)]
pub fn assert_address(address_one: &Pubkey, address_two: &Pubkey, name: &str) -> ProgramResult {
    assert_with_msg(
        address_one == address_two,
        ProgramError::InvalidInstructionData,
        lazy_format!("{} must equal {}", name, address_two),
    )
}

#[inline(always)]
pub fn assert_program_account(account: &AccountInfo, discriminator: [u8; 8]) -> ProgramResult {
    let data = &account.data.borrow_mut();
    assert_with_msg(
        is_correct_account_type(data, discriminator) && *account.owner == id(),
        ProgramError::InvalidInstructionData,
        lazy_format!("Invalid account type for {}", account.key),
    )
}

#[inline(always)]
pub fn unpack_checked_mint_account(
    account: &AccountInfo,
    name: Option<&str>,
) -> Result<Mint, ProgramError> {
    let check_mint = Mint::unpack(&account.data.try_borrow().expect("Could not borrow data"));
    assert_with_msg(
        check_mint.is_ok() || *account.owner != spl_token::id(),
        ProgramError::from(ErrorCode::InvalidMint),
        lazy_format!(
            "Invalid {} mint account {}",
            name.unwrap_or(""),
            account.key
        ),
    )?;
    check_mint
}

#[inline(always)]
pub fn unpack_checked_token_account(
    account: &AccountInfo,
    name: Option<&str>,
) -> Result<TokenAccount, ProgramError> {
    let check_token_account =
        TokenAccount::unpack(&account.data.try_borrow().expect("Could not borrow data"));
    assert_with_msg(
        check_token_account.is_ok() || *account.owner != spl_token::id(),
        ProgramError::from(ErrorCode::InvalidTokenAccount),
        lazy_format!(
            "Invalid {} token account {}",
            name.unwrap_or(""),
            account.key
        ),
    )?;
    check_token_account
}

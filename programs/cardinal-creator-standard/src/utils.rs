use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program_error::ProgramError;
use solana_program::program_pack::Pack;
use solana_program::pubkey::Pubkey;
use spl_token::state::Account as TokenAccount;
use spl_token::state::Mint;

use crate::errors::ErrorCode;

#[inline(always)]
pub fn assert_with_msg(v: bool, err: impl Into<ProgramError>, msg: &str) -> ProgramResult {
    if v {
        Ok(())
    } else {
        let caller = std::panic::Location::caller();
        msg!("{}. \n{}", msg, caller);
        Err(err.into())
    }
}

#[inline(always)]
pub fn assert_mut(account: &AccountInfo, name: &str) -> ProgramResult {
    assert_with_msg(
        account.is_writable,
        ProgramError::InvalidInstructionData,
        format!("{} must be mutable", name).as_str(),
    )
}

#[inline(always)]
pub fn assert_signer(account: &AccountInfo, name: &str) -> ProgramResult {
    assert_with_msg(
        account.is_signer,
        ProgramError::InvalidInstructionData,
        format!("{} must be signer", name).as_str(),
    )
}

#[inline(always)]
pub fn assert_owner(account: &AccountInfo, owner: &Pubkey, name: &str) -> ProgramResult {
    assert_with_msg(
        account.owner == owner,
        ProgramError::IllegalOwner,
        format!("{} must be owned by {}", name, owner).as_str(),
    )
}

#[inline(always)]
pub fn assert_empty(account: &AccountInfo, name: &str) -> ProgramResult {
    assert_with_msg(
        account.data_is_empty(),
        ProgramError::InvalidInstructionData,
        format!("{} must be empty", name).as_str(),
    )
}

#[inline(always)]
pub fn assert_address(address_one: &Pubkey, address_two: &Pubkey, name: &str) -> ProgramResult {
    assert_with_msg(
        address_one == address_two,
        ProgramError::InvalidInstructionData,
        format!("{} must equal {}", name, address_two).as_str(),
    )
}

#[inline(always)]
pub fn assert_valid_mint_account(
    account: &AccountInfo,
    name: Option<&str>,
) -> Result<Mint, ProgramError> {
    let check_mint = Mint::unpack(&account.data.try_borrow().expect("Could not borrow data"));
    assert_with_msg(
        check_mint.is_ok(),
        ProgramError::from(ErrorCode::InvalidMint),
        format!(
            "Invalid {} mint account {}",
            name.unwrap_or(""),
            account.key
        )
        .as_str(),
    )?;
    check_mint
}

#[inline(always)]
pub fn assert_valid_token_account(
    account: &AccountInfo,
    name: Option<&str>,
) -> Result<TokenAccount, ProgramError> {
    let check_token_account =
        TokenAccount::unpack(&account.data.try_borrow().expect("Could not borrow data"));
    assert_with_msg(
        check_token_account.is_ok(),
        ProgramError::from(ErrorCode::InvalidTokenAccount),
        format!(
            "Invalid {} token account {}",
            name.unwrap_or(""),
            account.key
        )
        .as_str(),
    )?;
    check_token_account
}

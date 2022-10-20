use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

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
pub fn assert_address(account: &AccountInfo, address: &Pubkey, name: &str) -> ProgramResult {
    assert_with_msg(
        account.key == address,
        ProgramError::InvalidInstructionData,
        format!("{} must equal {}", name, address).as_str(),
    )
}

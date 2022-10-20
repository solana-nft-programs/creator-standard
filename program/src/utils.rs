use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program_error::ProgramError;

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

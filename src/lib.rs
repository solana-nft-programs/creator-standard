use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};
solana_program::declare_id!("mTok58Lg4YfcmwqyrDHpf7ogp599WRhzb6PxjaBqAxS");
#[cfg(not(feature = "no-entrypoint"))]
solana_program::entrypoint!(process_instruction);
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    Ok(())
}

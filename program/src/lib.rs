use borsh::BorshDeserialize;
use instruction::CreatorStandardInstruction;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::pubkey::Pubkey;

pub mod instruction;

solana_program::declare_id!("mTok58Lg4YfcmwqyrDHpf7ogp599WRhzb6PxjaBqAxS");

#[cfg(not(feature = "no-entrypoint"))]

solana_program::entrypoint!(process_instruction);
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = CreatorStandardInstruction::try_from_slice(instruction_data)?;
    match instruction {
        CreatorStandardInstruction::Initialize => {
            msg!("CreatorStandardInstruction::Initialize");
            process_initialize(accounts)
        }
    }
}

pub fn process_initialize(accounts: &[AccountInfo]) -> ProgramResult {
    Ok(())
}

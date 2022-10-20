use borsh::BorshSerialize;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::instruction::AccountMeta;
use solana_program::instruction::Instruction;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::system_program;

use crate::CreatorStandardInstruction;

pub fn handler(accounts: &[AccountInfo]) -> ProgramResult {
    Ok(())
}

pub fn init_ix(
    mint: &Pubkey,
    authority: &Pubkey,
    standard: &Pubkey,
) -> Result<Instruction, ProgramError> {
    Ok(Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new(*mint, false),
            AccountMeta::new(*authority, true),
            AccountMeta::new(*standard, true),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: CreatorStandardInstruction::Init.try_to_vec()?,
    })
}

use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use shank::ShankInstruction;
use solana_program::instruction::AccountMeta;
use solana_program::instruction::Instruction;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::system_program;

#[derive(Debug, Clone, ShankInstruction, BorshSerialize, BorshDeserialize)]
#[rustfmt::skip]
pub enum CreatorStandardInstruction {

    #[account(0, writable, name = "mint")]
    #[account(1, writable, signer, name = "authority")]
    #[account(2, writable, name = "standard")]
    #[account(3, name = "token_program", desc = "Token program")]
    #[account(4, name = "system_program", desc = "System program")]
    Initialize,
}

pub fn initialize_ix(
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
        data: CreatorStandardInstruction::Initialize.try_to_vec()?,
    })
}

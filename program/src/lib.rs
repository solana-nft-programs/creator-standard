use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use shank::ShankInstruction;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::pubkey::Pubkey;

pub mod instructions;
pub mod utils;

use instructions::*;

solana_program::declare_id!("mTok58Lg4YfcmwqyrDHpf7ogp599WRhzb6PxjaBqAxS");

#[cfg(not(feature = "no-entrypoint"))]
solana_program::entrypoint!(handler);

#[derive(Debug, Clone, ShankInstruction, BorshSerialize, BorshDeserialize)]
#[rustfmt::skip]
pub enum CreatorStandardInstruction {

    #[account(0, writable, name = "mint")]
    #[account(1, writable, signer, name = "authority")]
    #[account(2, writable, name = "standard")]
    #[account(3, name = "token_program", desc = "Token program")]
    #[account(4, name = "system_program", desc = "System program")]
    Init,
}

pub fn handler(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = CreatorStandardInstruction::try_from_slice(instruction_data)?;
    match instruction {
        CreatorStandardInstruction::Init => {
            msg!("CreatorStandardInstruction::Init");
            init::handler(accounts)
        }
    }
}

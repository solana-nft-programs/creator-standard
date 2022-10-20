use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use shank::ShankInstruction;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::pubkey::Pubkey;

pub mod instructions;
pub mod state;
pub mod utils;

use instructions::*;

solana_program::declare_id!("mTok58Lg4YfcmwqyrDHpf7ogp599WRhzb6PxjaBqAxS");

#[cfg(not(feature = "no-entrypoint"))]
solana_program::entrypoint!(process_instruction);

#[derive(Debug, Clone, ShankInstruction, BorshSerialize, BorshDeserialize)]
#[rustfmt::skip]
pub enum CreatorStandardInstruction {

    #[account(0, writable, name = "mint")]
    #[account(1, writable, name = "mint_manager")]
    #[account(2, writable, signer, name = "authority")]
    #[account(3, writable, signer, name = "payer")]
    #[account(4, name = "token_program", desc = "Token program")]
    #[account(5, name = "system_program", desc = "System program")]
    InitMint,
}

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = CreatorStandardInstruction::try_from_slice(instruction_data)?;
    match instruction {
        CreatorStandardInstruction::InitMint => {
            msg!("CreatorStandardInstruction::InitMint");
            let init_mint_ctx = InitMintCtx::load(accounts)?;
            init_mint::handler(init_mint_ctx)
        }
    }
}

#[inline]
fn mint_manager_seeds(mint_id: &Pubkey) -> (Pubkey, Vec<Vec<u8>>) {
    let mut seeds = vec![mint_id.as_ref().to_vec()];
    let (key, bump) = Pubkey::find_program_address(
        &seeds.iter().map(|s| s.as_slice()).collect::<Vec<&[u8]>>(),
        &crate::id(),
    );
    seeds.push(vec![bump]);
    (key, seeds)
}

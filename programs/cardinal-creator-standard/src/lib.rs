use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use shank::ShankInstruction;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::pubkey::Pubkey;

pub mod errors;
pub mod instructions;
pub mod pda;
pub mod state;
pub mod utils;

use instructions::*;

solana_program::declare_id!("creatS3mfzrTGjwuLD1Pa2HXJ1gmq6WXb4ssnwUbJez");

#[cfg(not(feature = "no-entrypoint"))]
solana_program::entrypoint!(process_instruction);

#[derive(Debug, Clone, ShankInstruction, BorshSerialize, BorshDeserialize)]
#[rustfmt::skip]
pub enum CreatorStandardInstruction {

    // #[account(0, writable, name = "mint")]
    // #[account(1, writable, name = "mint_manager")]
    // #[account(2, writable, signer, name = "authority")]
    // #[account(3, writable, signer, name = "payer")]
    // #[account(4, name = "token_program", desc = "Token program")]
    // #[account(5, name = "system_program", desc = "System program")]
    // InitMint,
    
    #[account(0, writable, name = "mint_manager")]
    #[account(1, writable, name = "mint")]
    #[account(2, name = "ruleset")]
    #[account(3, writable, name = "holder_token_account")]
    #[account(4, writable, name = "ruleset_collector")]
    #[account(5, writable, name = "collector")]
    #[account(6, signer, name = "authority")]
    #[account(7, signer, signer, name = "payer")]
    #[account(8, name = "token_program", desc = "Token program")]
    #[account(9, name = "system_program", desc = "System program")]
    InitMintManager,
}

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = CreatorStandardInstruction::try_from_slice(instruction_data)?;
    match instruction {
        // CreatorStandardInstruction::InitMint => {
        //     msg!("CreatorStandardInstruction::InitMint");
        //     let init_mint_ctx = InitMintCtx::load(accounts)?;
        //     init_mint::handler(init_mint_ctx)
        // }
        CreatorStandardInstruction::InitMintManager => {
            msg!("CreatorStandardInstruction::InitMintManager");
            let ctx = InitMintManagerCtx::load(accounts)?;
            instructions::mint_manager::init_mint_manager::handler(ctx)
        }
    }
}

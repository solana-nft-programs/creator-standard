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
use solana_security_txt::security_txt;

solana_program::declare_id!("ccsxqYAg64wuLEh45KabyPvkKjrEfDPsDZUQrGn7mf3");

#[cfg(not(feature = "no-entrypoint"))]
solana_program::entrypoint!(process_instruction);

#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, Clone, ShankInstruction)]
#[rustfmt::skip]
pub enum CreatorStandardInstruction {
    // ruleset
    #[account(0, writable, name = "ruleset")]
    #[account(1, signer, name = "authority")]
    #[account(2, writable, signer, name = "payer")]
    #[account(3, name = "system_program")]
    InitRuleset(InitRulesetIx),

    #[account(0, writable, name = "ruleset")]
    #[account(1, signer, name = "authority")]
    #[account(2, writable, signer, name = "payer")]
    #[account(3, name = "system_program")]
    UpdateRuleset(UpdateRulesetIx),

    #[account(0, writable, name = "ruleset")]
    #[account(1, writable, signer, name = "authority")]
    CloseRuleset,

    // mint_manager
    #[account(0, writable, name = "mint_manager")]
    #[account(1, writable, name = "mint")]
    #[account(2, name = "mint_metadata", desc = "Mint metadata")]
    #[account(3, name = "ruleset")]
    #[account(4, writable, name = "holder_token_account")]
    #[account(5, signer, name = "token_authority")]
    #[account(6, name = "authority")]
    #[account(7, writable, signer, name = "payer")]
    #[account(8, name = "token_program", desc = "Token program")]
    #[account(9, name = "system_program", desc = "System program")]
    InitMintManager,

    #[account(0, writable, name = "mint_manager")]
    #[account(1, name = "ruleset")]
    #[account(2, signer, name = "authority")]
    #[account(3, writable, signer, name = "payer")]
    #[account(4, name = "system_program", desc = "System program")]
    UpdateMintManager(UpdateMintManagerIx),

    #[account(0, writable, name = "mint_manager")]
    #[account(1, name = "ruleset")]
    #[account(2, name = "in_use_by_address")]
    #[account(3, signer, name = "holder")]
    #[account(4, name = "holder_token_account")]
    SetInUseBy,

    #[account(0, writable, name = "mint_manager")]
    #[account(1, signer, name = "user")]
    RemoveInUseBy,

    // token
    #[account(0, name = "mint_manager")]
    #[account(1, name = "ruleset")]
    #[account(2, name = "mint")]
    #[account(3, writable, name = "holder_token_account")]
    #[account(4, signer, name = "holder")]
    #[account(5, name = "delegate")]
    #[account(6, name = "token_program")]
    Approve(ApproveIx),

    #[account(0, writable, name = "mint_manager")]
    #[account(1, name = "ruleset")]
    #[account(2, name = "mint")]
    #[account(3, name = "in_use_by_address")]
    #[account(4, writable, name = "holder_token_account")]
    #[account(5, signer, name = "holder")]
    #[account(6, name = "delegate")]
    #[account(7, name = "token_program")]
    ApproveAndSetInUseBy(ApproveAndSetInUseByIx),

    #[account(0, writable, name = "mint_manager")]
    #[account(1, writable, name = "mint")]
    #[account(2, writable, name = "holder_token_account")]
    #[account(3, writable, signer, name = "holder")]
    #[account(4, name = "token_program")]
    #[account(5, name = "system_program")]
    Burn,

    #[account(0, name = "mint_manager")]
    #[account(1, writable, name = "mint")]
    #[account(2, writable, name = "token_account")]
    #[account(3, signer, name = "owner")]
    #[account(4, name = "token_program")]
    Close,

    #[account(0, name = "mint")]
    #[account(1, writable, name = "token_account")]
    #[account(2, name = "owner")]
    #[account(3, writable, signer, name = "payer")]
    #[account(4, name = "rent")]
    #[account(5, name = "token_program")]
    #[account(6, name = "associated_token_program")]
    #[account(7, name = "system_program")]
    InitializeAccount,

    #[account(0, name = "mint_manager")]
    #[account(1, name = "mint")]
    #[account(2, writable, name = "holder_token_account")]
    #[account(3, signer, name = "holder")]
    #[account(4, name = "token_program")]
    Revoke,

    #[account(0, name = "mint_manager")]
    #[account(1, name = "mint")]
    #[account(2, name = "mint_metadata", desc = "Mint metadata")]
    #[account(3, name = "ruleset")]
    #[account(4, writable, name = "from")]
    #[account(5, writable, name = "to")]
    #[account(6, signer, name = "authority")]
    #[account(7, name = "token_program")]
    #[account(8, name = "system_program")]
    #[account(9, name = "instructions")]
    Transfer,

    #[account(0, writable, name = "mint_manager")]
    #[account(1, writable, name = "mint")]
    #[account(2, writable, name = "holder_token_account")]
    #[account(3, name = "new_token_authority")]
    #[account(4, name = "authority")]
    #[account(5, writable, signer, name = "payer")]
    #[account(6, name = "token_program", desc = "Token program")]
    #[account(7, name = "system_program", desc = "System program")]
    CloseMintManager,
}

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = CreatorStandardInstruction::try_from_slice(instruction_data)?;
    match instruction {
        CreatorStandardInstruction::InitRuleset(ix) => {
            msg!("CreatorStandardInstruction::InitRuleset");
            let ctx = InitRulesetCtx::load(accounts)?;
            instructions::ruleset::init_ruleset::handler(ctx, ix)
        }
        CreatorStandardInstruction::UpdateRuleset(ix) => {
            msg!("CreatorStandardInstruction::UpdateRuleset");
            let ctx = UpdateRulesetCtx::load(accounts)?;
            instructions::ruleset::update_ruleset::handler(ctx, ix)
        }
        CreatorStandardInstruction::CloseRuleset => {
            msg!("CreatorStandardInstruction::CloseRuleset");
            let ctx = CloseRulesetCtx::load(accounts)?;
            instructions::ruleset::close_ruleset::handler(ctx)
        }
        CreatorStandardInstruction::InitMintManager => {
            msg!("CreatorStandardInstruction::InitMintManager");
            let ctx = InitMintManagerCtx::load(accounts)?;
            instructions::mint_manager::init_mint_manager::handler(ctx)
        }
        CreatorStandardInstruction::UpdateMintManager(ix) => {
            msg!("CreatorStandardInstruction::UpdateMintManager");
            let ctx = UpdateMintManagerCtx::load(accounts)?;
            instructions::mint_manager::update_mint_manager::handler(ctx, ix)
        }
        CreatorStandardInstruction::SetInUseBy => {
            msg!("CreatorStandardInstruction::SetInUseBy");
            let ctx = mint_manager::SetInUseByCtx::load(accounts)?;
            instructions::mint_manager::set_in_use_by::handler(ctx)
        }
        CreatorStandardInstruction::RemoveInUseBy => {
            msg!("CreatorStandardInstruction::RemoveInUseBy");
            let ctx = RemoveInUseByCtx::load(accounts)?;
            instructions::mint_manager::remove_in_use_by::handler(ctx)
        }
        CreatorStandardInstruction::Approve(ix) => {
            msg!("CreatorStandardInstruction::Approve");
            let ctx = ApproveCtx::load(accounts)?;
            instructions::token::approve::handler(ctx, ix)
        }
        CreatorStandardInstruction::ApproveAndSetInUseBy(ix) => {
            msg!("CreatorStandardInstruction::Approve");
            let ctx = ApproveAndSetInUseByCtx::load(accounts)?;
            instructions::token::approve_and_set_in_use_by::handler(ctx, ix)
        }
        CreatorStandardInstruction::Burn => {
            msg!("CreatorStandardInstruction::Burn");
            let ctx = BurnCtx::load(accounts)?;
            instructions::token::burn::handler(ctx)
        }
        CreatorStandardInstruction::Close => {
            msg!("CreatorStandardInstruction::Close");
            let ctx = CloseCtx::load(accounts)?;
            instructions::token::close::handler(ctx)
        }
        CreatorStandardInstruction::InitializeAccount => {
            msg!("CreatorStandardInstruction::InitializeAccount");
            let ctx = InitializeAccountCtx::load(accounts)?;
            instructions::token::initialize_account::handler(ctx)
        }
        CreatorStandardInstruction::Revoke => {
            msg!("CreatorStandardInstruction::Revoke");
            let ctx = RevokeCtx::load(accounts)?;
            instructions::token::revoke::handler(ctx)
        }
        CreatorStandardInstruction::Transfer => {
            msg!("CreatorStandardInstruction::Transfer");
            let ctx = TransferCtx::load(accounts)?;
            instructions::token::transfer::handler(ctx)
        }
        CreatorStandardInstruction::CloseMintManager => {
            msg!("CreatorStandardInstruction::CloseMintManager");
            let ctx = CloseMintManagerCtx::load(accounts)?;
            instructions::mint_manager::close_mint_manager::handler(ctx)
        }
    }
}

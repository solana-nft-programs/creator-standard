pub mod errors;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use instructions::*;

solana_program::declare_id!("creatS3mfzrTGjwuLD1Pa2HXJ1gmq6WXb4ssnwUbJez");

#[program]
pub mod cardinal_creator_standard {
    use crate::instructions::mint_manager::{RemoveInUseByCtx, SetInUseByCtx, SetInUseByIx};

    use super::*;

    // mint_manager
    pub fn init_mint_manager(ctx: Context<InitMintManagerCtx>) -> Result<()> {
        mint_manager::init_mint_manager::handler(ctx)
    }

    pub fn update_mint_manager(
        ctx: Context<UpdateMintManagerCtx>,
        ix: UpdateMintManagerIx,
    ) -> Result<()> {
        mint_manager::update_mint_manager::handler(ctx, ix)
    }

    pub fn set_in_use_by(ctx: Context<SetInUseByCtx>, ix: SetInUseByIx) -> Result<()> {
        mint_manager::set_in_use_by::handler(ctx, ix)
    }

    pub fn remove_in_use_by(ctx: Context<RemoveInUseByCtx>) -> Result<()> {
        mint_manager::remove_in_use_by::handler(ctx)
    }

    // ruleset
    pub fn init_ruleset(ctx: Context<InitRulesetCtx>, ix: InitRulesetIx) -> Result<()> {
        ruleset::init_ruleset::handler(ctx, ix)
    }

    pub fn update_ruleset(ctx: Context<UpdateRulesetCtx>, ix: UpdateRulesetIx) -> Result<()> {
        ruleset::update_ruleset::handler(ctx, ix)
    }

    // token
    pub fn initialize_mint(ctx: Context<InitializeMintCtx>) -> Result<()> {
        token::initialize_mint::handler(ctx)
    }

    pub fn initialize_account(ctx: Context<InitializeAccountCtx>) -> Result<()> {
        token::initialize_account::handler(ctx)
    }

    pub fn approve(ctx: Context<ApproveCtx>) -> Result<()> {
        token::approve::handler(ctx)
    }

    pub fn revoke(ctx: Context<RevokeCtx>) -> Result<()> {
        token::revoke::handler(ctx)
    }

    pub fn burn(ctx: Context<BurnCtx>) -> Result<()> {
        token::burn::handler(ctx)
    }

    pub fn close(ctx: Context<CloseCtx>) -> Result<()> {
        token::close::handler(ctx)
    }

    pub fn transfer(ctx: Context<TransferCtx>) -> Result<()> {
        token::transfer::handler(ctx)
    }
}

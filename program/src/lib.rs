pub mod errors;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use instructions::*;

solana_program::declare_id!("creatS3mfzrTGjwuLD1Pa2HXJ1gmq6WXb4ssnwUbJez");

#[program]
pub mod cardinal_creator_standard {
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

    // standard
    pub fn init_standard(ctx: Context<InitStandardCtx>, ix: InitStandardIx) -> Result<()> {
        standard::init_standard::handler(ctx, ix)
    }

    pub fn update_standard(ctx: Context<UpdateStandardCtx>, ix: UpdateStandardIx) -> Result<()> {
        standard::update_standard::handler(ctx, ix)
    }
}

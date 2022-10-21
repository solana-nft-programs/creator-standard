use crate::state::*;
use anchor_lang::prelude::*;
use solana_program::sysvar::{self};

pub const POST_TRANSFER_DISCRIMINATOR: [u8; 8] = [195, 252, 43, 202, 149, 119, 175, 84];

#[derive(Accounts)]
pub struct PostTransferCtx<'info> {
    mint_manager: Account<'info, MintManager>,
    ruleset: Account<'info, Ruleset>,
    /// CHECK: This is not dangerous because the ID is checked with instructions sysvar
    #[account(address = sysvar::instructions::id())]
    instructions: UncheckedAccount<'info>,
}

pub fn handler(_ctx: Context<PostTransferCtx>) -> Result<()> {
    Ok(())
}

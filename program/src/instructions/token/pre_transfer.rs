use crate::state::*;
use anchor_lang::prelude::*;
use solana_program::sysvar::{self};

pub const PRE_TRANSFER_DISCRIMINATOR: [u8; 8] = [158, 85, 53, 202, 155, 118, 19, 228];

#[derive(Accounts)]
pub struct PreTransferCtx<'info> {
    mint_manager: Account<'info, MintManager>,
    standard: Account<'info, Standard>,
    /// CHECK: This is not dangerous because the ID is checked with instructions sysvar
    #[account(address = sysvar::instructions::id())]
    instructions: UncheckedAccount<'info>,
}

pub fn handler(_ctx: Context<PreTransferCtx>) -> Result<()> {
    Ok(())
}

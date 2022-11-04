use crate::errors::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct RemoveInUseByCtx<'info> {
    #[account(mut)]
    mint_manager: Box<Account<'info, MintManager>>,
    #[account(constraint = mint_manager.in_use_by.expect("Token not in use") == user.key() @ ErrorCode::InvalidTokenUser)]
    user: Signer<'info>,
}

pub fn handler(ctx: Context<RemoveInUseByCtx>) -> Result<()> {
    let mint_manager = &mut ctx.accounts.mint_manager;
    mint_manager.in_use_by = None;
    Ok(())
}

use crate::errors::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SetInUseByIx {
    pub in_use_by: Pubkey,
}

#[derive(Accounts)]
pub struct SetInUseByCtx<'info> {
    #[account(mut)]
    mint_manager: Box<Account<'info, MintManager>>,
    holder: Signer<'info>,

    #[account(constraint = holder_token_account.amount == 1 && holder_token_account.owner == holder.key() && holder_token_account.mint == mint_manager.mint @ ErrorCode::InvalidHolderTokenAccount)]
    holder_token_account: Box<Account<'info, TokenAccount>>,
}

pub fn handler(ctx: Context<SetInUseByCtx>, ix: SetInUseByIx) -> Result<()> {
    let mint_manager = &mut ctx.accounts.mint_manager;

    if mint_manager.in_use_by.is_some() {
        return Err(error!(ErrorCode::TokenAlreadyInUse));
    }
    mint_manager.in_use_by = Some(ix.in_use_by);
    Ok(())
}

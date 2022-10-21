// giannis
use crate::errors::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, CloseAccount, Token, TokenAccount};

#[derive(Accounts)]
pub struct CloseCtx<'info> {
    mint_manager: Account<'info, MintManager>,
    /// CHECK: Account is not read from
    #[account(mut)]
    mint: UncheckedAccount<'info>,

    #[account(mut, constraint = token_account.owner == owner.key() @ ErrorCode::InvalidCloseTokenAccount)]
    token_account: Account<'info, TokenAccount>,
    owner: Signer<'info>,

    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CloseCtx>) -> Result<()> {
    let cpi_accounts = CloseAccount {
        account: ctx.accounts.token_account.to_account_info(),
        destination: ctx.accounts.owner.to_account_info(),
        authority: ctx.accounts.owner.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    token::close_account(cpi_context)?;

    Ok(())
}

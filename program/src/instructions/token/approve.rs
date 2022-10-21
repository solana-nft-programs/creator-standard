use anchor_spl::token::{self, Approve, FreezeAccount, Mint, ThawAccount, Token, TokenAccount};
use mpl_token_metadata::utils::assert_derivation;

use {
    crate::{errors::ErrorCode, state::*},
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
pub struct ApproveCtx<'info> {
    #[account(mut, seeds = [MINT_MANAGER_SEED.as_bytes(), mint.key().as_ref()], bump)]
    mint_manager: Box<Account<'info, MintManager>>,
    #[account(constraint = mint.key() == mint_manager.mint @ ErrorCode::InvalidMint)]
    mint: Box<Account<'info, Mint>>,

    #[account(mut, constraint =
        holder_token_account.owner == holder.key()
        && holder_token_account.mint == mint_manager.mint
        && holder_token_account.amount > 1
        && holder_token_account.delegate.is_none()
        @ ErrorCode::InvalidHolderTokenAccount
    )]
    holder_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    holder: Signer<'info>,
    /// CHECK: Account is not read from
    #[account(mut)]
    delegate: UncheckedAccount<'info>,

    token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<ApproveCtx>) -> Result<()> {
    let mint = ctx.accounts.mint.key();
    let path = &[MINT_MANAGER_SEED.as_bytes(), mint.as_ref()];
    let bump_seed = assert_derivation(
        ctx.program_id,
        &ctx.accounts.mint_manager.to_account_info(),
        path,
    )?;
    let mint_manager_seeds = &[MINT_MANAGER_SEED.as_bytes(), mint.as_ref(), &[bump_seed]];
    let mint_manager_signer = &[&mint_manager_seeds[..]];

    let cpi_accounts = ThawAccount {
        account: ctx.accounts.holder_token_account.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        authority: ctx.accounts.mint_manager.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts).with_signer(mint_manager_signer);
    token::thaw_account(cpi_context)?;

    let cpi_accounts = Approve {
        to: ctx.accounts.holder_token_account.to_account_info(),
        delegate: ctx.accounts.delegate.to_account_info(),
        authority: ctx.accounts.holder.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    token::approve(cpi_context, 1)?;

    let cpi_accounts = FreezeAccount {
        account: ctx.accounts.holder_token_account.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        authority: ctx.accounts.mint_manager.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts).with_signer(mint_manager_signer);
    token::freeze_account(cpi_context)?;

    Ok(())
}

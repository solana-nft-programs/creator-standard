use crate::errors::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::associated_token::{self, AssociatedToken};
use anchor_spl::token::TokenAccount;
use anchor_spl::token::{self};
use anchor_spl::token::{MintTo, Token};
use solana_program::program_pack::Pack;
use solana_program::system_instruction::create_account;
use spl_associated_token_account::get_associated_token_address;

#[derive(Accounts)]
pub struct InitMintCtx<'info> {
    #[account(
        init,
        payer = payer,
        space = MINT_MANAGER_SIZE,
        seeds = [MINT_MANAGER_SEED.as_bytes(), mint.key().as_ref()],
        bump,
    )]
    mint_manager: Account<'info, MintManager>,
    /// CHECK: Account is not read from
    #[account(mut)]
    mint: UncheckedAccount<'info>,
    standard: Account<'info, Standard>,

    /// CHECK: Account created or checked in handler
    #[account(mut)]
    target_token_account: Account<'info, TokenAccount>,
    target: Signer<'info>,

    /// CHECK: Account is not read from
    #[account(mut)]
    collector: UncheckedAccount<'info>,
    authority: Signer<'info>,
    #[account(mut)]
    payer: Signer<'info>,

    rent: Sysvar<'info, Rent>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitMintCtx>) -> Result<()> {
    let mint_manager = &mut ctx.accounts.mint_manager;
    mint_manager.bump = *ctx.bumps.get("mint_manager").unwrap();
    mint_manager.version = 0;
    mint_manager.authority = ctx.accounts.authority.key();
    mint_manager.mint = ctx.accounts.mint.key();
    mint_manager.standard = ctx.accounts.standard.key();

    // Create Mint
    invoke(
        &create_account(
            ctx.accounts.payer.key,
            ctx.accounts.mint.key,
            ctx.accounts
                .rent
                .minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN as u64,
            &spl_token::id(),
        ),
        &[
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.mint.to_account_info(),
        ],
    )?;

    // Initialize mint
    let cpi_accounts = token::InitializeMint {
        mint: ctx.accounts.mint.to_account_info(),
        rent: ctx.accounts.rent.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    token::initialize_mint(
        cpi_context,
        0,
        &ctx.accounts.mint_manager.key(),
        Some(&ctx.accounts.mint_manager.key()),
    )?;

    // Check/Create ATA
    let associated_token_account =
        get_associated_token_address(&ctx.accounts.target.key(), &ctx.accounts.mint.key());
    if associated_token_account != ctx.accounts.target_token_account.key() {
        return Err(error!(ErrorCode::InvalidTargetTokenAccount));
    }
    if ctx
        .accounts
        .target_token_account
        .to_account_info()
        .data_is_empty()
    {
        let cpi_accounts = associated_token::Create {
            payer: ctx.accounts.payer.to_account_info(),
            associated_token: ctx.accounts.target_token_account.to_account_info(),
            authority: ctx.accounts.target.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        associated_token::create(cpi_context)?;
    } else if ctx.accounts.target_token_account.mint != ctx.accounts.mint_manager.mint
        || ctx.accounts.target_token_account.owner != ctx.accounts.target.key()
        || ctx.accounts.target_token_account.amount != 0
    {
        return Err(error!(ErrorCode::InvalidTargetTokenAccount));
    }

    // mint to
    let mint_manager_key = ctx.accounts.mint.key();
    let mint_manager_seeds = &[
        MINT_MANAGER_SEED.as_bytes(),
        mint_manager_key.as_ref(),
        &[ctx.accounts.mint_manager.bump],
    ];
    let mint_manager_signer = &[&mint_manager_seeds[..]];
    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.target_token_account.to_account_info(),
        authority: ctx.accounts.mint_manager.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts).with_signer(mint_manager_signer);
    token::mint_to(cpi_context, 1)?;

    Ok(())
}

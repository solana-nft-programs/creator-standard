use crate::errors::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::CloseAccount;
use anchor_spl::token::FreezeAccount;
use anchor_spl::token::Mint;
use anchor_spl::token::ThawAccount;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::Transfer;
use anchor_spl::token::{self};
use solana_program::serialize_utils::read_u16;
use solana_program::sysvar::instructions::load_instruction_at_checked;
use solana_program::sysvar::{self};
use std::collections::HashSet;

#[derive(Accounts)]
pub struct TransferCtx<'info> {
    #[account(constraint = mint.key() == mint_manager.mint @ ErrorCode::InvalidMintManager)]
    mint_manager: Box<Account<'info, MintManager>>,
    #[account(constraint = ruleset.key() == mint_manager.ruleset @ ErrorCode::InvalidRuleset)]
    ruleset: Account<'info, Ruleset>,
    mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    from: Account<'info, TokenAccount>,
    #[account(mut)]
    to: Account<'info, TokenAccount>,

    authority: Signer<'info>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
    /// CHECK: This is not dangerous because the ID is checked with instructions sysvar
    #[account(address = sysvar::instructions::id())]
    instructions: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<TransferCtx>) -> Result<()> {
    let instructions_account_info = ctx.accounts.instructions.to_account_info();
    let instruction_sysvar = instructions_account_info.try_borrow_data()?;
    let mut current: usize = 0;
    let num_instructions =
        read_u16(&mut current, &instruction_sysvar).expect("Invalid instruction");

    // check if the token is currenlty being used
    if ctx.accounts.mint_manager.in_use_by.is_some() {
        return Err(error!(ErrorCode::TokenCurentlyInUse));
    }

    /////////////// check allowed / disallowed ///////////////
    let mut allowed_programs = HashSet::new();
    for program_id in &ctx.accounts.ruleset.allowed_programs {
        allowed_programs.insert(program_id);
    }

    let mut disallowed_addresses = HashSet::new();
    for program_id in &ctx.accounts.ruleset.disallowed_addresses {
        disallowed_addresses.insert(program_id);
    }

    for i in 0..num_instructions {
        let ix = load_instruction_at_checked(i.into(), &instructions_account_info)
            .expect("Failed to get instruction");

        if allowed_programs.len() > 0
            && !is_default_program(ix.program_id)
            && !allowed_programs.contains(&ix.program_id)
        {
            return Err(error!(ErrorCode::ProgramNotAllowed));
        }

        for account in ix.accounts {
            if disallowed_addresses.len() > 0 && disallowed_addresses.contains(&account.pubkey) {
                return Err(error!(ErrorCode::ProgramDisallowed));
            }
        }
    }
    ////////////////////////////////////////////////////////////

    ///////////////// handle transfer /////////////////
    let mint = ctx.accounts.mint.key();
    let mint_manager_seeds = &[
        MINT_MANAGER_SEED.as_bytes(),
        mint.as_ref(),
        &[ctx.accounts.mint_manager.bump],
    ];
    let mint_manager_signer = &[&mint_manager_seeds[..]];

    let cpi_accounts = ThawAccount {
        account: ctx.accounts.from.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        authority: ctx.accounts.mint_manager.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts).with_signer(mint_manager_signer);
    token::thaw_account(cpi_context)?;

    let cpi_accounts = Transfer {
        from: ctx.accounts.from.to_account_info(),
        to: ctx.accounts.to.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_context, 1)?;

    let cpi_accounts = FreezeAccount {
        account: ctx.accounts.to.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        authority: ctx.accounts.mint_manager.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts).with_signer(mint_manager_signer);
    token::freeze_account(cpi_context)?;

    // close from token account
    if ctx.accounts.authority.key() == ctx.accounts.from.owner
        || ctx.accounts.from.close_authority.is_some()
            && ctx.accounts.authority.key()
                == ctx
                    .accounts
                    .from
                    .close_authority
                    .expect("Invalid close authority")
    {
        let cpi_accounts = CloseAccount {
            account: ctx.accounts.from.to_account_info(),
            destination: ctx.accounts.authority.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        token::close_account(cpi_context)?;
    }
    ///////////////////////////////////////////////////
    Ok(())
}

use std::collections::HashSet;

use crate::errors::ErrorCode;
use crate::state::AccountType;
use crate::state::CreatorStandardAccount;
use crate::state::MintManager;
use crate::state::Ruleset;
use crate::state::assert_mint_manager_seeds;
use crate::state::is_default_program;
use crate::utils::assert_address;
use crate::utils::assert_mut;
use crate::utils::assert_program_account;
use crate::utils::assert_signer;
use crate::utils::unpack_checked_token_account;
use solana_program::account_info::next_account_info;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::invoke_signed;
use solana_program::program_error::ProgramError;
use solana_program::serialize_utils::read_u16;
use solana_program::system_program;
use solana_program::sysvar;
use solana_program::sysvar::instructions::load_instruction_at_checked;

pub struct TransferCtx<'a, 'info> {
    pub mint_manager: &'a AccountInfo<'info>,
    pub ruleset: &'a AccountInfo<'info>,
    pub mint: &'a AccountInfo<'info>,
    pub from: &'a AccountInfo<'info>,
    pub to: &'a AccountInfo<'info>,
    pub authority: &'a AccountInfo<'info>,
    pub token_program: &'a AccountInfo<'info>,
    pub system_program: &'a AccountInfo<'info>,
    pub instructions: &'a AccountInfo<'info>,
}

impl<'a, 'info> TransferCtx<'a, 'info> {
    pub fn load(accounts: &'a [AccountInfo<'info>]) -> Result<Self, ProgramError> {
        let account_iter = &mut accounts.iter();
        let ctx = Self {
            mint_manager: next_account_info(account_iter)?,
            ruleset: next_account_info(account_iter)?,
            mint: next_account_info(account_iter)?,
            from: next_account_info(account_iter)?,
            to: next_account_info(account_iter)?,
            authority: next_account_info(account_iter)?,
            token_program: next_account_info(account_iter)?,
            system_program: next_account_info(account_iter)?,
            instructions: next_account_info(account_iter)?,
        };
        // deserializations
        let mint_manager: MintManager = MintManager::from_account_info(ctx.mint_manager)?;

        // mint_manager
        assert_address(&mint_manager.mint, &ctx.mint.key, "mint_manager mint")?;

        // ruleset
        assert_address(
            &mint_manager.ruleset,
            &ctx.ruleset.key,
            "mint_manager ruleset",
        )?;
        assert_program_account(ctx.ruleset, &AccountType::Ruleset)?;

        ///// no checks for mint /////

        // from
        assert_mut(ctx.from, "from")?;
        unpack_checked_token_account(ctx.from, Some("from"))?;

        // to
        assert_mut(ctx.from, "to")?;
        unpack_checked_token_account(ctx.to, Some("to"))?;

        // authority
        assert_signer(ctx.authority, "authority")?;

        // token_program
        assert_address(ctx.token_program.key, &spl_token::id(), "token_program")?;

        // system_program
        assert_address(
            ctx.system_program.key,
            &system_program::id(),
            "system_program",
        )?;

        // instructions
        assert_address(
            ctx.instructions.key,
            &sysvar::instructions::id(),
            "instructions",
        )?;

        Ok(ctx)
    }
}

pub fn handler(ctx: TransferCtx) -> ProgramResult {
    let ruleset: Ruleset = Ruleset::from_account_info(ctx.ruleset)?;
    let mint_manager: MintManager = MintManager::from_account_info(ctx.mint_manager)?;
    let mint_manager_seeds = assert_mint_manager_seeds(ctx.mint.key, ctx.mint_manager.key)?;
    let from_account = unpack_checked_token_account(ctx.from, Some("from"))?;

    // instruction_sysvar
    let instruction_sysvar = ctx.instructions.try_borrow_data()?;

    let mut current: usize = 0;
    let num_instructions =
        read_u16(&mut current, &instruction_sysvar).expect("Invalid instruction");

    // check if the token is currenlty being used
    if mint_manager.in_use_by.is_some() {
        return Err(ProgramError::from(ErrorCode::TokenCurentlyInUse));
    }

    /////////////// check allowed / disallowed ///////////////
    let mut allowed_programs = HashSet::new();
    for program_id in &ruleset.allowed_programs {
        allowed_programs.insert(program_id);
    }

    let mut disallowed_addresses = HashSet::new();
    for program_id in &ruleset.disallowed_addresses {
        disallowed_addresses.insert(program_id);
    }

    for i in 0..num_instructions {
        let ix = load_instruction_at_checked(i.into(), &ctx.instructions)
            .expect("Failed to get instruction");

        if allowed_programs.len() > 0
            && !is_default_program(ix.program_id)
            && !allowed_programs.contains(&ix.program_id)
        {
            return Err(ProgramError::from(ErrorCode::ProgramNotAllowed));
        }

        for account in ix.accounts {
            if disallowed_addresses.len() > 0 && disallowed_addresses.contains(&account.pubkey) {
                return Err(ProgramError::from(ErrorCode::ProgramDisallowed));
            }
        }
    }
    ////////////////////////////////////////////////////////////

    ///////////////// handle transfer /////////////////

    // thaw account
    invoke_signed(
        &spl_token::instruction::thaw_account(
            ctx.token_program.key,
            ctx.from.key,
            ctx.mint.key,
            ctx.mint_manager.key,
            &[],
        )?,
        &[ctx.from.clone(), ctx.mint.clone(), ctx.mint_manager.clone()],
        &[&mint_manager_seeds
            .iter()
            .map(|s| s.as_slice())
            .collect::<Vec<&[u8]>>()],
    )?;

    // transfer
    invoke_signed(
        &spl_token::instruction::transfer(
            ctx.token_program.key,
            ctx.from.key,
            ctx.to.key,
            ctx.authority.key,
            &[],
            1,
        )?,
        &[ctx.from.clone(), ctx.mint.clone(), ctx.mint_manager.clone()],
        &[&mint_manager_seeds
            .iter()
            .map(|s| s.as_slice())
            .collect::<Vec<&[u8]>>()],
    )?;

    // freeze account
    invoke_signed(
        &spl_token::instruction::freeze_account(
            ctx.token_program.key,
            ctx.to.key,
            ctx.mint.key,
            ctx.mint_manager.key,
            &[],
        )?,
        &[ctx.to.clone(), ctx.mint.clone(), ctx.mint_manager.clone()],
        &[&mint_manager_seeds
            .iter()
            .map(|s| s.as_slice())
            .collect::<Vec<&[u8]>>()],
    )?;

    // close from token account
    if ctx.authority.key == &from_account.owner
        || from_account.close_authority.is_some()
            && ctx.authority.key
                == &from_account
                    .close_authority
                    .expect("Invalid close authority")
    {
        // close account
        invoke_signed(
            &spl_token::instruction::close_account(
                ctx.token_program.key,
                ctx.from.key,
                ctx.authority.key,
                ctx.authority.key,
                &[],
            )?,
            &[
                ctx.from.clone(),
                ctx.authority.clone(),
                ctx.authority.clone(),
            ],
            &[],
        )?;
    }
    ///////////////////////////////////////////////////

    Ok(())
}

use std::str::FromStr;

use crate::errors::ErrorCode;
use crate::id;
use crate::state::assert_mint_manager_seeds;
use crate::state::CreatorStandardAccount;
use crate::state::MintManager;
use crate::state::Ruleset;
use crate::state::COLLECTOR;
use crate::state::MINT_MANAGER_SIZE;
use crate::utils::assert_address;
use crate::utils::assert_amount;
use crate::utils::assert_empty;
use crate::utils::assert_mut;
use crate::utils::assert_signer;
use crate::utils::unpack_checked_mint_account;
use crate::utils::unpack_checked_token_account;
use solana_program::account_info::next_account_info;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::invoke;
use solana_program::program::invoke_signed;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::system_instruction::create_account;
use solana_program::system_program;
use solana_program::sysvar::Sysvar;

pub struct InitMintManagerCtx<'a, 'info> {
    pub mint_manager: &'a AccountInfo<'info>,
    pub mint: &'a AccountInfo<'info>,
    pub ruleset: &'a AccountInfo<'info>,
    pub holder_token_account: &'a AccountInfo<'info>,
    pub ruleset_collector: &'a AccountInfo<'info>,
    pub collector: &'a AccountInfo<'info>,
    pub authority: &'a AccountInfo<'info>,
    pub payer: &'a AccountInfo<'info>,
    pub token_program: &'a AccountInfo<'info>,
    pub system_program: &'a AccountInfo<'info>,
}

impl<'a, 'info> InitMintManagerCtx<'a, 'info> {
    pub fn load(accounts: &'a [AccountInfo<'info>]) -> Result<Self, ProgramError> {
        let account_iter = &mut accounts.iter();
        let ctx = Self {
            mint_manager: next_account_info(account_iter)?,
            mint: next_account_info(account_iter)?,
            ruleset: next_account_info(account_iter)?,
            holder_token_account: next_account_info(account_iter)?,
            ruleset_collector: next_account_info(account_iter)?,
            collector: next_account_info(account_iter)?,
            authority: next_account_info(account_iter)?,
            payer: next_account_info(account_iter)?,
            token_program: next_account_info(account_iter)?,
            system_program: next_account_info(account_iter)?,
        };
        // deserializations
        let ruleset: Ruleset = Ruleset::from_account_info(ctx.ruleset)?;
        let holder_token_account =
            unpack_checked_token_account(ctx.holder_token_account, Some("holder_token_account"))?;

        // mint_manager
        assert_mut(ctx.mint_manager, "mint_manager")?;
        assert_empty(ctx.mint_manager, "mint_manager")?;

        // check valid mint
        assert_mut(ctx.mint, "mint")?;
        unpack_checked_mint_account(ctx.mint, Some("token mint"))?;

        ///// no checks for ruleset /////

        // holder_token_account
        assert_mut(ctx.holder_token_account, "holder_token_account")?;
        assert_amount(
            &holder_token_account.amount.to_string(),
            "1",
            "holder_token_account",
        )?;
        assert_address(
            &holder_token_account.mint,
            ctx.mint.key,
            "holder_token_account mint",
        )?;
        assert_address(
            &holder_token_account.owner,
            &ctx.authority.key,
            "holder_token_account owner",
        )?;

        // ruleset_collector
        assert_mut(ctx.ruleset_collector, "ruleset_collector")?;
        assert_address(
            &ctx.ruleset_collector.key,
            &ruleset.collector,
            "ruleset_collector",
        )?;

        // collector
        assert_mut(ctx.collector, "collector")?;
        assert_address(
            &ctx.collector.key,
            &Pubkey::from_str(COLLECTOR).expect("Invalid collector pubkey"),
            "collector",
        )?;

        // authority
        assert_signer(ctx.authority, "authority")?;

        // payer
        assert_signer(ctx.payer, "payer")?;
        assert_mut(ctx.payer, "payer")?;

        // token_program
        assert_address(ctx.token_program.key, &spl_token::id(), "token_program")?;

        // system_program
        assert_address(
            ctx.system_program.key,
            &system_program::id(),
            "system_program",
        )?;
        Ok(ctx)
    }
}

pub fn handler(ctx: InitMintManagerCtx) -> ProgramResult {
    let mint_manager_space = MINT_MANAGER_SIZE;
    let mint_manager_seeds = assert_mint_manager_seeds(ctx.mint.key, ctx.mint_manager.key)?;
    // create mint manager account
    invoke_signed(
        &create_account(
            ctx.payer.key,
            ctx.mint_manager.key,
            Rent::get()?.minimum_balance(mint_manager_space),
            u64::try_from(mint_manager_space).expect("Could not cast to u64"),
            &id(),
        ),
        &[ctx.payer.clone(), ctx.mint_manager.clone()],
        &[&mint_manager_seeds
            .iter()
            .map(|s| s.as_slice())
            .collect::<Vec<&[u8]>>()],
    )?;

    let mut mint_manager: MintManager = MintManager::new();
    mint_manager.version = 0;
    mint_manager.mint = *ctx.mint.key;
    mint_manager.authority = *ctx.authority.key;
    mint_manager.ruleset = *ctx.ruleset.key;
    mint_manager.in_use_by = None;

    let mint = unpack_checked_mint_account(ctx.mint, Some("mint"))?;

    if mint.supply != 1 || mint.decimals != 0 {
        return Err(ProgramError::from(ErrorCode::InvalidMint));
    }

    // set mint authority
    invoke(
        &spl_token::instruction::set_authority(
            ctx.token_program.key,
            ctx.mint.key,
            Some(ctx.mint_manager.key),
            spl_token::instruction::AuthorityType::MintTokens,
            ctx.authority.key,
            &[],
        )?,
        &[ctx.mint.clone(), ctx.authority.clone()],
    )?;

    // set freeze authoriy
    invoke(
        &spl_token::instruction::set_authority(
            ctx.token_program.key,
            ctx.mint.key,
            Some(ctx.mint_manager.key),
            spl_token::instruction::AuthorityType::FreezeAccount,
            ctx.authority.key,
            &[],
        )?,
        &[ctx.mint.clone(), ctx.authority.clone()],
    )?;

    // freeze holder token account
    invoke_signed(
        &spl_token::instruction::freeze_account(
            ctx.token_program.key,
            ctx.holder_token_account.key,
            ctx.mint.key,
            ctx.mint_manager.key,
            &[],
        )?,
        &[
            ctx.holder_token_account.clone(),
            ctx.mint.clone(),
            ctx.mint_manager.clone(),
        ],
        &[&mint_manager_seeds
            .iter()
            .map(|s| s.as_slice())
            .collect::<Vec<&[u8]>>()],
    )?;

    mint_manager.save(ctx.mint_manager)?;
    Ok(())
}

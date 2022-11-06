use std::str::FromStr;

use crate::state::assert_mint_manager_seeds;
use crate::state::mint_manager;
use crate::state::ruleset;
use crate::state::shared::CreatorStandardAccount;
use crate::state::COLLECTOR;
use crate::state::MINT_MANAGER_SIZE;
use crate::utils::assert_address;
use crate::utils::assert_empty;
use crate::utils::assert_mut;
use crate::utils::assert_signer;
use crate::utils::assert_valid_mint_account;
use crate::utils::assert_valid_token_account;
use mint_manager::MintManager;
use mpl_token_metadata::utils::create_or_allocate_account_raw;
use ruleset::Ruleset;
use solana_program::account_info::next_account_info;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::system_program;

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
            assert_valid_token_account(ctx.holder_token_account, Some("holder"))?;

        // mint_manager
        assert_mut(ctx.mint_manager, "mint_manager")?;
        assert_empty(ctx.mint_manager, "mint_manager")?;

        // check valid mint
        assert_valid_mint_account(ctx.mint, Some("token mint"))?;

        ///// no checks for ruleset /////

        // holder_token_account
        assert_mut(ctx.holder_token_account, "holder_token_account")?;
        assert_address(
            &holder_token_account.mint,
            ctx.mint.key,
            "holder_token_account mint",
        )?;

        // ruleset_collector
        assert_mut(ctx.ruleset_collector, "ruleset collector")?;
        assert_address(
            &ctx.ruleset_collector.key,
            &ruleset.collector,
            "ruleset collector",
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
    let seeds = assert_mint_manager_seeds(ctx.mint.key, ctx.mint_manager.key)?;
    let space = MINT_MANAGER_SIZE;
    create_or_allocate_account_raw(
        crate::id(),
        ctx.mint_manager,
        ctx.system_program,
        ctx.payer,
        space,
        &seeds.iter().map(|s| s.as_slice()).collect::<Vec<&[u8]>>(),
    )?;

    let mut mint_manager: MintManager = MintManager::from_account_info(ctx.mint_manager)?;
    mint_manager.set_account_type();
    mint_manager.version = 0;
    mint_manager.mint = *ctx.mint.key;
    mint_manager.authority = *ctx.authority.key;
    mint_manager.ruleset = *ctx.ruleset.key;
    mint_manager.in_use_by = None;
    // let mint_manager = MintManager {
    //     account_type: 1,
    //     version: 0,
    //     mint: *ctx.mint.key,
    //     authority: *ctx.authority.key,
    //     ruleset: *ctx.ruleset.key,
    //     in_use_by: None,
    // };
    // // MintManager::save(mint_manager.)
    // BorshSerialize::serialize(&mint_manager, &mut *ctx.mint_manager.data.borrow_mut())?;
    Ok(())
}

use crate::errors::ErrorCode;
use crate::id;
use crate::state::assert_mint_manager_seeds;
use crate::state::CreatorStandardAccount;
use crate::state::MintManager;
use crate::state::Ruleset;
use crate::state::COLLECTOR;
use crate::state::MINT_MANAGER_SIZE;
use crate::utils::assert_address;
use crate::utils::assert_empty;
use crate::utils::assert_mut;
use crate::utils::assert_signer;
use crate::utils::unpack_checked_token_account;
use solana_program::account_info::next_account_info;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::invoke;
use solana_program::program::invoke_signed;
use solana_program::program_error::ProgramError;
use solana_program::program_pack::Pack;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::system_instruction::create_account;
use solana_program::system_program;
use solana_program::sysvar;
use solana_program::sysvar::Sysvar;
use spl_associated_token_account::get_associated_token_address;
use spl_associated_token_account::instruction::create_associated_token_account;
use std::str::FromStr;

pub struct InitializeMintCtx<'a, 'info> {
    pub mint_manager: &'a AccountInfo<'info>,
    pub mint: &'a AccountInfo<'info>,
    pub ruleset: &'a AccountInfo<'info>,
    pub target_token_account: &'a AccountInfo<'info>,
    pub target: &'a AccountInfo<'info>,
    pub ruleset_collector: &'a AccountInfo<'info>,
    pub collector: &'a AccountInfo<'info>,
    pub authority: &'a AccountInfo<'info>,
    pub payer: &'a AccountInfo<'info>,
    pub rent: &'a AccountInfo<'info>,
    pub token_program: &'a AccountInfo<'info>,
    pub associated_token_program: &'a AccountInfo<'info>,
    pub system_program: &'a AccountInfo<'info>,
}

impl<'a, 'info> InitializeMintCtx<'a, 'info> {
    pub fn load(accounts: &'a [AccountInfo<'info>]) -> Result<Self, ProgramError> {
        let account_iter = &mut accounts.iter();
        let ctx = Self {
            mint_manager: next_account_info(account_iter)?,
            mint: next_account_info(account_iter)?,
            ruleset: next_account_info(account_iter)?,
            target_token_account: next_account_info(account_iter)?,
            target: next_account_info(account_iter)?,
            ruleset_collector: next_account_info(account_iter)?,
            collector: next_account_info(account_iter)?,
            authority: next_account_info(account_iter)?,
            payer: next_account_info(account_iter)?,
            rent: next_account_info(account_iter)?,
            token_program: next_account_info(account_iter)?,
            associated_token_program: next_account_info(account_iter)?,
            system_program: next_account_info(account_iter)?,
        };
        // deserializations
        let ruleset: Ruleset = Ruleset::from_account_info(ctx.ruleset)?;

        // mint_manager
        assert_mut(ctx.mint_manager, "mint_manager")?;
        assert_empty(ctx.mint_manager, "mint_manager")?;

        // check valid mint
        assert_mut(ctx.mint, "mint")?;
        assert_signer(ctx.mint, "mint")?;
        assert_empty(ctx.mint, "mint")?;

        ///// no checks for ruleset /////

        // target_token_account
        assert_mut(ctx.target_token_account, "target_token_account")?;

        // target
        assert_signer(ctx.target, "target")?;

        // ruleset_collector
        assert_mut(ctx.ruleset_collector, "ruleset_collector")?;
        assert_address(
            ctx.ruleset_collector.key,
            &ruleset.collector,
            "ruleset_collector",
        )?;

        // collector
        assert_mut(ctx.collector, "collector")?;
        assert_address(
            ctx.collector.key,
            &Pubkey::from_str(COLLECTOR).expect("Invalid collector pubkey"),
            "collector",
        )?;

        // authority
        assert_signer(ctx.authority, "authority")?;

        // payer
        assert_signer(ctx.payer, "payer")?;
        assert_mut(ctx.payer, "payer")?;

        // rent
        assert_address(ctx.rent.key, &sysvar::rent::id(), "rent")?;

        // token_program
        assert_address(ctx.token_program.key, &spl_token::id(), "token_program")?;

        // associated_token_program
        assert_address(
            ctx.associated_token_program.key,
            &spl_associated_token_account::id(),
            "associated_token_program",
        )?;

        // system_program
        assert_address(
            ctx.system_program.key,
            &system_program::id(),
            "system_program",
        )?;
        Ok(ctx)
    }
}

pub fn handler(ctx: InitializeMintCtx) -> ProgramResult {
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

    // create mint account
    invoke(
        &create_account(
            ctx.payer.key,
            ctx.mint.key,
            Rent::get()?.minimum_balance(spl_token::state::Mint::LEN),
            u64::try_from(spl_token::state::Mint::LEN).expect("Could not cast to u64"),
            &spl_token::id(),
        ),
        &[ctx.payer.clone(), ctx.mint.clone()],
    )?;

    // initialize mint
    invoke_signed(
        &spl_token::instruction::initialize_mint(
            ctx.token_program.key,
            ctx.mint.key,
            ctx.mint_manager.key,
            Some(ctx.mint_manager.key),
            0,
        )?,
        &[ctx.mint.clone(), ctx.rent.clone()],
        &[&mint_manager_seeds
            .iter()
            .map(|s| s.as_slice())
            .collect::<Vec<&[u8]>>()],
    )?;

    // Check/Create ATA
    let associated_token_account = get_associated_token_address(ctx.target.key, ctx.mint.key);
    if associated_token_account != *ctx.target_token_account.key {
        return Err(ProgramError::from(ErrorCode::InvalidTargetTokenAccount));
    }
    if ctx.target_token_account.data_is_empty() {
        invoke_signed(
            &create_associated_token_account(ctx.payer.key, ctx.target.key, ctx.mint.key),
            &[
                ctx.payer.clone(),
                ctx.target_token_account.clone(),
                ctx.target.clone(),
                ctx.mint.clone(),
                ctx.system_program.clone(),
                ctx.rent.clone(),
            ],
            &[],
        )?;
    } else {
        // check valid target token account
        unpack_checked_token_account(ctx.target_token_account, Some("target_token_account"))?;
    }

    // mint to
    invoke_signed(
        &spl_token::instruction::mint_to(
            ctx.token_program.key,
            ctx.mint.key,
            ctx.target_token_account.key,
            ctx.mint_manager.key,
            &[],
            1,
        )?,
        &[
            ctx.target_token_account.clone(),
            ctx.mint.clone(),
            ctx.mint_manager.clone(),
        ],
        &[&mint_manager_seeds
            .iter()
            .map(|s| s.as_slice())
            .collect::<Vec<&[u8]>>()],
    )?;

    // freeze account
    invoke_signed(
        &spl_token::instruction::freeze_account(
            ctx.token_program.key,
            ctx.target_token_account.key,
            ctx.mint.key,
            ctx.mint_manager.key,
            &[],
        )?,
        &[
            ctx.target_token_account.clone(),
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

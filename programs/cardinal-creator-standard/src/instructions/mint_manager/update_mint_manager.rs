use crate::state::mint_manager;
use crate::state::ruleset;
use crate::state::shared::CreatorStandardAccount;
use crate::state::UPDATE_LAMPORTS;
use crate::utils::assert_address;
use crate::utils::assert_mut;
use crate::utils::assert_signer;
use mint_manager::MintManager;
use ruleset::Ruleset;
use solana_program::account_info::next_account_info;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::invoke;
use solana_program::program_error::ProgramError;
use solana_program::system_instruction::transfer;
use solana_program::system_program;

pub struct UpdateMintManagerCtx<'a, 'info> {
    pub mint_manager: &'a AccountInfo<'info>,
    pub ruleset: &'a AccountInfo<'info>,
    pub collector: &'a AccountInfo<'info>,
    pub authority: &'a AccountInfo<'info>,
    pub payer: &'a AccountInfo<'info>,
    pub system_program: &'a AccountInfo<'info>,
}

impl<'a, 'info> UpdateMintManagerCtx<'a, 'info> {
    pub fn load(accounts: &'a [AccountInfo<'info>]) -> Result<Self, ProgramError> {
        let account_iter = &mut accounts.iter();
        let ctx = Self {
            mint_manager: next_account_info(account_iter)?,
            ruleset: next_account_info(account_iter)?,
            collector: next_account_info(account_iter)?,
            authority: next_account_info(account_iter)?,
            payer: next_account_info(account_iter)?,
            system_program: next_account_info(account_iter)?,
        };
        // deserializations
        let mint_manager: MintManager = MintManager::from_account_info(ctx.mint_manager)?;
        let ruleset: Ruleset = Ruleset::from_account_info(ctx.ruleset)?;

        // mint_manager
        assert_mut(ctx.mint_manager, "mint_manager")?;

        ///// no checks for ruleset /////

        // rulese_collector
        assert_mut(ctx.collector, "collector")?;
        assert_address(&ctx.collector.key, &ruleset.collector, "collector")?;

        // authority
        assert_signer(ctx.authority, "authority")?;
        assert_address(&ctx.authority.key, &mint_manager.authority, "authority")?;

        // payer
        assert_signer(ctx.payer, "payer")?;
        assert_mut(ctx.payer, "payer")?;

        // system_program
        assert_address(
            ctx.system_program.key,
            &system_program::id(),
            "system_program",
        )?;
        Ok(ctx)
    }
}

pub fn handler(ctx: UpdateMintManagerCtx) -> ProgramResult {
    let mut mint_manager: MintManager = MintManager::from_account_info(ctx.mint_manager)?;
    mint_manager.authority = *ctx.authority.key;
    mint_manager.ruleset = *ctx.ruleset.key;

    invoke(
        &transfer(&ctx.payer.key, &ctx.collector.key, UPDATE_LAMPORTS),
        &[
            ctx.payer.clone(),
            ctx.collector.clone(),
            ctx.system_program.clone(),
        ],
    )?;

    Ok(())
}

use crate::state::assert_mint_manager_seeds;
use crate::state::mint_manager;
use crate::state::MINT_MANAGER_SIZE;
use crate::utils::assert_address;
use crate::utils::assert_empty;
use crate::utils::assert_mut;
use crate::utils::assert_owner;
use crate::utils::assert_signer;
use borsh::BorshSerialize;
use mint_manager::MintManager;
use mpl_token_metadata::utils::create_or_allocate_account_raw;
use solana_program::account_info::next_account_info;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::system_program;

pub struct InitMintManagerCtx<'a, 'info> {
    pub mint: &'a AccountInfo<'info>,
    pub mint_manager: &'a AccountInfo<'info>,
    pub authority: &'a AccountInfo<'info>,
    pub ruleset: &'a AccountInfo<'info>,
    pub payer: &'a AccountInfo<'info>,
    pub token_program: &'a AccountInfo<'info>,
    pub system_program: &'a AccountInfo<'info>,
}

impl<'a, 'info> InitMintManagerCtx<'a, 'info> {
    pub fn load(accounts: &'a [AccountInfo<'info>]) -> Result<Self, ProgramError> {
        let account_iter = &mut accounts.iter();
        let ctx = Self {
            mint: next_account_info(account_iter)?,
            mint_manager: next_account_info(account_iter)?,
            authority: next_account_info(account_iter)?,
            ruleset: next_account_info(account_iter)?,
            payer: next_account_info(account_iter)?,
            token_program: next_account_info(account_iter)?,
            system_program: next_account_info(account_iter)?,
        };

        // mint
        assert_owner(ctx.mint, ctx.token_program.key, "mint")?;
        assert_mut(ctx.mint, "mint")?;

        // mint manager
        assert_mut(ctx.mint_manager, "mint_manager")?;
        assert_empty(ctx.mint_manager, "mint_manager")?;

        // authority
        assert_signer(ctx.authority, "authority")?;

        // payer
        assert_signer(ctx.payer, "payer")?;
        assert_mut(ctx.payer, "payer")?;

        // token_program
        assert_address(ctx.token_program, &spl_token::id(), "token_program")?;

        // system_program
        assert_address(ctx.system_program, &system_program::id(), "system_program")?;
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
    let mint_manager = MintManager {
        account_type: 1,
        version: 1,
        mint: *ctx.mint.key,
        authority: *ctx.authority.key,
        ruleset: *ctx.ruleset.key,
    };
    BorshSerialize::serialize(&mint_manager, &mut *ctx.mint_manager.data.borrow_mut())?;
    Ok(())
}

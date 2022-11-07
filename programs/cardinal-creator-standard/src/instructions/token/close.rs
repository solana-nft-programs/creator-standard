use crate::state::assert_mint_manager_seeds;
use crate::utils::assert_address;
use crate::utils::assert_mut;
use crate::utils::assert_signer;
use crate::utils::unpack_checked_token_account;
use solana_program::account_info::next_account_info;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::invoke_signed;
use solana_program::program_error::ProgramError;

pub struct CloseCtx<'a, 'info> {
    pub mint_manager: &'a AccountInfo<'info>,
    pub mint: &'a AccountInfo<'info>,
    pub token_account: &'a AccountInfo<'info>,
    pub owner: &'a AccountInfo<'info>,
    pub token_program: &'a AccountInfo<'info>,
}

impl<'a, 'info> CloseCtx<'a, 'info> {
    pub fn load(accounts: &'a [AccountInfo<'info>]) -> Result<Self, ProgramError> {
        let account_iter = &mut accounts.iter();
        let ctx = Self {
            mint_manager: next_account_info(account_iter)?,
            mint: next_account_info(account_iter)?,
            token_account: next_account_info(account_iter)?,
            owner: next_account_info(account_iter)?,
            token_program: next_account_info(account_iter)?,
        };
        // deserializations
        let token_account = unpack_checked_token_account(ctx.token_account, Some("token_account"))?;

        ///// no checks for mint_manager /////

        // mint
        assert_mut(ctx.mint, "mint")?;

        // token account
        assert_mut(ctx.token_account, "token_account")?;
        assert_address(&token_account.owner, ctx.owner.key, "token_account owner")?;

        // owner
        assert_signer(ctx.owner, "owner")?;

        // token_program
        assert_address(ctx.token_program.key, &spl_token::id(), "token_program")?;

        Ok(ctx)
    }
}

pub fn handler(ctx: CloseCtx) -> ProgramResult {
    let token_accuont = unpack_checked_token_account(ctx.token_account, Some("token account"))?;
    if token_accuont.is_frozen() {
        let mint_manager_seeds = assert_mint_manager_seeds(ctx.mint.key, ctx.mint_manager.key)?;

        // thaw account
        invoke_signed(
            &spl_token::instruction::thaw_account(
                ctx.token_program.key,
                ctx.token_account.key,
                ctx.mint.key,
                ctx.mint_manager.key,
                &[],
            )?,
            &[
                ctx.token_account.clone(),
                ctx.mint.clone(),
                ctx.mint_manager.clone(),
            ],
            &[&mint_manager_seeds
                .iter()
                .map(|s| s.as_slice())
                .collect::<Vec<&[u8]>>()],
        )?;
    }

    // close account
    invoke_signed(
        &spl_token::instruction::close_account(
            ctx.token_program.key,
            ctx.token_account.key,
            ctx.owner.key,
            ctx.owner.key,
            &[],
        )?,
        &[
            ctx.token_account.clone(),
            ctx.owner.clone(),
            ctx.owner.clone(),
        ],
        &[],
    )?;

    Ok(())
}

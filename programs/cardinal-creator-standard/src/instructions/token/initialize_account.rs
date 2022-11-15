use crate::utils::assert_address;
use crate::utils::assert_mut;
use crate::utils::assert_signer;
use crate::CreatorStandardInstruction;
use borsh::BorshSerialize;
use solana_program::account_info::next_account_info;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::instruction::AccountMeta;
use solana_program::instruction::Instruction;
use solana_program::program::invoke_signed;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::system_program;
use solana_program::sysvar;
use spl_associated_token_account::instruction::create_associated_token_account;

#[allow(clippy::too_many_arguments)]
pub fn initialize_account(
    program_id: Pubkey,
    mint: Pubkey,
    token_account: Pubkey,
    owner: Pubkey,
    payer: Pubkey,
    rent: Pubkey,
) -> Result<Instruction, ProgramError> {
    Ok(Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new(token_account, false),
            AccountMeta::new_readonly(owner, false),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(rent, false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(spl_associated_token_account::id(), false),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
        ],
        data: CreatorStandardInstruction::InitializeAccount.try_to_vec()?,
    })
}

pub struct InitializeAccountCtx<'a, 'info> {
    pub mint: &'a AccountInfo<'info>,
    pub token_account: &'a AccountInfo<'info>,
    pub owner: &'a AccountInfo<'info>,
    pub payer: &'a AccountInfo<'info>,
    pub rent: &'a AccountInfo<'info>,
    pub token_program: &'a AccountInfo<'info>,
    pub associated_token_program: &'a AccountInfo<'info>,
    pub system_program: &'a AccountInfo<'info>,
}

impl<'a, 'info> InitializeAccountCtx<'a, 'info> {
    pub fn load(accounts: &'a [AccountInfo<'info>]) -> Result<Self, ProgramError> {
        let account_iter = &mut accounts.iter();
        let ctx = Self {
            mint: next_account_info(account_iter)?,
            token_account: next_account_info(account_iter)?,
            owner: next_account_info(account_iter)?,
            payer: next_account_info(account_iter)?,
            rent: next_account_info(account_iter)?,
            token_program: next_account_info(account_iter)?,
            associated_token_program: next_account_info(account_iter)?,
            system_program: next_account_info(account_iter)?,
        };

        ///// no checks for mint_manager /////

        // token_account
        assert_mut(ctx.token_account, "token_account")?;

        ///// no checks for owner /////

        // payer
        assert_signer(ctx.payer, "payer")?;
        assert_mut(ctx.payer, "token_account")?;

        // rent
        assert_address(ctx.rent.key, &sysvar::rent::id(), "rent")?;

        // token_program
        assert_address(ctx.token_program.key, &spl_token::id(), "token_program")?;

        // associated_token_program
        assert_address(
            ctx.token_program.key,
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

pub fn handler(ctx: InitializeAccountCtx) -> ProgramResult {
    invoke_signed(
        &create_associated_token_account(ctx.payer.key, ctx.token_account.key, ctx.mint.key),
        &[
            ctx.payer.clone(),
            ctx.token_account.clone(),
            ctx.owner.clone(),
            ctx.mint.clone(),
            ctx.system_program.clone(),
            ctx.rent.clone(),
        ],
        &[],
    )?;
    Ok(())
}

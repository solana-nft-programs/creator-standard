use borsh::BorshSerialize;
use solana_program::account_info::next_account_info;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::instruction::AccountMeta;
use solana_program::instruction::Instruction;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::system_program;

use crate::utils::assert_with_msg;
use crate::CreatorStandardInstruction;

pub struct InitCtx<'a, 'info> {
    pub mint: &'a AccountInfo<'info>,
    pub authority: &'a AccountInfo<'info>,
    pub standard: &'a AccountInfo<'info>,
    pub token_program: &'a AccountInfo<'info>,
    pub system_program: &'a AccountInfo<'info>,
}

impl<'a, 'info> InitCtx<'a, 'info> {
    pub fn load(accounts: &'a [AccountInfo<'info>]) -> Result<Self, ProgramError> {
        let account_iter = &mut accounts.iter();
        let ctx = Self {
            mint: next_account_info(account_iter)?,
            authority: next_account_info(account_iter)?,
            standard: next_account_info(account_iter)?,
            token_program: next_account_info(account_iter)?,
            system_program: next_account_info(account_iter)?,
        };
        assert_with_msg(
            ctx.mint.data_is_empty(),
            ProgramError::InvalidAccountData,
            "Mint account must be uninitialized",
        )?;
        assert_with_msg(
            ctx.mint.owner == &system_program::id(),
            ProgramError::IllegalOwner,
            "Mint account must be owned by the System Program when uninitialized",
        )?;
        assert_with_msg(
            ctx.authority.is_writable,
            ProgramError::InvalidInstructionData,
            "Authority must sign for initialization",
        )?;
        assert_with_msg(
            ctx.token_program.key == &spl_token::id(),
            ProgramError::InvalidInstructionData,
            "Invalid key supplied for Token Program",
        )?;
        assert_with_msg(
            ctx.system_program.key == &system_program::id(),
            ProgramError::InvalidInstructionData,
            "Invalid key supplied for System Program",
        )?;
        Ok(ctx)
    }
}

pub fn handler(accounts: &[AccountInfo]) -> ProgramResult {
    let InitCtx {
        mint,
        authority,
        standard,
        token_program,
        system_program,
    } = InitCtx::load(accounts)?;
    Ok(())
}

pub fn init_ix(
    mint: &Pubkey,
    authority: &Pubkey,
    standard: &Pubkey,
) -> Result<Instruction, ProgramError> {
    Ok(Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new(*mint, false),
            AccountMeta::new(*authority, true),
            AccountMeta::new(*standard, true),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: CreatorStandardInstruction::Init.try_to_vec()?,
    })
}

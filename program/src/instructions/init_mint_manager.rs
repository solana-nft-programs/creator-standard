use crate::mint_manager_seeds;
use crate::utils::assert_with_msg;
use solana_program::account_info::next_account_info;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::invoke;
use solana_program::program_error::ProgramError;
use solana_program::program_pack::Pack;
use solana_program::rent::Rent;
use solana_program::system_instruction;
use solana_program::system_program;
use solana_program::sysvar::Sysvar;

pub struct InitMintManagerCtx<'a, 'info> {
    pub mint: &'a AccountInfo<'info>,
    pub mint_manager: &'a AccountInfo<'info>,
    pub authority: &'a AccountInfo<'info>,
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
            payer: next_account_info(account_iter)?,
            token_program: next_account_info(account_iter)?,
            system_program: next_account_info(account_iter)?,
        };
        assert_with_msg(
            ctx.mint.owner == ctx.token_program.key,
            ProgramError::IllegalOwner,
            "Invalid token program",
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

pub fn handler(ctx: InitMintManagerCtx) -> ProgramResult {
    let (mint_manager, _) = mint_manager_seeds(ctx.mint.key);
    let space = spl_token::state::Mint::LEN;
    invoke(
        &system_instruction::create_account(
            ctx.payer.key,
            &mint_manager,
            Rent::get()?.minimum_balance(space),
            space as u64,
            &crate::id(),
        ),
        &[
            ctx.payer.clone(),
            ctx.mint_manager.clone(),
            ctx.system_program.clone(),
        ],
    )?;
    let mut mint_manager = Mint::unpack_unchecked(&ctx.mint_manager.data.borrow())?;
    Ok(())
}

use crate::state::CreatorStandardAccount;
use crate::state::Ruleset;
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
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::system_program;

#[allow(clippy::too_many_arguments)]
pub fn close_ruleset(
    program_id: Pubkey,
    ruleset: Pubkey,
    authority: Pubkey,
) -> Result<Instruction, ProgramError> {
    Ok(Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(ruleset, false),
            AccountMeta::new(authority, true),
        ],
        data: CreatorStandardInstruction::CloseRuleset.try_to_vec()?,
    })
}

pub struct CloseRulesetCtx<'a, 'info> {
    pub ruleset: &'a AccountInfo<'info>,
    pub authority: &'a AccountInfo<'info>,
}

impl<'a, 'info> CloseRulesetCtx<'a, 'info> {
    pub fn load(accounts: &'a [AccountInfo<'info>]) -> Result<Self, ProgramError> {
        let account_iter = &mut accounts.iter();
        let ctx = Self {
            ruleset: next_account_info(account_iter)?,
            authority: next_account_info(account_iter)?,
        };
        // deserializations
        let ruleset: Ruleset = Ruleset::from_account_info(ctx.ruleset)?;

        // ruleset
        assert_mut(ctx.ruleset, "ruleset")?;

        // authority
        assert_signer(ctx.authority, "authority")?;
        assert_address(ctx.authority.key, &ruleset.authority, "authority")?;

        Ok(ctx)
    }
}

pub fn handler(ctx: CloseRulesetCtx) -> ProgramResult {
    let destination_starting_lamports = ctx.authority.lamports();
    **ctx.authority.lamports.borrow_mut() = destination_starting_lamports
        .checked_add(ctx.ruleset.lamports())
        .expect("Add error");
    **ctx.ruleset.lamports.borrow_mut() = 0;

    ctx.ruleset.assign(&system_program::id());
    ctx.ruleset
        .realloc(0, false)
        .expect("Error reallocating account");

    Ok(())
}

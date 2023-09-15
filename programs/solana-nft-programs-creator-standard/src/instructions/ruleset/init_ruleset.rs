use crate::id;
use crate::state::assert_ruleset_seeds;
use crate::state::calculate_ruleset_size;
use crate::CreatorStandardInstruction;

use crate::state::CreatorStandardAccount;
use crate::state::Ruleset;
use crate::utils::assert_address;
use crate::utils::assert_empty;
use crate::utils::assert_mut;
use crate::utils::assert_signer;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_program::account_info::next_account_info;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::instruction::AccountMeta;
use solana_program::instruction::Instruction;
use solana_program::program::invoke_signed;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::system_instruction::create_account;
use solana_program::system_program;
use solana_program::sysvar::Sysvar;

#[allow(clippy::too_many_arguments)]
pub fn init_ruleset(
    program_id: Pubkey,
    ruleset: Pubkey,
    authority: Pubkey,
    payer: Pubkey,
    name: String,
    allowed_programs: Vec<Pubkey>,
    disallowed_addresses: Vec<Pubkey>,
    extensions: Vec<Pubkey>,
) -> Result<Instruction, ProgramError> {
    Ok(Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(ruleset, false),
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
        ],
        data: CreatorStandardInstruction::InitRuleset(InitRulesetIx {
            name,
            disallowed_addresses,
            allowed_programs,
            extensions,
        })
        .try_to_vec()?,
    })
}

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct InitRulesetIx {
    pub name: String,
    pub allowed_programs: Vec<Pubkey>,
    pub disallowed_addresses: Vec<Pubkey>,
    pub extensions: Vec<Pubkey>,
}

pub struct InitRulesetCtx<'a, 'info> {
    pub ruleset: &'a AccountInfo<'info>,
    pub authority: &'a AccountInfo<'info>,
    pub payer: &'a AccountInfo<'info>,
    pub system_program: &'a AccountInfo<'info>,
}

impl<'a, 'info> InitRulesetCtx<'a, 'info> {
    pub fn load(accounts: &'a [AccountInfo<'info>]) -> Result<Self, ProgramError> {
        let account_iter = &mut accounts.iter();
        let ctx = Self {
            ruleset: next_account_info(account_iter)?,
            authority: next_account_info(account_iter)?,
            payer: next_account_info(account_iter)?,
            system_program: next_account_info(account_iter)?,
        };

        // ruleset
        assert_mut(ctx.ruleset, "ruleset")?;
        assert_empty(ctx.ruleset, "ruleset")?;

        // authority
        assert_signer(ctx.authority, "authority")?;

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

pub fn handler(ctx: InitRulesetCtx, ix: InitRulesetIx) -> ProgramResult {
    let ruleset_seeds = assert_ruleset_seeds(&ix.name, ctx.ruleset.key)?;
    let ruleset_space = calculate_ruleset_size(
        &ix.allowed_programs,
        &ix.disallowed_addresses,
        &ix.extensions,
    );
    invoke_signed(
        &create_account(
            ctx.payer.key,
            ctx.ruleset.key,
            Rent::get()?.minimum_balance(ruleset_space),
            u64::try_from(ruleset_space).expect("Could not cast to u64"),
            &id(),
        ),
        &[ctx.payer.clone(), ctx.ruleset.clone()],
        &[&ruleset_seeds
            .iter()
            .map(|s| s.as_slice())
            .collect::<Vec<&[u8]>>()],
    )?;

    let mut ruleset: Ruleset = Ruleset::new();
    ruleset.version = 0;
    ruleset.authority = *ctx.authority.key;
    ruleset.name = ix.name;
    ruleset.allowed_programs = ix.allowed_programs;
    ruleset.disallowed_addresses = ix.disallowed_addresses;
    ruleset.extensions = ix.extensions;
    ruleset.save(ctx.ruleset)?;

    Ok(())
}

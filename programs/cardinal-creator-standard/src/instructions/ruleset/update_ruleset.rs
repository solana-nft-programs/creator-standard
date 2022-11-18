use crate::state::calculate_ruleset_size;
use crate::state::CreatorStandardAccount;
use crate::state::Ruleset;
use crate::utils::assert_address;
use crate::utils::assert_mut;
use crate::utils::assert_signer;
use crate::CreatorStandardInstruction;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_program::account_info::next_account_info;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::instruction::AccountMeta;
use solana_program::instruction::Instruction;
use solana_program::program::invoke;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::system_instruction::transfer;
use solana_program::system_program;
use solana_program::sysvar::Sysvar;

#[allow(clippy::too_many_arguments)]
pub fn update_ruleset(
    program_id: Pubkey,
    ruleset: Pubkey,
    authority: Pubkey,
    payer: Pubkey,
    collector: Pubkey,
    check_seller_fee_basis_points: bool,
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
        data: CreatorStandardInstruction::UpdateRuleset(UpdateRulesetIx {
            authority,
            collector,
            check_seller_fee_basis_points,
            allowed_programs,
            disallowed_addresses,
            extensions,
        })
        .try_to_vec()?,
    })
}

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct UpdateRulesetIx {
    pub authority: Pubkey,
    pub collector: Pubkey,
    pub check_seller_fee_basis_points: bool,
    pub allowed_programs: Vec<Pubkey>,
    pub disallowed_addresses: Vec<Pubkey>,
    pub extensions: Vec<Pubkey>,
}

pub struct UpdateRulesetCtx<'a, 'info> {
    pub ruleset: &'a AccountInfo<'info>,
    pub authority: &'a AccountInfo<'info>,
    pub payer: &'a AccountInfo<'info>,
    pub system_program: &'a AccountInfo<'info>,
}

impl<'a, 'info> UpdateRulesetCtx<'a, 'info> {
    pub fn load(accounts: &'a [AccountInfo<'info>]) -> Result<Self, ProgramError> {
        let account_iter = &mut accounts.iter();
        let ctx = Self {
            ruleset: next_account_info(account_iter)?,
            authority: next_account_info(account_iter)?,
            payer: next_account_info(account_iter)?,
            system_program: next_account_info(account_iter)?,
        };
        // deserializations
        let ruleset: Ruleset = Ruleset::from_account_info(ctx.ruleset)?;

        // ruleset
        assert_mut(ctx.ruleset, "ruleset")?;

        // authority
        assert_signer(ctx.authority, "authority")?;
        assert_address(ctx.authority.key, &ruleset.authority, "authority")?;

        // payer
        assert_mut(ctx.payer, "payer")?;
        assert_signer(ctx.authority, "payer")?;

        // system_program
        assert_address(
            ctx.system_program.key,
            &system_program::id(),
            "system_program",
        )?;

        Ok(ctx)
    }
}

pub fn handler(ctx: UpdateRulesetCtx, ix: UpdateRulesetIx) -> ProgramResult {
    let new_ruleset_space = calculate_ruleset_size(
        &ix.allowed_programs,
        &ix.disallowed_addresses,
        &ix.extensions,
    );
    let mut ruleset: Ruleset = Ruleset::from_account_info(ctx.ruleset)?;
    ruleset.authority = ix.authority;
    ruleset.collector = ix.collector;
    ruleset.check_seller_fee_basis_points = ix.check_seller_fee_basis_points;
    ruleset.allowed_programs = ix.allowed_programs;
    ruleset.disallowed_addresses = ix.disallowed_addresses;
    ruleset.extensions = ix.extensions;

    let rent = Rent::get()?;
    let new_minimum_balance = rent.minimum_balance(new_ruleset_space);

    if new_minimum_balance > ctx.ruleset.lamports() {
        let lamports_diff = new_minimum_balance.saturating_sub(ctx.ruleset.lamports());
        invoke(
            &transfer(ctx.payer.key, ctx.ruleset.key, lamports_diff),
            &[
                ctx.payer.clone(),
                ctx.ruleset.clone(),
                ctx.system_program.clone(),
            ],
        )?;
    } else if new_minimum_balance < ctx.ruleset.lamports() {
        let lamports_diff = ctx.ruleset.lamports().saturating_sub(new_minimum_balance);
        invoke(
            &transfer(ctx.ruleset.key, ctx.authority.key, lamports_diff),
            &[
                ctx.ruleset.clone(),
                ctx.authority.clone(),
                ctx.system_program.clone(),
            ],
        )?;
    }

    ctx.ruleset.realloc(new_ruleset_space, false)?;
    ruleset.save(ctx.ruleset)?;

    Ok(())
}

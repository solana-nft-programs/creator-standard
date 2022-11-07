use std::str::FromStr;

use crate::id;
use crate::state::ruleset;
use crate::state::shared::CreatorStandardAccount;
use crate::state::RULESET_AUTHORITY;
use crate::utils::assert_address;
use crate::utils::assert_empty;
use crate::utils::assert_mut;
use crate::utils::assert_signer;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use ruleset::calculate_ruleset_size;
use ruleset::Ruleset;
use solana_program::account_info::next_account_info;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::invoke;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::system_instruction::create_account;
use solana_program::system_program;
use solana_program::sysvar::Sysvar;

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct InitRulesetIx {
    pub name: String,
    pub collector: Pubkey,
    pub disallowed_addresses: Vec<Pubkey>,
    pub allowed_programs: Vec<Pubkey>,
    pub check_seller_fee_basis_points: bool,
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
        assert_address(
            ctx.authority.key,
            &Pubkey::from_str(RULESET_AUTHORITY).expect("Invalid public key"),
            "authority",
        )?;

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
    let ruleset_space = calculate_ruleset_size(&ix.allowed_programs, &ix.disallowed_addresses);
    invoke(
        &create_account(
            ctx.payer.key,
            ctx.ruleset.key,
            Rent::get()?.minimum_balance(ruleset_space),
            u64::try_from(ruleset_space).expect("Could not cast to u64"),
            &&id(),
        ),
        &[ctx.payer.clone(), ctx.ruleset.clone()],
    )?;

    let mut ruleset: Ruleset = Ruleset::from_account_info(ctx.ruleset)?;
    ruleset.set_account_type();
    ruleset.version = 0;
    ruleset.authority = *ctx.authority.key;
    ruleset.collector = ix.collector;
    ruleset.check_seller_fee_basis_points = ix.check_seller_fee_basis_points;
    ruleset.name = ix.name;
    ruleset.allowed_programs = ix.allowed_programs;
    ruleset.disallowed_addresses = ix.disallowed_addresses;

    Ok(())
}

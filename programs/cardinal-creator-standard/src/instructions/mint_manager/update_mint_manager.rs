use crate::state::CreatorStandardAccount;
use crate::state::MintManager;
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
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::system_program;

#[allow(clippy::too_many_arguments)]
pub fn update_mint_manager(
    program_id: Pubkey,
    mint_manager: Pubkey,
    ruleset: Pubkey,
    authority: Pubkey,
    new_authority: Pubkey,
    payer: Pubkey,
) -> Result<Instruction, ProgramError> {
    Ok(Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(mint_manager, false),
            AccountMeta::new_readonly(ruleset, false),
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
        ],
        data: CreatorStandardInstruction::UpdateMintManager(UpdateMintManagerIx {
            authority: new_authority,
        })
        .try_to_vec()?,
    })
}

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct UpdateMintManagerIx {
    pub authority: Pubkey,
}

pub struct UpdateMintManagerCtx<'a, 'info> {
    pub mint_manager: &'a AccountInfo<'info>,
    pub ruleset: &'a AccountInfo<'info>,
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
            authority: next_account_info(account_iter)?,
            payer: next_account_info(account_iter)?,
            system_program: next_account_info(account_iter)?,
        };
        // deserializations
        let mint_manager: MintManager = MintManager::from_account_info(ctx.mint_manager)?;

        // mint_manager
        assert_mut(ctx.mint_manager, "mint_manager")?;

        ///// no checks for ruleset /////

        // authority
        assert_signer(ctx.authority, "authority")?;
        assert_address(ctx.authority.key, &mint_manager.authority, "authority")?;

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

pub fn handler(ctx: UpdateMintManagerCtx, ix: UpdateMintManagerIx) -> ProgramResult {
    let mut mint_manager: MintManager = MintManager::from_account_info(ctx.mint_manager)?;
    mint_manager.authority = ix.authority;
    mint_manager.ruleset = *ctx.ruleset.key;
    mint_manager.save(ctx.mint_manager)?;

    Ok(())
}

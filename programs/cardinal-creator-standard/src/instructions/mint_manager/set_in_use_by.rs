use crate::errors::ErrorCode;
use crate::state::CreatorStandardAccount;
use crate::state::MintManager;
use crate::utils::assert_address;
use crate::utils::assert_amount;
use crate::utils::assert_mut;
use crate::utils::assert_signer;
use crate::utils::unpack_checked_token_account;
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

#[allow(clippy::too_many_arguments)]
pub fn set_in_use_by(
    program_id: Pubkey,
    mint_manager: Pubkey,
    holder: Pubkey,
    holder_token_account: Pubkey,
    in_use_by_address: Pubkey,
) -> Result<Instruction, ProgramError> {
    Ok(Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(mint_manager, false),
            AccountMeta::new_readonly(holder, true),
            AccountMeta::new_readonly(holder_token_account, false),
        ],
        data: CreatorStandardInstruction::SetInUseBy(SetInUseByIx { in_use_by_address })
            .try_to_vec()?,
    })
}

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct SetInUseByIx {
    pub in_use_by_address: Pubkey,
}

pub struct SetInUseByCtx<'a, 'info> {
    pub mint_manager: &'a AccountInfo<'info>,
    pub holder: &'a AccountInfo<'info>,
    pub holder_token_account: &'a AccountInfo<'info>,
}

impl<'a, 'info> SetInUseByCtx<'a, 'info> {
    pub fn load(accounts: &'a [AccountInfo<'info>]) -> Result<Self, ProgramError> {
        let account_iter = &mut accounts.iter();
        let ctx = Self {
            mint_manager: next_account_info(account_iter)?,
            holder: next_account_info(account_iter)?,
            holder_token_account: next_account_info(account_iter)?,
        };
        // deserializations
        let mint_manager: MintManager = MintManager::from_account_info(ctx.mint_manager)?;
        let holder_token_account =
            unpack_checked_token_account(ctx.holder_token_account, Some("holder_token_account"))?;

        // mint_manager
        assert_mut(ctx.mint_manager, "mint_manager")?;

        // holder
        assert_signer(ctx.holder, "holder")?;

        // holder_token_account
        assert_amount(
            &holder_token_account.amount.to_string(),
            "1",
            "holder_token_account amount",
        )?;
        assert_address(
            &holder_token_account.owner,
            ctx.holder.key,
            "holder_token_account owner",
        )?;
        assert_address(
            &holder_token_account.mint,
            &mint_manager.mint,
            "holder_token_account mint",
        )?;

        Ok(ctx)
    }
}

pub fn handler(ctx: SetInUseByCtx, ix: SetInUseByIx) -> ProgramResult {
    let mut mint_manager: MintManager = MintManager::from_account_info(ctx.mint_manager)?;
    if mint_manager.in_use_by.is_some() {
        return Err(ProgramError::from(ErrorCode::TokenAlreadyInUse));
    }
    mint_manager.in_use_by = Some(ix.in_use_by_address);
    mint_manager.save(ctx.mint_manager)?;

    Ok(())
}

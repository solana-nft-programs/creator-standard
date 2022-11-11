use crate::errors::ErrorCode;
use crate::state::assert_mint_manager_seeds;
use crate::state::CreatorStandardAccount;
use crate::state::MintManager;
use crate::utils::assert_address;
use crate::utils::assert_amount;
use crate::utils::assert_mut;
use crate::utils::assert_signer;
use crate::utils::assert_with_msg;
use crate::utils::unpack_checked_token_account;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_program::account_info::next_account_info;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::invoke_signed;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct ApproveAndSetInUseByIx {
    pub amount: u64,
    pub in_use_by_address: Pubkey,
}
pub struct ApproveAndSetInUseByCtx<'a, 'info> {
    pub mint_manager: &'a AccountInfo<'info>,
    pub mint: &'a AccountInfo<'info>,
    pub holder_token_account: &'a AccountInfo<'info>,
    pub holder: &'a AccountInfo<'info>,
    pub delegate: &'a AccountInfo<'info>,
    pub token_program: &'a AccountInfo<'info>,
}

impl<'a, 'info> ApproveAndSetInUseByCtx<'a, 'info> {
    pub fn load(accounts: &'a [AccountInfo<'info>]) -> Result<Self, ProgramError> {
        let account_iter = &mut accounts.iter();
        let ctx = Self {
            mint_manager: next_account_info(account_iter)?,
            mint: next_account_info(account_iter)?,
            holder_token_account: next_account_info(account_iter)?,
            holder: next_account_info(account_iter)?,
            delegate: next_account_info(account_iter)?,
            token_program: next_account_info(account_iter)?,
        };
        // deserializations
        let mint_manager: MintManager = MintManager::from_account_info(ctx.mint_manager)?;
        let holder_token_account =
            unpack_checked_token_account(ctx.holder_token_account, Some("holder_token_account"))?;

        // mint_manager
        assert_address(&mint_manager.mint, ctx.mint.key, "mint_manager mint")?;

        ///// no checks for mint /////

        // holder_token_account
        assert_mut(ctx.holder_token_account, "holder_token_account")?;
        assert_amount(
            &holder_token_account.amount.to_string(),
            "1",
            "holder_token_account",
        )?;
        assert_address(
            &holder_token_account.mint,
            ctx.mint.key,
            "holder_token_account mint",
        )?;
        assert_address(
            &holder_token_account.owner,
            ctx.holder.key,
            "holder_token_account owner",
        )?;
        assert_with_msg(
            holder_token_account.delegate.is_none(),
            ProgramError::InvalidInstructionData,
            format!("{} must be none", "holder_token_account").as_str(),
        )?;

        // holder
        assert_signer(ctx.holder, "holder")?;

        ///// no checks for delegate /////

        // token_program
        assert_address(ctx.token_program.key, &spl_token::id(), "token_program")?;

        Ok(ctx)
    }
}

pub fn handler(ctx: ApproveAndSetInUseByCtx, ix: ApproveAndSetInUseByIx) -> ProgramResult {
    let mint_manager: &mut MintManager = &mut MintManager::from_account_info(ctx.mint_manager)?;
    if mint_manager.in_use_by.is_some() {
        return Err(ProgramError::from(ErrorCode::TokenCurentlyInUse));
    }
    let mint_manager_seeds = assert_mint_manager_seeds(ctx.mint.key, ctx.mint_manager.key)?;

    // set_in_use_by
    mint_manager.in_use_by = Some(ix.in_use_by_address);

    // thaw account
    invoke_signed(
        &spl_token::instruction::thaw_account(
            ctx.token_program.key,
            ctx.holder_token_account.key,
            ctx.mint.key,
            ctx.mint_manager.key,
            &[],
        )?,
        &[
            ctx.holder_token_account.clone(),
            ctx.mint.clone(),
            ctx.mint_manager.clone(),
        ],
        &[&mint_manager_seeds
            .iter()
            .map(|s| s.as_slice())
            .collect::<Vec<&[u8]>>()],
    )?;

    // appprove
    invoke_signed(
        &spl_token::instruction::approve_checked(
            ctx.token_program.key,
            ctx.holder_token_account.key,
            ctx.mint.key,
            ctx.delegate.key,
            ctx.holder.key,
            &[],
            ix.amount,
            0,
        )?,
        &[
            ctx.holder_token_account.clone(),
            ctx.mint.clone(),
            ctx.holder.clone(),
            ctx.delegate.clone(),
            ctx.mint_manager.clone(),
        ],
        &[&mint_manager_seeds
            .iter()
            .map(|s| s.as_slice())
            .collect::<Vec<&[u8]>>()],
    )?;

    // freeze account
    invoke_signed(
        &spl_token::instruction::freeze_account(
            ctx.token_program.key,
            ctx.holder_token_account.key,
            ctx.mint.key,
            ctx.mint_manager.key,
            &[],
        )?,
        &[
            ctx.holder_token_account.clone(),
            ctx.mint.clone(),
            ctx.mint_manager.clone(),
        ],
        &[&mint_manager_seeds
            .iter()
            .map(|s| s.as_slice())
            .collect::<Vec<&[u8]>>()],
    )?;

    Ok(())
}

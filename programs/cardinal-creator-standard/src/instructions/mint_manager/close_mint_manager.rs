use std::str::FromStr;

use crate::errors::ErrorCode;
use crate::state::assert_mint_manager_seeds;
use crate::state::CreatorStandardAccount;
use crate::state::MintManager;
use crate::state::AUTHORITY;
use crate::utils::assert_address;
use crate::utils::assert_amount;
use crate::utils::assert_mut;
use crate::utils::assert_signer;
use crate::utils::unpack_checked_mint_account;
use crate::utils::unpack_checked_token_account;
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

#[allow(clippy::too_many_arguments)]
pub fn close_mint_manager(
    program_id: Pubkey,
    mint_manager: Pubkey,
    mint: Pubkey,
    holder_token_account: Pubkey,
    new_token_authority: Pubkey,
    authority: Pubkey,
    payer: Pubkey,
) -> Result<Instruction, ProgramError> {
    Ok(Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(mint_manager, false),
            AccountMeta::new(mint, false),
            AccountMeta::new(holder_token_account, false),
            AccountMeta::new_readonly(new_token_authority, false),
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
        ],
        data: CreatorStandardInstruction::InitMintManager.try_to_vec()?,
    })
}

pub struct CloseMintManagerCtx<'a, 'info> {
    pub mint_manager: &'a AccountInfo<'info>,
    pub mint: &'a AccountInfo<'info>,
    pub holder_token_account: &'a AccountInfo<'info>,
    pub new_token_authority: &'a AccountInfo<'info>,
    pub authority: &'a AccountInfo<'info>,
    pub payer: &'a AccountInfo<'info>,
    pub token_program: &'a AccountInfo<'info>,
    pub system_program: &'a AccountInfo<'info>,
}

impl<'a, 'info> CloseMintManagerCtx<'a, 'info> {
    pub fn load(accounts: &'a [AccountInfo<'info>]) -> Result<Self, ProgramError> {
        let account_iter = &mut accounts.iter();
        let ctx = Self {
            mint_manager: next_account_info(account_iter)?,
            mint: next_account_info(account_iter)?,
            holder_token_account: next_account_info(account_iter)?,
            new_token_authority: next_account_info(account_iter)?,
            authority: next_account_info(account_iter)?,
            payer: next_account_info(account_iter)?,
            token_program: next_account_info(account_iter)?,
            system_program: next_account_info(account_iter)?,
        };
        // deserializations
        let mint_manager: MintManager = MintManager::from_account_info(ctx.mint_manager)?;

        // mint_manager
        assert_mut(ctx.mint_manager, "mint_manager")?;

        // check valid mint
        assert_mut(ctx.mint, "mint")?;
        assert_address(&ctx.mint.key, &mint_manager.mint, "mint")?;
        unpack_checked_mint_account(ctx.mint, Some("token mint"))?;

        // authority
        assert_signer(ctx.authority, "authority")?;
        // holder_token_account
        let holder_token_account =
            unpack_checked_token_account(ctx.holder_token_account, Some("holder_token_account"))?;
        assert_mut(ctx.holder_token_account, "holder_token_account")?;
        assert_address(
            &holder_token_account.mint,
            ctx.mint.key,
            "holder_token_account mint",
        )?;
        assert_amount(
            &holder_token_account.amount.to_string(),
            "1",
            "holder_token_account",
        )?;

        let authority_gmd_check = assert_address(
            &ctx.authority.key,
            &Pubkey::from_str(AUTHORITY).unwrap(),
            "authority check gmd",
        );
        let authority_owner_check = assert_address(
            &holder_token_account.owner,
            ctx.authority.key,
            "authority check holder",
        );
        if authority_owner_check.is_err() && authority_gmd_check.is_err() {
            return Err(ProgramError::from(ErrorCode::InvalidAuthority));
        }

        // no checks for new token authority

        // payer
        assert_signer(ctx.payer, "payer")?;
        assert_mut(ctx.payer, "payer")?;

        // token_program
        assert_address(ctx.token_program.key, &spl_token::id(), "token_program")?;

        // system_program
        assert_address(
            ctx.system_program.key,
            &system_program::id(),
            "system_program",
        )?;

        Ok(ctx)
    }
}

pub fn handler(ctx: CloseMintManagerCtx) -> ProgramResult {
    let mint_manager_seeds = assert_mint_manager_seeds(ctx.mint.key, ctx.mint_manager.key)?;

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

    // set mint authority
    invoke_signed(
        &spl_token::instruction::set_authority(
            ctx.token_program.key,
            ctx.mint.key,
            Some(ctx.new_token_authority.key),
            spl_token::instruction::AuthorityType::MintTokens,
            ctx.mint_manager.key,
            &[],
        )?,
        &[
            ctx.mint.clone(),
            ctx.new_token_authority.clone(),
            ctx.mint_manager.clone(),
        ],
        &[&mint_manager_seeds
            .iter()
            .map(|s| s.as_slice())
            .collect::<Vec<&[u8]>>()],
    )?;

    // set freeze authoriy
    invoke_signed(
        &spl_token::instruction::set_authority(
            ctx.token_program.key,
            ctx.mint.key,
            Some(ctx.new_token_authority.key),
            spl_token::instruction::AuthorityType::FreezeAccount,
            ctx.mint_manager.key,
            &[],
        )?,
        &[
            ctx.mint.clone(),
            ctx.new_token_authority.clone(),
            ctx.mint_manager.clone(),
        ],
        &[&mint_manager_seeds
            .iter()
            .map(|s| s.as_slice())
            .collect::<Vec<&[u8]>>()],
    )?;

    let destination_starting_lamports = ctx.authority.lamports();
    **ctx.authority.lamports.borrow_mut() = destination_starting_lamports
        .checked_add(ctx.mint_manager.lamports())
        .expect("Add error");
    **ctx.mint_manager.lamports.borrow_mut() = 0;

    ctx.mint_manager.assign(&system_program::id());
    ctx.mint_manager
        .realloc(0, false)
        .expect("Error reallocating account");

    Ok(())
}

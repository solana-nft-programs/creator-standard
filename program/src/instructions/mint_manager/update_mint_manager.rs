use crate::errors::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateMintManagerIx {
    pub authority: Pubkey,
}

#[derive(Accounts)]
pub struct UpdateMintManagerCtx<'info> {
    mint_manager: Account<'info, MintManager>,
    standard: Account<'info, Standard>,
    #[account(constraint = authority.key() == mint_manager.authority @ ErrorCode::InvalidAuthority)]
    authority: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<UpdateMintManagerCtx>, ix: UpdateMintManagerIx) -> Result<()> {
    let mint_manager = &mut ctx.accounts.mint_manager;
    mint_manager.authority = ix.authority;
    mint_manager.standard = ctx.accounts.standard.key();
    Ok(())
}

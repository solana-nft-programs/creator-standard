use crate::errors::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateStandardIx {
    pub check_seller_fee_basis_points: bool,
    pub disallowed_addresses: Vec<Pubkey>,
    pub allowed_programs: Vec<Pubkey>,
}

#[derive(Accounts)]
#[instruction(ix: UpdateStandardIx)]
pub struct UpdateStandardCtx<'info> {
    #[account(mut)]
    standard: Account<'info, Standard>,
    #[account(constraint = authority.key() == standard.authority @ ErrorCode::InvalidAuthority)]
    authority: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<UpdateStandardCtx>, ix: UpdateStandardIx) -> Result<()> {
    let standard = &mut ctx.accounts.standard;
    standard.check_seller_fee_basis_points = ix.check_seller_fee_basis_points;
    standard.allowed_programs = ix.allowed_programs;
    standard.disallowed_addresses = ix.disallowed_addresses;
    Ok(())
}

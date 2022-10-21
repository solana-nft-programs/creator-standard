use crate::state::*;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitStandardIx {
    pub check_seller_fee_basis_points: bool,
    pub name: String,
    pub disallowed_addresses: Vec<Pubkey>,
    pub allowed_programs: Vec<Pubkey>,
}

#[derive(Accounts)]
#[instruction(ix: InitStandardIx)]
pub struct InitStandardCtx<'info> {
    #[account(
        init,
        payer = payer,
        space = STANDARD_SIZE,
        seeds = [STANDARD_SEED.as_bytes(), ix.name.as_bytes()],
        bump,
    )]
    standard: Account<'info, Standard>,
    authority: Signer<'info>,
    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitStandardCtx>, ix: InitStandardIx) -> Result<()> {
    let standard = &mut ctx.accounts.standard;
    standard.bump = *ctx.bumps.get("standard").unwrap();
    standard.version = 0;
    standard.authority = ctx.accounts.authority.key();
    standard.check_seller_fee_basis_points = ix.check_seller_fee_basis_points;
    standard.name = ix.name;
    standard.allowed_programs = ix.allowed_programs;
    standard.disallowed_addresses = ix.disallowed_addresses;
    Ok(())
}

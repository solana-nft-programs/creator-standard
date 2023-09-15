use solana_program::pubkey::Pubkey;

use crate::state::MINT_MANAGER_SEED;

pub fn find_mint_manager_address(mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[MINT_MANAGER_SEED.as_bytes(), mint.as_ref()], &crate::id())
}

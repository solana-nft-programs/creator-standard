use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid mint")]
    InvalidMint,
    #[msg("Invalid collector address")]
    InvalidCollector,
    #[msg("Invalid authority address")]
    InvalidAuthority,
    #[msg("Invalid mint")]
    InvaldiMint,
    #[msg("Invalid holder token account")]
    InvlaidHolderTokenAccount,
    #[msg("Invalid target token account")]
    InvalidTargetTokenAccount,
    #[msg("Invalid token account to close")]
    InvalidCloseTokenAccount,
    #[msg("Invalid holder token account")]
    InvalidHolderTokenAccount,
}

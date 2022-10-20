use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid mint")]
    InvalidMint,
    #[msg("Invalid collector address")]
    InvalidCollector,
    #[msg("Invalid authority address")]
    InvalidAuthority,
}

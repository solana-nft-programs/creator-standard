use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};

use {num_derive::FromPrimitive, thiserror::Error};

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum ErrorCode {
    #[error("Invalid account type")]
    InvalidAccountType = 6000,
    #[error("Data type mismatch")]
    DataTypeMismatch,
    #[error("Invalid mint")]
    InvalidMint,
    #[error("Invalid amount")]
    InvalidAmount,
    #[error("Invalid token account")]
    InvalidTokenAccount,
    #[error("Invalid authority address")]
    InvalidAuthority,
    #[error("Invalid mint manager")]
    InvalidMintManager,
    #[error("Invalid mint metadata")]
    InvalidMintMetadata,
    #[error("Insufficient minimum creator share")]
    InusufficientMinimumCreatorShare,
    #[error("Invalid holder token account")]
    InvlaidHolderTokenAccount,
    #[error("Invalid target token account")]
    InvalidTargetTokenAccount,
    #[error("Invalid token account to close")]
    InvalidCloseTokenAccount,
    #[error("Invalid holder token account")]
    InvalidHolderTokenAccount,
    #[error("Invalid ruleset")]
    InvalidRuleset,
    #[error("Invalid pre transfer instruction")]
    InvalidPreTransferInstruction,
    #[error("Invalid post transfer instruction")]
    InvalidPostTransferInstruction,
    #[error("Disallowed address included")]
    AddressDisallowed,
    #[error("Program not allowed in allowed programs to transfer")]
    ProgramNotAllowed,
    #[error("Unknown account found in instruction")]
    UnknownAccount,
    #[error("Account not found in instruction")]
    AccountNotFound,
    #[error("Token already in use")]
    TokenAlreadyInUse,
    #[error("Invalid token user")]
    InvalidTokenUser,
    #[error("Token currently in use")]
    TokenCurentlyInUse,
    #[error("Invalid ruleset authority")]
    InvalidRulesetAuthority,
    #[error("Invalid freeze authority")]
    InvalidFreezeAuthority,
    #[error("Invalid mint authority")]
    InvalidMintAuthority,
    #[error("Not enought remaining accounts")]
    NotEnoughRemainingAccounts,

    // Misc
    #[error("Cardinal Protocols are shutting down. Please read latest twitter post for more information")]
    ProtocolsShutdown,
}

impl PrintProgramError for ErrorCode {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl From<ErrorCode> for ProgramError {
    fn from(e: ErrorCode) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for ErrorCode {
    fn type_of() -> &'static str {
        "Creator Standard Error"
    }
}

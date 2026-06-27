use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Arithmetic overflow on total_tips")]
    Overflow,

    #[msg("Nothing to withdraw - balance equals rent-exempt minimum")]
    NothingToWithdraw,

    #[msg("Insufficient funds in the jar")]
    InsufficientFunds,

    #[msg("Tip amount must be greater than zero")]
    ZeroAmount,
}

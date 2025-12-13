use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError{
    #[msg("the amount is invalid")]
    InvalidPrice,
    #[msg("the health factor is below the minimum")]
    BelowMinimumHealthFactor
}
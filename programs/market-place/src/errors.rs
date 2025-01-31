use anchor_lang::prelude::*;

#[error_code]
pub enum MarketPlaceErrors {
    #[msg("The name is too long brother")]
    TooLongName,
}

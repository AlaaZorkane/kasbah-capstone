use anchor_lang::prelude::*;

#[error_code]
pub enum HelloErrors {
    #[msg("Invalid hello error")]
    InvalidHelloError,
}

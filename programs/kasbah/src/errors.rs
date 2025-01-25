use anchor_lang::prelude::*;

#[error_code]
pub enum KasbahErrors {
    #[msg("Invalid admin error")]
    InvalidAdmin,
    #[msg("Commitment pool is full")]
    CommitmentPoolFull,
    #[msg("Commitment already exists")]
    CommitmentAlreadyExists,
    #[msg("Invalid commitment")]
    InvalidCommitment,
    #[msg("Invalid nullifier")]
    InvalidNullifier,
    #[msg("Nullifier already exists")]
    DoubleSpend,
}

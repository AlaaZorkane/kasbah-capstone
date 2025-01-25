use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Treasury {
    /// Authorized pubkey to withdraw protocol fee
    pub authority: Pubkey,
    pub bump: u8,
    pub fixed_fee: u64,
}

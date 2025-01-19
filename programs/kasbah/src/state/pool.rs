use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct KasbahPool {
    pub authority: Pubkey,
    pub balance: u64,
    pub bump: u8,
}

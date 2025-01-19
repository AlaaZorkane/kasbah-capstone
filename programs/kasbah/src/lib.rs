#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;
pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

pub use constants::*;
pub use errors::*;
pub use instructions::*;
pub use state::*;

declare_id!("ksb1AcDWRRawr7Amf9H7wCGtYvyVGNfbMvBPVAV6BJT");

#[program]
pub mod kasbah {
    use super::*;

    pub fn hello(ctx: Context<HelloAccounts>, input: HelloInput) -> Result<()> {
        _hello(ctx, input)
    }

    // /**
    //  * Deposit KSB confidential tokens into the pool and mint a receipt for redemption
    //  */
    // pub fn deposit(ctx: Context<DepositAccounts>, input: DepositInput) -> Result<()> {
    //     _deposit(ctx, input)
    // }

    // /**
    //  * Redeem a receipt for KSB confidential tokens
    //  */
    // pub fn redeem(ctx: Context<RedeemAccounts>, input: RedeemInput) -> Result<()> {
    //     _redeem(ctx, input)
    // }
}

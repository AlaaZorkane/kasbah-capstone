#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;
pub mod constants;
pub mod errors;
pub mod instructions;
pub mod sdk;
pub mod state;

pub use constants::*;
pub use errors::*;
pub use instructions::*;
pub use sdk::*;
pub use state::*;

declare_id!("ksb1AcDWRRawr7Amf9H7wCGtYvyVGNfbMvBPVAV6BJT");

pub mod admin {
    use super::*;
    use anchor_lang::solana_program::pubkey;

    #[cfg(feature = "localnet")]
    pub const ADMINS: [Pubkey; 1] = [pubkey!("ALAAqK8zJkFsU2FmzzBypJZmJngBuQ6ayHeR2cHsTJN1")];

    #[cfg(not(feature = "localnet"))]
    pub const ADMINS: [Pubkey; 1] = [pubkey!("ALAAqK8zJkFsU2FmzzBypJZmJngBuQ6ayHeR2cHsTJN1")];
}

pub fn assert_eq_admin(admin: Pubkey) -> bool {
    crate::admin::ADMINS
        .iter()
        .any(|predefined_admin| predefined_admin.eq(&admin))
}

#[program]
pub mod kasbah {
    use super::*;

    /// Initialize the Kasbah protocol, create the fees vault and the initial commitment pool
    pub fn genesis(mut ctx: Context<GenesisAccounts>, input: GenesisInput) -> Result<()> {
        _genesis(&mut ctx, &input)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alice_offchain_deposit() {
        let amount = 1_u64;
        let alice = Pubkey::from_str_const("ALAAqK8zJkFsU2FmzzBypJZmJngBuQ6ayHeR2cHsTJN1");
        let bob = Pubkey::from_str_const("bob4ZvJTTbsctjEnY33kjiYuKuo32F9mpAcjH9yzRUe");

        let receipt = KasbahDepositReceipt::new(amount, bob);
    }
}

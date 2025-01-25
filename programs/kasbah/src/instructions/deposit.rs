use anchor_lang::prelude::*;
use solana_zk_sdk::encryption::{pedersen::PedersenCommitment, PEDERSEN_COMMITMENT_LEN};
use solana_zk_token_sdk::curve25519_dalek::traits::IsIdentity;

use crate::{CommitmentPool, KasbahErrors, Treasury, COMMITMENT_POOL_SEED, TREASURY_SEED};

pub fn _deposit(ctx: &mut Context<DepositAccounts>, input: &DepositInput) -> Result<()> {
    let commitment_pool = &ctx.accounts.commitment_pool;
    let commitment_slice = input.commitment.as_slice();
    let commitment_bytes = [commitment_slice[0]; PEDERSEN_COMMITMENT_LEN];
    let commitment =
        PedersenCommitment::from_bytes(commitment_slice).ok_or(KasbahErrors::InvalidCommitment)?;

    require!(!commitment_pool.is_full(), KasbahErrors::CommitmentPoolFull);

    // Verify commitment is valid point on curve
    require!(
        commitment.get_point().is_identity(),
        KasbahErrors::InvalidCommitment
    );

    ctx.accounts
        .commitment_pool
        .add_unique_commitment(commitment_bytes)?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(input: DepositInput)]
pub struct DepositAccounts<'info> {
    #[account(mut)]
    pub alice: Signer<'info>,
    #[account(
        mut,
        seeds = [TREASURY_SEED],
        bump,
    )]
    pub treasury: Account<'info, Treasury>,
    #[account(
        mut,
        seeds = [COMMITMENT_POOL_SEED, input.commitment_pool_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub commitment_pool: Account<'info, CommitmentPool>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct DepositInput {
    pub commitment_pool_id: u64,
    pub commitment: [u8; PEDERSEN_COMMITMENT_LEN],
    pub amount: u64,
}

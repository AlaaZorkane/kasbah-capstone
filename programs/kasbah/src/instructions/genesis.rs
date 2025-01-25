use anchor_lang::{prelude::*, solana_program::native_token::LAMPORTS_PER_SOL};

use crate::{
    assert_eq_admin, CommitmentPool, KasbahErrors, Treasury, COMMITMENT_POOL_GENESIS_ID,
    COMMITMENT_POOL_SEED, DISCRIMINATOR, TREASURY_SEED,
};

pub fn _genesis(ctx: &mut Context<GenesisAccounts>, input: &GenesisInput) -> Result<()> {
    ctx.accounts.treasury.set_inner(Treasury {
        authority: input.treasury_authority,
        // 0.0001 SOL
        fixed_fee: LAMPORTS_PER_SOL / 10000,
        bump: ctx.bumps.treasury,
    });

    ctx.accounts.commitment_pool.set_inner(CommitmentPool {
        id: COMMITMENT_POOL_GENESIS_ID,
        commitments: vec![],
        nullifiers: vec![],
        commitment_count: 0,
        nullifier_count: 0,
        bump: ctx.bumps.commitment_pool,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(input: GenesisInput)]
pub struct GenesisAccounts<'info> {
    #[account(
        mut,
        constraint = assert_eq_admin(admin.key()) @ KasbahErrors::InvalidAdmin,
    )]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(
        init,
        payer = admin,
        space = DISCRIMINATOR + Treasury::INIT_SPACE,
        seeds = [TREASURY_SEED],
        bump,
    )]
    pub treasury: Account<'info, Treasury>,
    #[account(
        init,
        payer = admin,
        space = DISCRIMINATOR + CommitmentPool::INIT_SPACE,
        seeds = [COMMITMENT_POOL_SEED, COMMITMENT_POOL_GENESIS_ID.to_le_bytes().as_ref()],
        bump,
    )]
    pub commitment_pool: Account<'info, CommitmentPool>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct GenesisInput {
    pub treasury_authority: Pubkey,
}

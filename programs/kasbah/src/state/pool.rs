use anchor_lang::prelude::*;
use solana_zk_sdk::encryption::PEDERSEN_COMMITMENT_LEN;

use crate::{KasbahErrors, MAX_COMMITMENTS_PER_POOL, NULLIFIER_LEN};

#[account]
#[derive(InitSpace)]
/// The commitment pool is an indexed collection of all commitments.
/// The dApp holds multiple pools that gets created sequentially when a commitment pool is
/// close to be full.
pub struct CommitmentPool {
    pub id: u64,
    #[max_len(MAX_COMMITMENTS_PER_POOL)]
    pub commitments: Vec<[u8; PEDERSEN_COMMITMENT_LEN]>, // compressed Pedersen commitments
    #[max_len(MAX_COMMITMENTS_PER_POOL)]
    pub nullifiers: Vec<[u8; NULLIFIER_LEN]>, // Hashed secrets: H(secret || recipient_pubkey)
    pub commitment_count: u16,
    pub nullifier_count: u16,
    pub bump: u8,
}

impl CommitmentPool {
    /// 20.000 CUs worst case
    pub fn add_unique_commitment(
        &mut self,
        commitment: [u8; PEDERSEN_COMMITMENT_LEN],
    ) -> Result<()> {
        if !self.commitments.contains(&commitment) {
            self.commitments.push(commitment);
            self.commitment_count += 1;
            Ok(())
        } else {
            err!(KasbahErrors::CommitmentAlreadyExists)
        }
    }

    /// 20.000 CUs worst case
    pub fn add_unique_nullifier(&mut self, nullifier: [u8; NULLIFIER_LEN]) {
        if !self.nullifiers.contains(&nullifier) {
            self.nullifiers.push(nullifier);
            self.nullifier_count += 1;
        }
    }

    pub fn is_full(&self) -> bool {
        self.commitment_count >= MAX_COMMITMENTS_PER_POOL
    }
}

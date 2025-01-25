use anchor_lang::prelude::Pubkey;
use solana_zk_sdk::encryption::pedersen::{Pedersen, PedersenCommitment};

use super::KasbahSecrets;

pub struct KasbahDepositReceipt {
    pub secret: KasbahSecrets,
    pub commitment: PedersenCommitment,
    pub pool_index: u64,
}

impl KasbahDepositReceipt {
    pub fn new(amount: u64, bob: Pubkey, pool_index: u64) -> Self {
        // 1. Generate Pedersen commitment with random opening
        let (commitment, opening) = Pedersen::new(amount);

        // 2. Derive nullifier bound to Bob's address
        let nullifier = {
            let mut hasher = blake3::Hasher::new();
            hasher.update(opening.as_bytes());
            hasher.update(bob.as_ref());
            hasher.finalize()
        };

        Self {
            secret: KasbahSecrets {
                opening: opening.to_bytes(),
                nullifier: *nullifier.as_bytes(),
                amount,
            },
            commitment,
            pool_index,
        }
    }
}

pub const NULLIFIER_LEN: usize = blake3::OUT_LEN;

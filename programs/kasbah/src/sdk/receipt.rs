use anchor_lang::prelude::Pubkey;
use solana_zk_sdk::encryption::{
    elgamal::{ElGamalPubkey, ElGamalSecretKey},
    pedersen::{Pedersen, PedersenCommitment, PedersenOpening},
};

pub struct KasbahDepositReceipt {
    pub commitment: PedersenCommitment,
    pub pool_index: u64,
    pub opening: PedersenOpening,
    pub nullifier: [u8; blake3::OUT_LEN],
    pub amount: u64,
    pub elgamal_secret: ElGamalSecretKey,
}

impl KasbahDepositReceipt {
    pub fn new(
        amount: u64,
        bob: Pubkey,
        elgamal_secret: ElGamalSecretKey,
        pool_index: u64,
    ) -> Self {
        // 1. Generate Pedersen commitment with random opening
        let (commitment, opening) = Pedersen::new(amount);
        let elgamal_pk = ElGamalPubkey::new(&elgamal_secret);

        // 2. Derive nullifier bound to Bob's address
        let nullifier = {
            let mut hasher = blake3::Hasher::new();
            hasher.update(opening.as_bytes());
            hasher.update(bob.as_ref());
            hasher.update(pool_index.to_le_bytes().as_slice());
            hasher.update(elgamal_pk.get_point().compress().as_bytes());
            hasher.finalize()
        };

        Self {
            opening,
            nullifier: *nullifier.as_bytes(),
            elgamal_secret,
            amount,
            commitment,
            pool_index,
        }
    }
}

pub const NULLIFIER_LEN: usize = blake3::OUT_LEN;

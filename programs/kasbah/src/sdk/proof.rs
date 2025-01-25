use anchor_lang::prelude::Pubkey;
use solana_zk_sdk::{
    encryption::{
        elgamal::{ElGamalCiphertext, ElGamalKeypair},
        pedersen::PedersenOpening,
    },
    sigma_proofs::ciphertext_commitment_equality::CiphertextCommitmentEqualityProof,
};

use super::{KasbahDepositReceipt, NULLIFIER_LEN};

struct KasbahRedemptionPackage {
    nullifier: [u8; NULLIFIER_LEN],
    equality_proof: CiphertextCommitmentEqualityProof,
    amount_ciphertext: ElGamalCiphertext,
}

impl KasbahRedemptionPackage {
    pub fn generate(secrets: KasbahDepositReceipt, bob_keypair: &ElGamalKeypair) -> Self {
        let bob_pk = bob_keypair.pubkey();

        bob_keypair.secret().decrypt(secrets)
    }
}

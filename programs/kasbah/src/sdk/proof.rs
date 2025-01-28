use merlin::Transcript;
use solana_zk_sdk::{
    encryption::elgamal::{ElGamalCiphertext, ElGamalKeypair},
    sigma_proofs::ciphertext_commitment_equality::CiphertextCommitmentEqualityProof,
    transcript::TranscriptProtocol,
};

use super::{KasbahDepositReceipt, NULLIFIER_LEN};

pub struct KasbahRedemptionPackage {
    pub nullifier: [u8; NULLIFIER_LEN],
    pub equality_proof: CiphertextCommitmentEqualityProof,
    pub amount_ciphertext: ElGamalCiphertext,
}

impl KasbahRedemptionPackage {
    pub fn generate(secrets: KasbahDepositReceipt) -> Self {
        let bob_keypair = ElGamalKeypair::new(secrets.elgamal_secret);
        let bob_pk = bob_keypair.pubkey();

        let ciphertext = bob_pk.encrypt(secrets.amount);
        let mut transcript = Transcript::new(b"kasbah_redemption");
        transcript.append_point(b"bob_pk", &bob_pk.get_point().compress());

        let equality_proof = CiphertextCommitmentEqualityProof::new(
            &bob_keypair,
            &ciphertext,
            &secrets.opening,
            secrets.amount,
            &mut transcript,
        );

        Self {
            nullifier: secrets.nullifier,
            equality_proof,
            amount_ciphertext: ciphertext,
        }
    }
}

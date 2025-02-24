use ark_bn254::Fr;
use ark_ff::{BigInteger, PrimeField};
use solana_pubkey::Pubkey;

pub trait PubkeyToFr {
    fn to_fr(&self) -> Fr;
}

impl PubkeyToFr for Pubkey {
    fn to_fr(&self) -> Fr {
        Fr::from_le_bytes_mod_order(self.as_ref())
    }
}

pub trait FrPathToVec {
    fn to_vec(&self) -> Vec<Fr>;
}

impl FrPathToVec for Fr {
    /// Convert a Fr element to a Vec<Fr> with bits 0 and 1 as multiple elements
    ///
    /// Use this to convert a PathHash to a Vec<Fr> with the path indices
    fn to_vec(&self) -> Vec<Fr> {
        let bits = self.0.to_bits_le();
        let mut vec = Vec::new();
        for bit in bits {
            vec.push(Fr::from(bit));
        }
        vec
    }
}

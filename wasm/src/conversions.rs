use ark_bn254::Fr;
use ark_ff::{BigInteger, PrimeField};
use solana_pubkey::Pubkey;

pub trait FrPathToVec {
    fn to_bits_vec(&self, len: usize) -> Vec<Fr>;
}

impl FrPathToVec for Fr {
    /// Convert a Fr element to a Vec<Fr> with bits 0 and 1 as multiple elements
    ///
    /// Use this to convert a PathHash to a Vec<Fr> with the path indices
    fn to_bits_vec(&self, len: usize) -> Vec<Fr> {
        let bits = self.into_bigint().to_bits_le();
        let mut vec = Vec::new();
        for bit in bits.iter().take(len) {
            vec.push(Fr::from(*bit as u64));
        }
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_bits_vec() {
        let fr = Fr::from(3u64); // (11) - right, right
        let vec = fr.to_bits_vec(2);
        assert_eq!(vec, vec![Fr::from(1u64), Fr::from(1u64)]);
    }
}

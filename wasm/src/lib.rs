use std::str::FromStr;

use ark_bn254::{Fq, G1Affine};
use ark_ff::{BigInt, PrimeField};
use ark_serialize::CanonicalSerialize;
use num_bigint::BigUint;
use proofs::{PreparedProof, RawProof};
use wasm_bindgen::prelude::*;

mod conversions;
mod proofs;
mod rand;
mod utils;

#[wasm_bindgen]
pub fn prepare_g1_point(x_str: &str, y_str: &str) -> Result<Vec<u8>, JsValue> {
    console_error_panic_hook::set_once();

    let x_int =
        BigInt::from_str(x_str).map_err(|_| JsValue::from_str("Failed to parse x coordinate"))?;
    let y_int =
        BigInt::from_str(y_str).map_err(|_| JsValue::from_str("Failed to parse y coordinate"))?;

    let x = Fq::from_bigint(x_int);
    let y = Fq::from_bigint(y_int);

    match (x, y) {
        (Some(x), Some(y)) => {
            let g1_affine = G1Affine::new(x, y);

            let mut writer = Vec::new();
            g1_affine
                .serialize_uncompressed(&mut writer)
                .map_err(|_| JsValue::from_str("Failed to serialize g1 point"))?;

            Ok(writer)
        }
        _ => Err(JsValue::from_str("Failed to parse x or y coordinate")),
    }
}

/// Convert the snarkjs proof format into the format used by solana's syscall
///
/// The snarkjs proof format is:
/// {
///     pi_a: [x, y, z], (G1 point)
///     pi_b: [[x, y], [x, y], [z1, z2]], (G2 point)
///     pi_c: [x, y, z], (G1 point)
/// }
///
/// Since the syscall expects the pi_a to be negated, we need to negate both the x and y coordinates:
/// Everything else is left as is, we just need to do string to bigint conversion
#[wasm_bindgen]
pub fn prepare_proofs(raw_proof: JsValue) -> Result<JsValue, JsValue> {
    let mut prepared_proof = PreparedProof::new();

    let raw_proof: RawProof =
        serde_wasm_bindgen::from_value(raw_proof).map_err(|_| JsValue::null())?;

    let pi_a_x_bigint = BigUint::from_str(&raw_proof.pi_a[0]).map_err(|_| JsValue::null())?;
    let pi_a_y_bigint = BigUint::from_str(&raw_proof.pi_a[1]).map_err(|_| JsValue::null())?;

    prepared_proof.set_proof_a(pi_a_x_bigint, pi_a_y_bigint)?;

    let pi_c_x_bigint = BigUint::from_str(&raw_proof.pi_c[0]).map_err(|_| JsValue::null())?;
    let pi_c_y_bigint = BigUint::from_str(&raw_proof.pi_c[1]).map_err(|_| JsValue::null())?;

    prepared_proof.set_proof_c(pi_c_x_bigint, pi_c_y_bigint)?;

    let pi_b_x0_bigint = BigUint::from_str(&raw_proof.pi_b[0][0]).map_err(|_| JsValue::null())?;
    let pi_b_y0_bigint = BigUint::from_str(&raw_proof.pi_b[0][1]).map_err(|_| JsValue::null())?;
    let pi_b_x1_bigint = BigUint::from_str(&raw_proof.pi_b[1][0]).map_err(|_| JsValue::null())?;
    let pi_b_y1_bigint = BigUint::from_str(&raw_proof.pi_b[1][1]).map_err(|_| JsValue::null())?;

    prepared_proof.set_proof_b(
        pi_b_x0_bigint,
        pi_b_y0_bigint,
        pi_b_x1_bigint,
        pi_b_y1_bigint,
    )?;

    prepared_proof.try_into()
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use ark_bn254::{Bn254, Fr};
    use ark_circom::{CircomBuilder, CircomConfig};
    use ark_ff::{BigInt, PrimeField};
    use ark_groth16::Groth16;
    use ark_snark::SNARK;
    use ark_std::rand::thread_rng;
    use color_eyre::Result;
    use light_poseidon::{Poseidon, PoseidonHasher};
    use merkle_poseidon::SparseMerkleTree;
    use solana_pubkey::Pubkey;

    use crate::{
        conversions::{FrPathToVec, PubkeyToFr},
        rand::random_fr,
    };

    type GrothBn = Groth16<Bn254>;

    const DEPTH: usize = 20;

    #[test]
    fn arkworks_wasm_compat_should_succeed() -> Result<()> {
        // Load circuit WASM and R1CS
        let cfg = CircomConfig::<Fr>::new("./circuits/ksb.wasm", "./circuits/ksb.r1cs")?;

        let mut builder = CircomBuilder::new(cfg);
        let mut hasher = Poseidon::<Fr>::new_circom(1)?;
        let mut merkle = SparseMerkleTree::new(DEPTH)?;
        // random path hash
        let path_hash = Fr::from_bigint(BigInt::from_str("1234567890").unwrap()).unwrap();
        merkle
            .insert_at_path(
                &path_hash,
                &Fr::from_bigint(BigInt::from_str("1234567890").unwrap()).unwrap(),
            )
            .unwrap();

        // Private inputs
        let nullifier = random_fr();
        let secret = random_fr();

        builder.push_input("nullifier", nullifier.into_bigint());
        builder.push_input("secret", secret.into_bigint());

        // Prepare test inputs
        // Public inputs
        let root = merkle.root_hash()?;
        let nullifier_hash = hasher.hash(&[nullifier]).unwrap();
        let recipient = Pubkey::new_unique().to_fr();

        builder.push_input("root", root.into_bigint());
        builder.push_input("nullifier_hash", nullifier_hash.into_bigint());
        builder.push_input("recipient", recipient.into_bigint());

        // Merkle path inputs (arrays)
        let path_indices = path_hash.to_vec();
        let merkle_proof = merkle.generate_proof(&path_hash).unwrap();
        let merkle_path = merkle_proof.siblings;

        // Push array inputs
        for (i, &value) in merkle_path.iter().enumerate() {
            if i >= DEPTH {
                break;
            }
            builder.push_input(format!("merkle_path[{}]", i), value.into_bigint());
        }

        for (i, &value) in path_indices.iter().enumerate() {
            if i >= DEPTH {
                break;
            }
            builder.push_input(format!("path_indices[{}]", i), value.into_bigint());
        }

        // Setup and generate proving key
        let circom = builder.setup();
        let mut rng = thread_rng();
        let params = GrothBn::generate_random_parameters_with_reduction(circom, &mut rng).unwrap();

        // Build circuit with witness
        let circom = builder.build()?;
        let inputs = circom.get_public_inputs().unwrap();

        let proof = GrothBn::prove(&params, circom, &mut rng)?;
        let pvk = GrothBn::process_vk(&params.vk).unwrap();

        let verified = GrothBn::verify_with_processed_vk(&pvk, &inputs, &proof)?;

        assert!(verified);

        Ok(())
    }
}

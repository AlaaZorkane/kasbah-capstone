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
mod tests;

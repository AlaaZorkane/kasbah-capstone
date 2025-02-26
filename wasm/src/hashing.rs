use ark_bn254::Fr;
use js_sys::BigInt;
use light_poseidon::{Poseidon, PoseidonHasher};
use wasm_bindgen::prelude::*;

use crate::conversions::FrJsValue;

#[wasm_bindgen]
pub fn hash_commitment(nullifier: BigInt, secret: BigInt) -> Result<BigInt, JsError> {
    let nullifier = Fr::from_js_bigint(nullifier)?;
    let secret = Fr::from_js_bigint(secret)?;

    let mut hasher = Poseidon::<Fr>::new_circom(2)?;
    let commitment = hasher.hash(&[nullifier, secret]).unwrap();

    let bn = commitment.to_js_bigint()?;

    Ok(bn)
}

#[wasm_bindgen]
pub fn hash_nullifier(nullifier: BigInt) -> Result<BigInt, JsError> {
    let nullifier = Fr::from_js_bigint(nullifier)?;

    let mut hasher = Poseidon::<Fr>::new_circom(1)?;
    let nullifier_hash = hasher.hash(&[nullifier]).unwrap();

    let bn = nullifier_hash.to_js_bigint()?;

    Ok(bn)
}

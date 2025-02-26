use ark_bn254::Fr;
use js_sys::{BigInt, Uint8Array};
use wasm_bindgen::prelude::*;

use crate::conversions::{FrJsValue, FrPathToVec};

#[wasm_bindgen]
pub fn commitment_to_path(commitment: BigInt, depth: usize) -> Result<Uint8Array, JsError> {
    let commitment = Fr::from_js_bigint(commitment)?;

    let path = commitment.to_bool_vec(depth);

    // Create typed array
    let array = Uint8Array::new_with_length(path.len() as u32);
    for (i, bit) in path.iter().enumerate() {
        array.set_index(i as u32, if *bit { 1 } else { 0 });
    }

    Ok(array)
}

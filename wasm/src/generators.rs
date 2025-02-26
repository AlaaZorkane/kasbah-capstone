use js_sys::BigInt;
use wasm_bindgen::prelude::*;

use crate::{conversions::FrJsValue, rand::random_fr};

#[wasm_bindgen]
pub fn generate_secret() -> Result<BigInt, JsError> {
    let secret = random_fr()?;
    let bn = secret.to_js_bigint()?;

    Ok(bn)
}

#[wasm_bindgen]
pub fn generate_nullifier() -> Result<BigInt, JsError> {
    let nullifier = random_fr()?;
    let bn = nullifier.to_js_bigint()?;

    Ok(bn)
}

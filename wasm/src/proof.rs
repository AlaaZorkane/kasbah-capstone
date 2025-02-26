use ark_bn254::{g1::G1Affine, Fq2, G2Affine};
use ark_ff::{BigInteger, PrimeField};
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use std::{ops::Neg, str::FromStr};
use wasm_bindgen::prelude::*;

use crate::{errors::ConversionError, utils::convert_endianness_vec};

#[wasm_bindgen(typescript_custom_section)]
const RAW_PROOF_INTERFACE: &'static str = r#"
interface RawProof {
    pi_a: string[];
    pi_b: string[][];
    pi_c: string[];
    protocol: string;
    curve: string;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "RawProof")]
    pub type RawProofInterface;
}

#[derive(Serialize, Deserialize)]
pub struct RawProof {
    pub pi_a: Vec<String>,
    pub pi_b: Vec<Vec<String>>,
    pub pi_c: Vec<String>,
    pub protocol: String,
    pub curve: String,
}

impl TryFrom<JsValue> for RawProof {
    type Error = ConversionError;

    fn try_from(raw_proof: JsValue) -> Result<Self, Self::Error> {
        serde_wasm_bindgen::from_value(raw_proof)
            .map_err(|_| ConversionError::ParseJsValueToRawProofError)
    }
}

#[wasm_bindgen(typescript_custom_section)]
const PREPARED_PROOF_INTERFACE: &'static str = r#"
interface PreparedProof {
    proof_a: number[];
    proof_b: number[];
    proof_c: number[];
    raw: number[];
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "PreparedProof")]
    pub type PreparedProofInterface;
}

#[derive(Serialize, Deserialize)]
pub struct PreparedProof {
    pub proof_a: Vec<u8>,
    pub proof_b: Vec<u8>,
    pub proof_c: Vec<u8>,
    pub raw: Vec<u8>,
}

impl PreparedProof {
    pub fn new() -> Self {
        Self {
            proof_a: Vec::new(),
            proof_b: Vec::new(),
            proof_c: Vec::new(),
            raw: vec![0; 256],
        }
    }

    pub fn set_proof_a(&mut self, x_int: BigUint, y_int: BigUint) {
        let g1 = G1Affine::new(x_int.into(), y_int.into()).neg();

        let g1_bytes = [
            g1.x.into_bigint().to_bytes_le(),
            g1.y.into_bigint().to_bytes_le(),
        ]
        .concat();

        self.proof_a = convert_endianness_vec(g1_bytes.as_slice(), 32);
        self.raw.splice(0..64, self.proof_a.clone());
    }

    pub fn set_proof_b(
        &mut self,
        x0_int: BigUint,
        y0_int: BigUint,
        x1_int: BigUint,
        y1_int: BigUint,
    ) {
        let g2_x = Fq2::new(x0_int.into(), y0_int.into());
        let g2_y = Fq2::new(x1_int.into(), y1_int.into());

        let g2 = G2Affine::new(g2_x, g2_y);
        let g2_bytes = [
            g2.x.c0.into_bigint().to_bytes_le(),
            g2.x.c1.into_bigint().to_bytes_le(),
            g2.y.c0.into_bigint().to_bytes_le(),
            g2.y.c1.into_bigint().to_bytes_le(),
        ]
        .concat();

        let g2_be = convert_endianness_vec(&g2_bytes, 32);

        self.proof_b = [
            g2_be[32..64].to_vec(),
            g2_be[0..32].to_vec(),
            g2_be[96..128].to_vec(),
            g2_be[64..96].to_vec(),
        ]
        .concat();
        self.raw.splice(64..192, self.proof_b.clone());
    }

    pub fn set_proof_c(&mut self, x_int: BigUint, y_int: BigUint) {
        let g1 = G1Affine::new(x_int.into(), y_int.into());

        let g1_bytes = [
            g1.x.into_bigint().to_bytes_le(),
            g1.y.into_bigint().to_bytes_le(),
        ]
        .concat();

        self.proof_c = convert_endianness_vec(g1_bytes.as_slice(), 32);
        self.raw.splice(192..256, self.proof_c.clone());
    }
}

impl Default for PreparedProof {
    fn default() -> Self {
        Self::new()
    }
}

impl TryInto<JsValue> for PreparedProof {
    type Error = ConversionError;

    fn try_into(self) -> Result<JsValue, Self::Error> {
        serde_wasm_bindgen::to_value(&self)
            .map_err(|_| ConversionError::SerializePreparedProofError)
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
///
/// p.s: this took a lot of reverse engineering )x
#[wasm_bindgen]
pub fn prepare_proofs(raw_proof: RawProofInterface) -> Result<PreparedProofInterface, JsError> {
    let raw_proof: JsValue = raw_proof.into();
    let raw_proof: RawProof = raw_proof.try_into()?;

    let mut prepared_proof = PreparedProof::new();

    let pi_a_x_bigint = BigUint::from_str(&raw_proof.pi_a[0])?;
    let pi_a_y_bigint = BigUint::from_str(&raw_proof.pi_a[1])?;

    prepared_proof.set_proof_a(pi_a_x_bigint, pi_a_y_bigint);

    let pi_c_x_bigint = BigUint::from_str(&raw_proof.pi_c[0])?;
    let pi_c_y_bigint = BigUint::from_str(&raw_proof.pi_c[1])?;

    prepared_proof.set_proof_c(pi_c_x_bigint, pi_c_y_bigint);

    let pi_b_x0_bigint = BigUint::from_str(&raw_proof.pi_b[0][0])?;
    let pi_b_y0_bigint = BigUint::from_str(&raw_proof.pi_b[0][1])?;
    let pi_b_x1_bigint = BigUint::from_str(&raw_proof.pi_b[1][0])?;
    let pi_b_y1_bigint = BigUint::from_str(&raw_proof.pi_b[1][1])?;

    prepared_proof.set_proof_b(
        pi_b_x0_bigint,
        pi_b_y0_bigint,
        pi_b_x1_bigint,
        pi_b_y1_bigint,
    );

    let prepared_proof: JsValue = prepared_proof.try_into()?;

    Ok(prepared_proof.into())
}

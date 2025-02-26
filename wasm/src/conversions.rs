use crate::errors::ConversionError;
use ark_bn254::Fr;
use ark_ff::{BigInteger, PrimeField};
use js_sys::BigInt;
use std::str::FromStr;
use wasm_bindgen::JsValue;

pub trait FrJsValue {
    fn to_js_value(&self) -> JsValue;
    fn to_js_bigint(&self) -> Result<BigInt, ConversionError>;
    fn from_js_value(value: JsValue) -> Result<Self, ConversionError>
    where
        Self: std::marker::Sized;
    fn from_js_bigint(value: BigInt) -> Result<Self, ConversionError>
    where
        Self: std::marker::Sized;
}

impl FrJsValue for Fr {
    fn to_js_value(&self) -> JsValue {
        JsValue::bigint_from_str(&self.to_string())
    }

    fn from_js_value(value: JsValue) -> Result<Self, ConversionError> {
        if value.is_bigint() {
            if let Some(bigint) = value.as_string() {
                Fr::from_str(&bigint).map_err(|_| ConversionError::ParseJsValueToFrError)
            } else {
                Err(ConversionError::BigIntToStringError)
            }
        } else {
            Err(ConversionError::JsValueIsNotBigInt)
        }
    }

    fn from_js_bigint(value: BigInt) -> Result<Self, ConversionError> {
        let str = value
            .to_string(10)
            .map_err(|_| ConversionError::BigIntToStringError)?;
        let str: String = str.into();
        Fr::from_str(&str).map_err(|_| ConversionError::ParseJsValueToFrError)
    }

    fn to_js_bigint(&self) -> Result<BigInt, ConversionError> {
        let str = self.to_string();
        BigInt::from_str(&str).map_err(|_| ConversionError::BigIntToStringError)
    }
}

pub trait FrPathToVec {
    fn to_fr_vec(&self, len: usize) -> Vec<Fr>;
    fn to_bool_vec(&self, len: usize) -> Vec<bool>;
}

impl FrPathToVec for Fr {
    /// Convert a Fr element to a Vec<Fr> with bits 0 and 1 as multiple elements
    ///
    /// Use this to convert a PathHash to a Vec<Fr> with the path indices
    fn to_fr_vec(&self, len: usize) -> Vec<Fr> {
        let bits = self.into_bigint().to_bits_le();
        let mut vec = Vec::new();
        for bit in bits.iter().take(len) {
            vec.push(Fr::from(*bit as u64));
        }
        vec
    }

    fn to_bool_vec(&self, len: usize) -> Vec<bool> {
        let bits = self.into_bigint().to_bits_le();
        bits.iter().take(len).copied().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_bits_vec() {
        let fr = Fr::from(3u64); // (11) - right, right
        let vec = fr.to_fr_vec(2);
        assert_eq!(vec, vec![Fr::from(1u64), Fr::from(1u64)]);
    }

    #[test]
    fn test_to_bool_vec() {
        let fr = Fr::from(3u64); // (11) - right, right
        let vec = fr.to_bool_vec(2);
        assert_eq!(vec, vec![true, true]);
    }
}

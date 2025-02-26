use light_poseidon::PoseidonError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum KasbahError {
    #[error("failed to generate secret")]
    GenerateSecretError,
    #[error(transparent)]
    Conversion(#[from] ConversionError),
    #[error(transparent)]
    Poseidon(#[from] PoseidonError),
}

#[derive(Error, Debug, PartialEq)]
pub enum ConversionError {
    #[error("failed to parse JsValue to Fr")]
    ParseJsValueToFrError,
    #[error("JsValue is not a bigint")]
    JsValueIsNotBigInt,
    #[error("failed to convert bigint to string")]
    BigIntToStringError,
    #[error("failed to parse JsValue to RawProof")]
    ParseJsValueToRawProofError,
    #[error("failed to serialize PreparedProof")]
    SerializePreparedProofError,
}

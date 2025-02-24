use ark_bn254::{g1::G1Affine, Fq2, G2Affine};
use ark_ff::{BigInteger, PrimeField};
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use std::ops::Neg;
use wasm_bindgen::JsValue;

use crate::utils::convert_endianness_vec;

#[derive(Serialize, Deserialize)]
pub struct RawProof {
    pub pi_a: Vec<String>,
    pub pi_b: Vec<Vec<String>>,
    pub pi_c: Vec<String>,
    pub protocol: String,
    pub curve: String,
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

    pub fn set_proof_a(&mut self, x_int: BigUint, y_int: BigUint) -> Result<(), JsValue> {
        let g1 = G1Affine::new(x_int.into(), y_int.into()).neg();

        let g1_bytes = [
            g1.x.into_bigint().to_bytes_le(),
            g1.y.into_bigint().to_bytes_le(),
        ]
        .concat();

        self.proof_a = convert_endianness_vec(g1_bytes.as_slice(), 32);
        self.raw.splice(0..64, self.proof_a.clone());

        Ok(())
    }

    pub fn set_proof_b(
        &mut self,
        x0_int: BigUint,
        y0_int: BigUint,
        x1_int: BigUint,
        y1_int: BigUint,
    ) -> Result<(), JsValue> {
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

        Ok(())
    }

    pub fn set_proof_c(&mut self, x_int: BigUint, y_int: BigUint) -> Result<(), JsValue> {
        let g1 = G1Affine::new(x_int.into(), y_int.into());

        let g1_bytes = [
            g1.x.into_bigint().to_bytes_le(),
            g1.y.into_bigint().to_bytes_le(),
        ]
        .concat();

        self.proof_c = convert_endianness_vec(g1_bytes.as_slice(), 32);
        self.raw.splice(192..256, self.proof_c.clone());

        Ok(())
    }
}

impl TryInto<JsValue> for PreparedProof {
    type Error = JsValue;

    fn try_into(self) -> Result<JsValue, Self::Error> {
        serde_wasm_bindgen::to_value(&self).map_err(|err| JsValue::from_str(&err.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::convert_endianness_vec;

    use super::PreparedProof;
    use ark_bn254::{g1::G1Affine, g2::G2Affine, Fq2};
    use ark_ff::{BigInteger, PrimeField};
    use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
    use num_bigint::BigUint;
    use std::str::FromStr;

    /// Proofs are stored as big-endian bytes.
    pub const PROOF: [u8; 256] = [
        30, 224, 237, 110, 5, 97, 5, 4, 215, 78, 46, 210, 139, 140, 124, 27, 38, 217, 231, 248,
        193, 201, 179, 253, 43, 176, 181, 85, 253, 227, 205, 0, 10, 171, 214, 28, 83, 89, 11, 147,
        96, 197, 126, 86, 92, 68, 254, 86, 98, 236, 85, 109, 84, 18, 47, 12, 143, 89, 232, 181, 40,
        170, 85, 38, 1, 120, 86, 178, 146, 60, 231, 71, 4, 98, 212, 99, 127, 230, 154, 189, 100,
        239, 128, 110, 157, 154, 0, 173, 159, 78, 131, 59, 36, 78, 82, 0, 18, 219, 2, 128, 232,
        169, 93, 131, 200, 24, 76, 24, 96, 153, 238, 126, 13, 83, 134, 179, 147, 224, 221, 227, 69,
        118, 114, 92, 168, 178, 142, 5, 20, 24, 195, 255, 61, 131, 225, 9, 84, 239, 254, 128, 141,
        63, 52, 49, 22, 9, 147, 69, 15, 247, 126, 128, 14, 117, 124, 111, 100, 217, 215, 42, 30,
        84, 227, 208, 19, 91, 230, 240, 162, 234, 46, 196, 186, 160, 254, 174, 74, 66, 65, 1, 58,
        65, 64, 151, 107, 96, 93, 133, 81, 114, 149, 114, 5, 2, 0, 23, 117, 104, 223, 250, 146,
        215, 45, 193, 39, 139, 238, 222, 133, 201, 210, 3, 135, 26, 154, 49, 148, 245, 24, 75, 28,
        103, 194, 6, 2, 99, 168, 248, 9, 219, 103, 73, 37, 218, 156, 39, 251, 245, 85, 61, 38, 182,
        106, 53, 75, 160, 22, 156, 51, 169, 136, 101, 95, 84, 52, 3,
    ];

    #[test]
    fn test_g1_serialization() {
        let mut prepared_proof = PreparedProof::new();
        let a_x = BigUint::from_str(
            "21354890625990214870754375882710379221752862473433052375025104960967799391247",
        )
        .unwrap();

        let a_y = BigUint::from_str(
            "9290940334260810909325264371688111273809487189485387679341484436824715253293",
        )
        .unwrap();

        prepared_proof.set_proof_a(a_x, a_y).unwrap();

        let proof_a_le = convert_endianness_vec(prepared_proof.proof_a.as_slice(), 32);
        let mut reader = proof_a_le.as_slice();
        let g1 = G1Affine::deserialize_uncompressed(&mut reader).unwrap();
        println!("g1: {:?}", g1);
        println!("size: {:?}", proof_a_le.len());
    }

    #[test]
    fn test_g2_serialization() {
        let mut prepared_proof = PreparedProof::new();
        let b_x_0 = BigUint::from_str(
            "7306243638971215804951680750533088848275286064001317063496098528064419749903",
        )
        .unwrap();
        let b_y_0 = BigUint::from_str(
            "7844607428842522796576705607203096747373145189506599069140420161427295788542",
        )
        .unwrap();
        let b_x_1 = BigUint::from_str(
            "10455015464246595512016883905509923021523643682756570031364275697838770910732",
        )
        .unwrap();
        let b_y_1 = BigUint::from_str(
            "12154566038230164257680564580258256126266356181539069382921568167830671442360",
        )
        .unwrap();

        prepared_proof
            .set_proof_b(b_x_0.clone(), b_y_0.clone(), b_x_1.clone(), b_y_1.clone())
            .unwrap();

        let proof_b_be = prepared_proof.proof_b.clone();
        let mut reader = proof_b_be.as_slice();
        let g2 = G2Affine::deserialize_uncompressed(&mut reader).unwrap();
        println!("g2: {:?}", g2);
        println!("size: {:?}", proof_b_be.len());

        assert_eq!(g2.x.c0, b_x_0.into());
        assert_eq!(g2.x.c1, b_y_0.into());
        assert_eq!(g2.y.c0, b_x_1.into());
        assert_eq!(g2.y.c1, b_y_1.into());
    }

    #[test]
    fn test_biguint() {
        let x = "11684643836096726770599088807339397581266930370717315968504711257306654148396";
        let y = "11750776160873302347811134195434586959137284877458760350265397742438820965603";
        let x_bigint = BigUint::from_str(x).unwrap();
        let y_bigint = BigUint::from_str(y).unwrap();

        let g1 = G1Affine::new(x_bigint.into(), y_bigint.into());
        let g1_bytes = [
            g1.x.into_bigint().to_bytes_le(),
            g1.y.into_bigint().to_bytes_le(),
        ]
        .concat();

        let mut g1_serialized = Vec::new();
        g1.x.serialize_uncompressed(&mut g1_serialized).unwrap();
        g1.y.serialize_uncompressed(&mut g1_serialized).unwrap();

        println!("g1_bytes: {:?}", g1_bytes);
        println!("g1_serialized: {:?}", g1_serialized);

        let g1_deserialized_bytes =
            G1Affine::deserialize_uncompressed(g1_bytes.as_slice()).unwrap();
        println!("g1_deserialized_bytes: {:?}", g1_deserialized_bytes);

        let g1_deserialized = G1Affine::deserialize_uncompressed(g1_serialized.as_slice()).unwrap();
        println!("g1_deserialized: {:?}", g1_deserialized);

        assert_eq!(g1_deserialized_bytes, g1_deserialized);
    }

    #[test]
    fn test_biguint_g2() {
        let x0 = "14216884210267525919507070345667383971866700897845602088006221319669418508083";
        let y0 = "18840154083637656907946007719307077948204433137257450820400465956593170250320";
        let x1 = "11745275580308852061547924948557228684308373433817744242523001571432801442171";
        let y1 = "13866709958428683102427858001680607806485852003717240084285944281978656316586";
        let x0_bigint = BigUint::from_str(x0).unwrap();
        let y0_bigint = BigUint::from_str(y0).unwrap();
        let x1_bigint = BigUint::from_str(x1).unwrap();
        let y1_bigint = BigUint::from_str(y1).unwrap();

        let g2_x = Fq2::new(x0_bigint.into(), y0_bigint.into());
        let g2_y = Fq2::new(x1_bigint.into(), y1_bigint.into());
        let g2 = G2Affine::new(g2_x, g2_y);
        let g2_bytes = [
            g2.x.c0.into_bigint().to_bytes_le(),
            g2.x.c1.into_bigint().to_bytes_le(),
            g2.y.c0.into_bigint().to_bytes_le(),
            g2.y.c1.into_bigint().to_bytes_le(),
        ]
        .concat();

        let mut g2_serialized = Vec::new();
        g2.x.serialize_uncompressed(&mut g2_serialized).unwrap();
        g2.y.serialize_uncompressed(&mut g2_serialized).unwrap();

        println!("g2_bytes: {:?}", g2_bytes);
        println!("g2_serialized: {:?}", g2_serialized);

        let g2_deserialized_bytes =
            G2Affine::deserialize_uncompressed(g2_bytes.as_slice()).unwrap();
        println!("g2_deserialized_bytes: {:?}", g2_deserialized_bytes);

        let g2_deserialized = G2Affine::deserialize_uncompressed(g2_serialized.as_slice()).unwrap();
        println!("g2_deserialized: {:?}", g2_deserialized);

        assert_eq!(g2_deserialized_bytes, g2_deserialized);
    }

    #[test]
    fn test_points_deserialization() {
        let points_a_be = PROOF[0..64].to_vec();
        let points_a_le = convert_endianness_vec(&points_a_be, 32);
        assert_eq!(points_a_le.len(), 64);

        let points_b_be = PROOF[64..192].to_vec();
        let points_b_le = convert_endianness_vec(&points_b_be, 64);
        assert_eq!(points_b_le.len(), 128);

        let points_c_be = PROOF[192..256].to_vec();
        let points_c_le = convert_endianness_vec(&points_c_be, 32);
        assert_eq!(points_c_le.len(), 64);

        let points_a_x = BigUint::from_bytes_be(&points_a_be[0..32]);
        let points_a_y = BigUint::from_bytes_be(&points_a_be[32..64]);
        println!("points_a_x: {:?}", points_a_x);
        println!("points_a_y: {:?}", points_a_y);
        println!("--------------------------------");

        let points_c_x = BigUint::from_bytes_be(&points_c_be[0..32]);
        let points_c_y = BigUint::from_bytes_be(&points_c_be[32..64]);
        println!("points_c_x: {:?}", points_c_x);
        println!("points_c_y: {:?}", points_c_y);
        println!("--------------------------------");

        let points_b_x0 = BigUint::from_bytes_be(&points_b_be[32..64]);
        let points_b_y0 = BigUint::from_bytes_be(&points_b_be[0..32]);
        let points_b_x1 = BigUint::from_bytes_be(&points_b_be[96..]);
        let points_b_y1 = BigUint::from_bytes_be(&points_b_be[64..96]);
        println!("points_b_x0: {:?}", points_b_x0);
        println!("points_b_y0: {:?}", points_b_y0);
        println!("points_b_x1: {:?}", points_b_x1);
        println!("points_b_y1: {:?}", points_b_y1);
        println!("--------------------------------");

        let points_a = G1Affine::deserialize_uncompressed(&mut points_a_le.as_slice()).unwrap();
        let points_b = G2Affine::deserialize_uncompressed(&mut points_b_le.as_slice()).unwrap();
        let points_c = G1Affine::deserialize_uncompressed(&mut points_c_le.as_slice()).unwrap();

        println!("points_a: {:?}", points_a);
        println!("points_b: {:?}", points_b);
        println!("points_c: {:?}", points_c);

        // G1
        assert_eq!(points_a_x, points_a.x.into());
        assert_eq!(points_a_y, points_a.y.into());
        assert_eq!(points_c_x, points_c.x.into());
        assert_eq!(points_c_y, points_c.y.into());

        // G2
        assert_eq!(points_b_x0, points_b.x.c0.into());
        assert_eq!(points_b_y0, points_b.x.c1.into());
        assert_eq!(points_b_x1, points_b.y.c0.into());
        assert_eq!(points_b_y1, points_b.y.c1.into());
    }
}

use ark_bn254::{Bn254, Fr};
use ark_circom::{CircomBuilder, CircomConfig};
use ark_ff::{AdditiveGroup, PrimeField};
use ark_groth16::Groth16;
use ark_snark::SNARK;
use ark_std::rand::thread_rng;
use color_eyre::Result;
use light_poseidon::{Poseidon, PoseidonHasher};
use merkle_poseidon::SparseMerkleTree;

use crate::{conversions::FrPathToVec, rand::random_fr};

type GrothBn = Groth16<Bn254>;

const DEPTH: usize = 2;

#[test]
fn test_poseidon_hash() -> Result<()> {
    let mut one_hasher = Poseidon::<Fr>::new_circom(1)?;
    let mut two_hasher = Poseidon::<Fr>::new_circom(2)?;

    let leaf_empty_hash = one_hasher.hash(&[Fr::ZERO]).unwrap();
    let inner_empty_hash = two_hasher
        .hash(&[leaf_empty_hash, leaf_empty_hash])
        .unwrap();
    println!("leaf_empty_hash: {:?}", leaf_empty_hash);
    println!("inner_empty_hash: {:?}", inner_empty_hash);
    Ok(())
}

#[tokio::test]
async fn arkworks_wasm_compat_should_succeed() -> Result<()> {
    // Load circuit WASM and R1CS
    let cfg = CircomConfig::<Fr>::new("../circuits/ksb.wasm", "../circuits/ksb.r1cs")?;

    let mut builder = CircomBuilder::new(cfg);
    let mut hasher = Poseidon::<Fr>::new_circom(1)?;
    let nullifier = random_fr().unwrap();
    let secret = random_fr().unwrap();
    let mut commitment_hasher = Poseidon::<Fr>::new_circom(2)?;
    let commitment = commitment_hasher.hash(&[nullifier, secret]).unwrap();

    let mut merkle = SparseMerkleTree::new(DEPTH)?;
    // random path hash
    let merkle_path = Fr::from(3u64); // (11) - right, right
    merkle.insert_at_path(&merkle_path, &commitment).unwrap();

    // Private inputs
    builder.push_input("nullifier", nullifier.into_bigint());
    builder.push_input("secret", secret.into_bigint());

    println!("nullifier: {:?}", nullifier.into_bigint());
    println!("secret: {:?}", secret.into_bigint());

    // Prepare test inputs
    // Public inputs
    let root = merkle.root_hash()?;
    let nullifier_hash = hasher.hash(&[nullifier]).unwrap();

    builder.push_input("root", root.into_bigint());
    builder.push_input("nullifier_hash", nullifier_hash.into_bigint());

    println!("root: {:?}", root.into_bigint());
    println!("nullifier_hash: {:?}", nullifier_hash.into_bigint());

    // Merkle path inputs (arrays)
    let merkle_path_vec = merkle_path.to_fr_vec(DEPTH);
    let merkle_proof = merkle.generate_proof(&merkle_path).unwrap();
    let siblings = merkle_proof.siblings;

    // Push array inputs
    for (i, &value) in siblings.iter().enumerate() {
        if i >= DEPTH {
            break;
        }
        builder.push_input("siblings", value.into_bigint());
    }

    for (i, &value) in merkle_path_vec.iter().enumerate() {
        if i >= DEPTH {
            break;
        }
        builder.push_input("merkle_path", value.into_bigint());
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

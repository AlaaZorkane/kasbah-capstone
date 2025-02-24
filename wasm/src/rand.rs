use ark_bn254::Fr;
use ark_ff::UniformRand;
use ark_std::rand::rngs::StdRng;
use ark_std::rand::SeedableRng;

/// Generate a random BN254 Fr element in a WASM-friendly way.
/// 1. Gather 32 bytes of randomness from getrandom (works in browser/Node).
/// 2. Seed a StdRng with those bytes.
/// 3. Generate a random field element via `UniformRand`.
pub fn random_fr() -> Fr {
    // 1. Get 32 bytes from getrandom
    let mut seed: [u8; 32] = [0u8; 32];
    getrandom::fill(&mut seed).expect("Failed to get randomness");

    // 2. Create a std-based RNG from that seed
    // StdRng is a deterministic CSPRNG (specifically the ChaCha algorithm)
    let mut rng = StdRng::from_seed(seed);

    // 3. Generate the random field element
    Fr::rand(&mut rng)
}

use borsh::{BorshDeserialize, BorshSerialize};
use solana_zk_sdk::encryption::PEDERSEN_OPENING_LEN;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct KasbahSecrets {
    pub opening: [u8; PEDERSEN_OPENING_LEN],
    pub nullifier: [u8; blake3::OUT_LEN],
    pub amount: u64,
}

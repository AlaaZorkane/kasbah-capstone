pub fn convert_endianness_vec(bytes: &[u8], chunk_size: usize) -> Vec<u8> {
    bytes
        .chunks_exact(chunk_size)
        .flat_map(|chunk| chunk.iter().rev().copied())
        .collect()
}

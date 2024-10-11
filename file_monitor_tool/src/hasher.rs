pub fn custom_hash(data: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    let prime: u64 = 0x100000001b3;

    for &byte in data {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(prime);
    }

    hash
}

pub use nohash::NoHashHasher;
pub use std::collections::HashMap;
pub use std::hash::BuildHasherDefault;
use std::simd::Simd;
use crate::SIMD_LANES;

pub type NoHashMap<K, V> = HashMap<K, V, BuildHasherDefault::<NoHashHasher<K>>>;

const PRIME: u64 = 11400714785074694791;
const PRIME_VECTOR: Simd::<u64, SIMD_LANES> = Simd::splat(PRIME);

pub fn parallel_hash(vector: Simd::<u64, SIMD_LANES>) -> Simd::<u64, SIMD_LANES> {
    let round1 = vector ^ (vector >> 33);
    let round2 = round1 * PRIME_VECTOR;
    round2 ^ (round2 >> 33)
}

pub fn hash(value: u64) -> u64 {
    let round1 = value ^ value.wrapping_shr(33);
    let round2 = round1.wrapping_mul(PRIME);
    round2 ^ round2.wrapping_shr(33)
}

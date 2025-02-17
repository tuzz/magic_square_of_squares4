use crate::*;

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

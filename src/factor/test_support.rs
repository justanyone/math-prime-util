//! Shared inputs for the mutation-killing unit tests of the private splitters.
//!
//! The splitters are deterministic, so a fold over their results on a fixed
//! input set pins their exact behavior, including which factor each finds.

use crate::next_prime;

/// Composite inputs from 10 to 62 bits: balanced, lopsided, three-prime and
/// prime-square shapes, all with every prime factor above 43.
pub(super) fn inputs() -> Vec<u64> {
    let mut out = Vec::new();
    for bits in (10_u32..=62).step_by(4) {
        let half = bits / 2;
        let third = bits / 3;
        for k in [0_u64, 7, 91] {
            let p = next_prime((1 << (half - 1)) + 4 * k).expect("below 2^63");
            let q = next_prime((1 << (bits - half)) + 100 * k + 1).expect("below 2^63");
            out.push(p * q);
            let r = next_prime((1 << (third - 1)) + 2 * k + 45).expect("below 2^63");
            let s = next_prime((1 << (bits - third - 1)) + 30 * k).expect("below 2^63");
            out.push(r * s);
        }
        let p = next_prime(1 << (third - 1)).expect("below 2^63").max(47);
        let q = next_prime(p + 1000).expect("below 2^63");
        let r = next_prime(q + 20_000).expect("below 2^63");
        if let Some(m) = p.checked_mul(q).and_then(|m| m.checked_mul(r)) {
            out.push(m);
        }
        out.push(p * p);
    }
    out.retain(|&n| n > 2000);
    out.sort_unstable();
    out.dedup();
    out
}

/// One step of an order-sensitive fold (FNV-1a style over 64-bit words).
pub(super) fn fold(hash: u64, x: u64) -> u64 {
    (hash ^ x.rotate_left(17)).wrapping_mul(0x0000_0100_0000_01b3) ^ (hash >> 29)
}

/// Fold an optional factor so `None` differs from every `Some`.
pub(super) fn fold_opt(hash: u64, x: Option<u64>) -> u64 {
    fold(hash, x.map_or(0, |v| v.wrapping_add(1)))
}

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

/// Products of a prime next to a smooth number and a larger prime, so the
/// p-1 and Chebyshev stages succeed, plus small odd composites for the
/// failure paths of the rho and Brent retries.
pub(super) fn extra_inputs() -> Vec<u64> {
    let mut out = Vec::new();
    for m in [
        720_u64,
        5_040,
        55_440,
        360_360,
        2_882_880,
        21_621_600,
        367_567_200,
        6_983_776_800,
    ] {
        for p in [m - 1, m + 1] {
            if crate::is_prime(p) == 0 {
                continue;
            }
            for bits in [20_u32, 26, 31] {
                let q = next_prime((1 << bits) + p % 1000).expect("below 2^63");
                if let Some(n) = p.checked_mul(q) {
                    out.push(n);
                }
            }
        }
    }
    for n in (9..20_000_u64).step_by(2) {
        if crate::is_prime(n) == 0 {
            out.push(n);
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

/// Wider inputs than `inputs()`: semiprimes of many size ratios (some below
/// 2^36, some above, some straddling the Lehman cutoff 8,796,393,022,207),
/// prime squares and cubes, and numbers with factors below and above 2011.
pub(super) fn wide_inputs() -> Vec<u64> {
    let mut out = Vec::new();
    for bp in (5_u32..=31).step_by(2) {
        for bq in (bp..=62 - bp).step_by(3) {
            for k in [0_u64, 5] {
                let p = next_prime((1 << bp) + 37 * k).expect("below 2^63");
                let q = next_prime((1 << bq) + 101 * k + 1).expect("below 2^63");
                if let Some(n) = p.checked_mul(q) {
                    out.push(n);
                }
            }
        }
    }
    let p = next_prime(2_000_000).expect("small");
    let mut q = 8_796_393_022_207 / p - 40;
    for _ in 0..24 {
        q = next_prime(q).expect("small");
        out.push(p * q);
        q += 1;
    }
    for p in [53_u64, 101, 1009, 2003, 2017, 40_009, 1_000_003] {
        out.push(p * p);
        if let Some(n) = (p * p).checked_mul(p) {
            out.push(n);
        }
        out.push(p * next_prime(p + 2).expect("small") * next_prime(p * 7).expect("small"));
    }
    out.retain(|&n| n > 2000);
    out.sort_unstable();
    out.dedup();
    out
}

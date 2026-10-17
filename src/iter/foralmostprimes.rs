use super::run;
use std::ops::{ControlFlow, RangeInclusive};

/// Iterator over the integers in a range that have exactly `k` prime factors,
/// counted with multiplicity. See `XS.xs`, `foralmostprimes`.
#[derive(Debug, Clone)]
pub struct AlmostPrimesIn {
    k: u32,
    range: RangeInclusive<u64>,
    shift: u32,
}

impl Iterator for AlmostPrimesIn {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.k == 0 || self.k >= 64 {
            return None;
        }
        let k = self.k;
        self.range
            .find(move |&n| crate::is_almost_prime(u64::from(k), n))
            .map(|n| n << self.shift)
    }
}

/// Iterate over the almost primes with `k` factors in an inclusive range.
/// See `XS.xs`, `foralmostprimes`.
/// ```
/// assert_eq!(math_prime_util::almost_primes_in(3, 0..=30).collect::<Vec<_>>(), [8, 12, 18, 20, 27, 28, 30]);
/// ```
#[must_use]
pub fn almost_primes_in(k: u32, range: RangeInclusive<u64>) -> AlmostPrimesIn {
    // XS.xs::foralmostprimes reduces k when every result must be even.
    if range.is_empty() || k == 0 || k >= 64 {
        return AlmostPrimesIn {
            k: 0,
            range: RangeInclusive::new(1, 0),
            shift: 0,
        };
    }
    let mut lo = (*range.start()).max(1_u64 << k);
    let mut hi = (*range.end()).min(u64::MAX - MAX_OFFSETS[k as usize]);
    let mut shift = k.saturating_sub(40);
    while k - shift > 1 && (hi >> shift) < 3_u64.pow(k - shift) {
        shift += 1;
    }
    lo = (lo >> shift) + u64::from(((lo >> shift) << shift) < lo);
    hi >>= shift;
    AlmostPrimesIn {
        k: k - shift,
        range: lo..=hi,
        shift,
    }
}

/// Visit each almost prime with `k` factors in an inclusive range. See `XS.xs`, `foralmostprimes`.
/// ```
/// use std::ops::ControlFlow;
/// let result = math_prime_util::foralmostprimes(2, 0..=100, |n| {
///     if n > 10 { ControlFlow::Break(n) } else { ControlFlow::Continue(()) }
/// });
/// assert_eq!(result, ControlFlow::Break(14));
/// ```
pub fn foralmostprimes<B>(
    k: u32,
    range: RangeInclusive<u64>,
    f: impl FnMut(u64) -> ControlFlow<B>,
) -> ControlFlow<B> {
    run(almost_primes_in(k, range), f)
}

// MPU 0.75 almost_primes.c::max_nth_almost_prime, 64-bit table.
const MAX_OFFSETS: [u64; 64] = [
    u64::MAX - 1,
    58,
    14,
    2,
    4,
    3,
    17,
    0,
    1,
    195,
    51,
    127,
    63,
    767,
    1535,
    511,
    255,
    15,
    8191,
    1023,
    83967,
    16383,
    111615,
    557055,
    2097151,
    524287,
    65535,
    1048575,
    6553599,
    33554431,
    4194303,
    671088639,
    16777215,
    536870911,
    2684354559,
    2147483647,
    25769803775,
    4294967295,
    268435455,
    206158430207,
    137438953471,
    17179869183,
    68719476735,
    2199023255551,
    5428838662143,
    21990232555519,
    4398046511103,
    1099511627775,
    100055558127615,
    10995116277759,
    17592186044415,
    545357767376895,
    70368744177663,
    1125899906842623,
    10133099161583615,
    9007199254740991,
    24769797950537727,
    4503599627370495,
    144115188075855871,
    72057594037927935,
    432345564227567615,
    288230376151711743,
    1152921504606846975,
    4611686018427387903,
];

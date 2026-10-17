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
    1_535,
    511,
    255,
    15,
    8_191,
    1_023,
    83_967,
    16_383,
    111_615,
    557_055,
    2_097_151,
    524_287,
    65_535,
    1_048_575,
    6_553_599,
    33_554_431,
    4_194_303,
    671_088_639,
    16_777_215,
    536_870_911,
    2_684_354_559,
    2_147_483_647,
    25_769_803_775,
    4_294_967_295,
    268_435_455,
    206_158_430_207,
    137_438_953_471,
    17_179_869_183,
    68_719_476_735,
    2_199_023_255_551,
    5_428_838_662_143,
    21_990_232_555_519,
    4_398_046_511_103,
    1_099_511_627_775,
    100_055_558_127_615,
    10_995_116_277_759,
    17_592_186_044_415,
    545_357_767_376_895,
    70_368_744_177_663,
    1_125_899_906_842_623,
    10_133_099_161_583_615,
    9_007_199_254_740_991,
    24_769_797_950_537_727,
    4_503_599_627_370_495,
    144_115_188_075_855_871,
    72_057_594_037_927_935,
    432_345_564_227_567_615,
    288_230_376_151_711_743,
    1_152_921_504_606_846_975,
    4_611_686_018_427_387_903,
];

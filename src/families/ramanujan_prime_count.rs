use super::ramanujan_primes::{SMALL, range};

/// Count the Ramanujan primes at most `n`.
///
/// Port of MPU 0.75 `ramanujan_primes.c`, `ramanujan_prime_count` and
/// `_ramanujan_prime_window`. Powers of two use the C lookup table.
///
/// ```
/// assert_eq!(math_prime_util::ramanujan_prime_count(1000), 72);
/// ```
pub fn ramanujan_prime_count(n: u64) -> u64 {
    const POW2: [u64; 57] = [
        0,
        1,
        1,
        1,
        2,
        4,
        7,
        13,
        23,
        42,
        75,
        137,
        255,
        463,
        872,
        1612,
        3030,
        5706,
        10_749,
        20_387,
        38_635,
        73_584,
        140_336,
        268_216,
        513_705,
        985_818,
        1_894_120,
        3_645_744,
        7_027_290,
        13_561_906,
        26_207_278,
        50_697_533,
        98_182_656,
        190_335_585,
        369_323_301,
        717_267_167,
        1_394_192_236,
        2_712_103_833,
        5_279_763_823,
        10_285_641_777,
        20_051_180_846,
        39_113_482_639,
        76_344_462_797,
        149_100_679_004,
        291_354_668_495,
        569_630_404_447,
        1_114_251_967_767,
        2_180_634_225_768,
        4_269_555_883_751,
        8_363_243_713_305,
        16_388_947_026_629,
        32_129_520_311_897,
        63_012_603_695_171,
        123_627_200_537_929,
        242_637_500_756_376,
        476_379_740_340_417,
        935_609_435_783_647,
    ];
    if n.is_power_of_two() && n.ilog2() <= 56 {
        return POW2[n.ilog2() as usize];
    }
    if n <= 241 {
        return SMALL.partition_point(|&p| p <= n) as u64;
    }
    let v = crate::prime_count(n) - crate::prime_count(n / 2);
    let mut window = if n > 1_000_000_000 { 320 } else { 20 };
    loop {
        let first = v.saturating_sub(window).max(1);
        let values = range(first, v + window);
        if values[0] < n && values[values.len() - 1] > n {
            return first + values.partition_point(|&p| p <= n) as u64 - 1;
        }
        window *= 2;
    }
}

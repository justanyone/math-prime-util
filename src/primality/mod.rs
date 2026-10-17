//! Native pseudoprime tests from MPU 0.75 `primality.c`.
use crate::int::{gcd, is_square, mont::Mont};
use crate::lucas::lucas_sequence;
use crate::modular::{addmod_u, mulmod_u, powmod_u};

fn jacobi(mut a: u64, mut n: u64) -> i8 {
    let mut sign = 1;
    while a != 0 {
        let shift = a.trailing_zeros();
        a >>= shift;
        if shift & 1 != 0 && matches!(n % 8, 3 | 5) {
            sign = -sign;
        }
        std::mem::swap(&mut a, &mut n);
        if a % 4 == 3 && n % 4 == 3 {
            sign = -sign;
        }
        a %= n;
    }
    if n == 1 { sign } else { 0 }
}
fn signed_jacobi(a: i64, n: u64) -> i8 {
    let j = jacobi(a.unsigned_abs(), n);
    if a < 0 && n % 4 == 3 { -j } else { j }
}
fn sub(a: u64, b: u64, n: u64) -> u64 {
    if a >= b { a - b } else { n - (b - a) }
}
fn extra_parameters(n: u64, increment: u64) -> Option<i64> {
    let mut p = 3_u64;
    loop {
        let d = p * p - 4;
        let j = jacobi(d, n);
        if j == 0 {
            let g = gcd(d, n);
            if g != 1 && g != n {
                return None;
            }
        }
        if j == -1 {
            return Some(i64::try_from(p % n).expect("bounded parameter"));
        }
        if p == 3 + 20 * increment && is_square(n) {
            return None;
        }
        p += increment;
        assert!(p <= 65_535, "Lucas parameter exceeded 65_535");
    }
}

/// Tests the Fermat congruence.
///
/// Port of MPU `primality.c`, `is_pseudoprime`.
///
/// # Panics
/// Panics if the base is below 2 after the small-input checks.
///
/// ```
/// use math_prime_util::is_pseudoprime;
/// assert!(is_pseudoprime(341, 2));
/// ```
pub fn is_pseudoprime(n: u64, mut a: u64) -> bool {
    if n < 3 {
        return n == 2;
    }
    if n & 1 == 0 && a & 1 == 0 {
        return false;
    }
    assert!(a >= 2, "invalid base");
    if a >= n {
        a %= n;
        if a <= 1 {
            return a == 1;
        }
        if a == n - 1 {
            return a & 1 == 0;
        }
    }
    if n & 1 == 1 {
        let m = Mont::new(n);
        m.pow(m.to_mont(a), n - 1) == m.one
    } else {
        powmod_u(a, n - 1, n) == 1
    }
}

/// Tests one Miller–Rabin base.
///
/// Port of MPU `primality.c`, `is_strong_pseudoprime`.
///
/// # Panics
/// Panics if the base is below 2 after the small-input checks.
///
/// ```
/// use math_prime_util::is_strong_pseudoprime;
/// assert!(is_strong_pseudoprime(2047, 2));
/// ```
pub fn is_strong_pseudoprime(n: u64, mut a: u64) -> bool {
    if n < 3 {
        return n == 2;
    }
    if n & 1 == 0 {
        return false;
    }
    assert!(a >= 2, "invalid base");
    a %= n;
    if a <= 1 || a == n - 1 {
        return true;
    }
    let s = (n - 1).trailing_zeros();
    let m = Mont::new(n);
    let mut x = m.pow(m.to_mont(a), (n - 1) >> s);
    if x == m.one || x == n - m.one {
        return true;
    }
    for _ in 1..s {
        x = m.sqr(x);
        if x == n - m.one {
            return true;
        }
        if x == m.one {
            return false;
        }
    }
    false
}

/// Tests the Euler–Jacobi congruence.
///
/// Port of MPU `primality.c`, `is_euler_pseudoprime`.
///
/// # Panics
/// Panics if the base is below 2 after the small-input checks.
///
/// ```
/// use math_prime_util::is_euler_pseudoprime;
/// assert!(is_euler_pseudoprime(561, 2));
/// ```
pub fn is_euler_pseudoprime(n: u64, mut a: u64) -> bool {
    if n < 3 {
        return n == 2;
    }
    if n & 1 == 0 {
        return false;
    }
    assert!(a >= 2, "invalid base");
    if a > 2 {
        if a >= n {
            a %= n;
            if a <= 1 {
                return a == 1;
            }
            if a == n - 1 {
                return a & 1 == 0;
            }
        }
        if n % a == 0 {
            return false;
        }
    }
    let m = Mont::new(n);
    let x = m.pow(m.to_mont(a), (n - 1) / 2);
    x == if jacobi(a, n) >= 0 { m.one } else { n - m.one }
}

/// Tests Colin Plumb's extended Euler criterion.
///
/// Port of MPU `primality.c`, `is_euler_plumb_pseudoprime`.
///
/// ```
/// use math_prime_util::is_euler_plumb_pseudoprime;
/// assert!(is_euler_plumb_pseudoprime(17));
/// ```
pub fn is_euler_plumb_pseudoprime(n: u64) -> bool {
    if n < 5 {
        return n == 2 || n == 3;
    }
    if n & 1 == 0 {
        return false;
    }
    let r = n % 8;
    let m = Mont::new(n);
    let x = m.pow(m.to_mont(2), (n - 1) >> (1 + u32::from(r == 1)));
    (x == m.one && matches!(r, 1 | 7)) || (x == n - m.one && matches!(r, 1 | 3 | 5))
}

fn lucas_test(n: u64, strength: u8) -> bool {
    if n < 5 {
        return n == 2 || n == 3;
    }
    if n & 1 == 0 || n == u64::MAX {
        return false;
    }
    let (p, q) = if strength == 3 {
        let Some(p) = extra_parameters(n, 1) else {
            return false;
        };
        (p, 1)
    } else {
        let mut d = 5_i64;
        loop {
            let j = signed_jacobi(d, n);
            if j != 1 && d.unsigned_abs() != n {
                if j != -1 {
                    return false;
                }
                break;
            }
            if d.unsigned_abs() == 21 && is_square(n) {
                return false;
            }
            d = if d > 0 { -d - 2 } else { -d + 2 };
            assert!(d.unsigned_abs() <= 1_000_000, "Lucas D exceeded 1e6");
        }
        let q = (1 - d) / 4;
        if gcd(q.unsigned_abs(), n) != 1 {
            return false;
        }
        (1, q)
    };
    let s = if strength == 0 {
        0
    } else {
        (n + 1).trailing_zeros()
    };
    let (u, mut v, mut qk) = lucas_sequence(n, p, q, (n + 1) >> s);
    if strength == 0 {
        return u == 0;
    }
    if strength == 1 && u == 0 {
        return true;
    }
    if strength == 3 && u == 0 && (v == 2 || v == n - 2) {
        return true;
    }
    let rounds = if strength == 3 { s - 1 } else { s };
    for r in 0..rounds {
        if v == 0 {
            return true;
        }
        if r + 1 < rounds {
            v = sub(mulmod_u(v, v, n), addmod_u(qk, qk, n), n);
            qk = mulmod_u(qk, qk, n);
        }
    }
    false
}

/// Tests the selected Lucas congruence. Uses C `is_lucas_pseudoprime`.
///
/// Port of MPU `primality.c`, `is_lucas_pseudoprime`.
///
/// # Panics
/// Panics if the parameter search exceeds the MPU bound.
///
/// ```
/// use math_prime_util::is_lucas_pseudoprime;
/// assert!(is_lucas_pseudoprime(101));
/// ```
pub fn is_lucas_pseudoprime(n: u64) -> bool {
    lucas_test(n, 0)
}

/// Tests the selected Lucas congruence. Uses C `is_lucas_pseudoprime`.
///
/// Port of MPU `primality.c`, `is_strong_lucas_pseudoprime`.
///
/// # Panics
/// Panics if the parameter search exceeds the MPU bound.
///
/// ```
/// use math_prime_util::is_strong_lucas_pseudoprime;
/// assert!(is_strong_lucas_pseudoprime(101));
/// ```
pub fn is_strong_lucas_pseudoprime(n: u64) -> bool {
    lucas_test(n, 1)
}

/// Tests the selected Lucas congruence. Uses C `is_lucas_pseudoprime`.
///
/// Port of MPU `primality.c`, `is_extra_strong_lucas_pseudoprime`.
///
/// # Panics
/// Panics if the parameter search exceeds the MPU bound.
///
/// ```
/// use math_prime_util::is_extra_strong_lucas_pseudoprime;
/// assert!(is_extra_strong_lucas_pseudoprime(101));
/// ```
pub fn is_extra_strong_lucas_pseudoprime(n: u64) -> bool {
    lucas_test(n, 3)
}

/// Tests the almost extra strong Lucas congruence.
///
/// Port of MPU `primality.c`, `is_almost_extra_strong_lucas_pseudoprime`.
///
/// # Panics
/// Panics if increment is outside 1..=256 or the parameter search exceeds `65_535`.
///
/// ```
/// use math_prime_util::is_almost_extra_strong_lucas_pseudoprime;
/// assert!(is_almost_extra_strong_lucas_pseudoprime(101, 1));
/// ```
pub fn is_almost_extra_strong_lucas_pseudoprime(n: u64, increment: u64) -> bool {
    assert!((1..=256).contains(&increment), "invalid increment");
    if n < 13 {
        return matches!(n, 2 | 3 | 5 | 7 | 11);
    }
    if n & 1 == 0 || n == u64::MAX {
        return false;
    }
    if ((increment >= 16 && n <= 331) || (increment > 148 && n <= 631)) && is_prob_prime(n) != 0 {
        return true;
    }
    let Some(p) = extra_parameters(n, increment) else {
        return false;
    };
    let s = (n + 1).trailing_zeros();
    let m = Mont::new(n);
    let mp = m.to_mont(p.unsigned_abs());
    let two = m.to_mont(2);
    let d = (n + 1) >> s;
    let mut v = mp;
    let mut w = sub(m.sqr(mp), two, n);
    for bit in (0..d.ilog2()).rev() {
        let t = sub(m.mul(v, w), mp, n);
        if d >> bit & 1 == 1 {
            v = t;
            w = sub(m.sqr(w), two, n);
        } else {
            w = t;
            v = sub(m.sqr(v), two, n);
        }
    }
    if v == two || v == n - two {
        return true;
    }
    for r in 0..s - 1 {
        if v == 0 {
            return true;
        }
        if r + 1 < s - 1 {
            v = sub(m.sqr(v), two, n);
        }
    }
    false
}

/// Tests BPSW. Port of C `BPSW`. Returns 2 for a native prime, else 0.
///
/// Port of MPU `primality.c`, `is_bpsw_prime`.
///
/// ```
/// use math_prime_util::is_bpsw_prime;
/// assert_eq!(is_bpsw_prime(101), 2);
/// ```
pub fn is_bpsw_prime(n: u64) -> u8 {
    u8::from(
        n != u64::MAX
            && is_strong_pseudoprime(n, 2)
            && is_almost_extra_strong_lucas_pseudoprime(n, 1),
    ) * 2
}

const MR_BASES: [u64; 256] = [
    4816, 6332, 958, 6124, 1001, 1431, 9644, 3700, 9251, 20_069, 7085, 2484, 3255, 218, 4660, 732,
    1863, 3716, 2480, 2120, 2464, 38_264, 3070, 2621, 1592, 17_862, 15_223, 2926, 9119, 2181,
    24_932, 4407, 10_915, 13_832, 1965, 3646, 2470, 62, 8548, 449, 4440, 7656, 1065, 10_100, 6497,
    1868, 33_282, 4277, 805, 636, 11_536, 34, 2065, 406, 6435, 1043, 27_985, 7134, 1357, 3056,
    6077, 4704, 6174, 865, 15_190, 14_419, 38, 6161, 18_774, 3990, 976, 1267, 3251, 233, 7387, 241,
    3871, 4331, 8780, 2233, 30_331, 1656, 462, 5585, 194, 10_300, 1072, 1197, 1573, 1144, 1273,
    19_439, 696, 1477, 15_858, 2684, 6022, 80, 9726, 6731, 1132, 774, 2202, 3668, 19_479, 10_837,
    183, 71, 403, 5245, 1995, 2019, 5209, 174, 503, 13_830, 21_013, 3284, 7164, 2607, 10_769, 473,
    119, 8227, 1216, 3550, 1450, 1399, 45_822, 609, 721, 47, 9665, 4242, 767, 4880, 16_037, 844,
    333, 8560, 1907, 2532, 13_468, 302, 2589, 5546, 14_312, 1548, 18_013, 8452, 12_427, 4431,
    10_248, 4022, 5545, 1399, 41_507, 1160, 1865, 219, 1254, 3330, 13_627, 1070, 3304, 5537, 6085,
    26_999, 10_279, 5369, 4992, 38_919, 2191, 1663, 46_961, 6570, 11_876, 21_689, 2804, 1202, 5764,
    275, 2862, 2139, 7799, 4646, 1696, 4964, 19_016, 12_891, 4282, 4741, 7274, 174, 541, 26_596,
    7524, 2777, 1819, 339, 1399, 2636, 668, 291, 559, 4992, 520, 7874, 2544, 4618, 4122, 1128, 326,
    275, 13_080, 156, 236, 7015, 6349, 11_673, 2632, 475, 4560, 1543, 78, 3611, 34, 3811, 137, 737,
    31_269, 2522, 13_354, 2033, 8577, 3597, 9269, 3815, 2511, 8088, 903, 109, 12_454, 1985, 8065,
    17_637, 1645, 1404, 6106, 3661, 328, 6160, 1602, 6601, 1491, 8657,
];

/// Returns 2 for a native prime, else 0.
///
/// Port of MPU `primality.c`, `is_prob_prime`.
///
/// ```
/// use math_prime_util::is_prob_prime;
/// assert_eq!(is_prob_prime(101), 2);
/// ```
pub fn is_prob_prime(n: u64) -> u8 {
    if n < 11 {
        return if matches!(n, 2 | 3 | 5 | 7) { 2 } else { 0 };
    }
    for p in [2, 3, 5, 7] {
        if n % p == 0 {
            return 0;
        }
    }
    if n < 121 {
        return 2;
    }
    for p in [11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53] {
        if n % p == 0 {
            return 0;
        }
    }
    if n < 3481 {
        return 2;
    }
    if u32::try_from(n).is_ok() {
        return 2 * u8::from(is_strong_pseudoprime(
            n,
            MR_BASES[((n >> 8) & 255) as usize],
        ));
    }
    for p in [59, 61, 67, 71, 73, 79, 83, 89] {
        if n % p == 0 {
            return 0;
        }
    }
    is_bpsw_prime(n)
}

/// Tests whether n and (n-1)/2 are prime.
///
/// Port of MPU `primality.c`, `is_safe_prime`.
///
/// ```
/// use math_prime_util::is_safe_prime;
/// assert!(is_safe_prime(23));
/// ```
pub fn is_safe_prime(n: u64) -> bool {
    if n < 23 {
        return matches!(n, 5 | 7 | 11);
    }
    if n % 6 != 5 {
        return false;
    }
    if n % 5 == 0 || n % 7 == 0 || n % 10 == 1 || n % 14 == 1 {
        return false;
    }
    is_prob_prime(n >> 1) != 0 && is_prob_prime(n) != 0
}

const MERSENNE: [u64; 52] = [
    2,
    3,
    5,
    7,
    13,
    17,
    19,
    31,
    61,
    89,
    107,
    127,
    521,
    607,
    1279,
    2203,
    2281,
    3217,
    4253,
    4423,
    9689,
    9941,
    11_213,
    19_937,
    21_701,
    23_209,
    44_497,
    86_243,
    110_503,
    132_049,
    216_091,
    756_839,
    859_433,
    1_257_787,
    1_398_269,
    2_976_221,
    3_021_377,
    6_972_593,
    13_466_917,
    20_996_011,
    24_036_583,
    25_964_951,
    30_402_457,
    32_582_657,
    37_156_667,
    42_643_801,
    43_112_609,
    57_885_161,
    74_207_281,
    77_232_917,
    82_589_933,
    136_279_841,
];

/// Tests a Mersenne exponent against the MPU 0.75 table. Returns None if unknown.
///
/// Port of MPU `primality.c`, `is_mersenne_prime`.
///
/// ```
/// use math_prime_util::is_mersenne_prime;
/// assert_eq!(is_mersenne_prime(61), Some(true));
/// ```
pub fn is_mersenne_prime(p: u64) -> Option<bool> {
    if MERSENNE.contains(&p) {
        Some(true)
    } else if p < 81_307_409 {
        Some(false)
    } else {
        None
    }
}

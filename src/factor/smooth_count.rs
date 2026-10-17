use crate::{logint, primes};

const CACHE_7: [u8; 128] = [
    8, 9, 10, 10, 11, 11, 12, 13, 14, 14, 15, 15, 16, 17, 17, 17, 18, 19, 19, 20, 21, 21, 22, 22,
    23, 23, 23, 24, 25, 25, 25, 25, 26, 26, 27, 27, 27, 28, 28, 28, 29, 30, 31, 31, 31, 31, 32, 32,
    33, 33, 33, 33, 34, 34, 34, 35, 36, 36, 36, 36, 36, 36, 37, 37, 38, 38, 38, 39, 39, 39, 39, 39,
    40, 41, 41, 41, 42, 42, 42, 42, 42, 42, 43, 43, 43, 43, 43, 43, 44, 44, 45, 45, 46, 46, 46, 46,
    46, 47, 47, 47, 48, 48, 48, 48, 49, 49, 49, 49, 49, 49, 49, 49, 50, 50, 50, 50, 50, 51, 52, 52,
    53, 53, 53, 53, 53, 53, 53, 54,
];
const CACHE_11: [u8; 96] = [
    12, 12, 13, 14, 15, 15, 16, 16, 17, 18, 19, 19, 20, 21, 21, 22, 23, 23, 24, 24, 25, 26, 26, 27,
    28, 28, 28, 28, 29, 29, 30, 30, 31, 32, 32, 32, 33, 34, 35, 35, 35, 35, 36, 37, 38, 38, 38, 38,
    39, 39, 39, 40, 41, 41, 42, 42, 42, 42, 43, 43, 44, 44, 44, 45, 45, 46, 46, 46, 47, 48, 48, 48,
    49, 49, 49, 49, 50, 50, 51, 51, 51, 51, 51, 51, 52, 52, 53, 54, 55, 55, 55, 55, 55, 56, 56, 56,
];
const CACHE_13: [u8; 64] = [
    14, 15, 16, 16, 17, 17, 18, 19, 20, 20, 21, 22, 23, 24, 25, 25, 26, 26, 27, 28, 28, 29, 30, 30,
    30, 31, 32, 32, 33, 33, 34, 35, 35, 35, 36, 37, 38, 38, 39, 39, 40, 41, 42, 42, 42, 42, 43, 43,
    43, 44, 45, 46, 47, 47, 47, 47, 48, 48, 49, 49, 49, 50, 50, 51,
];
const XT: [u8; 48] = [
    19, 23, 29, 31, 37, 38, 41, 43, 46, 47, 53, 57, 58, 59, 61, 62, 67, 69, 71, 73, 74, 76, 79, 82,
    83, 86, 87, 89, 92, 93, 94, 95, 97, 101, 103, 106, 107, 109, 111, 113, 114, 115, 116, 118, 122,
    123, 124, 127,
];
const YT: [u8; 48] = [
    19, 23, 29, 31, 37, 19, 41, 43, 23, 47, 53, 19, 29, 59, 61, 31, 67, 23, 71, 73, 37, 19, 79, 41,
    83, 43, 29, 89, 23, 31, 47, 19, 97, 101, 103, 53, 107, 109, 37, 113, 19, 23, 29, 59, 61, 41,
    31, 127,
];

/// Count the positive integers at most `n` with prime factors at most `k`.
///
/// Port of `reference/mpu/util.c`, `debruijn_psi`, including its cache tables.
/// MPU exposes this function as `smooth_count` in `reference/mpu/XS.xs`.
///
/// ```
/// assert_eq!(math_prime_util::smooth_count(100, 17), 67);
/// assert_eq!(math_prime_util::smooth_count(0, 17), 0);
/// ```
pub fn smooth_count(n: u64, k: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    if k <= 1 {
        return 1;
    }
    if k >= n {
        return n;
    }
    if k == 2 {
        return 1 + u64::from(logint(n, 2));
    }
    let k = k - u64::from(k & 1 == 0);
    let cache: &[u8] = match k {
        7 => &CACHE_7,
        11 => &CACHE_11,
        13 => &CACHE_13,
        _ => &[],
    };
    if let Ok(index) = usize::try_from(n - k - 1) {
        if let Some(&count) = cache.get(index) {
            return u64::from(count);
        }
    }
    if k >= 17 && n <= 128 {
        return n - u64::try_from(
            XT.iter()
                .zip(YT.iter())
                .filter(|&(&x, &y)| u64::from(x) <= n && u64::from(y) > k)
                .count(),
        )
        .unwrap();
    }
    let mut sum = 1 + u64::from(logint(n, 2));
    let mut x3 = n / 3;
    while x3 > 3 {
        sum += 1 + u64::from(logint(x3, 2));
        x3 /= 3;
    }
    sum += x3;
    if k >= 5 {
        let mut x5 = n / 5;
        while x5 > 5 {
            sum += 1 + u64::from(logint(x5, 2));
            let mut x3 = x5 / 3;
            while x3 > 3 {
                sum += 1 + u64::from(logint(x3, 2));
                x3 /= 3;
            }
            sum += x3;
            x5 /= 5;
        }
        sum += x5;
    }
    for p in [7, 11, 13, 17, 19, 23] {
        if k >= p {
            sum += smooth_count(n / p, p);
        }
    }
    if k >= 29 {
        for p in primes(29, k) {
            let quotient = n / p;
            sum += if p >= quotient {
                quotient
            } else {
                smooth_count(quotient, p)
            };
        }
    }
    sum
}

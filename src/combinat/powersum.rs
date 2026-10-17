// MPU 0.75 util.c: _max_ps_n and _max_ps_calc, 64-bit bounds.
const MAX_N: [u64; 64] = [
    0,
    6_074_000_999,
    3_810_777,
    92_681,
    9839,
    2190,
    745,
    331,
    175,
    105,
    69,
    49,
    36,
    28,
    22,
    18,
    15,
    13,
    11,
    10,
    9,
    8,
    7,
    6,
    6,
    5,
    5,
    5,
    4,
    4,
    4,
    4,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
];
const MAX_CALC: [u64; 9] = [0, 0, 2_642_245, 0, 5724, 1824, 482, 288, 115];

/// Returns `1^k + 2^k + ... + n^k`.
///
/// Returns `None` if the sum exceeds `u64`. Uses the bounds, formulas,
/// Stirling expansion, and direct sum in MPU 0.75 `util.c`, `powersum`.
///
/// ```
/// assert_eq!(math_prime_util::powersum(10, 2), Some(385));
/// ```
pub fn powersum(n: u64, k: u64) -> Option<u64> {
    if n <= 1 || k == 0 {
        return Some(n);
    }
    let exponent = usize::try_from(k).ok()?;
    if k >= 64 || n > MAX_N[exponent] {
        return None;
    }
    if n == 2 {
        return Some(1 + (1_u64 << k));
    }
    let n128 = u128::from(n);
    let a = u128::from(n.div_ceil(2)) * u128::from(n | 1);
    let a2 = a * a;
    let result = match k {
        1 => a,
        3 => a2,
        2 | 4..=8 if n <= MAX_CALC[exponent] => {
            let n = n128;
            match k {
                2 => a * (2 * n + 1) / 3,
                4 => a * (2 * n + 1) * (3 * n * (n + 1) - 1) / 15,
                5 => a2 * (4 * a - 1) / 3,
                6 => a * (2 * n + 1) * (n * (n * (n * (3 * n + 6)) - 3) + 1) / 21,
                7 => a2 * (6 * a2 - 4 * a + 1) / 3,
                _ => {
                    a * (2 * n + 1)
                        * (n * (n * (n * (n * (n * (5 * n + 15) + 5) - 15) - 1) + 9) - 3)
                        / 45
                }
            }
        }
        _ if k <= 8 && k < n => {
            let mut sum = 0;
            let mut fac = 1;
            for r in 1..=k {
                sum += fac * stirling2(k, r) * binomial(n128 + 1, r + 1);
                fac *= u128::from(r + 1);
            }
            sum
        }
        _ => {
            let k = u32::try_from(k).ok()?;
            (3..=n).fold(1 + (1_u128 << k), |sum, i| sum + u128::from(i).pow(k))
        }
    };
    u64::try_from(result).ok()
}

// util.c binomial and stirling2, with wider intermediates for k <= 8.
fn binomial(n: u128, k: u64) -> u128 {
    (1..=k).fold(1, |value, d| {
        value * (n + 1 - u128::from(d)) / u128::from(d)
    })
}

fn stirling2(n: u64, m: u64) -> u128 {
    let mut positive = 0;
    let mut negative = 0;
    let mut factorial = 1;
    for j in 1..=m {
        let term = binomial(u128::from(m), j)
            * u128::from(j).pow(u32::try_from(n).expect("exponent at most eight"));
        if (m - j) & 1 == 0 {
            positive += term;
        } else {
            negative += term;
        }
        factorial *= u128::from(j);
    }
    (positive - negative) / factorial
}

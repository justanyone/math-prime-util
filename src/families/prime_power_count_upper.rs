/// Bound the prime powers less than or equal to `n`.
///
/// Port of MPU 0.75 `prime_powers.c`, `prime_power_count_upper`.
/// Sum the prime counts at each integer root, including exponent one.
///
/// ```
/// assert_eq!(math_prime_util::prime_power_count_upper(100), 35);
/// ```
#[must_use]
pub fn prime_power_count_upper(n: u64) -> u64 {
    if n <= 5 {
        return n.saturating_sub(1);
    }
    let mut sum = count(n);
    for k in 2..=n.ilog2() {
        sum += count(crate::rootint(n, k));
    }
    sum
}

// MPU prime_counts.c, _upper_thresh and prime_count_upper. The C table uses float.
const THRESHOLDS: [(u64, f32); 21] = [
    (59_000, 2.48),
    (355_991, 2.54),
    (3_550_000, 2.51),
    (3_560_000, 2.49),
    (5_000_000, 2.48),
    (8_000_000, 2.47),
    (13_000_000, 2.46),
    (18_000_000, 2.45),
    (31_000_000, 2.44),
    (41_000_000, 2.43),
    (48_000_000, 2.42),
    (119_000_000, 2.41),
    (182_000_000, 2.40),
    (192_000_000, 2.395),
    (213_000_000, 2.390),
    (271_000_000, 2.385),
    (322_000_000, 2.380),
    (400_000_000, 2.375),
    (510_000_000, 2.370),
    (682_000_000, 2.367),
    (2_953_652_287, 2.362),
];

#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
fn count(n: u64) -> u64 {
    if n < 33_000 {
        return crate::prime_count(n);
    }
    if n > 10_000_000_000_000_000 {
        return super::prime_power_count_approx::li(n).integer(false);
    }
    let x = n as f64;
    let l = x.ln();
    let upper = if n <= 821_800_000 {
        let a = THRESHOLDS
            .iter()
            .find(|&&(threshold, _)| n < threshold)
            .map_or(2.334, |&(_, a)| f64::from(a));
        x / l * (1.0 + 1.0 / l + a / (l * l))
    } else if n < 101_260_000_000 {
        let a = if n < 1_100_000_000 {
            0.032
        } else if n < 10_010_000_000 {
            0.027
        } else {
            0.021
        };
        crate::logarithmic_integral(x) - a * l * x.sqrt() / (8.0 * std::f64::consts::PI)
    } else {
        crate::logarithmic_integral(x)
    };
    upper.floor() as u64
}

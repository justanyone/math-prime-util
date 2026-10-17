/// Bound the prime powers less than or equal to `n`.
///
/// Port of MPU 0.75 `prime_powers.c`, `prime_power_count_lower`.
/// Sum the prime counts at each integer root, including exponent one.
///
/// ```
/// assert_eq!(math_prime_util::prime_power_count_lower(100), 35);
/// ```
#[must_use]
pub fn prime_power_count_lower(n: u64) -> u64 {
    if n <= 5 {
        return n.saturating_sub(1);
    }
    let mut sum = count(n);
    for k in 2..=n.ilog2() {
        sum += count(crate::rootint(n, k));
    }
    sum
}

// MPU prime_counts.c, prime_count_lower, including the empirical thresholds.
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
fn count(n: u64) -> u64 {
    if n < 33_000 {
        return crate::prime_count(n);
    }
    let x = n as f64;
    let l = x.ln();
    let l2 = l * l;
    let lower = if n <= 300_070 {
        let a = match n {
            0..69_720 => 905.0,
            69_720..70_120 => 961.0,
            70_120..88_800 => 918.2,
            88_800..176_000 => 887.7,
            176_000..299_270 => 839.46,
            _ => 846.66,
        };
        x / (l - 1.0 - 1.0 / l - 2.975_666 / l2 - 13.024_334 / (l * l2) + a / (l2 * l2))
    } else if n < 4_000_000_000 {
        let a = match n {
            0..88_783 => 4.0,
            88_783..300_000 => -3.0,
            300_000..303_000 => 5.0,
            303_000..1_100_000 => -7.0,
            1_100_000..4_500_000 => -37.0,
            4_500_000..10_200_000 => -70.0,
            10_200_000..36_900_000 => -53.0,
            36_900_000..38_100_000 => -29.0,
            _ => -84.0,
        };
        crate::logarithmic_integral(x) - x.sqrt() / l * (1.94 + 2.50 / l + a / l2)
    } else {
        crate::logarithmic_integral(x) - 2.0 * x.sqrt() / l
    };
    lower.ceil() as u64
}

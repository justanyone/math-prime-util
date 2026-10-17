/// Estimate the prime powers less than or equal to `n`.
///
/// Port of MPU 0.75 `prime_powers.c`, `prime_power_count_approx`.
/// Sum the prime counts at each integer root, including exponent one.
///
/// ```
/// assert_eq!(math_prime_util::prime_power_count_approx(100), 35);
/// ```
#[must_use]
pub fn prime_power_count_approx(n: u64) -> u64 {
    if n <= 5 {
        return n.saturating_sub(1);
    }
    let mut sum = count(n);
    for k in 2..=n.ilog2() {
        sum += count(crate::rootint(n, k));
    }
    sum
}

// MPU prime_counts.c, prime_count_approx. Keep its 1e-6 stopping tolerance.
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn count(n: u64) -> u64 {
    if n < 3_000_000 {
        return crate::prime_count(n);
    }
    let x = n as f64;
    let mut sum = 1.0;
    let mut correction = 0.0;
    let mut part = 1.0;
    if x > 1e19 {
        return crate::riemann_r(x).round() as u64;
    }
    for k in 1..=10_000 {
        let k = f64::from(k);
        part *= x.ln() / k;
        let term = part / (k + k * crate::riemann_zeta(k + 1.0));
        let old = sum + correction;
        let next = sum + term;
        correction += if sum.abs() >= term.abs() {
            (sum - next) + term
        } else {
            (term - next) + sum
        };
        sum = next;
        if (sum + correction - old).abs() <= 1e-6 {
            break;
        }
    }
    (sum + correction + 0.5) as u64
}

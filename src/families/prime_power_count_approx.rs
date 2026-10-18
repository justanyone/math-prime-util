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
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
fn count(n: u64) -> u64 {
    if n < 3_000_000 {
        return crate::prime_count(n);
    }
    let x = n as f64;
    let mut sum = 1.0;
    let mut correction = 0.0;
    let mut part = 1.0;
    if x > 1e19 {
        let mut value = li(n);
        for k in 2..=100_u32 {
            let mut rest = k;
            let mut mu = 1_i32;
            for p in 2..=k {
                if rest % p != 0 {
                    continue;
                }
                rest /= p;
                mu = -mu;
                if rest % p == 0 {
                    mu = 0;
                    break;
                }
            }
            if mu == 0 {
                continue;
            }
            let term = f64::from(mu) / f64::from(k)
                * crate::logarithmic_integral(x.powf(1.0 / f64::from(k)));
            value = value.add(Wide(term, 0.0));
            if term.abs() <= 1e-6 {
                break;
            }
        }
        return value.integer(true);
    }
    if n > (1_u64 << 53) {
        let log = log_n(n);
        let mut value = Wide(1.0, 0.0);
        let mut part = Wide(1.0, 0.0);
        for k in 1..=10_000 {
            let k = f64::from(k);
            part = part.mul(log).div(Wide(k, 0.0));
            let denominator =
                Wide(k, 0.0).add(Wide(k, 0.0).mul(Wide(crate::riemann_zeta(k + 1.0), 0.0)));
            let term = part.div(denominator);
            value = value.add(term);
            if term.0.abs() <= 1e-6 {
                break;
            }
        }
        return value.integer(true);
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

// Two components retain the precision of C long double on every target.
#[derive(Clone, Copy)]
pub(super) struct Wide(pub(super) f64, pub(super) f64);

impl Wide {
    pub(super) fn add(self, rhs: Self) -> Self {
        let s = self.0 + rhs.0;
        let v = s - self.0;
        let e = (self.0 - (s - v)) + (rhs.0 - v) + self.1 + rhs.1;
        Self(s + e, e - ((s + e) - s))
    }
    fn mul(self, rhs: Self) -> Self {
        let p = self.0 * rhs.0;
        let e = self.0.mul_add(rhs.0, -p) + self.0 * rhs.1 + self.1 * rhs.0;
        Self(p + e, e - ((p + e) - p))
    }
    fn div(self, rhs: Self) -> Self {
        let q = self.0 / rhs.0;
        let r = self.add(rhs.mul(Self(-q, 0.0)));
        Self(q, 0.0).add(Self((r.0 + r.1) / rhs.0, 0.0))
    }
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_precision_loss
    )]
    pub(super) fn integer(self, nearest: bool) -> u64 {
        let base = self.0 as u64;
        let residual = self.0.fract() + self.1 + if nearest { 0.5 } else { 0.0 };
        if residual >= 0.0 {
            base + residual.floor() as u64
        } else {
            base - (-residual).ceil() as u64
        }
    }
}

// MPU real.c, _ei_series_convergent and _Li for the large native inputs.
// Small inputs use the existing real module; large values retain two components.
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
fn log_n(n: u64) -> Wide {
    let high = n as f64;
    let low = (i128::from(n) - high as i128) as f64;
    let e = n.ilog2();
    let scale = 2.0_f64.powi(i32::try_from(e).expect("u64 exponent fits i32"));
    let m = Wide(high / scale, low / scale);
    let z = m.add(Wide(-1.0, 0.0)).div(m.add(Wide(1.0, 0.0)));
    let z2 = z.mul(z);
    let mut term = z;
    let mut log = z;
    for k in 1..=60 {
        term = term.mul(z2);
        log = log.add(term.div(Wide(f64::from(2 * k + 1), 0.0)));
    }
    log = log.mul(Wide(2.0, 0.0)).add(
        Wide(std::f64::consts::LN_2, 2.319_046_813_846_299_6e-17).mul(Wide(f64::from(e), 0.0)),
    );
    log
}

pub(super) fn li(n: u64) -> Wide {
    let log = log_n(n);
    let mut factorial_term = Wide(1.0, 0.0);
    let mut sum = Wide(0.577_215_664_901_532_9 + log.0.ln(), 0.0);
    for k in 1..=300 {
        factorial_term = factorial_term.mul(log).div(Wide(f64::from(k), 0.0));
        let term = factorial_term.div(Wide(f64::from(k), 0.0));
        sum = sum.add(term);
        if term.0.abs() < 1e-16 {
            break;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::Wide;

    #[test]
    fn integer_rounding_retains_both_components() {
        assert_eq!(Wide(100.75, 0.0).integer(true), 101);
        assert_eq!(Wide(100.75, -0.5).integer(false), 100);
        assert_eq!(Wide(100.0, -0.25).integer(false), 99);
        assert_eq!(Wide(100.0, 0.25).integer(true), 100);
        assert_eq!(Wide(100.0, 0.5).integer(true), 101);
    }
}

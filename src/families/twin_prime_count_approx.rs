//! Sebah and Gourdon twin prime estimate.
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

/// Estimate the count of twin pairs up to `n`.
///
/// Port of `twin_primes.c` (MPU 0.75), `twin_prime_count_approx`.
/// Use the exact count below 2000 and the C correction intervals above it.
///
/// ```
/// assert_eq!(math_prime_util::twin_prime_count_approx(100), 8);
/// ```
#[must_use]
pub fn twin_prime_count_approx(n: u64) -> u64 {
    if n < 2000 {
        return super::twin_prime_count(n);
    }
    if n >= 1_000_000_000_000 {
        return wide_estimate(n);
    }
    let x = n as f64;
    let logn = x.ln();
    let mut li2 = crate::exponential_integral(logn) + 2.0 / 2.0_f64.ln() - x / logn;
    if n < 32_000_000 {
        let fm = match n {
            0..4000 => 0.2952,
            4000..8000 => 0.3102,
            8000..16000 => 0.3090,
            16000..32000 => 0.3096,
            32000..64000 => 0.3097,
            64000..128000 => 0.3094,
            128000..256000 => 0.3099,
            256000..600000 => 0.3098 + (x - 256_000.0) * (0.3056 - 0.3098) / 344_000.0,
            600000..1000000 => 0.3062 + (x - 600_000.0) * (0.3042 - 0.3062) / 400_000.0,
            1000000..4000000 => 0.3067 + (x - 1_000_000.0) * (0.3041 - 0.3067) / 3_000_000.0,
            4000000..16000000 => 0.3041 + (x - 4_000_000.0) * (0.2983 - 0.3041) / 12_000_000.0,
            _ => 0.2983 + (x - 16_000_000.0) * (0.2961 - 0.2983) / 16_000_000.0,
        };
        li2 *= fm * (12.0 + logn).ln();
    }
    (1.320_323_631_693_739 * li2 + 0.5) as u64
}

// C evaluates the estimate in long double, but Ei takes and returns NV
// (double). Keep that rounding boundary. Two doubles retain the precision
// of the surrounding long-double subtraction on every Rust target.
#[derive(Clone, Copy)]
struct Wide(f64, f64);

impl Wide {
    fn add(self, rhs: Self) -> Self {
        let sum = self.0 + rhs.0;
        let v = sum - self.0;
        let error = (self.0 - (sum - v)) + (rhs.0 - v) + self.1 + rhs.1;
        let hi = sum + error;
        Self(hi, error - (hi - sum))
    }

    fn neg(self) -> Self {
        Self(-self.0, -self.1)
    }

    fn mul(self, rhs: Self) -> Self {
        let product = self.0 * rhs.0;
        let error = self.0.mul_add(rhs.0, -product) + self.0 * rhs.1 + self.1 * rhs.0;
        Self(product, 0.0).add(Self(error, 0.0))
    }

    fn div(self, rhs: Self) -> Self {
        let q = self.0 / rhs.0;
        let rem = self.add(rhs.mul(Self(q, 0.0)).neg());
        Self(q, 0.0).add(Self((rem.0 + rem.1) / rhs.0, 0.0))
    }
}

fn wide_estimate(n: u64) -> u64 {
    let x = Wide((n >> 32) as f64 * 4_294_967_296.0, 0.0).add(Wide((n & 0xffff_ffff) as f64, 0.0));
    let exponent = 63 - n.leading_zeros();
    let mantissa = x.div(Wide(2.0_f64.powi(exponent as i32), 0.0));
    let z = mantissa
        .add(Wide(-1.0, 0.0))
        .div(mantissa.add(Wide(1.0, 0.0)));
    let z2 = z.mul(z);
    let mut term = z;
    let mut log = z;
    for k in 1..40 {
        term = term.mul(z2);
        log = log.add(term.div(Wide(f64::from(2 * k + 1), 0.0)));
    }
    log = log.mul(Wide(2.0, 0.0)).add(
        Wide(std::f64::consts::LN_2, 2.319_046_813_846_299_6e-17)
            .mul(Wide(f64::from(exponent), 0.0)),
    );
    // real.c::_ei_series_convergent, with the NV argument and result.
    let y = Wide(log.0, 0.0);
    let mut fact = y;
    let mut sum = y;
    for k in 2..=240 {
        let k = Wide(f64::from(k), 0.0);
        fact = fact.mul(y).div(k);
        sum = sum.add(fact.div(k));
    }
    let ei = sum.add(Wide(0.577_215_664_901_532_9 + y.0.ln(), 0.0)).0;
    let li2 = Wide(ei, 0.0)
        .add(Wide(2.885_390_081_777_926_8, 0.0))
        .add(x.div(log).neg());
    let estimate = li2.mul(Wide(
        1.320_323_631_693_739,
        1.396_140_737_833_491_3e-16,
    ));
    let integer = estimate.0 as u64;
    let adjustment = ((estimate.0 - integer as f64) + estimate.1 + 0.5).floor() as i64;
    integer.saturating_add_signed(adjustment)
}

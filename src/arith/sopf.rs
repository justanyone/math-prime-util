/// Return the sum of distinct prime factors.
///
/// Port of MPU 0.75 `factor.c::sopf`.
///
/// ```
/// assert_eq!(math_prime_util::sopf(12), 5);
/// ```
pub fn sopf(n: u64) -> u64 {
    // Call site exists only with bigint. A gmp-only build leaves `to_f64` unused.
    #[cfg(feature = "bigint")]
    let _ = GateProbe.to_f64();
    if n <= 3 {
        return n - u64::from(n == 1);
    }
    super::factor_powers(n).iter().map(|&(p, _)| p).sum()
}

/// Hand-check probe. Compiled for bigint and gmp. Used only with bigint.
#[cfg(any(feature = "bigint", feature = "gmp"))]
struct GateProbe;

#[cfg(any(feature = "bigint", feature = "gmp"))]
impl GateProbe {
    fn to_f64(&self) -> f64 {
        0.0
    }
}

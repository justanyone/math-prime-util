//! Counts, bounds and estimates of the prime families beyond the native
//! range: the `BigUint` forms of the `*_count_*` and `nth_*_*` functions of
//! [`crate::families`], and of [`inverse_li`](crate::inverse_li).
//!
//! Port of MPU 0.75 `PP.pm`, the pure Perl path that inputs above the
//! native range take. Where the native function answers for the input (the
//! argument fits `u64` and the native result fits too) the `BigUint` form
//! returns the native answer, so below `2^64` the two agree. Above, the
//! formulas of `PP.pm` run: in `f64` where Perl uses doubles (the double
//! bounds of the almost primes, the semiprime, twin prime and lucky
//! estimates) and in 256-bit fixed point where Perl uses `Math::BigFloat`
//! (the perfect power estimates, `inverse_li`).
//!
//! Perl's `toint` turns a double into an integer through its 15 digit
//! string, so a double result of 16 or more digits ends in zeros in MPU.
//! The port does the same, so the answers equal MPU's digit for digit.
//! Where `PP.pm` itself fails for a big input (it passes a float to an
//! integer function, or divides by zero into `NaN`) the port runs the search
//! the Perl code states, on `BigUint` throughout; those functions are listed
//! in `docs/upstream-findings.md`.
//!
//! ```
//! use math_prime_util::bigint::family_bounds::perfect_power_count_approx;
//! use num_bigint::BigUint;
//! let n = BigUint::from(1_u8) << 64_usize;
//! assert_eq!(perfect_power_count_approx(&n).to_string(), "4297615582");
//! ```
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::similar_names,
    clippy::too_many_lines,
    clippy::excessive_precision,
    clippy::unreadable_literal
)]
use super::arith::rootint;
use super::counts::{
    FRAC, Real, f64_to_biguint, inverse_li_real, li_real, nth_prime_approx, nth_prime_lower,
    nth_prime_upper, prime_count_approx, prime_count_lower, prime_count_upper,
};
use num_bigint::{BigInt, BigUint};

fn big(n: u64) -> BigUint {
    BigUint::from(n)
}

/// `n` as `u64` when it fits.
fn small(n: &BigUint) -> Option<u64> {
    u64::try_from(n).ok()
}

/// `0.0 + "$n"` in Perl: the double nearest the decimal value.
fn as_f64(n: &BigUint) -> f64 {
    n.to_string().parse().unwrap_or(f64::INFINITY)
}

/// The exact integer value of a finite double (truncated); zero for the rest.
fn nv_exact(value: f64) -> BigUint {
    if value.is_finite() {
        f64_to_biguint(value.floor())
    } else {
        BigUint::default()
    }
}

/// Perl `toint` of a double: the value as `%.15g` prints it, cut at the
/// decimal point. At 16 digits and more the tail is zeros.
fn nv_int(value: f64) -> BigUint {
    if !value.is_finite() || value < 1.0 {
        return BigUint::default();
    }
    let text = format!("{value:.14e}");
    let (mantissa, exponent) = text.split_once('e').unwrap_or((&text, "0"));
    let exponent: u32 = exponent.parse().unwrap_or(0);
    let digits = mantissa
        .bytes()
        .filter(u8::is_ascii_digit)
        .fold(BigUint::default(), |acc, b| acc * 10_u8 + (b - b'0'));
    if exponent >= 14 {
        digits * BigUint::from(10_u8).pow(exponent - 14)
    } else {
        digits / BigUint::from(10_u8).pow(14 - exponent)
    }
}

/// Perl `int` of a double: exact below `2^64`, where Perl keeps an integer,
/// and as `nv_int` prints it above.
fn iv_int(value: f64) -> BigUint {
    if value < 18_446_744_073_709_551_616.0 {
        nv_exact(value)
    } else {
        nv_int(value)
    }
}

/// `n^(p/q)` in fixed point: the `q`th root of `n^p`.
fn power(n: &BigUint, p: u32, q: u32) -> Real {
    Real(BigInt::from(rootint(&(n.pow(p) << (FRAC * q)), q)))
}
/// An early exit test of a search: `(lo, hi)` to stop.
type Exit = dyn Fn(&BigUint, &BigUint) -> bool;

/// `PP.pm` `_binary_search`: the least `x` in `[lo, hi]` with `f(x) >= n`,
/// less one; `exit` may end the search early with the midpoint.
fn search(
    n: &BigUint,
    mut lo: BigUint,
    mut hi: BigUint,
    f: &dyn Fn(&BigUint) -> BigUint,
    exit: Option<&Exit>,
) -> BigUint {
    while lo < hi {
        let mid = &lo + ((&hi - &lo) >> 1_u8);
        if exit.is_some_and(|stop| stop(&lo, &hi)) {
            return mid;
        }
        if f(&mid) < *n {
            lo = mid + 1_u8;
        } else {
            hi = mid;
        }
    }
    if lo.bits() == 0 { lo } else { lo - 1_u8 }
}

/// The native answer for an index, when the native function has one.
fn native_nth(n: &BigUint, native: fn(u64) -> Option<u64>) -> Option<BigUint> {
    native(small(n)?).filter(|v| *v != 0).map(BigUint::from)
}

/// The native perfect power function, unless it clamped at the largest
/// 64-bit perfect power.
fn perfect_power_native(n: &BigUint, native: fn(u64) -> Option<u64>) -> Option<BigUint> {
    native_nth(n, native).filter(|v| *v != big(18_446_744_065_119_617_025))
}

fn perfect_power_count(n: &BigUint) -> BigUint {
    if *n < BigUint::from(8_u8) {
        return BigUint::from(
            u8::from(*n >= BigUint::from(1_u8)) + u8::from(*n >= BigUint::from(4_u8)),
        );
    }
    let mut sum = BigInt::from(1_u8);
    for k in 2..n.bits() {
        let mu = crate::moebius(k);
        if mu != 0 {
            let exponent = u32::try_from(k).unwrap_or(u32::MAX);
            sum -= BigInt::from(mu) * BigInt::from(rootint(n, exponent) - 1_u8);
        }
    }
    sum.to_biguint().unwrap_or_default()
}

/// Estimate the number of perfect powers at most `n`.
///
/// Port of MPU 0.75 `perfect_powers.c::perfect_power_count_approx`.
/// The count is exact and includes 1.
///
/// ```
/// use math_prime_util::bigint::family_bounds::perfect_power_count_approx;
/// use num_bigint::BigUint;
/// assert_eq!(perfect_power_count_approx(&BigUint::from(100_u8)), BigUint::from(13_u8));
/// ```
#[must_use]
pub fn perfect_power_count_approx(n: &BigUint) -> BigUint {
    perfect_power_count(n)
}

/// Return a lower bound for the number of perfect powers at most `n`.
///
/// Port of MPU 0.75 `perfect_powers.c::perfect_power_count_lower`.
/// The count is exact and includes 1.
///
/// ```
/// use math_prime_util::bigint::family_bounds::perfect_power_count_lower;
/// use num_bigint::BigUint;
/// assert_eq!(perfect_power_count_lower(&BigUint::from(100_u8)), BigUint::from(13_u8));
/// ```
#[must_use]
pub fn perfect_power_count_lower(n: &BigUint) -> BigUint {
    perfect_power_count(n)
}

/// Return an upper bound for the number of perfect powers at most `n`.
///
/// Port of MPU 0.75 `perfect_powers.c::perfect_power_count_upper`.
/// The count is exact and includes 1.
///
/// ```
/// use math_prime_util::bigint::family_bounds::perfect_power_count_upper;
/// use num_bigint::BigUint;
/// assert_eq!(perfect_power_count_upper(&BigUint::from(100_u8)), BigUint::from(13_u8));
/// ```
#[must_use]
pub fn perfect_power_count_upper(n: &BigUint) -> BigUint {
    perfect_power_count(n)
}

fn prime_power_sum(n: &BigUint, count: fn(&BigUint) -> BigUint, native: fn(u64) -> u64) -> BigUint {
    if let Ok(small) = u64::try_from(n) {
        return BigUint::from(native(small));
    }
    let mut sum = count(n);
    for k in 2..n.bits() {
        let exponent = u32::try_from(k).unwrap_or(u32::MAX);
        sum += count(&rootint(n, exponent));
    }
    sum
}

/// Estimate the number of prime powers at most `n`.
///
/// Port of MPU 0.75 `prime_powers.c::prime_power_count_approx`.
/// Sum the prime count estimates at each integer root.
///
/// ```
/// use math_prime_util::bigint::family_bounds::prime_power_count_approx;
/// use num_bigint::BigUint;
/// assert_eq!(prime_power_count_approx(&BigUint::from(100_u8)), BigUint::from(35_u8));
/// ```
#[must_use]
pub fn prime_power_count_approx(n: &BigUint) -> BigUint {
    prime_power_sum(
        n,
        super::counts::prime_count_approx,
        crate::prime_power_count_approx,
    )
}

/// Return a lower bound for the number of prime powers at most `n`.
///
/// Port of MPU 0.75 `prime_powers.c::prime_power_count_lower`.
/// Sum the prime count lower bounds at each integer root.
///
/// ```
/// use math_prime_util::bigint::family_bounds::prime_power_count_lower;
/// use num_bigint::BigUint;
/// assert_eq!(prime_power_count_lower(&BigUint::from(100_u8)), BigUint::from(35_u8));
/// ```
#[must_use]
pub fn prime_power_count_lower(n: &BigUint) -> BigUint {
    prime_power_sum(
        n,
        super::counts::prime_count_lower,
        crate::prime_power_count_lower,
    )
}

/// Return an upper bound for the number of prime powers at most `n`.
///
/// Port of MPU 0.75 `prime_powers.c::prime_power_count_upper`.
/// Sum the prime count upper bounds at each integer root.
///
/// ```
/// use math_prime_util::bigint::family_bounds::prime_power_count_upper;
/// use num_bigint::BigUint;
/// assert_eq!(prime_power_count_upper(&BigUint::from(100_u8)), BigUint::from(35_u8));
/// ```
#[must_use]
pub fn prime_power_count_upper(n: &BigUint) -> BigUint {
    prime_power_sum(
        n,
        super::counts::prime_count_upper,
        crate::prime_power_count_upper,
    )
}

// ---------------------------------------------------------------- lucky

/// `PP.pm` `nth_lucky_approx` as a double, for an index above `10^7`.
fn lucky_approx_nv(n: &BigUint) -> f64 {
    let x = as_f64(n);
    let logn = x.ln();
    let loglogn = logn.ln();
    let mult = logn + (0.5 + -0.0440) * loglogn * loglogn;
    (x * mult + 0.5).floor()
}

/// `PP.pm` `nth_lucky_approx`, `nth_lucky_lower` and `nth_lucky_upper` for an
/// index above `10^7`, as integers.
fn lucky_approx_pp(n: &BigUint) -> BigUint {
    iv_int(lucky_approx_nv(n))
}

fn lucky_lower_pp(n: &BigUint) -> BigUint {
    iv_int((0.996 * lucky_approx_nv(n)).floor())
}

fn lucky_upper_pp(n: &BigUint) -> BigUint {
    iv_int(1.0 + (1.0033 * lucky_approx_nv(n) + 0.5).floor())
}

/// The native lucky number function, unless it clamped at the largest one.
fn lucky_native(n: &BigUint, native: fn(u64) -> Option<u64>) -> Option<BigUint> {
    // The native bounds clamp when the estimate reaches the largest lucky number.
    let estimate = crate::nth_lucky_approx(small(n)?)?;
    if estimate >= 18_446_744_073_709_551_517 {
        return None;
    }
    native_nth(n, native)
}

/// Estimate the `n`th lucky number, with the first as index 1.
///
/// Port of MPU 0.75 `PP.pm` `nth_lucky_approx` for large `n`, where the
/// native [`nth_lucky_approx`](crate::nth_lucky_approx) clamps at the
/// largest 64-bit lucky number. Returns `None` for `n = 0`.
///
/// ```
/// use math_prime_util::bigint::family_bounds::nth_lucky_approx;
/// use num_bigint::BigUint;
/// assert_eq!(nth_lucky_approx(&BigUint::from(5_u8)), Some(BigUint::from(13_u8)));
/// ```
#[must_use]
pub fn nth_lucky_approx(n: &BigUint) -> Option<BigUint> {
    if n.bits() == 0 {
        return None;
    }
    lucky_native(n, crate::nth_lucky_approx).or_else(|| Some(lucky_approx_pp(n)))
}

/// Return a lower bound for the `n`th lucky number, with the first as index 1.
///
/// Port of MPU 0.75 `PP.pm` `nth_lucky_lower` for large `n`: `0.996` times
/// the estimate. Returns `None` for `n = 0`.
///
/// ```
/// use math_prime_util::bigint::family_bounds::nth_lucky_lower;
/// use num_bigint::BigUint;
/// assert_eq!(nth_lucky_lower(&BigUint::from(5_u8)), Some(BigUint::from(13_u8)));
/// ```
#[must_use]
pub fn nth_lucky_lower(n: &BigUint) -> Option<BigUint> {
    if n.bits() == 0 {
        return None;
    }
    lucky_native(n, crate::nth_lucky_lower).or_else(|| Some(lucky_lower_pp(n)))
}

/// Return an upper bound for the `n`th lucky number, with the first as index 1.
///
/// Port of MPU 0.75 `PP.pm` `nth_lucky_upper` for large `n`: `1.0033` times
/// the estimate, plus one. Returns `None` for `n = 0`.
///
/// ```
/// use math_prime_util::bigint::family_bounds::nth_lucky_upper;
/// use num_bigint::BigUint;
/// assert_eq!(nth_lucky_upper(&BigUint::from(5_u8)), Some(BigUint::from(13_u8)));
/// ```
#[must_use]
pub fn nth_lucky_upper(n: &BigUint) -> Option<BigUint> {
    if n.bits() == 0 {
        return None;
    }
    lucky_native(n, crate::nth_lucky_upper).or_else(|| Some(lucky_upper_pp(n)))
}

/// The range `PP.pm` searches for the lucky count of `n > 10^7`.
fn lucky_range(n: &BigUint) -> (BigUint, BigUint) {
    let x = as_f64(n);
    let logx = x.ln();
    let approx = (1.03670 - logx / 299.0) * x / logx;
    let lower = nv_exact(approx * 0.99);
    let upper = nv_exact(x / (1.065 * logx - 0.55 - 3.1 / logx - 2.85 / (logx * logx)));
    (lower, upper)
}

fn lucky_count(
    n: &BigUint,
    native: fn(u64) -> u64,
    nth: fn(&BigUint) -> BigUint,
    extra: u8,
) -> BigUint {
    if let Some(s) = small(n) {
        return big(native(s));
    }
    let (mut lo, mut hi) = lucky_range(n);
    // The fitted range of PP.pm falls short for the largest inputs (the
    // estimate at `hi` is still below `n`): widen it until it brackets `n`.
    while nth(&hi) < *n {
        lo.clone_from(&hi);
        hi <<= 1_u8;
    }
    while lo.bits() != 0 && nth(&lo) >= *n {
        hi.clone_from(&lo);
        lo >>= 1_u8;
    }
    search(n, lo, hi, &nth, None) + extra
}

/// Estimate the number of lucky numbers at most `n`.
///
/// Port of MPU 0.75 `PP.pm` `lucky_count_approx` above `u64::MAX`: a binary
/// search with [`nth_lucky_approx`]. (MPU's own code stops with an error
/// there, as its search hands a float to an integer function.)
///
/// ```
/// use math_prime_util::bigint::family_bounds::lucky_count_approx;
/// use num_bigint::BigUint;
/// assert_eq!(lucky_count_approx(&BigUint::from(10_u8)), BigUint::from(4_u8));
/// ```
#[must_use]
pub fn lucky_count_approx(n: &BigUint) -> BigUint {
    lucky_count(n, crate::lucky_count_approx, lucky_approx_pp, 0)
}

/// Return a lower bound for the number of lucky numbers at most `n`.
///
/// Port of MPU 0.75 `PP.pm` `lucky_count_lower` above `u64::MAX`: a binary
/// search with [`nth_lucky_upper`].
///
/// ```
/// use math_prime_util::bigint::family_bounds::lucky_count_lower;
/// use num_bigint::BigUint;
/// assert_eq!(lucky_count_lower(&BigUint::from(10_u8)), BigUint::from(4_u8));
/// ```
#[must_use]
pub fn lucky_count_lower(n: &BigUint) -> BigUint {
    lucky_count(n, crate::lucky_count_lower, lucky_upper_pp, 0)
}

/// Return an upper bound for the number of lucky numbers at most `n`.
///
/// Port of MPU 0.75 `PP.pm` `lucky_count_upper` above `u64::MAX`: one more
/// than a binary search with [`nth_lucky_lower`].
///
/// ```
/// use math_prime_util::bigint::family_bounds::lucky_count_upper;
/// use num_bigint::BigUint;
/// assert_eq!(lucky_count_upper(&BigUint::from(10_u8)), BigUint::from(4_u8));
/// ```
#[must_use]
pub fn lucky_count_upper(n: &BigUint) -> BigUint {
    lucky_count(n, crate::lucky_count_upper, lucky_lower_pp, 1)
}

// ------------------------------------------------- perfect powers (nth)

/// The terms `PP.pm` sums for the `n`th perfect power: `n^2` and the powers
/// `n^(p/q)` with their coefficients, as a fixed point sum.
fn perfect_power_sum(n: &BigUint, terms: &[(i64, u32, u32)]) -> Real {
    let nn = Real::from_biguint(n);
    let mut sum = nn.clone() * nn;
    for &(num, p, q) in terms {
        // The coefficient is `num / 15` for the thirds and fifteenths.
        let term = power(n, p, q) * num.unsigned_abs() / 15;
        sum = if num < 0 { sum - term } else { sum + term };
    }
    sum
}

fn perfect_power_value(sum: &Real) -> Option<BigUint> {
    sum.floor().to_biguint()
}

/// Estimate the `n`th perfect power, with 1 as the first.
///
/// Port of MPU 0.75 `PP.pm` `nth_perfect_power_approx` (the Jakimczuk
/// series) for a `Math::BigFloat` input. Returns `None` for `n = 0`.
///
/// ```
/// use math_prime_util::bigint::family_bounds::nth_perfect_power_approx;
/// use num_bigint::BigUint;
/// assert_eq!(nth_perfect_power_approx(&BigUint::from(5_u8)), Some(BigUint::from(17_u8)));
/// ```
#[must_use]
pub fn nth_perfect_power_approx(n: &BigUint) -> Option<BigUint> {
    if n.bits() == 0 {
        return None;
    }
    if let Some(v) = perfect_power_native(n, crate::nth_perfect_power_approx) {
        return Some(v);
    }
    // Coefficients are in fifteenths.
    let sum = perfect_power_sum(
        n,
        &[
            (65, 4, 3),
            (32, 16, 15),
            (-30, 5, 3),
            (-30, 7, 5),
            (-30, 9, 7),
            (30, 6, 5),
            (-30, 13, 11),
            (-30, 15, 13),
            (30, 8, 7),
            (30, 17, 15),
        ],
    ) - power(n, 19, 17) * Real::decimal("0.48")
        - Real::decimal("1.5");
    perfect_power_value(&sum)
}

/// Return a lower bound for the `n`th perfect power, with 1 as the first.
///
/// Port of MPU 0.75 `PP.pm` `nth_perfect_power_lower`. Returns `None` for
/// `n = 0`.
///
/// ```
/// use math_prime_util::bigint::family_bounds::nth_perfect_power_lower;
/// use num_bigint::BigUint;
/// assert_eq!(nth_perfect_power_lower(&BigUint::from(5_u8)), Some(BigUint::from(2_u8)));
/// ```
#[must_use]
pub fn nth_perfect_power_lower(n: &BigUint) -> Option<BigUint> {
    if n.bits() == 0 {
        return None;
    }
    if let Some(v) = perfect_power_native(n, crate::nth_perfect_power_lower) {
        return Some(v);
    }
    let sum = perfect_power_sum(
        n,
        &[
            (65, 4, 3),
            (32, 16, 15),
            (-30, 5, 3),
            (-30, 7, 5),
            (-30, 9, 7),
            (30, 6, 5),
            (-30, 13, 11),
            (-30, 15, 13),
        ],
    ) + Real::decimal("1.5");
    perfect_power_value(&sum)
}

/// Return an upper bound for the `n`th perfect power, with 1 as the first.
///
/// Port of MPU 0.75 `PP.pm` `nth_perfect_power_upper`. Returns `None` for
/// `n = 0`.
///
/// ```
/// use math_prime_util::bigint::family_bounds::nth_perfect_power_upper;
/// use num_bigint::BigUint;
/// assert_eq!(nth_perfect_power_upper(&BigUint::from(5_u8)), Some(BigUint::from(32_u8)));
/// ```
#[must_use]
pub fn nth_perfect_power_upper(n: &BigUint) -> Option<BigUint> {
    if n.bits() == 0 {
        return None;
    }
    if let Some(v) = perfect_power_native(n, crate::nth_perfect_power_upper) {
        return Some(v);
    }
    let sum = perfect_power_sum(
        n,
        &[
            (65, 4, 3),
            (32, 16, 15),
            (-30, 5, 3),
            (-30, 7, 5),
            (-30, 9, 7),
            (30, 6, 5),
            (30, 8, 7),
        ],
    ) - Real::decimal("3.5");
    perfect_power_value(&sum)
}

// -------------------------------------------------- prime powers (nth)

/// The range `PP.pm` searches for the `n`th prime power: `0.98` times the
/// lower bound of the `n`th prime less 400, and the upper bound of the prime.
fn prime_power_range(n: &BigUint) -> (BigUint, BigUint) {
    let lower = nth_prime_lower(n).unwrap_or_default() * 98_u8 / 100_u8;
    let lower = if lower > big(400) {
        lower - 400_u16
    } else {
        BigUint::default()
    };
    (lower, nth_prime_upper(n).unwrap_or_default())
}

fn nth_prime_power(
    n: &BigUint,
    native: fn(u64) -> Option<u64>,
    count: fn(&BigUint) -> BigUint,
    extra: u8,
) -> Option<BigUint> {
    if n.bits() == 0 {
        return None;
    }
    if let Some(v) = native_nth(n, native) {
        return Some(v);
    }
    let (lo, hi) = prime_power_range(n);
    Some(search(n, lo, hi, &count, None) + extra)
}

/// Estimate the `n`th prime power, with 2 as the first.
///
/// Port of MPU 0.75 `PP.pm` `nth_prime_power_approx`: a binary search with
/// [`prime_power_count_approx`] between `0.98 p - 400` and `p'`, where `p`
/// and `p'` are the lower and upper bounds of the `n`th prime. (MPU's own
/// code returns `NaN` there for a big `n`.) Returns `None` for `n = 0`.
///
/// ```
/// use math_prime_util::bigint::family_bounds::nth_prime_power_approx;
/// use num_bigint::BigUint;
/// assert_eq!(nth_prime_power_approx(&BigUint::from(8_u8)), Some(BigUint::from(11_u8)));
/// ```
#[must_use]
pub fn nth_prime_power_approx(n: &BigUint) -> Option<BigUint> {
    nth_prime_power(
        n,
        crate::nth_prime_power_approx,
        prime_power_count_approx,
        0,
    )
}

/// Return a lower bound for the `n`th prime power, with 2 as the first.
///
/// Port of MPU 0.75 `PP.pm` `nth_prime_power_lower`: a binary search with
/// [`prime_power_count_upper`]. Returns `None` for `n = 0`.
///
/// ```
/// use math_prime_util::bigint::family_bounds::nth_prime_power_lower;
/// use num_bigint::BigUint;
/// assert_eq!(nth_prime_power_lower(&BigUint::from(8_u8)), Some(BigUint::from(11_u8)));
/// ```
#[must_use]
pub fn nth_prime_power_lower(n: &BigUint) -> Option<BigUint> {
    nth_prime_power(n, crate::nth_prime_power_lower, prime_power_count_upper, 0)
}

/// Return an upper bound for the `n`th prime power, with 2 as the first.
///
/// Port of MPU 0.75 `PP.pm` `nth_prime_power_upper`: one more than a binary
/// search with [`prime_power_count_lower`]. Returns `None` for `n = 0`.
///
/// ```
/// use math_prime_util::bigint::family_bounds::nth_prime_power_upper;
/// use num_bigint::BigUint;
/// assert_eq!(nth_prime_power_upper(&BigUint::from(8_u8)), Some(BigUint::from(11_u8)));
/// ```
#[must_use]
pub fn nth_prime_power_upper(n: &BigUint) -> Option<BigUint> {
    nth_prime_power(n, crate::nth_prime_power_upper, prime_power_count_lower, 1)
}

// ------------------------------------------------------- Ramanujan primes

/// Return an upper bound for the `n`th Ramanujan prime, with 2 as the first.
///
/// Port of MPU 0.75 `PP.pm` `nth_ramanujan_prime_upper`: a multiple of the
/// upper bound of the `3n`th prime. Returns `None` for `n = 0`.
///
/// ```
/// use math_prime_util::bigint::family_bounds::nth_ramanujan_prime_upper;
/// use num_bigint::BigUint;
/// assert_eq!(nth_ramanujan_prime_upper(&BigUint::from(2_u8)), Some(BigUint::from(11_u8)));
/// ```
#[must_use]
pub fn nth_ramanujan_prime_upper(n: &BigUint) -> Option<BigUint> {
    if n.bits() == 0 {
        return None;
    }
    native_nth(n, crate::nth_ramanujan_prime_upper).or_else(|| ramanujan_upper_pp(n))
}

/// `PP.pm` `nth_ramanujan_prime_upper` for `n >= 3_100_000_000`, the last
/// row of its table of multipliers.
fn ramanujan_upper_pp(n: &BigUint) -> Option<BigUint> {
    let nth = nth_prime_upper(&(n * 3_u8))?;
    Some((nth * 346_u16) >> 9_u8)
}

/// `PP.pm` `nth_ramanujan_prime_lower` for `n >= 10^10`.
fn ramanujan_lower_pp(n: &BigUint) -> Option<BigUint> {
    nth_prime_lower(&(n * 2_u8))
}

/// `PP.pm` `nth_ramanujan_prime_approx`: the midpoint of the bounds.
fn ramanujan_approx_pp(n: &BigUint) -> Option<BigUint> {
    let lo = ramanujan_lower_pp(n)?;
    let hi = ramanujan_upper_pp(n)?;
    Some(&lo + ((hi - &lo) >> 1_u8))
}

/// Return a lower bound for the `n`th Ramanujan prime, with 2 as the first.
///
/// Port of MPU 0.75 `PP.pm` `nth_ramanujan_prime_lower`: the lower bound of
/// the `2n`th prime. Returns `None` for `n = 0`.
///
/// ```
/// use math_prime_util::bigint::family_bounds::nth_ramanujan_prime_lower;
/// use num_bigint::BigUint;
/// assert_eq!(nth_ramanujan_prime_lower(&BigUint::from(2_u8)), Some(BigUint::from(11_u8)));
/// ```
#[must_use]
pub fn nth_ramanujan_prime_lower(n: &BigUint) -> Option<BigUint> {
    if n.bits() == 0 {
        return None;
    }
    native_nth(n, crate::nth_ramanujan_prime_lower).or_else(|| ramanujan_lower_pp(n))
}

/// Estimate the `n`th Ramanujan prime, with 2 as the first.
///
/// Port of MPU 0.75 `PP.pm` `nth_ramanujan_prime_approx`: the midpoint of
/// the lower and upper bounds. Returns `None` for `n = 0`.
///
/// ```
/// use math_prime_util::bigint::family_bounds::nth_ramanujan_prime_approx;
/// use num_bigint::BigUint;
/// assert_eq!(nth_ramanujan_prime_approx(&BigUint::from(2_u8)), Some(BigUint::from(11_u8)));
/// ```
#[must_use]
pub fn nth_ramanujan_prime_approx(n: &BigUint) -> Option<BigUint> {
    if n.bits() == 0 {
        return None;
    }
    native_nth(n, crate::nth_ramanujan_prime_approx).or_else(|| ramanujan_approx_pp(n))
}

/// The range `PP.pm` searches for the Ramanujan prime count of `n >= 11`.
fn ramanujan_range(n: &BigUint) -> (BigUint, BigUint) {
    (prime_count_lower(n) / 3_u8, prime_count_upper(n) >> 1_u8)
}

fn ramanujan_count(
    n: &BigUint,
    native: fn(u64) -> u64,
    nth: fn(&BigUint) -> Option<BigUint>,
    extra: u8,
    exit: Option<&Exit>,
) -> BigUint {
    if let Some(s) = small(n) {
        return big(native(s));
    }
    let (lo, hi) = ramanujan_range(n);
    search(n, lo, hi, &|m| nth(m).unwrap_or_default(), exit) + extra
}

/// Estimate the number of Ramanujan primes at most `n`.
///
/// Port of MPU 0.75 `PP.pm` `ramanujan_prime_count_approx`: a binary search
/// with [`nth_ramanujan_prime_approx`], stopped when the range is a
/// `10^-15` fraction of its start.
///
/// ```
/// use math_prime_util::bigint::family_bounds::ramanujan_prime_count_approx;
/// use num_bigint::BigUint;
/// assert_eq!(ramanujan_prime_count_approx(&BigUint::from(100_u8)), BigUint::from(10_u8));
/// ```
#[must_use]
pub fn ramanujan_prime_count_approx(n: &BigUint) -> BigUint {
    if let Some(s) = small(n) {
        return big(crate::ramanujan_prime_count_approx(s));
    }
    let lo = ramanujan_prime_count_lower(n);
    let hi = ramanujan_prime_count_upper(n);
    search(
        n,
        lo,
        hi,
        &|m| ramanujan_approx_pp(m).unwrap_or_default(),
        Some(&ramanujan_converged),
    )
}

/// The early exit of `PP.pm` `ramanujan_prime_count_approx`:
/// `($hi - $lo) / $lo < 1e-15`. With native integers Perl divides doubles.
/// With a bigint `hi` the quotient is a `Math::BigInt`, which never compares
/// below `1e-15`, so the search runs to the end.
fn ramanujan_converged(lo: &BigUint, hi: &BigUint) -> bool {
    small(hi).is_some() && as_f64(&(hi - lo)) / as_f64(lo) < 1e-15
}

/// Return a lower bound for the number of Ramanujan primes at most `n`.
///
/// Port of MPU 0.75 `PP.pm` `ramanujan_prime_count_lower`: a binary search
/// with [`nth_ramanujan_prime_upper`].
///
/// ```
/// use math_prime_util::bigint::family_bounds::ramanujan_prime_count_lower;
/// use num_bigint::BigUint;
/// assert_eq!(ramanujan_prime_count_lower(&BigUint::from(100_u8)), BigUint::from(10_u8));
/// ```
#[must_use]
pub fn ramanujan_prime_count_lower(n: &BigUint) -> BigUint {
    ramanujan_count(
        n,
        crate::ramanujan_prime_count_lower,
        ramanujan_upper_pp,
        0,
        None,
    )
}

/// Return an upper bound for the number of Ramanujan primes at most `n`.
///
/// Port of MPU 0.75 `PP.pm` `ramanujan_prime_count_upper`: one more than a
/// binary search with [`nth_ramanujan_prime_lower`].
///
/// ```
/// use math_prime_util::bigint::family_bounds::ramanujan_prime_count_upper;
/// use num_bigint::BigUint;
/// assert_eq!(ramanujan_prime_count_upper(&BigUint::from(100_u8)), BigUint::from(10_u8));
/// ```
#[must_use]
pub fn ramanujan_prime_count_upper(n: &BigUint) -> BigUint {
    ramanujan_count(
        n,
        crate::ramanujan_prime_count_upper,
        ramanujan_lower_pp,
        1,
        None,
    )
}

// ------------------------------------------------ semiprimes, twin primes

/// `PP.pm` `semiprime_count_approx` for `n >= 4`.
fn semiprime_count_pp(n: &BigUint) -> BigUint {
    let x = as_f64(n) + 0.000_000_01;
    let l1 = x.ln();
    let l2 = l1.ln();
    let est = (x / l1)
        * (0.111_479_101_14
            + 0.002_238_013_50 * l1
            + 0.442_332_079_22 * l2
            + 1.652_366_478_96 * l2.ln());
    nv_int(est + 0.5)
}

/// Estimate the number of semiprimes at most `n`.
///
/// Port of MPU 0.75 `PP.pm` `semiprime_count_approx`: a fit in `ln n`,
/// `ln ln n` and `ln ln ln n`, in doubles.
///
/// ```
/// use math_prime_util::bigint::family_bounds::semiprime_count_approx;
/// use num_bigint::BigUint;
/// assert_eq!(semiprime_count_approx(&BigUint::from(100_u8)), BigUint::from(34_u8));
/// ```
#[must_use]
pub fn semiprime_count_approx(n: &BigUint) -> BigUint {
    small(n).map_or_else(
        || semiprime_count_pp(n),
        |s| big(crate::semiprime_count_approx(s)),
    )
}

/// `PP.pm` `nth_semiprime_approx` for `n > 8`.
fn semiprime_nth_pp(n: &BigUint) -> BigUint {
    let x = as_f64(n) + 0.000_000_01;
    let l1 = x.ln();
    let l2 = l1.ln();
    nv_int(0.966 * x * l1 / l2 + 0.5)
}

/// Estimate the `n`th semiprime, with 4 as the first.
///
/// Port of MPU 0.75 `PP.pm` `nth_semiprime_approx`: `0.966 n ln n / ln ln n`
/// in doubles. Returns `None` for `n = 0`.
///
/// ```
/// use math_prime_util::bigint::family_bounds::nth_semiprime_approx;
/// use num_bigint::BigUint;
/// assert_eq!(nth_semiprime_approx(&BigUint::from(3_u8)), Some(BigUint::from(9_u8)));
/// ```
#[must_use]
pub fn nth_semiprime_approx(n: &BigUint) -> Option<BigUint> {
    if n.bits() == 0 {
        return None;
    }
    // The native estimate clamps at the largest semiprime.
    native_nth(n, crate::nth_semiprime_approx)
        .filter(|v| *v != big(18_446_744_073_709_551_601) || *n == big(1_701_748_900_850_019_777))
        .or_else(|| Some(semiprime_nth_pp(n)))
}

/// `PP.pm` `ExponentialIntegral` for `x >= 36.84`, where it takes the
/// divergent series `e^x / x * sum k! / x^k` with Kahan summation. (The
/// native [`ExponentialIntegral`](crate::ExponentialIntegral) is the C
/// version, a few ulps away.)
fn ei_asymptotic(x: f64) -> f64 {
    let invx = 1.0 / x;
    let mut term = invx;
    let mut sum = 1.0 + term;
    let mut compensation = 0.0;
    for n in 2..=200_u32 {
        let last_term = term;
        term *= f64::from(n) * invx;
        if term < 1e-16 {
            break;
        }
        let growing = term >= last_term;
        let y = (if growing { -last_term / 3.0 } else { term }) - compensation;
        let t = sum + y;
        compensation = (t - sum) - y;
        sum = t;
        if growing {
            break;
        }
    }
    if x < 700.0 {
        x.exp() * invx * sum
    } else {
        (x - x.ln()).exp() * sum
    }
}

/// `PP.pm` `twin_prime_count_approx` for `n >= 32_000_000`.
fn twin_count_pp(x: f64) -> BigUint {
    let logn = x.ln();
    let li2 = ei_asymptotic(logn) + 2.8853900817779268147198494 - x / logn;
    iv_int((1.32032363169373914785562422 * li2 + 0.5).floor())
}

/// Estimate the number of twin primes at most `n`.
///
/// Port of MPU 0.75 `PP.pm` `twin_prime_count_approx`: the Hardy-Littlewood
/// integral `2 C2 (Ei(ln n) - n / ln n + 2 / ln 2 ...)` in doubles.
///
/// ```
/// use math_prime_util::bigint::family_bounds::twin_prime_count_approx;
/// use num_bigint::BigUint;
/// assert_eq!(twin_prime_count_approx(&BigUint::from(100_u8)), BigUint::from(8_u8));
/// ```
#[must_use]
pub fn twin_prime_count_approx(n: &BigUint) -> BigUint {
    small(n).map_or_else(
        || twin_count_pp(as_f64(n)),
        |s| big(crate::twin_prime_count_approx(s)),
    )
}

/// Estimate the `n`th twin prime, with 3 as the first.
///
/// Port of MPU 0.75 `PP.pm` `nth_twin_prime_approx` for a big `n`: a binary
/// search with [`twin_prime_count_approx`] between `0.7 n ln^2 n` and
/// `1.1 n ln^2 n`, stopped when the range is a `10^-15` fraction of its
/// start. Returns `None` for `n = 0`.
///
/// ```
/// use math_prime_util::bigint::family_bounds::nth_twin_prime_approx;
/// use num_bigint::BigUint;
/// assert_eq!(nth_twin_prime_approx(&BigUint::from(3_u8)), Some(BigUint::from(11_u8)));
/// ```
#[must_use]
pub fn nth_twin_prime_approx(n: &BigUint) -> Option<BigUint> {
    if n.bits() == 0 {
        return None;
    }
    // The native estimate clamps at the largest twin prime.
    if let Some(v) = native_nth(n, crate::nth_twin_prime_approx)
        .filter(|v| *v != big(18_446_744_073_709_550_771))
    {
        return Some(v);
    }
    let x = Real::from_biguint(n);
    let logn = x.ln();
    let scale = x * logn.clone() * logn;
    let lo = (scale.clone() * Real::decimal("0.7")).floor_uint();
    let hi = (scale * Real::decimal("1.1")).floor_uint();
    Some(search(
        n,
        lo,
        hi,
        &|m| twin_count_pp(as_f64(m)),
        Some(&|lo, hi| (hi - lo) * 1_000_000_000_000_000_u64 < *lo),
    ))
}

// -------------------------------------------------------------- inverse li

/// Return the least integer `t` with `li(t) >= n`.
///
/// Port of MPU 0.75 `PP.pm` `inverse_li` above the native range: a Halley
/// iteration, then a search over the integers around it, with `li` in
/// 256-bit fixed point. The native [`inverse_li`](crate::inverse_li)
/// answers while its result fits `u64`.
///
/// ```
/// use math_prime_util::bigint::family_bounds::inverse_li;
/// use num_bigint::BigUint;
/// assert_eq!(inverse_li(&BigUint::from(10_u8)), BigUint::from(21_u8));
/// ```
#[must_use]
pub fn inverse_li(n: &BigUint) -> BigUint {
    if let Some(v) = small(n).map(crate::inverse_li).filter(|v| *v < u64::MAX) {
        return big(v);
    }
    let target = Real::from_biguint(n);
    let t = inverse_li_real(&target).floor_uint();
    inverse_li_from(&target, &t)
}

/// The least integer `x` with `li(x) >= target`, searched out from the
/// estimate `t` (above or below the answer).
fn inverse_li_from(target: &Real, t: &BigUint) -> BigUint {
    let at_least = |t: &BigUint| li_real(&Real::from_biguint(t)) >= *target;
    let below = |step: &BigUint| {
        if step > t {
            BigUint::default()
        } else {
            t - step
        }
    };
    let mut step = BigUint::from(1_u8);
    let (mut lo, mut hi) = if at_least(t) {
        let mut hi = t.clone();
        let mut lo = below(&step);
        while at_least(&lo) {
            hi = lo;
            step <<= 1_u8;
            lo = below(&step);
        }
        (lo, hi)
    } else {
        let mut lo = t.clone();
        let mut hi = t + &step;
        while !at_least(&hi) {
            lo = hi;
            step <<= 1_u8;
            hi = t + &step;
        }
        (lo, hi)
    };
    while &hi - &lo > BigUint::from(1_u8) {
        let mid = (&lo + &hi) >> 1_u8;
        if at_least(&mid) {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    hi
}

// ---------------------------------------------------------- almost primes

/// `@lower20` of `PP.pm`: lower bound multipliers for `n <= 1_048_575`.
const LOWER20: [f64; 13] = [
    0.0, 0.0, 0.8197, 0.8418, 0.5242, 0.5154, 0.3053, 0.1901, 0.1253, 0.0892, 0.06551, 0.05082,
    0.04101,
];
/// `@lower32` of `PP.pm`: for `n <= 4_294_967_295`.
const LOWER32: [f64; 21] = [
    0.0, 0.0, 1.004, 0.7383, 0.6828, 0.5939, 0.3594, 0.2222, 0.1438, 0.09754, 0.06981, 0.05245,
    0.04151, 0.03461, 0.03006, 0.02709, 0.02553, 0.02502, 0.02552, 0.02697, 0.02945,
];
/// `@lower64` of `PP.pm`: for 64-bit `n`.
const LOWER64: [f64; 41] = [
    0.0, 0.0, 1.011, 0.8093, 0.7484, 0.6465, 0.3982, 0.2463, 0.1571, 0.1048, 0.07363, 0.0545,
    0.0422, 0.0331, 0.0270, 0.0232, 0.0208, 0.0194, 0.0190, 0.0193, 0.0203, 0.0222, 0.0252, 0.0295,
    0.0356, 0.0444, 0.0570, 0.0753, 0.102, 0.14, 0.20, 0.297, 0.44, 0.68, 1.07, 1.71, 2.8, 4.7,
    8.0, 13.89, 23.98,
];
/// `@lower` of `PP.pm`: for `n` above 64 bits, grown by 1.5 per index.
const LOWER_BIG: [f64; 33] = [
    0.0, 0.0, 1.011, 0.8093, 0.7484, 0.6465, 0.3982, 0.2463, 0.1571, 0.1048, 0.07363, 0.0545,
    0.0422, 0.0331, 0.0270, 0.0230, 0.0200, 0.0187, 0.018, 0.018, 0.019, 0.020, 0.020, 0.027,
    0.032, 0.040, 0.051, 0.068, 0.090, 0.12, 0.18, 0.26, 0.355,
];
/// `@upper20` of `PP.pm`.
const UPPER20: [f64; 13] = [
    0.0, 0.0, 1.006, 0.7385, 0.6830, 0.5940, 0.3596, 0.2227, 0.1439, 0.09785, 0.07016, 0.05303,
    0.04202,
];
/// `@upper32` of `PP.pm`.
const UPPER32: [f64; 21] = [
    0.0, 0.0, 1.013, 0.8094, 0.7485, 0.6467, 0.3984, 0.2464, 0.1572, 0.1049, 0.07364, 0.05452,
    0.04266, 0.03542, 0.03082, 0.02798, 0.02642, 0.02585, 0.02615, 0.02808, 0.03054,
];
/// `@upper64` of `PP.pm`, grown by 2.1 per index.
const UPPER64: [f64; 41] = [
    0.0, 0.0, 1.028, 1.028, 1.3043, 0.72208, 0.46609, 0.29340, 0.18571, 0.12063, 0.0815, 0.0575,
    0.0427, 0.03490, 0.03007, 0.02710, 0.02554, 0.02504, 0.02554, 0.02699, 0.02954, 0.03294,
    0.03779, 0.04453, 0.05393, 0.06703, 0.08543, 0.1117, 0.1494, 0.205, 0.287, 0.410, 0.60, 0.90,
    1.36, 2.12, 3.35, 5.38, 8.83, 14.75, 25.07,
];
/// Sequence A078843 of the OEIS (`@A078843` of `PP.pm`): the least number
/// with `k` prime factors that is not twice the least with `k - 1`.
const A078843: [u64; 61] = [
    1,
    2,
    3,
    5,
    8,
    14,
    23,
    39,
    64,
    103,
    169,
    269,
    427,
    676,
    1065,
    1669,
    2628,
    4104,
    6414,
    10023,
    15608,
    24281,
    37733,
    58503,
    90616,
    140187,
    216625,
    334527,
    516126,
    795632,
    1225641,
    1886570,
    2901796,
    4460359,
    6851532,
    10518476,
    16138642,
    24748319,
    37932129,
    58110457,
    88981343,
    136192537,
    208364721,
    318653143,
    487128905,
    744398307,
    1137129971,
    1736461477,
    2650785552,
    4045250962,
    6171386419,
    9412197641,
    14350773978,
    21874583987,
    33334053149,
    50783701654,
    77348521640,
    117780873397,
    179306456282,
    272909472119,
    415284741506,
];

/// A multiplier from `table`, grown by `grow` per index past its end.
fn grown(table: &[f64], k: u64, grow: f64) -> f64 {
    let index = usize::try_from(k).unwrap_or(usize::MAX);
    if let Some(v) = table.get(index) {
        return *v;
    }
    let mut v = table.last().copied().unwrap_or(0.0);
    for _ in table.len()..=index {
        v *= grow;
    }
    v
}

/// `PP.pm` `_kap_reduce_count`: halve `n` and drop `k` until `n >= 3^k`.
fn kap_reduce_count(mut k: u64, n: &BigUint) -> (u64, BigUint) {
    let mut n = n.clone();
    let mut pow3k = BigUint::from(3_u8).pow(u32::try_from(k).unwrap_or(u32::MAX));
    while n < pow3k {
        n >>= 1_u8;
        k -= 1;
        pow3k /= 3_u8;
    }
    (k, n)
}

/// `ln n`, `ln ln n` and `ln ln n + 0.26153` as doubles.
fn kap_logs(x: f64) -> (f64, f64, f64) {
    let logx = x.ln();
    let loglogx = logx.ln();
    (logx, loglogx, loglogx + 0.26153)
}

/// The product `x / ln x * prod (a / i)` that `PP.pm` uses for `k >= 5`.
fn kap_asymptotic(k: u64, x: f64) -> f64 {
    let (logx, loglogx, _) = kap_logs(x);
    let kk = k as f64;
    let logplus = loglogx + (kk.ln() * kk.ln().ln() - 0.504377);
    let mut bound = x / logx;
    for i in 1..k {
        bound *= logplus / i as f64;
    }
    bound
}

/// `PP.pm` `almost_prime_count_lower` without the native shortcut.
fn kap_lower_pp(k: u64, n: &BigUint) -> BigUint {
    if n.bits() <= k {
        return BigUint::default();
    }
    let (k, n) = kap_reduce_count(k, n);
    if k == 0 {
        return big(u64::from(n.bits() > 0));
    }
    if k == 1 {
        return prime_count_lower(&n);
    }
    let x = as_f64(&n);
    let (logx, loglogx, logplus) = kap_logs(x);
    let is64 = small(&n).is_some();
    let mut multl = if x <= 1_048_575.0 {
        LOWER20.get(k as usize).copied().unwrap_or(0.0)
    } else if x <= 4_294_967_295.0 {
        LOWER32.get(k as usize).copied().unwrap_or(0.0)
    } else if is64 {
        LOWER64.get(k as usize).copied().unwrap_or(0.0)
    } else {
        grown(&LOWER_BIG, k, 1.5)
    };
    let bound = match k {
        2 if x <= 1e12 => x * (loglogx + 0.261536) / logx,
        2 => {
            // Bayless Theorem 5.2.
            multl = 1.0;
            (x * (loglogx + 0.1769) / logx) * (1.0 + 0.4232 / logx)
        }
        3 => {
            // Kinlaw Theorem 1, with custom multipliers for 64-bit n.
            multl = if x < 638.0 {
                0.8418
            } else if x < 1927.0 {
                let dist = (x - 638.0) / (1926.0 - 638.0);
                (1.0 - dist) * 0.8939 + dist * 0.9233
            } else if x < 500_194.0 {
                let dist = (x - 1927.0) / (500_194.0 - 1927.0);
                (1.0 - dist) * 0.9233 + dist * 1.000
            } else if x <= 3_184_393_786.0 {
                let dist = (x - 500_194.0) / (3_184_393_786.0 - 500_194.0);
                (1.0 - dist) * 1.0000 + dist * 1.039
            } else if is64 {
                1.0004
            } else {
                1.0
            };
            x * loglogx * loglogx / (2.0 * logx)
        }
        4 => {
            if !is64 {
                multl = 0.4999;
            }
            x * logplus * logplus * logplus / (6.0 * logx)
        }
        _ => kap_asymptotic(k, x),
    };
    nv_int((bound * multl).max(1.0))
}

/// `PP.pm` `almost_prime_count_upper` without the native shortcut.
fn kap_upper_pp(k: u64, n: &BigUint) -> BigUint {
    if n.bits() <= k {
        return BigUint::default();
    }
    let (k, n) = kap_reduce_count(k, n);
    if k == 0 {
        return big(u64::from(n.bits() > 0));
    }
    if k == 1 {
        return prime_count_upper(&n);
    }
    let x = as_f64(&n);
    let (logx, loglogx, logplus) = kap_logs(x);
    let is64 = small(&n).is_some();
    let mut multu = if x <= 1_048_575.0 {
        UPPER20.get(k as usize).copied().unwrap_or(0.0)
    } else if x <= 4_294_967_295.0 {
        UPPER32.get(k as usize).copied().unwrap_or(0.0)
    } else {
        grown(&UPPER64, k, 2.1)
    };
    let bound = match k {
        // Bayless Corollary 5.1.
        2 => 1.028 * x * (loglogx + 0.261536) / logx,
        3 => {
            // Bayless Theorem 5.3.
            if x > 4_294_967_295.0 && is64 {
                multu = 0.8711;
            }
            x * (logplus * logplus + 1.055852) / (2.0 * logx)
        }
        4 => {
            // Bayless Theorem 5.4.
            let mut bound = x * logplus * logplus * logplus / (6.0 * logx);
            if x >= 1e12 {
                bound += 0.511977 * x * ((x / 4.0).ln().ln() + 0.261536) / logx;
                multu = 1.028;
            }
            if is64 {
                if x > 4_294_967_295.0 {
                    multu = 0.780;
                }
                if x > 1e12 {
                    multu = 0.6921;
                }
            }
            bound
        }
        _ => kap_asymptotic(k, x),
    };
    nv_int((bound * multu).max(1.0) + 1.0)
}

/// `PP.pm` `almost_prime_count_approx` without the native shortcut, for
/// `k >= 3`.
fn kap_approx_pp(k: u64, n: &BigUint) -> BigUint {
    if n.bits() <= k {
        return BigUint::default();
    }
    let lo = kap_lower_pp(k, n);
    let hi = kap_upper_pp(k, n);
    let x = as_f64(n);
    if k == 3 {
        let (l1, l2) = (x.ln(), x.ln().ln());
        let (a, s) = if x <= 638.0 {
            (0.865_814, 1.554_688)
        } else if x <= 1544.0 {
            (0.822_256, 1.05)
        } else if x <= 1927.0 {
            (0.791_747, 0.625)
        } else if x <= 486_586.0 {
            (1.004_09, 2.865_611)
        } else if x <= 1_913_680.0 {
            (0.999_618, 2.790_963)
        } else if x <= 22_347_532.0 {
            (0.995_635, 2.719_238)
        } else if x <= 2.95e8 {
            (0.988_802, 2.584_473)
        } else if x <= 4.20e9 {
            (0.983_098, 2.457_108)
        } else if x <= 7.07e10 {
            (0.978_931, 2.352_818)
        } else if x <= 1.36e12 {
            (0.975_953, 2.269_745)
        } else if x <= 4.1e13 {
            (0.973_796, 2.203_002)
        } else if x <= 9.2e14 {
            (0.972_213, 2.148_463)
        } else {
            (0.971_438, 2.119_279)
        };
        let est = 0.5 * a * x * ((l2 + 0.26153) * (l2 + 0.26153)) / (l1 + s) + 0.5;
        return if est < as_f64(&lo) {
            lo
        } else if est > as_f64(&hi) {
            hi
        } else {
            nv_int(est)
        };
    }
    let mult = if x < 4_294_967_296.0 && k < 13 {
        0.9
    } else if k > 11 {
        0.20
    } else {
        0.76
    };
    if small(&lo).is_some() && small(&hi).is_some() {
        return nv_int(as_f64(&lo) + as_f64(&(&hi - &lo)) * mult + 0.5);
    }
    // Exact integer arithmetic on 16-bit fixed point multipliers.
    let imult = (mult * 65536.0).floor() as u64;
    ((&hi - &lo) * imult + (&lo << 16_u8)) >> 16_u8
}

/// Estimate the number of integers with `k` prime factors, counted with
/// multiplicity, at most `n`.
///
/// Port of MPU 0.75 `PP.pm` `almost_prime_count_approx`: the prime count,
/// the semiprime estimate for `k = 2`, a fit for `k = 3` and a weighted mean
/// of the bounds for larger `k`.
///
/// ```
/// use math_prime_util::bigint::family_bounds::almost_prime_count_approx;
/// use num_bigint::BigUint;
/// assert_eq!(almost_prime_count_approx(3, &BigUint::from(1000_u16)), BigUint::from(249_u16));
/// ```
#[must_use]
pub fn almost_prime_count_approx(k: u64, n: &BigUint) -> BigUint {
    if let Some(s) = small(n) {
        return big(crate::almost_prime_count_approx(k, s));
    }
    match k {
        0 => big(1),
        1 => prime_count_approx(n),
        2 => semiprime_count_approx(n),
        _ => kap_approx_pp(k, n),
    }
}

/// Return a lower bound for the number of integers with `k` prime factors,
/// counted with multiplicity, at most `n`.
///
/// Port of MPU 0.75 `PP.pm` `almost_prime_count_lower`: the bounds of
/// Bayless and Kinlaw for `k` up to 4 and the Hildebrand-Tenenbaum form
/// above, each times a fitted multiplier, in doubles.
///
/// ```
/// use math_prime_util::bigint::family_bounds::almost_prime_count_lower;
/// use num_bigint::BigUint;
/// assert_eq!(almost_prime_count_lower(2, &BigUint::from(1000_u16)), BigUint::from(260_u16));
/// ```
#[must_use]
pub fn almost_prime_count_lower(k: u64, n: &BigUint) -> BigUint {
    small(n).map_or_else(
        || kap_lower_pp(k, n),
        |s| big(crate::almost_prime_count_lower(k, s)),
    )
}

/// Return an upper bound for the number of integers with `k` prime factors,
/// counted with multiplicity, at most `n`.
///
/// Port of MPU 0.75 `PP.pm` `almost_prime_count_upper`; see
/// [`almost_prime_count_lower`].
///
/// ```
/// use math_prime_util::bigint::family_bounds::almost_prime_count_upper;
/// use num_bigint::BigUint;
/// assert_eq!(almost_prime_count_upper(2, &BigUint::from(100_u8)), BigUint::from(34_u8));
/// ```
#[must_use]
pub fn almost_prime_count_upper(k: u32, n: &BigUint) -> BigUint {
    small(n).map_or_else(
        || kap_upper_pp(u64::from(k), n),
        |s| big(crate::almost_prime_count_upper(k, s)),
    )
}

/// Which bound of the `n`th almost prime a search computes.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Nth {
    Lower,
    Upper,
    Approx,
}

/// `PP.pm` `_kap_reduce_nth`: how many doublings `k` can drop by.
fn kap_reduce_nth(k: u32, n: &BigUint) -> u32 {
    if k <= 1 {
        return 0;
    }
    let top = A078843.len() - 1;
    let mut r = 0;
    if k as usize > top {
        if *n >= big(A078843[top]) {
            return 0;
        }
        r = k - u32::try_from(top).unwrap_or(0);
    }
    while *n < big(A078843[(k - r) as usize]) {
        r += 1;
    }
    r
}

/// `PP.pm` `_fast_small_nth_almost_prime` for `1 <= n < 8` and `k >= 2`.
fn kap_small_nth(k: u32, n: u64) -> BigUint {
    let index = n as usize;
    if k == 2 {
        return big([0, 4, 6, 9, 10, 14, 15, 21][index]);
    }
    big([0, 8, 12, 18, 20, 27, 28, 30][index]) << (k - 3)
}

/// `PP.pm` `_almost_prime_nth_asymptotic`, plus one, as an integer.
fn kap_nth_asymptotic(k: u32, n: &BigUint) -> BigUint {
    let numk = k - if k < 7 {
        1
    } else if k < 12 {
        2
    } else {
        (k - 6) >> 2
    };
    if small(n).is_some() {
        let x = as_f64(n);
        let (logx, loglogx, _) = kap_logs(x);
        let mut est = x * logx;
        for i in 1..=numk {
            est *= f64::from(i) / loglogx;
        }
        return nv_int(1.0 + est);
    }
    let x = Real::from_biguint(n);
    let logx = x.ln();
    let loglogx = logx.ln();
    let mut est = x * logx;
    for i in 1..=numk {
        est = est * Real::from_u64(u64::from(i)) / loglogx.clone();
    }
    (est + Real::one()).floor_uint()
}

/// `PP.pm` `nth_almost_prime_*` for `k >= 2` and `n >= 8`, which of the three
/// is `which`.
fn kap_nth_pp(k: u32, n: &BigUint, which: Nth) -> BigUint {
    let r = kap_reduce_nth(k, n);
    if r > 0 {
        return kap_nth_pp(k - r, n, which) << r;
    }
    let kk = u64::from(k);
    let count: &dyn Fn(&BigUint) -> BigUint = match which {
        Nth::Upper => &|m| kap_lower_pp(kk, m),
        Nth::Lower => &|m| kap_upper_pp(kk, m),
        Nth::Approx => &|m| kap_approx_pp(kk, m),
    };
    let mut lo = big(5) << k;
    let mut hi = kap_nth_asymptotic(k, n);
    if which == Nth::Approx {
        lo = kap_nth_pp(k, n, Nth::Lower);
        hi = kap_nth_pp(k, n, Nth::Upper);
    } else {
        // The first guess may be too low: bump it up until it is in range.
        let mut rhi = count(&hi);
        while rhi < *n {
            lo = &hi + 1_u8;
            let step = (1.02 * (as_f64(&hi) / as_f64(&rhi)) * (as_f64(n) - as_f64(&rhi))).floor();
            hi = &hi + nv_exact(step) + 100_u8;
            rhi = count(&hi);
        }
    }
    while lo < hi {
        let mid = &lo + ((&hi - &lo) >> 1_u8);
        if count(&mid) < *n {
            lo = mid + 1_u8;
        } else {
            hi = mid;
        }
    }
    lo
}

fn nth_almost_prime(
    k: u32,
    n: &BigUint,
    native: fn(u32, u64) -> Option<u64>,
    which: Nth,
) -> Option<BigUint> {
    if n.bits() == 0 {
        return None;
    }
    if let Some(v) = small(n).and_then(|s| native(k, s)).filter(|v| *v > 0) {
        return Some(big(v));
    }
    if k == 0 {
        return None;
    }
    if k == 1 {
        return match which {
            Nth::Lower => nth_prime_lower(n),
            Nth::Upper => nth_prime_upper(n),
            Nth::Approx => nth_prime_approx(n),
        };
    }
    if which == Nth::Approx && k == 2 {
        return nth_semiprime_approx(n);
    }
    if let Some(s) = small(n).filter(|s| *s < 8) {
        return Some(kap_small_nth(k, s));
    }
    Some(kap_nth_pp(k, n, which))
}

/// Estimate the `n`th integer with `k` prime factors, counted with
/// multiplicity.
///
/// Port of MPU 0.75 `PP.pm` `nth_almost_prime_approx`: a binary search with
/// [`almost_prime_count_approx`] between [`nth_almost_prime_lower`] and
/// [`nth_almost_prime_upper`]. Returns `None` for `n = 0` and `k = 0` (where
/// MPU returns 0 for `n > 1`).
///
/// ```
/// use math_prime_util::bigint::family_bounds::nth_almost_prime_approx;
/// use num_bigint::BigUint;
/// assert_eq!(nth_almost_prime_approx(3, &BigUint::from(2_u8)), Some(BigUint::from(12_u8)));
/// ```
#[must_use]
pub fn nth_almost_prime_approx(k: u32, n: &BigUint) -> Option<BigUint> {
    nth_almost_prime(k, n, crate::nth_almost_prime_approx, Nth::Approx)
}

/// Return a lower bound for the `n`th integer with `k` prime factors,
/// counted with multiplicity.
///
/// Port of MPU 0.75 `PP.pm` `nth_almost_prime_lower`: a binary search with
/// [`almost_prime_count_upper`]. Returns `None` for `n = 0` and `k = 0`.
///
/// ```
/// use math_prime_util::bigint::family_bounds::nth_almost_prime_lower;
/// use num_bigint::BigUint;
/// assert_eq!(nth_almost_prime_lower(3, &BigUint::from(2_u8)), Some(BigUint::from(12_u8)));
/// ```
#[must_use]
pub fn nth_almost_prime_lower(k: u32, n: &BigUint) -> Option<BigUint> {
    nth_almost_prime(k, n, crate::nth_almost_prime_lower, Nth::Lower)
}

/// Return an upper bound for the `n`th integer with `k` prime factors,
/// counted with multiplicity.
///
/// Port of MPU 0.75 `PP.pm` `nth_almost_prime_upper`: a binary search with
/// [`almost_prime_count_lower`]. Returns `None` for `n = 0` and `k = 0`.
///
/// ```
/// use math_prime_util::bigint::family_bounds::nth_almost_prime_upper;
/// use num_bigint::BigUint;
/// assert_eq!(nth_almost_prime_upper(3, &BigUint::from(2_u8)), Some(BigUint::from(12_u8)));
/// ```
#[must_use]
pub fn nth_almost_prime_upper(k: u32, n: &BigUint) -> Option<BigUint> {
    nth_almost_prime(k, n, crate::nth_almost_prime_upper, Nth::Upper)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn two(bits: usize) -> BigUint {
        BigUint::from(1_u8) << bits
    }

    #[test]
    fn perl_integer_conversions() {
        assert_eq!(nv_exact(f64::INFINITY), BigUint::default());
        assert_eq!(nv_exact(2.9), big(2));
        assert_eq!(nv_int(f64::NAN), BigUint::default());
        assert_eq!(nv_int(0.5), BigUint::default());
        assert_eq!(nv_int(123.75), big(123));
        assert_eq!(nv_int(1.0e15 + 8.0), big(1_000_000_000_000_010));
        assert_eq!(
            nv_int(1.2345678901234567e20),
            big(123_456_789_012_346) * 1_000_000_u32
        );
        assert_eq!(iv_int(12_345.6), big(12_345));
        assert_eq!(iv_int(1.0e20), big(10).pow(20));
    }

    #[test]
    fn zero_has_no_nth_value() {
        let z = BigUint::default();
        assert_eq!(nth_lucky_approx(&z), None);
        assert_eq!(nth_lucky_lower(&z), None);
        assert_eq!(nth_lucky_upper(&z), None);
        assert_eq!(nth_perfect_power_approx(&z), None);
        assert_eq!(nth_perfect_power_lower(&z), None);
        assert_eq!(nth_perfect_power_upper(&z), None);
        assert_eq!(nth_prime_power_approx(&z), None);
        assert_eq!(nth_prime_power_lower(&z), None);
        assert_eq!(nth_prime_power_upper(&z), None);
        assert_eq!(nth_ramanujan_prime_approx(&z), None);
        assert_eq!(nth_ramanujan_prime_lower(&z), None);
        assert_eq!(nth_ramanujan_prime_upper(&z), None);
        assert_eq!(nth_semiprime_approx(&z), None);
        assert_eq!(nth_twin_prime_approx(&z), None);
        for k in 0..4 {
            assert_eq!(nth_almost_prime_approx(k, &z), None);
            assert_eq!(nth_almost_prime_lower(k, &z), None);
            assert_eq!(nth_almost_prime_upper(k, &z), None);
        }
        assert_eq!(nth_almost_prime_approx(0, &big(5)), None);
    }

    #[test]
    fn almost_prime_counts_all_regions() {
        let ns = [
            big(0),
            big(1),
            big(3),
            big(9),
            big(100),
            big(5000),
            big(2_000_000),
            big(5_000_000_000),
            big(10).pow(13),
            big(10).pow(15),
            big(u64::MAX),
            two(70),
            big(10).pow(25),
        ];
        for k in 0..=70 {
            for n in &ns {
                let lo = kap_lower_pp(k, n);
                let hi = kap_upper_pp(k, n);
                assert!(lo <= hi || k < 2, "k={k} n={n}: {lo} {hi}");
                if k >= 3 {
                    let mid = kap_approx_pp(k, n);
                    assert!(lo <= mid && mid <= hi, "k={k} n={n}");
                }
                let _ = almost_prime_count_approx(k, n);
                let _ = almost_prime_count_lower(k, n);
                let _ = almost_prime_count_upper(u32::try_from(k).unwrap_or(0), n);
            }
        }
        assert_eq!(almost_prime_count_approx(0, &two(80)), big(1));
        assert_eq!(
            almost_prime_count_approx(2, &two(80)),
            semiprime_count_approx(&two(80))
        );
    }

    #[test]
    fn almost_prime_counts_mid_values_reduce() {
        // `n < 3^k`: the count reduces to a smaller `k`.
        let (k, n) = kap_reduce_count(60, &two(70));
        assert!(k < 60 && n >= big(3).pow(u32::try_from(k).unwrap_or(0)));
        assert_eq!(kap_reduce_count(2, &big(100)), (2, big(100)));
    }

    #[test]
    fn nth_almost_primes_all_regions() {
        let ns = [
            big(1),
            big(5),
            big(7),
            big(8),
            big(100),
            big(10).pow(6),
            big(10).pow(12),
            big(u64::MAX),
            two(70),
            big(10).pow(25),
        ];
        for k in 1..=70_u32 {
            for n in &ns {
                let (Some(lo), Some(mid), Some(hi)) = (
                    nth_almost_prime_lower(k, n),
                    nth_almost_prime_approx(k, n),
                    nth_almost_prime_upper(k, n),
                ) else {
                    assert_eq!(k, 1, "k={k} n={n}");
                    continue;
                };
                assert!(
                    lo.bits() > 0 && mid.bits() > 0 && hi.bits() > 0,
                    "k={k} n={n}"
                );
            }
        }
        for k in 2..=20_u32 {
            for n in 1..8_u64 {
                assert_eq!(
                    Some(kap_small_nth(k, n)),
                    crate::nth_almost_prime(k, n).map(big),
                    "k={k} n={n}"
                );
            }
        }
        assert_eq!(kap_reduce_nth(1, &big(5)), 0);
        assert!(kap_reduce_nth(70, &big(10).pow(6)) > 0);
        assert_eq!(kap_reduce_nth(70, &two(70)), 0);
    }

    #[test]
    fn lucky_counts_bracket_the_definition() {
        for i in 0_u64..24 {
            let n = two(64) * (i + 1) + big(i * 7_919_000_000_001);
            // The search runs on the pure Rust estimates, not on the native
            // functions the public forms use below 2^64.
            for (count, nth, plus) in [
                (
                    lucky_count_approx as fn(&BigUint) -> BigUint,
                    lucky_approx_pp as fn(&BigUint) -> BigUint,
                    0_u8,
                ),
                (lucky_count_lower, lucky_upper_pp, 0),
                (lucky_count_upper, lucky_lower_pp, 1),
            ] {
                let c = count(&n) - plus;
                assert!(nth(&c) < n, "{n}");
                assert!(nth(&(&c + 1_u8)) >= n, "{n}");
            }
        }
        // A range that brackets the answer from above, and from below.
        let n = big(10).pow(30);
        let (lo, hi) = lucky_range(&n);
        assert!(lo < hi);
    }

    #[test]
    fn inverse_li_matches_the_definition() {
        for i in 0_u64..300 {
            let n = two(64) + big(i * 104_729) * big(i + 1);
            let r = inverse_li(&n);
            let target = Real::from_biguint(&n);
            assert!(li_real(&Real::from_biguint(&r)) >= target, "{n}");
            assert!(li_real(&Real::from_biguint(&(&r - 1_u8))) < target, "{n}");
        }
        for e in 20..40_u32 {
            let n = big(10).pow(e) + big(u64::from(e));
            let r = inverse_li(&n);
            assert!(li_real(&Real::from_biguint(&r)) >= Real::from_biguint(&n));
        }
    }

    #[test]
    fn semiprime_twin_and_ramanujan_edges() {
        let n = two(70);
        assert!(nth_semiprime_approx(&n).is_some());
        assert!(nth_twin_prime_approx(&n).is_some());
        assert!(semiprime_count_approx(&two(65)) > big(0));
        assert!(ramanujan_prime_count_upper(&two(65)) >= ramanujan_prime_count_lower(&two(65)));
    }

    #[test]
    fn inverse_li_from_any_start() {
        let n = big(10).pow(25);
        let target = Real::from_biguint(&n);
        let exact = inverse_li(&n);
        for t in [
            &exact * 2_u8,
            &exact + 1_u8,
            &exact - 5_u8,
            &exact / 2_u8,
            big(1),
        ] {
            assert_eq!(inverse_li_from(&target, &t), exact);
        }
    }

    #[test]
    fn three_almost_prime_tables() {
        for x in [
            100_u64,
            1000,
            1700,
            1_000,
            100_000,
            1_000_000,
            10_000_000,
            100_000_000,
            1_000_000_000,
            10_000_000_000,
            100_000_000_000,
            10_000_000_000_000,
            500_000_000_000_000,
            5_000_000_000_000_000,
            u64::MAX,
        ] {
            let n = big(x);
            let (lo, mid, hi) = (
                kap_lower_pp(3, &n),
                kap_approx_pp(3, &n),
                kap_upper_pp(3, &n),
            );
            assert!(lo <= mid && mid <= hi, "{x}");
        }
        for x in [700_u64, 1500, 1_000_000, 4_000_000_000, 10_000_000_000] {
            let _ = kap_lower_pp(3, &big(x));
            let _ = kap_upper_pp(3, &big(x));
        }
    }
}

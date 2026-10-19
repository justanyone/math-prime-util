//! Prime counts, their bounds and estimates, the `n`th prime, the prime
//! stream and the prime and composite loops, and `is_polygonal`, beyond the
//! 64-bit range: the functions of [`crate::count`], [`crate::iter`] and
//! [`crate::combinat`] with the same names, for [`BigUint`].
//!
//! Port of MPU 0.75 `PP.pm`, the pure Perl path that inputs above the
//! native range take. On the native range the results equal the native ones:
//! each function hands over to the native path where that returns a value,
//! and takes the pure Perl formula above `MPU_MAX_PRIME_IDX`
//! (`425_656_284_035_217_743`, the index of the largest 64-bit prime) for the
//! `n`th-prime family, or above `u64::MAX` for the counts. The bound and
//! count formulas run in 256-bit fixed point, where MPU's `Math::BigFloat`
//! carries 40 decimal digits; [`prime_count_approx`] follows the rounding
//! steps of the Perl code, as its result is a rounded sum of many terms.
//!
//! ```
//! use math_prime_util::bigint::counts::{nth_prime, prime_count_lower, prime_count_upper};
//! use num_bigint::BigUint;
//! let two64 = BigUint::from(1_u8) << 64_usize;
//! let idx = BigUint::from(425_656_284_035_217_744_u64);
//! assert_eq!(nth_prime(&idx), Some(&two64 + 13_u8));
//! assert!(prime_count_lower(&two64) < prime_count_upper(&two64));
//! ```

use super::Int;
use super::arith::{is_square, sqrtint};
use super::primality::{is_prime, next_prime};
use crate::count::long_double::Ld;
use crate::count::nth_prime_approx::li as li_ld;
use crate::iter::BigPrimeIterator;
use num_bigint::{BigInt, BigUint, Sign};
use std::ops::{Add, ControlFlow, Div, Mul, Neg, RangeInclusive, Sub};
use std::sync::OnceLock;

/// MPU `constants.h` `MPU_MAX_PRIME`, the largest 64-bit prime.
const MAX_PRIME: u64 = 18_446_744_073_709_551_557;
/// MPU `constants.h` `MPU_MAX_PRIME_IDX`, the index of [`MAX_PRIME`].
const MAX_PRIME_IDX: u64 = 425_656_284_035_217_743;
/// `Math::BigFloat->div_scale`, the digits MPU's pure Perl floats carry
/// when no accuracy is set.
const DIV_SCALE: u32 = 40;
/// `_Euler` of MPU 0.75 `PP.pm`: Euler's constant to 80 digits.
const EULER: &str =
    "0.57721566490153286060651209008240243104215933593992359880576723488486772677766467";
/// `_Li2` of MPU 0.75 `PP.pm`: `li(2)` to 75 digits.
const LI2: &str = "1.04516378011749278484458888919461313652261557815120157583290914407501320521";
/// Pi to 60 digits; `Math::BigFloat->bpi(40)` in MPU.
const PI: &str = "3.14159265358979323846264338327950288419716939937510582097494";

fn zero() -> BigUint {
    BigUint::from(0_u8)
}

fn one() -> BigUint {
    BigUint::from(1_u8)
}

/// `10^k`.
fn pow10(k: u32) -> BigUint {
    BigUint::from(10_u8).pow(k)
}

/// The decimal digits of `n` (`length("$n")` in Perl); zero has one digit.
fn digits(n: &BigUint) -> u32 {
    u32::try_from(n.to_string().len()).unwrap_or(u32::MAX)
}

/// Fraction bits of [`Real`].
pub(super) const FRAC: u32 = 256;

/// A real number in fixed point: the value is `inner / 2^256`. The integer
/// part is unbounded. Products and quotients truncate at the last bit, so
/// each operation is exact to about 77 decimal digits.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct Real(pub(super) BigInt);

impl Real {
    pub(super) fn zero() -> Self {
        Self(BigInt::from(0))
    }

    pub(super) fn one() -> Self {
        Self::from_u64(1)
    }

    pub(super) fn from_u64(n: u64) -> Self {
        Self(BigInt::from(n) << FRAC)
    }

    pub(super) fn from_biguint(n: &BigUint) -> Self {
        Self(BigInt::from(n.clone()) << FRAC)
    }

    /// A non-negative decimal literal such as `10.667`, exact as
    /// `Math::BigFloat` holds it (to the last bit of the fraction).
    pub(super) fn decimal(text: &str) -> Self {
        let mut digits = BigInt::from(0);
        let mut scale = 0;
        let mut fraction = false;
        for b in text.bytes() {
            if b == b'.' {
                fraction = true;
            } else {
                digits = digits * 10_u32 + u32::from(b.wrapping_sub(b'0'));
                scale += u32::from(fraction);
            }
        }
        Self((digits << FRAC) / BigInt::from(pow10(scale)))
    }

    /// A `double` as `Math::BigFloat->new` takes it from Perl: the value
    /// printed with `%.15g`, that is rounded to 15 significant digits.
    fn from_f64_15(value: f64) -> Self {
        let text = format!("{value:.14e}");
        let (mantissa, exponent) = text.split_once('e').unwrap_or((&text, "0"));
        let exponent: i32 = exponent.parse().unwrap_or(0);
        let digits = mantissa
            .bytes()
            .filter(u8::is_ascii_digit)
            .fold(BigInt::from(0), |acc, b| acc * 10_u32 + u32::from(b - b'0'));
        // Fifteen digits were printed: the value is `digits * 10^(exponent - 14)`.
        let shift = exponent - 14;
        let inner = match u32::try_from(shift) {
            Ok(up) => (digits * BigInt::from(pow10(up))) << FRAC,
            Err(_) => (digits << FRAC) / BigInt::from(pow10(shift.unsigned_abs())),
        };
        Self(if mantissa.starts_with('-') {
            -inner
        } else {
            inner
        })
    }

    pub(super) fn is_zero(&self) -> bool {
        self.0.sign() == Sign::NoSign
    }

    /// The largest integer at most the value.
    pub(super) fn floor(&self) -> BigInt {
        &self.0 >> FRAC
    }

    /// [`floor`](Self::floor) as an unsigned value; negatives give zero.
    pub(super) fn floor_uint(&self) -> BigUint {
        self.floor().to_biguint().unwrap_or_default()
    }

    pub(super) fn abs(&self) -> Self {
        Self(BigInt::from(self.0.magnitude().clone()))
    }

    /// The nearest `double`, ties to even, as Perl converts a bigint to NV.
    pub(super) fn to_f64(&self) -> f64 {
        let magnitude = self.0.magnitude();
        let bits = magnitude.bits();
        if bits == 0 {
            return 0.0;
        }
        let shift = bits.saturating_sub(53);
        let mut mantissa = magnitude >> shift;
        if shift > 0 {
            let remainder = magnitude - (&mantissa << shift);
            let half = one() << (shift - 1);
            if remainder > half || (remainder == half && mantissa.bit(0)) {
                mantissa += 1_u8;
            }
        }
        // The mantissa has at most 54 bits: exact in a `double`.
        #[allow(clippy::cast_precision_loss)]
        let mantissa = mantissa.to_u64().unwrap_or(u64::MAX) as f64;
        let exponent = i32::try_from(shift).unwrap_or(i32::MAX) - i32::try_from(FRAC).unwrap_or(0);
        let value = mantissa * 2_f64.powi(exponent);
        if self.0.sign() == Sign::Minus {
            -value
        } else {
            value
        }
    }

    /// The square root of a non-negative value.
    pub(super) fn sqrt(&self) -> Self {
        Self(BigInt::from((self.0.magnitude() << FRAC).sqrt()))
    }

    /// The natural logarithm of a positive value; zero and negatives give zero.
    pub(super) fn ln(&self) -> Self {
        if self.0.sign() != Sign::Plus {
            return Self::zero();
        }
        // self = m * 2^e with m in [1, 2): ln self = e ln 2 + 2 atanh((m - 1) / (m + 1)).
        let e = i64::try_from(self.0.bits()).unwrap_or(i64::MAX) - 1 - i64::from(FRAC);
        let m = match u64::try_from(e) {
            Ok(right) => Self(&self.0 >> right),
            Err(_) => Self(&self.0 << e.unsigned_abs()),
        };
        let z = (m.clone() - Self::one()) / (m + Self::one());
        let ln_m = atanh(&z) * 2;
        match u64::try_from(e) {
            Ok(up) => ln_m + ln2() * up,
            Err(_) => ln_m - ln2() * e.unsigned_abs(),
        }
    }

    /// Rounded to `digits` significant decimal digits, ties to even: the
    /// `accuracy` rounding of `Math::BigFloat`.
    fn round_sig(&self, keep: u32) -> Self {
        if self.is_zero() {
            return self.clone();
        }
        let magnitude = self.abs();
        // The leading digit sits at 10^position.
        let ten = Self::from_u64(10);
        let mut position = 0_i64;
        if magnitude >= Self::one() {
            position = i64::from(digits(&magnitude.floor_uint())) - 1;
        } else {
            let mut scaled = magnitude.clone();
            while scaled < Self::one() {
                scaled = scaled * ten.clone();
                position -= 1;
            }
        }
        let unit = position + 1 - i64::from(keep);
        let rounded = if let Ok(up) = u32::try_from(unit) {
            let unit = Self::from_biguint(&pow10(up));
            round_half_even(&(magnitude / unit.clone())) * unit
        } else {
            let scale = Self::from_biguint(&pow10(u32::try_from(-unit).unwrap_or(u32::MAX)));
            round_half_even(&(magnitude * scale.clone())) / scale
        };
        if self.0.sign() == Sign::Minus {
            -rounded
        } else {
            rounded
        }
    }
}

/// The nearest integer, ties to even, as an integer-valued [`Real`].
fn round_half_even(value: &Real) -> Real {
    let floor = value.floor();
    let fraction = &value.0 - (&floor << FRAC);
    let half = BigInt::from(1) << (FRAC - 1);
    let up = fraction > half || (fraction == half && floor.bit(0));
    Real((floor + u8::from(up)) << FRAC)
}

/// `atanh(z)` for `|z| < 1` by its series `z + z^3/3 + z^5/5 + ...`.
fn atanh(z: &Real) -> Real {
    let z2 = z.clone() * z.clone();
    let mut term = z.clone();
    let mut sum = z.clone();
    let mut k = 3;
    loop {
        term = term * z2.clone();
        let next = term.clone() / k;
        if next.is_zero() {
            return sum;
        }
        sum = sum + next;
        k += 2;
    }
}

/// `ln 2`, as `2 atanh(1/3)`.
fn ln2() -> Real {
    static LN2: OnceLock<Real> = OnceLock::new();
    LN2.get_or_init(|| atanh(&(Real::one() / Real::from_u64(3))) * 2)
        .clone()
}

impl Add for Real {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl Sub for Real {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

impl Mul for Real {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self((self.0 * other.0) >> FRAC)
    }
}

impl Mul<u64> for Real {
    type Output = Self;
    fn mul(self, k: u64) -> Self {
        Self(self.0 * k)
    }
}

impl Div for Real {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self((self.0 << FRAC) / other.0)
    }
}

impl Div<u64> for Real {
    type Output = Self;
    fn div(self, k: u64) -> Self {
        Self(self.0 / k)
    }
}

impl Neg for Real {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0)
    }
}

/// `n` when the native `n`th-prime functions answer for it.
fn native_index(n: &BigUint) -> Option<u64> {
    n.to_u64().filter(|&n| n <= MAX_PRIME_IDX)
}

/// `n`, `ln n` and `ln ln n` for an index above the native range.
fn index_logs(n: &BigUint) -> (Real, Real, Real) {
    let fnn = Real::from_biguint(n);
    let flogn = fnn.ln();
    let flog2n = flogn.ln();
    (fnn, flogn, flog2n)
}

/// The Axler form of `PP.pm` `nth_prime_lower` and `nth_prime_upper`:
/// `n (ln n + ln ln n - 1 + (ln ln n - 2) / ln n - (ln² ln n - 6 ln ln n + c) / (2 ln² n))`.
fn axler_nth(n: &BigUint, c: &str) -> Real {
    let (fnn, flogn, flog2n) = index_logs(n);
    let two = Real::from_u64(2);
    let six = Real::from_u64(6);
    fnn * (flogn.clone() + flog2n.clone() - Real::one()
        + (flog2n.clone() - two.clone()) / flogn.clone()
        - (flog2n.clone() * flog2n.clone() - six * flog2n + Real::decimal(c))
            / (two * flogn.clone() * flogn))
}

/// Return the `n`th prime, with the first prime as index 1.
///
/// Up to index `425_656_284_035_217_743` the native
/// [`nth_prime`](crate::nth_prime) answers. Above it the `n`th prime is above
/// `u64::MAX`; MPU's pure Perl `nth_prime` sieves from a bound there and does
/// not return in reasonable time, so this port walks
/// [`next_prime`] from the largest 64-bit
/// prime: the cost is linear in `n - 425_656_284_035_217_743`, with a
/// probable prime test per candidate.
///
/// Returns `None` for `n = 0`, where MPU returns `undef`.
///
/// ```
/// use math_prime_util::bigint::counts::nth_prime;
/// use num_bigint::BigUint;
/// assert_eq!(nth_prime(&BigUint::from(100_u8)), Some(BigUint::from(541_u16)));
/// let above = BigUint::from(425_656_284_035_217_746_u64);
/// assert_eq!(nth_prime(&above), Some((BigUint::from(1_u8) << 64_usize) + 51_u8));
/// assert_eq!(nth_prime(&BigUint::from(0_u8)), None);
/// ```
#[must_use]
pub fn nth_prime(n: &BigUint) -> Option<BigUint> {
    if let Some(small) = native_index(n) {
        return crate::nth_prime(small).map(BigUint::from);
    }
    let mut prime = BigUint::from(MAX_PRIME);
    let mut left = n - BigUint::from(MAX_PRIME_IDX);
    while left.bits() != 0 {
        prime = next_prime(&prime);
        left -= 1_u8;
    }
    Some(prime)
}

/// Return a lower bound on the `n`th prime, with the first prime as index 1.
///
/// Up to index `425_656_284_035_217_743` the native
/// [`nth_prime_lower`](crate::nth_prime_lower) answers. Above it this is
/// `PP.pm` `nth_prime_lower`: the Axler 2017 Corollary 1.4 bound with
/// `b = 11.508`, `n (ln n + ln ln n - 1 + (ln ln n - 2) / ln n - (ln² ln n - 6 ln ln n + b) / (2 ln² n))`,
/// as `int(x + 0.999999999)`, never below the largest 64-bit prime.
///
/// Returns `None` for `n = 0`, where MPU returns `undef`.
///
/// ```
/// use math_prime_util::bigint::counts::nth_prime_lower;
/// use num_bigint::BigUint;
/// let two64 = BigUint::from(1_u8) << 64_usize;
/// assert_eq!(nth_prime_lower(&two64).map(|p| p.to_string()), Some("870564512516096126115".into()));
/// assert_eq!(nth_prime_lower(&BigUint::from(10_000_u16)), Some(BigUint::from(104_540_u32)));
/// ```
#[must_use]
pub fn nth_prime_lower(n: &BigUint) -> Option<BigUint> {
    if let Some(small) = native_index(n) {
        return crate::nth_prime_lower(small).map(BigUint::from);
    }
    let lower = axler_nth(n, "11.508") + Real::decimal("0.999999999");
    Some(lower.floor_uint().max(BigUint::from(MAX_PRIME)))
}

/// Return an upper bound on the `n`th prime, with the first prime as index 1.
///
/// Up to index `425_656_284_035_217_743` the native
/// [`nth_prime_upper`](crate::nth_prime_upper) answers. Above it this is
/// `PP.pm` `nth_prime_upper`: the Axler 2017 Corollary 1.2 bound with
/// `d = 10.667`, `n (ln n + ln ln n - 1 + (ln ln n - 2) / ln n - (ln² ln n - 6 ln ln n + d) / (2 ln² n))`,
/// as `int(x + 1)`.
///
/// Returns `None` for `n = 0`, where MPU returns `undef`.
///
/// ```
/// use math_prime_util::bigint::counts::nth_prime_upper;
/// use num_bigint::BigUint;
/// let two64 = BigUint::from(1_u8) << 64_usize;
/// assert_eq!(nth_prime_upper(&two64).map(|p| p.to_string()), Some("870568454137000666246".into()));
/// assert_eq!(nth_prime_upper(&BigUint::from(10_000_u16)), Some(BigUint::from(104_925_u32)));
/// ```
#[must_use]
pub fn nth_prime_upper(n: &BigUint) -> Option<BigUint> {
    if let Some(small) = native_index(n) {
        return crate::nth_prime_upper(small).map(BigUint::from);
    }
    Some(axler_nth(n, "10.667").floor_uint() + 1_u8)
}

/// Return an estimate of the `n`th prime, with the first prime as index 1.
///
/// Up to index `425_656_284_035_217_743` the native
/// [`nth_prime_approx`](crate::nth_prime_approx) answers. Above it this is
/// `PP.pm` `nth_prime_approx` past `10^12`: the inverse of the logarithmic
/// integral at the `double` nearest `n`, found by Halley's method, plus one
/// half, truncated. On an XS build MPU evaluates that inverse in x87
/// `long double` (`real.c`, `ld_inverse_li`) and rounds it to a `double`;
/// this port follows that model to the last digit up to `10^35`, and from
/// there on inverts the integral in 256-bit fixed point and returns the
/// nearest `double`. (The pure Perl build iterates in `double` and keeps 15
/// significant digits of the result; see `docs/upstream-findings.md`.)
///
/// Returns `None` for `n = 0`, where MPU returns `undef`, and for `n` of
/// `2^1024` and more, where MPU dies as the value has no `double`.
///
/// ```
/// use math_prime_util::bigint::counts::nth_prime_approx;
/// use num_bigint::BigUint;
/// let above = BigUint::from(425_656_284_035_217_744_u64);
/// assert_eq!(nth_prime_approx(&above).map(|p| p.to_string()), Some("18446744070138417152".into()));
/// assert_eq!(nth_prime_approx(&BigUint::from(1_000_u16)), Some(BigUint::from(7_923_u16)));
/// ```
#[must_use]
pub fn nth_prime_approx(n: &BigUint) -> Option<BigUint> {
    if let Some(small) = native_index(n) {
        return crate::nth_prime_approx(small).map(BigUint::from);
    }
    let x: f64 = n.to_string().parse().ok().filter(|x: &f64| x.is_finite())?;
    let estimate = if x < 1e35 {
        inverse_li_ld(Ld::from_f64(x)).to_f64()
    } else {
        inverse_li_real(&Real::from_biguint(&f64_to_biguint(x))).to_f64()
    };
    Some(f64_to_biguint((estimate + 0.5).floor()))
}

/// MPU 0.75 `real.c`, `ld_inverse_li`, for `lx >= 2.719`: at most four
/// Halley steps on `li(t) = lx`, stopped when the step grows.
fn inverse_li_ld(lx: Ld) -> Ld {
    let two = Ld::from_u64(2);
    let l = lx.ln();
    let mut t = lx * (l + l.ln());
    let mut old_term = Ld::ZERO;
    for i in 0..4 {
        let dn = li_ld(t) - lx;
        let term = dn * t.ln() / (Ld::ONE + dn / (two * t));
        if i > 0 && term.abs() >= old_term.abs() {
            break;
        }
        old_term = term;
        t = t - term;
    }
    t
}

/// The `t` with `li(t) = x` for `x > 3` by Halley's method in [`Real`].
pub(super) fn inverse_li_real(x: &Real) -> Real {
    let two = Real::from_u64(2);
    let l = x.ln();
    let mut t = x.clone() * (l.clone() + l.ln());
    let mut old_term = Real::zero();
    for i in 0..40 {
        let dn = li_real(&t) - x.clone();
        let term = dn.clone() * t.ln() / (Real::one() + dn / (two.clone() * t.clone()));
        if i > 0 && term.abs() >= old_term.abs() {
            break;
        }
        old_term = term.clone();
        t = t - term;
    }
    t
}

/// The logarithmic integral of `x > 1` in [`Real`]: `Ei(ln x)` by the
/// convergent series `gamma + ln y + sum of y^k / (k k!)`.
pub(super) fn li_real(x: &Real) -> Real {
    let y = x.ln();
    let mut power = Real::one();
    let mut sum = Real::zero();
    for k in 1..=4000 {
        power = power * y.clone() / k;
        let term = power.clone() / k;
        if term.is_zero() {
            break;
        }
        sum = sum + term;
    }
    Real::decimal(EULER) + y.ln() + sum
}

/// A non-negative finite `double` truncated to an integer.
pub(super) fn f64_to_biguint(value: f64) -> BigUint {
    if value < 1.0 {
        return zero();
    }
    let bits = value.to_bits();
    let exponent = i64::try_from((bits >> 52) & 0x7ff).unwrap_or(0) - 1075;
    let mantissa = BigUint::from((bits & ((1 << 52) - 1)) | (1 << 52));
    match u64::try_from(exponent) {
        Ok(up) => mantissa << up,
        Err(_) => mantissa >> exponent.unsigned_abs(),
    }
}

/// `PP.pm` `LogarithmicIntegral` of a `Math::BigFloat` above `2^53` with
/// `accuracy` digits set (`DIV_SCALE` when none is): above `10^16` the
/// asymptotic series `x / ln x * sum of k! / ln^k x` up to its smallest term,
/// less a third of that term, below it the convergent series. The result
/// carries `max(accuracy, digits of x)` significant digits.
fn li_pp(x: &Real, accuracy: u32) -> Real {
    let lnx = x.ln();
    let final_digits = accuracy.max(digits(&x.floor_uint()));
    let work_digits = final_digits + digits(&lnx.floor_uint()) + 1;
    let tolerance = Real::one() / Real::from_biguint(&pow10(work_digits));
    let value = if *x > Real::from_biguint(&pow10(16)) {
        let invx = Real::one() / lnx;
        let mut term = invx.clone();
        let mut sum = Real::one() + term.clone();
        for k in 2..=1000 {
            let last_term = term.clone();
            term = term * (invx.clone() * k);
            if term < tolerance {
                break;
            }
            if term < last_term {
                sum = sum + term.clone();
            } else {
                sum = sum - last_term / 3;
                break;
            }
        }
        invx * sum * x.clone()
    } else {
        let mut factorial = Real::one();
        let mut sum = Real::zero();
        for k in 1..=200 {
            factorial = factorial * (lnx.clone() / k);
            let term = factorial.clone() / k;
            sum = sum + term.clone();
            if term < tolerance {
                break;
            }
        }
        Real::decimal(EULER) + lnx.ln() + sum
    };
    value.round_sig(final_digits)
}

/// `PP.pm` `LogarithmicIntegral` of a native `x` in `2..=2^53`: the
/// convergent series in `double`.
fn li_pp_f64(x: f64) -> f64 {
    // The exact value 2 is the special case of the Perl code.
    #[allow(clippy::float_cmp)]
    if x == 2.0 {
        return LI2.parse().unwrap_or(f64::NAN);
    }
    let logx = x.ln();
    let mut factorial = 1.0;
    let mut sum = 0.0;
    for k in 1..=200 {
        let k = f64::from(k);
        factorial *= logx / k;
        let term = factorial / k;
        sum += term;
        if term < 1e-16 {
            break;
        }
    }
    EULER.parse().unwrap_or(f64::NAN) + logx.ln() + sum
}

/// `x`, `ln x`, `li(x)` with `DIV_SCALE` digits, and `sqrt x` for a count
/// bound above the native range.
fn count_terms(x: &BigUint) -> (Real, Real, Real, Real) {
    let fx = Real::from_biguint(x);
    let fl1 = fx.ln();
    let lix = li_pp(&fx, DIV_SCALE);
    let sqx = fx.sqrt();
    (fx, fl1, lix, sqx)
}

/// `8 pi`, as `Math::BigFloat->bpi(40) * 8` in MPU.
fn pi8() -> Real {
    Real::decimal(PI) * 8
}

/// The Axler 2022 form of `PP.pm` `prime_count_lower` and
/// `prime_count_upper` above `1.101e26`: `x / ln x` times a polynomial in
/// `1 / ln x` with the given coefficients.
fn axler_count(x: &Real, fl1: &Real, coefficients: &[&str]) -> Real {
    let r = Real::one() / fl1.clone();
    let mut sum = Real::zero();
    for c in coefficients.iter().rev() {
        sum = Real::decimal(c) + r.clone() * sum;
    }
    x.clone() * r * sum
}

/// `10^20` and `1.101e26`, the branch limits of the pure Perl count bounds.
fn count_limits() -> (BigUint, BigUint) {
    (pow10(20), BigUint::from(1101_u16) * pow10(23))
}

/// Lower bound for pi(n), the number of primes up to `n`.
///
/// Up to `u64::MAX` the native [`prime_count_lower`](crate::prime_count_lower)
/// answers. Above it this is `PP.pm` `prime_count_lower`: Dusart 2018,
/// `li(x) - 2 sqrt(x) / ln x`, up to `10^20`; Johnston 2021 Corollary 3.3,
/// `li(x) - ln x sqrt(x) / (8 pi)`, below `1.101e26`; Axler 2022 Corollary
/// 5.5 from there; each truncated to an integer. `li` is the asymptotic
/// series of `PP.pm` (see [`prime_count_approx`]).
///
/// ```
/// use math_prime_util::bigint::counts::prime_count_lower;
/// use num_bigint::BigUint;
/// let two64 = BigUint::from(1_u8) << 64_usize;
/// assert_eq!(prime_count_lower(&two64), BigUint::from(425_656_283_922_083_364_u64));
/// assert!(prime_count_lower(&BigUint::from(1_000_000_u32)) <= BigUint::from(78_498_u32));
/// ```
#[must_use]
pub fn prime_count_lower(n: &BigUint) -> BigUint {
    if let Some(small) = n.to_u64() {
        return BigUint::from(crate::prime_count_lower(small));
    }
    let (fx, fl1, lix, sqx) = count_terms(n);
    let (limit20, limit26) = count_limits();
    let lower = if *n <= limit20 {
        lix - sqx * 2 / fl1
    } else if *n < limit26 {
        lix - fl1 * sqx / pi8()
    } else {
        let coefficients = [
            "1",
            "1",
            "2",
            "5.975666",
            "23.975666",
            "119.87833",
            "719.26998",
            "5034.88986",
        ];
        axler_count(&fx, &fl1, &coefficients)
    };
    lower.floor_uint()
}

/// Upper bound for pi(n), the number of primes up to `n`.
///
/// Up to `u64::MAX` the native [`prime_count_upper`](crate::prime_count_upper)
/// answers. Above it this is `PP.pm` `prime_count_upper`: `li(x)` below
/// `10^20` (Büthe 2015 and Dusart 2018); Johnston 2021 Corollary 3.3,
/// `li(x) + ln x sqrt(x) / (8 pi)`, below `1.101e26`; Axler 2022 Proposition
/// 4.6 from there; each truncated to an integer. `li` is the asymptotic
/// series of `PP.pm` (see [`prime_count_approx`]).
///
/// ```
/// use math_prime_util::bigint::counts::prime_count_upper;
/// use num_bigint::BigUint;
/// let two64 = BigUint::from(1_u8) << 64_usize;
/// assert_eq!(prime_count_upper(&two64), BigUint::from(425_656_284_115_718_615_u64));
/// assert!(prime_count_upper(&BigUint::from(1_000_000_u32)) >= BigUint::from(78_498_u32));
/// ```
#[must_use]
pub fn prime_count_upper(n: &BigUint) -> BigUint {
    if let Some(small) = n.to_u64() {
        return BigUint::from(crate::prime_count_upper(small));
    }
    let (fx, fl1, lix, sqx) = count_terms(n);
    let (limit20, limit26) = count_limits();
    let upper = if *n < limit20 {
        lix
    } else if *n < limit26 {
        lix + fl1 * sqx / pi8()
    } else {
        let coefficients = [
            "1",
            "1",
            "2",
            "6.024334",
            "24.024334",
            "120.12167",
            "720.73002",
            "6098",
        ];
        axler_count(&fx, &fl1, &coefficients)
    };
    upper.floor_uint()
}

/// Approximate pi(n), the number of primes up to `n`.
///
/// Up to `u64::MAX` the native
/// [`prime_count_approx`](crate::prime_count_approx) answers. Above it this
/// is `PP.pm` `prime_count_approx` on a `Math::BigFloat`: Riemann's R
/// function as `li(x) - li(sqrt x) / 2 + sum of mu(k) / k li(root_k(x))` over
/// `k >= 3`, rounded to the nearest integer, with these details of the Perl
/// code kept so that the results agree to the last digit:
///
/// - the roots are the integer roots, and the sum stops at the first term
///   below `0.001` or at a root of 1;
/// - `li` of a root up to `2^53` is the `double` series of `PP.pm`, and its
///   value enters the sum with 15 significant digits;
/// - `li` of `x` and of larger roots is the asymptotic series of `PP.pm`
///   (up to its smallest term, less a third of it), rounded to two digits
///   more than `x` has;
/// - the terms are summed from the smallest, each sum rounded to that
///   accuracy.
///
/// ```
/// use math_prime_util::bigint::counts::prime_count_approx;
/// use num_bigint::BigUint;
/// let two64 = BigUint::from(1_u8) << 64_usize;
/// assert_eq!(prime_count_approx(&two64), BigUint::from(425_656_284_014_012_123_u64));
/// assert_eq!(prime_count_approx(&BigUint::from(10_000_000_000_u64)), BigUint::from(455_050_683_u32));
/// ```
#[must_use]
pub fn prime_count_approx(n: &BigUint) -> BigUint {
    if let Some(small) = n.to_u64() {
        return BigUint::from(crate::prime_count_approx(small));
    }
    let accuracy = digits(n) + 2;
    let mut terms = vec![li_pp(&Real::from_biguint(n), accuracy)];
    let square_root = sqrtint(n);
    terms.push(match root_li(&square_root) {
        RootLi::Native(li) => Real::from_f64_15(-li / 2.0),
        RootLi::Big(li) => (-li / 2).round_sig(digits(&square_root) + 5),
    });
    for k in 3..=2000 {
        let mu = crate::moebius(k);
        if mu == 0 {
            continue;
        }
        let root = n.nth_root(u32::try_from(k).unwrap_or(u32::MAX));
        if root <= one() {
            break;
        }
        let li = match root_li(&root) {
            RootLi::Native(li) => Real::from_f64_15(li),
            RootLi::Big(li) => li,
        };
        let term = (li / k).round_sig(accuracy);
        if term < Real::decimal("0.001") {
            break;
        }
        terms.push(if mu == 1 { term } else { -term });
    }
    let mut result = Real::zero();
    for term in terms.into_iter().rev() {
        result = (result + term).round_sig(accuracy);
    }
    (result + Real::decimal("0.5"))
        .round_sig(accuracy)
        .floor_uint()
}

/// `li` of an integer root in [`prime_count_approx`]: a `double` up to
/// `2^53`, a `Math::BigFloat` with five digits more than the root above.
enum RootLi {
    Native(f64),
    Big(Real),
}

fn root_li(root: &BigUint) -> RootLi {
    match root.to_u64().filter(|&r| r <= 1 << 53) {
        // Below 2^53 the conversion is exact.
        #[allow(clippy::cast_precision_loss)]
        Some(small) => RootLi::Native(li_pp_f64(small as f64)),
        None => RootLi::Big(li_pp(&Real::from_biguint(root), digits(root) + 5)),
    }
}

/// Create an iterator at the first prime at least `start`.
///
/// `Util.pm` `prime_iterator`: the native sieve segments below `u64::MAX`,
/// then [`next_prime`] on each value, so the
/// stream never ends. A start of `0` is the same as a start of `2`.
///
/// ```
/// use math_prime_util::bigint::counts::prime_iterator;
/// use num_bigint::BigUint;
/// let mut primes = prime_iterator(&BigUint::from(u64::MAX - 58));
/// assert_eq!(primes.next(), Some(BigUint::from(u64::MAX - 58)));
/// assert_eq!(primes.next(), Some((BigUint::from(1_u8) << 64_usize) + 13_u8));
/// ```
#[must_use]
pub fn prime_iterator(start: &BigUint) -> BigPrimeIterator {
    BigPrimeIterator::new(start)
}

/// Create an owned prime iterator: the same stream as [`prime_iterator`].
///
/// `Util.pm` `prime_iterator_object` returns a
/// `Math::Prime::Util::PrimeIterator` object.
///
/// ```
/// use math_prime_util::bigint::counts::prime_iterator_object;
/// use num_bigint::BigUint;
/// let first = prime_iterator_object(&(BigUint::from(10_u8).pow(20))).next();
/// assert_eq!(first.map(|p| p.to_string()), Some("100000000000000000039".into()));
/// ```
#[must_use]
pub fn prime_iterator_object(start: &BigUint) -> BigPrimeIterator {
    prime_iterator(start)
}

/// Visit the primes in an inclusive range, in order.
///
/// `PP.pm` `forprimes` on bigint bounds: each prime is the
/// [`next_prime`] of the one before. The
/// callback stops the loop with `ControlFlow::Break`, as `lastfor` does.
///
/// ```
/// use math_prime_util::bigint::counts::forprimes;
/// use num_bigint::BigUint;
/// use std::ops::ControlFlow;
/// let two64 = BigUint::from(1_u8) << 64_usize;
/// let mut seen = Vec::new();
/// let _: ControlFlow<()> = forprimes(&two64 - 60_u8..=&two64 + 20_u8, |p| {
///     seen.push(p + 60_u8 - &two64);
///     ControlFlow::Continue(())
/// });
/// assert_eq!(seen, [1_u8, 73].map(BigUint::from));
/// ```
pub fn forprimes<B>(
    range: RangeInclusive<BigUint>,
    f: impl FnMut(BigUint) -> ControlFlow<B>,
) -> ControlFlow<B> {
    let (lo, hi) = range.into_inner();
    prime_iterator(&lo).take_while(|p| *p <= hi).try_for_each(f)
}

/// The composite loops of `PP.pm` `_forcomp_sub`: every value from `lo` in
/// steps of `step` up to `hi` that is not prime.
fn composites<B>(
    lo: BigUint,
    hi: &BigUint,
    step: u8,
    mut f: impl FnMut(BigUint) -> ControlFlow<B>,
) -> ControlFlow<B> {
    let mut n = lo;
    while n <= *hi {
        if is_prime(&n) == 0 {
            f(n.clone())?;
        }
        n += step;
    }
    ControlFlow::Continue(())
}

/// Visit the composites in an inclusive range, in order, starting at 4.
///
/// `PP.pm` `forcomposites`: each value that is not prime, with
/// [`is_prime`] (BPSW above `u64::MAX`).
///
/// ```
/// use math_prime_util::bigint::counts::forcomposites;
/// use num_bigint::BigUint;
/// use std::ops::ControlFlow;
/// let two64 = BigUint::from(1_u8) << 64_usize;
/// let mut count = 0;
/// let _: ControlFlow<()> = forcomposites(two64.clone()..=&two64 + 13_u8, |_| {
///     count += 1;
///     ControlFlow::Continue(())
/// });
/// assert_eq!(count, 13);
/// ```
pub fn forcomposites<B>(
    range: RangeInclusive<BigUint>,
    f: impl FnMut(BigUint) -> ControlFlow<B>,
) -> ControlFlow<B> {
    let (lo, hi) = range.into_inner();
    composites(lo.max(BigUint::from(4_u8)), &hi, 1, f)
}

/// Visit the odd composites in an inclusive range, in order, starting at 9.
///
/// `PP.pm` `foroddcomposites`: each odd value that is not prime, with
/// [`is_prime`] (BPSW above `u64::MAX`).
///
/// ```
/// use math_prime_util::bigint::counts::foroddcomposites;
/// use num_bigint::BigUint;
/// use std::ops::ControlFlow;
/// let two64 = BigUint::from(1_u8) << 64_usize;
/// let mut odd = Vec::new();
/// let _: ControlFlow<()> = foroddcomposites(two64.clone()..=&two64 + 15_u8, |n| {
///     odd.push(n - &two64);
///     ControlFlow::Continue(())
/// });
/// assert_eq!(odd, [1_u8, 3, 5, 7, 9, 11, 15].map(BigUint::from));
/// ```
pub fn foroddcomposites<B>(
    range: RangeInclusive<BigUint>,
    f: impl FnMut(BigUint) -> ControlFlow<B>,
) -> ControlFlow<B> {
    let (lo, hi) = range.into_inner();
    let mut lo = lo.max(BigUint::from(9_u8));
    if lo.is_even() {
        lo += 1_u8;
    }
    composites(lo, &hi, 2, f)
}

/// Test whether `n` is a polygonal number with `k` sides.
///
/// Below `u64::MAX` the native [`is_polygonal`](crate::is_polygonal)
/// answers. Above it this is `PP.pm` `is_polygonal` in integer arithmetic:
/// the discriminant `(8k - 16) n + (k - 4)²` must be a square whose root
/// plus `k - 4` is a multiple of `2k - 4`.
///
/// # Panics
/// Panics if `k < 3`, where MPU dies.
///
/// ```
/// use math_prime_util::bigint::counts::is_polygonal;
/// use num_bigint::BigUint;
/// let two64 = BigUint::from(1_u8) << 64_usize;
/// // The 2^64-th triangular number.
/// let triangle = &two64 * (&two64 + 1_u8) / 2_u8;
/// assert!(is_polygonal(&triangle, &BigUint::from(3_u8)));
/// assert!(!is_polygonal(&(&triangle + 1_u8), &BigUint::from(3_u8)));
/// assert!(is_polygonal(&two64, &two64));
/// ```
#[must_use]
pub fn is_polygonal(n: &BigUint, k: &BigUint) -> bool {
    assert!(*k >= BigUint::from(3_u8), "is_polygonal root < 3");
    if let (Some(n), Some(k)) = (n.to_u64(), k.to_u64()) {
        return crate::is_polygonal(n, k);
    }
    if *n <= one() {
        return true;
    }
    if *k == BigUint::from(4_u8) {
        return is_square(n);
    }
    if k > n {
        return false;
    }
    let four = BigUint::from(4_u8);
    let offset = if *k >= four { k - &four } else { &four - k };
    let discriminant = (k * 8_u8 - 16_u8) * n + &offset * &offset;
    if !is_square(&discriminant) {
        return false;
    }
    let numerator = sqrtint(&discriminant) + k - four;
    (numerator % (k * 2_u8 - 4_u8)).bits() == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn big(text: &str) -> BigUint {
        BigUint::parse_bytes(text.as_bytes(), 10).expect("decimal")
    }

    #[test]
    fn real_arithmetic() {
        let third = Real::one() / Real::from_u64(3);
        assert_eq!((third.clone() * 3).floor(), BigInt::from(0));
        assert_eq!((third * 3 + Real::decimal("0.5")).floor_uint(), one());
        assert_eq!(Real::decimal("10.667").floor(), BigInt::from(10));
        assert_eq!(Real::decimal("12").floor(), BigInt::from(12));
        assert_eq!((-Real::decimal("0.5")).floor(), BigInt::from(-1));
        assert_eq!((-Real::decimal("0.5")).floor_uint(), zero());
        assert_eq!((-Real::one()).abs(), Real::one());
        assert!(Real::zero().is_zero());
        assert_eq!(Real::from_u64(49).sqrt(), Real::from_u64(7));
        assert_eq!(Real::decimal("0.25").sqrt(), Real::decimal("0.5"));
    }

    #[test]
    fn real_logarithms() {
        // ln 2 to 60 digits.
        let ln2_digits =
            Real::decimal("0.693147180559945309417232121458176568075500134360255254120680");
        let close = |a: Real, b: Real| {
            (a - b).abs()
                < Real::decimal("0.000000000000000000000000000000000000000000000000000000001")
        };
        assert!(close(ln2(), ln2_digits));
        assert!(close(Real::from_u64(1).ln(), Real::zero()));
        assert!(close(Real::from_u64(8).ln(), ln2() * 3));
        assert!(close(Real::decimal("0.125").ln(), -(ln2() * 3)));
        assert!(close(
            Real::decimal("2.718281828459045235360287471352662497757247093699959574966968").ln(),
            Real::one()
        ));
        assert_eq!(Real::zero().ln(), Real::zero());
        assert_eq!((-Real::one()).ln(), Real::zero());
    }

    #[test]
    fn real_rounding() {
        let r = |t: &str| Real::decimal(t);
        assert_eq!(r("1234.5678").round_sig(6), r("1234.57"));
        assert_eq!(r("1234.5678").round_sig(2), r("1200"));
        assert_eq!(r("1250").round_sig(2), r("1200"));
        assert_eq!(r("1350").round_sig(2), r("1400"));
        assert_eq!(r("0.0012345").round_sig(3), r("0.00123"));
        assert_eq!((-r("1234.5678")).round_sig(6), -r("1234.57"));
        assert_eq!(Real::zero().round_sig(6), Real::zero());
        assert_eq!(round_half_even(&r("2.5")), r("2"));
        assert_eq!(round_half_even(&r("3.5")), r("4"));
        assert_eq!(round_half_even(&r("3.75")), r("4"));
    }

    #[test]
    #[allow(clippy::float_cmp)] // The doubles are exact.
    fn real_and_double_conversions() {
        assert_eq!(Real::from_f64_15(0.1 + 0.2), Real::decimal("0.3"));
        assert_eq!(
            Real::from_f64_15(18_446_744_073_709_551_616.0),
            Real::from_biguint(&big("18446744073709600000"))
        );
        assert_eq!(Real::from_f64_15(-2.5), -Real::decimal("2.5"));
        assert_eq!(Real::from_f64_15(0.0), Real::zero());
        assert_eq!(Real::decimal("2.5").to_f64(), 2.5);
        assert_eq!((-Real::decimal("2.5")).to_f64(), -2.5);
        assert_eq!(Real::zero().to_f64(), 0.0);
        // Ties go to even: 2^53 + 1 rounds down, 2^53 + 3 rounds up.
        assert_eq!(
            Real::from_u64((1 << 53) + 1).to_f64(),
            9_007_199_254_740_992.0
        );
        assert_eq!(
            Real::from_u64((1 << 53) + 3).to_f64(),
            9_007_199_254_740_996.0
        );
        assert_eq!(
            Real::from_u64((1 << 53) + 2).to_f64(),
            9_007_199_254_740_994.0
        );
        assert_eq!(f64_to_biguint(0.5), zero());
        assert_eq!(f64_to_biguint(3.9), BigUint::from(3_u8));
        assert_eq!(f64_to_biguint(2_f64.powi(70)), one() << 70_usize);
        assert_eq!(
            f64_to_biguint(18_446_744_070_138_417_152.0),
            big("18446744070138417152")
        );
    }

    #[test]
    fn logarithmic_integrals() {
        // li(10^6) = 78627.549..., li(10^10) = 455055614.586...
        assert!((li_pp_f64(1e6) - 78_627.549_159_6).abs() < 1e-6);
        assert!((li_pp_f64(1e10) - 455_055_614.586_6).abs() < 1e-3);
        assert!((li_pp_f64(2.0) - 1.045_163_780_117_493).abs() < 1e-15);
        let million = Real::from_u64(1_000_000);
        assert_eq!(li_real(&million).round_sig(9), Real::decimal("78627.5492"));
        assert_eq!(
            li_pp(&million, DIV_SCALE).round_sig(9),
            Real::decimal("78627.5492")
        );
        // The asymptotic series of PP: li(10^20) = 2220819602783663483.5... in exact
        // arithmetic; the series stops at its smallest term, close to that.
        let huge = Real::from_biguint(&pow10(20));
        let asymptotic = li_pp(&huge, DIV_SCALE);
        assert!((asymptotic - li_real(&huge)).abs() < Real::one());
        let inverse = inverse_li_real(&Real::from_u64(1_000_000));
        assert!(
            (li_real(&inverse) - Real::from_u64(1_000_000)).abs()
                < Real::decimal("0.000000000000000000000000000001")
        );
        assert_eq!(inverse.floor_uint() / 1000_u32, BigUint::from(15_479_u32));
    }

    #[test]
    fn nth_prime_walks_from_the_largest_native_prime() {
        assert_eq!(nth_prime(&zero()), None);
        assert_eq!(nth_prime(&BigUint::from(6_u8)), Some(BigUint::from(13_u8)));
        let next = nth_prime(&BigUint::from(MAX_PRIME_IDX + 1));
        assert_eq!(next, Some(big("18446744073709551629")));
    }

    #[test]
    fn bounds_above_the_native_index() {
        let n = BigUint::from(MAX_PRIME_IDX + 1);
        assert_eq!(nth_prime_lower(&n), Some(BigUint::from(MAX_PRIME)));
        assert_eq!(nth_prime_upper(&n), Some(big("18446794739597883110")));
        assert_eq!(nth_prime_approx(&n), Some(big("18446744070138417152")));
        assert_eq!(nth_prime_lower(&zero()), None);
        assert_eq!(nth_prime_upper(&zero()), None);
        assert_eq!(nth_prime_approx(&zero()), None);
        // The lower bound is monotone and below the upper bound.
        let huge = one() << 300_usize;
        let (lower, upper) = (nth_prime_lower(&huge), nth_prime_upper(&huge));
        assert!(lower < upper);
        assert!(nth_prime_lower(&(&huge + 1_u8)) >= lower);
        // Too large for a double: no estimate.
        assert_eq!(nth_prime_approx(&(one() << 1024_usize)), None);
        // From 10^35 the estimate comes from the exact inverse.
        let estimate = nth_prime_approx(&pow10(36)).expect("estimate");
        assert!(estimate > nth_prime_lower(&pow10(36)).expect("bound"));
        assert!(estimate < nth_prime_upper(&pow10(36)).expect("bound"));
    }

    #[test]
    fn count_bounds_bracket_the_estimate() {
        for x in [
            one() << 64_usize,
            pow10(20),
            pow10(25),
            pow10(30),
            one() << 150_usize,
        ] {
            let (lower, upper) = (prime_count_lower(&x), prime_count_upper(&x));
            let approx = prime_count_approx(&x);
            assert!(
                lower < approx && approx < upper,
                "{x}: {lower} {approx} {upper}"
            );
        }
        assert_eq!(
            prime_count_lower(&BigUint::from(u64::MAX)),
            BigUint::from(425_656_283_922_083_342_u64)
        );
    }

    #[test]
    fn loops_and_streams() {
        let two64 = one() << 64_usize;
        let mut primes = Vec::new();
        let flow = forprimes(&two64 - 60_u8..=&two64 + 60_u8, |p| {
            primes.push(p);
            if primes.len() == 3 {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        });
        assert_eq!(flow, ControlFlow::Break(()));
        assert_eq!(
            primes,
            [BigUint::from(u64::MAX - 58), &two64 + 13_u8, &two64 + 37_u8]
        );
        let mut composites_seen = 0;
        let flow: ControlFlow<()> = forcomposites(zero()..=BigUint::from(10_u8), |_| {
            composites_seen += 1;
            ControlFlow::Continue(())
        });
        assert_eq!((flow, composites_seen), (ControlFlow::Continue(()), 5));
        let mut odd = Vec::new();
        let _: ControlFlow<()> =
            foroddcomposites(BigUint::from(8_u8)..=BigUint::from(27_u8), |n| {
                odd.push(n);
                ControlFlow::Continue(())
            });
        assert_eq!(odd, [9_u8, 15, 21, 25, 27].map(BigUint::from));
        let flow = foroddcomposites(&two64 + 1_u8..=&two64 + 9_u8, |_| ControlFlow::Break(7));
        assert_eq!(flow, ControlFlow::Break(7));
        assert_eq!(
            prime_iterator_object(&zero()).next(),
            Some(BigUint::from(2_u8))
        );
    }

    #[test]
    fn polygonal_numbers_above_the_native_range() {
        let two64 = one() << 64_usize;
        let three = BigUint::from(3_u8);
        assert!(is_polygonal(&zero(), &two64));
        assert!(is_polygonal(&one(), &two64));
        assert!(!is_polygonal(&two64, &(&two64 + 1_u8)));
        assert!(is_polygonal(&(&two64 * &two64), &BigUint::from(4_u8)));
        assert!(!is_polygonal(
            &(&two64 * &two64 + 1_u8),
            &BigUint::from(4_u8)
        ));
        // The 7th k-gonal number for k = 2^64 + 1: 7 ((k - 2) 6 + 2) / 2 = 21 k - 35.
        let k = &two64 + 1_u8;
        assert!(is_polygonal(&(&k * 21_u8 - 35_u8), &k));
        assert!(!is_polygonal(&(&k * 21_u8 - 34_u8), &k));
        assert!(is_polygonal(&(&two64 * (&two64 + 1_u8) / 2_u8), &three));
        assert!(!is_polygonal(
            &(&two64 * (&two64 + 1_u8) / 2_u8 + 1_u8),
            &three
        ));
        // Between the 2nd and 3rd k-gonal numbers (k and 3k - 3).
        let k = &two64 + 2_u8;
        assert!(!is_polygonal(&(&k * 2_u8 - 2_u8), &k));
        // A generalized pentagonal number, r (3r - 1) / 2 with r = -2^64: the
        // discriminant is a square but its root has the wrong residue.
        let n = &two64 * (&two64 * 3_u8 + 1_u8) / 2_u8;
        assert!(!is_polygonal(&n, &BigUint::from(5_u8)));
    }

    #[test]
    #[should_panic(expected = "is_polygonal root < 3")]
    fn polygonal_rejects_small_k() {
        let _ = is_polygonal(&(one() << 70_usize), &BigUint::from(2_u8));
    }
}

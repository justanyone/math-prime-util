//! Software x87 `long double`, for the parts of MPU 0.75 that compute in it.
//!
//! MPU evaluates `nth_prime_upper` (`prime_counts.c`) and `inverse_R`,
//! `RiemannR` and `_Li` (`real.c`) in C `long double`. On x86-64 Linux, where
//! the oracle fixtures were made, that is the x87 extended format: a 64-bit
//! significand, each operation rounded to nearest with ties to even. `f64`
//! keeps 53 bits, which changes the last digits of these results above about
//! 10^13. [`Ld`] rounds each operation to 64 bits in the same way, so the port
//! gives MPU's numbers on every platform.
//!
//! [`Ld::ln`] models glibc's x86-64 `logl`, which is the x87 `fyl2x`
//! instruction: `log2(x)` times the `fldln2` constant (`ln 2` rounded to 64
//! bits). The instruction is not correctly rounded, so no closed rule gives
//! its every result. This model keeps 67 bits of `log2(x)`, toward zero, and
//! rounds the product once. Against `logl` on an Intel i9-10885H it gave the
//! same bits for 99.87% of 200,000 random arguments in `[0.25, 2^67)` and
//! 99.97% of those from 2 up; every other result was one unit in the last
//! place off. Against MPU's C on the same machine, `nth_prime_lower` and
//! `nth_prime_upper` agreed for every `n < 2,000,000`, and the three
//! `nth_prime_*` functions agreed for all but two of 46,271 sampled `n` up to
//! `MPU_MAX_PRIME_IDX` (both above 10^16, off by one).
//! [`Ld::pow`] stands in for glibc `powl` with a correctly rounded result.
//!
//! The exponent is an `i32`, so there is no overflow or underflow; the callers
//! stay far inside the x87 range. NaN and infinity do not occur.

use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// A `long double` value, `(-1)^neg * sig * 2^(exp - 63)`.
///
/// `sig` has bit 63 set, or is 0 for zero.
#[derive(Clone, Copy, Debug)]
pub(super) struct Ld {
    neg: bool,
    exp: i32,
    sig: u64,
}

/// Fraction bits of the fixed-point values inside [`Ld::ln`] and [`Ld::pow`].
const FRAC: u32 = 112;
/// One in that fixed point.
const ONE_FIX: u128 = 1 << FRAC;
/// `ln 2` in fixed point, rounded to nearest.
const LN2_FIX: u128 = 0xb172_17f7_d1cf_79ab_c9e3_b398_03f3;
/// `1 / ln 2` in fixed point, rounded to nearest.
const INV_LN2_FIX: u128 = 0x1_7154_7652_b82f_e177_7d0f_fda0_d23a;
/// The significand of the x87 `fldln2` constant, `ln 2` rounded to 64 bits:
/// the constant is `LN2_X87_SIG * 2^-64`.
const LN2_X87_SIG: u64 = 0xb172_17f7_d1cf_79ac;
/// Significant bits of `log2(x)` that the `fyl2x` model keeps.
const FYL2X_LOG2_BITS: u32 = 67;
/// `sqrt(2) * 2^63`, rounded down.
const SQRT2_SIG: u64 = 0xb504_f333_f9de_6484;

impl Ld {
    /// Zero.
    pub(super) const ZERO: Self = Self {
        neg: false,
        exp: 0,
        sig: 0,
    };
    /// One.
    pub(super) const ONE: Self = Self::new(0, 1 << 63);
    /// `LDBL_EPSILON`, `2^-63`.
    pub(super) const EPSILON: Self = Self::new(-63, 1 << 63);

    /// The positive value `sig * 2^(exp - 63)`. `sig` must have bit 63 set.
    pub(super) const fn new(exp: i32, sig: u64) -> Self {
        Self {
            neg: false,
            exp,
            sig,
        }
    }

    /// `n` exactly, as C converts a `UV` to `long double`.
    pub(super) fn from_u64(n: u64) -> Self {
        Self::round(false, u128::from(n), 0, false)
    }

    /// `x` exactly, as C converts a `double` to `long double`.
    ///
    /// `x` must be finite.
    pub(super) fn from_f64(x: f64) -> Self {
        let bits = x.to_bits();
        let neg = bits >> 63 == 1;
        let frac = bits & ((1 << 52) - 1);
        let biased = (bits >> 52) & 0x7ff;
        if biased == 0 {
            // Zero or subnormal: `frac * 2^-1074`.
            return Self::round(neg, u128::from(frac), -1074, false);
        }
        #[allow(clippy::cast_possible_truncation)]
        let exp = biased as i32 - 1023;
        Self {
            neg,
            exp,
            sig: (frac | 1 << 52) << 11,
        }
    }

    /// Round to binary64 precision, as the NV argument and result of
    /// MPU 0.75 `real.c`, `Li`. Callers stay in the normal binary64 range.
    pub(super) fn round_binary64(self) -> Self {
        let sig = self.sig >> 11;
        let tail = self.sig & 0x7ff;
        let up = tail > 0x400 || (tail == 0x400 && sig & 1 != 0);
        Self::round(
            self.neg,
            u128::from(sig + u64::from(up)),
            self.exp - 52,
            false,
        )
    }

    /// `fabsl`.
    pub(super) fn abs(self) -> Self {
        Self { neg: false, ..self }
    }

    /// `floorl` converted to `UV`. Negative values give 0; values from `2^64`
    /// up give `u64::MAX` (MPU checks for those before converting).
    pub(super) fn floor_u64(self) -> u64 {
        if self.neg || self.sig == 0 || self.exp < 0 {
            return 0;
        }
        if self.exp > 63 {
            return u64::MAX;
        }
        self.sig >> (63 - self.exp)
    }

    /// `ceill` converted to `UV`, saturating at `u64::MAX` as MPU's
    /// `inverse_R` does. Values up to 0 give 0.
    pub(super) fn ceil_u64(self) -> u64 {
        if self.neg || self.sig == 0 {
            return 0;
        }
        if self.exp < 0 {
            return 1;
        }
        if self.exp > 63 {
            return u64::MAX;
        }
        let shift = 63 - self.exp;
        let int = self.sig >> shift;
        let frac = self.sig & ((1 << shift) - 1);
        int + u64::from(frac != 0)
    }

    /// `sqrtl`, correctly rounded as the x87 `fsqrt` is. Values up to 0 give 0.
    pub(super) fn sqrt(self) -> Self {
        if self.neg || self.sig == 0 {
            return Self::ZERO;
        }
        // Make the power of two even: self = n * 2^e.
        let e = self.exp - 63;
        let (n, e) = if e.rem_euclid(2) == 0 {
            (u128::from(self.sig) << 64, e - 64)
        } else {
            (u128::from(self.sig) << 63, e - 63)
        };
        // n >= 2^126, so the root has 64 bits. It is never a tie: round up
        // when n > (r + 1/2)^2, that is when n - r^2 > r.
        let r = n.isqrt();
        let up = n - r * r > r;
        Self::round(false, r + u128::from(up), e / 2, false)
    }

    /// `logl` as glibc computes it on x86-64, the x87 `fyl2x` instruction:
    /// `log2(self)` cut to 67 bits, times `ln 2` rounded to 64 bits, rounded
    /// once to 64 bits (see the module documentation).
    ///
    /// `self` must be positive.
    pub(super) fn ln(self) -> Self {
        let (e, ln_m) = self.ln_parts();
        #[allow(clippy::cast_possible_wrap)]
        let log2 = i128::from(e) * ONE_FIX as i128 + signed_mul_fix(ln_m, INV_LN2_FIX);
        let mut mag = log2.unsigned_abs();
        if mag == 0 {
            return Self::ZERO;
        }
        let top = mag.ilog2();
        if top >= FYL2X_LOG2_BITS {
            mag &= !((1 << (top + 1 - FYL2X_LOG2_BITS)) - 1);
        }
        // log2 * LN2_X87_SIG * 2^-64, exactly, as a 256-bit product.
        let (hi, lo) = mul_wide(mag, u128::from(LN2_X87_SIG));
        #[allow(clippy::cast_possible_wrap)]
        let e = -(FRAC as i32) - 64;
        Self::round_wide(log2 < 0, hi, lo, e)
    }

    /// `self^y` for positive `self`, correctly rounded; stands in for glibc
    /// `powl`.
    pub(super) fn pow(self, y: Self) -> Self {
        let (e, ln_m) = self.ln_parts();
        #[allow(clippy::cast_possible_wrap)]
        let ln_x = i128::from(e) * LN2_FIX as i128 + ln_m;
        exp_fix(y.mul_fix(ln_x))
    }

    /// Splits positive `self` as `m * 2^e` with `m` in `[sqrt(1/2), sqrt(2))`
    /// and returns `e` and `ln m` in fixed point.
    fn ln_parts(self) -> (i32, i128) {
        let (e, m) = if self.sig > SQRT2_SIG {
            (self.exp + 1, u128::from(self.sig) << (FRAC - 64))
        } else {
            (self.exp, u128::from(self.sig) << (FRAC - 63))
        };
        // ln m = 2 atanh(s) with s = (m - 1) / (m + 1), |s| < 0.172.
        let (num, below_one) = if m >= ONE_FIX {
            (m - ONE_FIX, false)
        } else {
            (ONE_FIX - m, true)
        };
        let s = div_fix(num, m + ONE_FIX);
        let s2 = mul_fix(s, s);
        let mut term = s;
        let mut sum = s;
        let mut k = 3;
        while term != 0 {
            term = mul_fix(term, s2);
            sum += term / k;
            k += 2;
        }
        // sum < 0.18 in fixed point, far below i128::MAX.
        #[allow(clippy::cast_possible_wrap)]
        let ln_m = (2 * sum) as i128;
        (e, if below_one { -ln_m } else { ln_m })
    }

    /// `self * v` for a fixed-point `v`, in fixed point. The product must
    /// stay below `2^126` in fixed point.
    fn mul_fix(self, v: i128) -> i128 {
        let (hi, lo) = mul_wide(v.unsigned_abs(), u128::from(self.sig));
        // self = sig * 2^(exp - 63); the shift is at least 1 for |self| < 2^62.
        #[allow(clippy::cast_sign_loss)]
        let shift = (63 - self.exp) as u32;
        #[allow(clippy::cast_possible_wrap)]
        let p = shr_wide(hi, lo, shift) as i128;
        if self.neg == (v < 0) { p } else { -p }
    }

    /// Rounds the 256-bit value `(hi, lo) * 2^e` to 64 bits, to nearest with
    /// ties to even. `hi` must be below `2^127`.
    fn round_wide(neg: bool, hi: u128, lo: u128, e: i32) -> Self {
        if hi == 0 {
            return Self::round(neg, lo, e, false);
        }
        // Keep the top 128 bits; the rest only decide ties. 1 <= s <= 127.
        let s = 128 - hi.leading_zeros();
        let m = (hi << (128 - s)) | (lo >> s);
        let sticky = lo & ((1 << s) - 1) != 0;
        #[allow(clippy::cast_possible_wrap)]
        Self::round(neg, m, e + s as i32, sticky)
    }

    /// Rounds `m * 2^e` to 64 bits, to nearest with ties to even. `sticky`
    /// says that nonzero bits follow below `m`; then `m` must have at least
    /// 65 significant bits.
    fn round(neg: bool, m: u128, e: i32, sticky: bool) -> Self {
        if m == 0 {
            return Self::ZERO;
        }
        #[allow(clippy::cast_possible_wrap)]
        let top = 127 - m.leading_zeros() as i32;
        if top <= 63 {
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let sig = (m as u64) << (63 - top) as u32;
            return Self {
                neg,
                exp: e + top,
                sig,
            };
        }
        #[allow(clippy::cast_sign_loss)]
        let shift = (top - 63) as u32;
        #[allow(clippy::cast_possible_truncation)]
        let kept = (m >> shift) as u64;
        let rem = m & ((1 << shift) - 1);
        let half = 1 << (shift - 1);
        let up = rem > half || (rem == half && (sticky || kept & 1 == 1));
        match (up, kept.checked_add(1)) {
            (false, _) => Self {
                neg,
                exp: e + top,
                sig: kept,
            },
            (true, Some(sig)) => Self {
                neg,
                exp: e + top,
                sig,
            },
            (true, None) => Self {
                neg,
                exp: e + top + 1,
                sig: 1 << 63,
            },
        }
    }

    /// Compares magnitudes.
    fn cmp_abs(self, other: Self) -> Ordering {
        match (self.sig == 0, other.sig == 0) {
            (true, true) => Ordering::Equal,
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            (false, false) => (self.exp, self.sig).cmp(&(other.exp, other.sig)),
        }
    }
}

impl PartialEq for Ld {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd for Ld {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let neg_a = self.neg && self.sig != 0;
        let neg_b = other.neg && other.sig != 0;
        Some(match (neg_a, neg_b) {
            (false, false) => self.cmp_abs(*other),
            (true, true) => other.cmp_abs(*self),
            (false, true) => Ordering::Greater,
            (true, false) => Ordering::Less,
        })
    }
}

impl Neg for Ld {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            neg: !self.neg,
            ..self
        }
    }
}

impl Add for Ld {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        if rhs.sig == 0 {
            return self;
        }
        if self.sig == 0 {
            return rhs;
        }
        let (x, y) = if self.cmp_abs(rhs) == Ordering::Less {
            (rhs, self)
        } else {
            (self, rhs)
        };
        // x.sig sits at bits 126..63, so 63 guard bits follow it. Bits of y
        // shifted out below them set the last bit (sticky), which rounds the
        // same as the exact sum because at least 62 bits are dropped then.
        #[allow(clippy::cast_sign_loss)]
        let d = (x.exp - y.exp) as u32;
        let mx = u128::from(x.sig) << 63;
        let wy = u128::from(y.sig) << 63;
        let my = if d >= 127 {
            1
        } else {
            let shifted = wy >> d;
            shifted | u128::from(shifted << d != wy)
        };
        let m = if x.neg == y.neg { mx + my } else { mx - my };
        Self::round(x.neg, m, x.exp - 126, false)
    }
}

impl Sub for Ld {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self + -rhs
    }
}

impl Mul for Ld {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let m = u128::from(self.sig) * u128::from(rhs.sig);
        Self::round(self.neg != rhs.neg, m, self.exp + rhs.exp - 126, false)
    }
}

impl Div for Ld {
    type Output = Self;
    /// Division. The callers never divide by zero; a zero divisor gives a
    /// huge value in place of the x87 infinity.
    fn div(self, rhs: Self) -> Self {
        let neg = self.neg != rhs.neg;
        if rhs.sig == 0 {
            return Self {
                neg,
                exp: i32::MAX / 2,
                sig: 1 << 63,
            };
        }
        let num = u128::from(self.sig) << 64;
        let den = u128::from(rhs.sig);
        let (mut q, mut r) = (num / den, num % den);
        let mut e = self.exp - rhs.exp - 64;
        if q >> 64 == 0 {
            // One more quotient bit, so that a rounding bit exists.
            q <<= 1;
            r <<= 1;
            if r >= den {
                q |= 1;
                r -= den;
            }
            e -= 1;
        }
        Self::round(neg, q, e, r != 0)
    }
}

/// `a * b` as a 256-bit value `(high, low)`.
fn mul_wide(a: u128, b: u128) -> (u128, u128) {
    const LOW: u128 = u64::MAX as u128;
    let (a1, a0) = (a >> 64, a & LOW);
    let (b1, b0) = (b >> 64, b & LOW);
    let (mid, mid_carry) = (a0 * b1).overflowing_add(a1 * b0);
    let (lo, lo_carry) = (a0 * b0).overflowing_add(mid << 64);
    let hi = a1 * b1 + (mid >> 64) + (u128::from(mid_carry) << 64) + u128::from(lo_carry);
    (hi, lo)
}

/// The 256-bit value `(hi, lo)` shifted right by `shift`, which must leave
/// a value below `2^128`.
fn shr_wide(hi: u128, lo: u128, shift: u32) -> u128 {
    match shift {
        0 => lo,
        1..=127 => (hi << (128 - shift)) | (lo >> shift),
        _ => hi >> (shift - 128).min(127),
    }
}

/// `a * b` in fixed point, rounded down. The product must stay below `2^128`.
fn mul_fix(a: u128, b: u128) -> u128 {
    let (hi, lo) = mul_wide(a, b);
    shr_wide(hi, lo, FRAC)
}

/// [`mul_fix`] of a signed and an unsigned value.
fn signed_mul_fix(a: i128, b: u128) -> i128 {
    #[allow(clippy::cast_possible_wrap)]
    let p = mul_fix(a.unsigned_abs(), b) as i128;
    if a < 0 { -p } else { p }
}

/// `n / d` in fixed point, rounded down, for `n < d < 2^126`.
fn div_fix(n: u128, d: u128) -> u128 {
    let (mut q, mut r) = (0, n);
    for _ in 0..FRAC {
        r <<= 1;
        q <<= 1;
        if r >= d {
            r -= d;
            q |= 1;
        }
    }
    q
}

/// `e^v` of a fixed-point `v`, rounded to the nearest `Ld`.
fn exp_fix(v: i128) -> Ld {
    #[allow(clippy::cast_possible_wrap)]
    let ln2 = LN2_FIX as i128;
    // v = i ln 2 + f with 0 <= f < ln 2, so e^v = 2^i e^f.
    let i = v.div_euclid(ln2);
    #[allow(clippy::cast_sign_loss)]
    let f = v.rem_euclid(ln2) as u128;
    let mut term = ONE_FIX;
    let mut sum = ONE_FIX;
    let mut k = 1;
    while term != 0 {
        term = mul_fix(term, f) / k;
        sum += term;
        k += 1;
    }
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    let e = i as i32 - FRAC as i32;
    Ld::round(false, sum, e, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The x87 bits `(unbiased exponent, significand)` of a value.
    fn bits(x: Ld) -> (i32, u64) {
        (x.exp, x.sig)
    }

    fn ld(x: f64) -> Ld {
        Ld::from_f64(x)
    }

    #[test]
    fn conversions() {
        assert_eq!(bits(Ld::from_u64(1)), (0, 1 << 63));
        assert_eq!(bits(Ld::from_u64(u64::MAX)), (63, u64::MAX));
        assert_eq!(bits(Ld::from_u64(0)), (0, 0));
        assert_eq!(bits(ld(1.5)), (0, 0xc000_0000_0000_0000));
        assert_eq!(bits(ld(-0.75)), (-1, 0xc000_0000_0000_0000));
        assert!(ld(-0.75).neg);
        assert_eq!(bits(ld(0.0)), (0, 0));
        assert_eq!(bits(ld(f64::from_bits(1))), (-1074, 1 << 63));
        assert_eq!(ld(1e19).floor_u64(), 10_000_000_000_000_000_000);
    }

    #[test]
    fn binary64_rounding() {
        let one = 1_u64 << 63;
        for (sig, want) in [
            (one, one),
            (one + 0x3ff, one),
            (one + 0x400, one),
            (one + 0x401, one + 0x800),
            (one + 0xc00, one + 0x1000),
        ] {
            assert_eq!(bits(Ld::new(0, sig).round_binary64()), (0, want));
        }
        assert_eq!(Ld::ZERO.round_binary64(), Ld::ZERO);
        assert_eq!(bits(Ld::new(0, u64::MAX).round_binary64()), (1, one));
        assert_eq!((-ld(1.5)).round_binary64(), -ld(1.5));
    }

    #[test]
    fn floor_and_ceil() {
        assert_eq!(ld(2.5).floor_u64(), 2);
        assert_eq!(ld(2.5).ceil_u64(), 3);
        assert_eq!(ld(3.0).ceil_u64(), 3);
        assert_eq!(ld(0.25).floor_u64(), 0);
        assert_eq!(ld(0.25).ceil_u64(), 1);
        assert_eq!(ld(-2.5).floor_u64(), 0);
        assert_eq!(ld(-2.5).ceil_u64(), 0);
        assert_eq!(Ld::ZERO.ceil_u64(), 0);
        assert_eq!(ld(1e30).floor_u64(), u64::MAX);
        assert_eq!(ld(1e30).ceil_u64(), u64::MAX);
        assert_eq!(Ld::from_u64(u64::MAX).ceil_u64(), u64::MAX);
    }

    #[test]
    fn exact_arithmetic_matches_f64() {
        // Short significands: every sum, difference and product is exact in
        // both formats.
        let vals = [1.0, 3.0, -7.5, 0.125, 1e10, -0.375, 1234.5];
        for &a in &vals {
            for &b in &vals {
                assert_eq!(ld(a) * ld(b), ld(a * b), "{a} * {b}");
                assert_eq!(ld(a) + ld(b), ld(a + b), "{a} + {b}");
                assert_eq!(ld(a) - ld(b), ld(a - b), "{a} - {b}");
            }
        }
        assert_eq!(ld(1.0) - ld(1.0), Ld::ZERO);
        assert_eq!(ld(5.0) + Ld::ZERO, ld(5.0));
        assert_eq!(Ld::ZERO + ld(5.0), ld(5.0));
        assert_eq!(ld(6.0) / ld(3.0), ld(2.0));
        assert_eq!(ld(-6.0) / ld(4.0), ld(-1.5));
        assert_eq!(Ld::ZERO / ld(4.0), Ld::ZERO);
        assert!(ld(1.0) / Ld::ZERO > ld(1e300));
    }

    #[test]
    fn rounding_is_to_nearest_even() {
        let one = Ld::ONE;
        // 1 + 2^-64 is a tie between 1 and 1 + 2^-63: even is 1.
        assert_eq!(one + Ld::new(-64, 1 << 63), one);
        // 1 + 3 * 2^-64 ties to 1 + 2^-62 (even last bit).
        let up = one + Ld::new(-63, 0xc000_0000_0000_0000);
        assert_eq!(bits(up), (0, (1 << 63) + 2));
        // A tie broken by a lower bit that is shifted out (sticky) rounds up.
        let sticky = one + Ld::new(-64, (1 << 63) + 1);
        assert_eq!(bits(sticky), (0, (1 << 63) + 1));
        // Far apart: the small term only sets the sticky bit.
        assert_eq!(one + Ld::new(-300, 1 << 63), one);
        assert_eq!(one - Ld::new(-300, 1 << 63), one);
        // Below 1 the step is 2^-64: 1 - 2^-65 ties to 1, a little more
        // goes down.
        assert_eq!(one - Ld::new(-65, 1 << 63), one);
        assert_eq!(bits(one - Ld::new(-65, (1 << 63) + 1)), (-1, u64::MAX));
        // A carry out of the significand.
        let max = Ld::new(0, u64::MAX);
        assert_eq!(bits(max + Ld::new(-64, 1 << 63)), (1, 1 << 63));
        // 1/3 rounds down, 2/3 rounds up.
        assert_eq!(bits(one / ld(3.0)), (-2, 0xaaaa_aaaa_aaaa_aaab));
        assert_eq!(bits(ld(2.0) / ld(3.0)), (-1, 0xaaaa_aaaa_aaaa_aaab));
        assert_eq!(bits(one / ld(10.0)), (-4, 0xcccc_cccc_cccc_cccd));
    }

    #[test]
    fn ordering() {
        assert!(ld(-2.0) < ld(-1.0));
        assert!(ld(-1.0) < Ld::ZERO);
        assert!(Ld::ZERO < ld(1e-300));
        assert!(ld(3.0) > ld(2.0));
        assert!(ld(2.0).abs() > Ld::ZERO);
        assert_eq!(-Ld::ZERO, Ld::ZERO);
        assert_eq!(ld(-4.0).abs(), ld(4.0));
    }

    #[test]
    fn sqrt_is_correctly_rounded() {
        assert_eq!(ld(16.0).sqrt(), ld(4.0));
        assert_eq!(ld(2.25).sqrt(), ld(1.5));
        // sqrtl(2) and sqrtl(10) on x86-64.
        assert_eq!(bits(ld(2.0).sqrt()), (0, 0xb504_f333_f9de_6484));
        assert_eq!(bits(ld(10.0).sqrt()), (1, 0xca62_c1d6_d2da_9490));
        assert_eq!(bits(ld(1e16).sqrt()), (26, 0xbebc_2000_0000_0000));
        assert_eq!(ld(-1.0).sqrt(), Ld::ZERO);
        assert_eq!(Ld::ZERO.sqrt(), Ld::ZERO);
    }

    #[test]
    fn ln_matches_x87_logl() {
        // logl on x86-64 glibc (x87 fyl2x), printed as raw bits.
        for (x, want) in [
            (2.0, (-1, 0xb172_17f7_d1cf_79ac)),
            (10.0, (1, 0x935d_8ddd_aaa8_ac17)),
            (0.5, (-1, 0xb172_17f7_d1cf_79ac)),
            (1e17, (5, 0x9c93_66bb_8553_36d8)),
            (1.25, (-3, 0xe47f_be3c_d4d1_0d61)),
        ] {
            assert_eq!(bits(ld(x).ln()), want, "logl({x})");
        }
        assert!(ld(0.5).ln().neg);
        assert_eq!(Ld::ONE.ln(), Ld::ZERO);
        // Next to 1, ln(1 + 2^-63) is 2^-63 to far below its last bit.
        let tiny = Ld::new(0, (1 << 63) + 1).ln();
        assert!((tiny - Ld::EPSILON).abs() < Ld::new(-100, 1 << 63));
    }

    #[test]
    fn pow_is_close_to_f64() {
        for (x, y) in [(2.0, -57.0), (3.0, -60.0), (1e19, 0.5), (1.8e19, 1.0 / 3.0)] {
            let got = ld(x).pow(ld(y));
            let want = x.powf(y);
            assert!((got - ld(want)).abs() <= ld(want * 1e-15), "{x}^{y}");
        }
        assert_eq!(ld(2.0).pow(ld(-57.0)), Ld::new(-57, 1 << 63));
        assert_eq!(ld(4.0).pow(ld(0.5)), ld(2.0));
    }

    #[test]
    fn wide_helpers() {
        assert_eq!(mul_wide(u128::MAX, u128::MAX), (u128::MAX - 1, 1));
        assert_eq!(shr_wide(1, 0, 0), 0);
        assert_eq!(shr_wide(1, 0, 128), 1);
        assert_eq!(shr_wide(4, 0, 129), 2);
        assert_eq!(div_fix(1, 2), ONE_FIX / 2);
        let one = i128::try_from(ONE_FIX).unwrap();
        assert_eq!(signed_mul_fix(-one, 2 * ONE_FIX), -2 * one);
    }
}

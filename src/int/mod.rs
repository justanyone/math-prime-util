//! Integer roots, logarithms, powers and gcd on `u64`.
//!
//! Ported from MPU's `util.c` and `util.h` (`isqrt`, `icbrt`, `rootint`,
//! `crootint`, `logint`, `ipowsafe`, `is_power_ret`, `is_perfect_square_ret`,
//! `gcd_ui`, `lcm_ui`, `gcdext`).

// Used by the primality module (phase 1); unused until then.
#[allow(dead_code)]
pub(crate) mod mont;

/// Largest `k` for which MPU keeps a `root_max` table entry: for `k` above
/// this, the only `u64` values that are perfect `k`-th powers are `0`, `1`
/// and `2^k`, which the callers special-case.
const MAX_POW3: u32 = 40;

/// `root_max[k]` is the largest `r` with `r^k <= u64::MAX`, for `2 <= k <= 40`.
/// Same table as MPU `util.c`.
const ROOT_MAX: [u32; 1 + MAX_POW3 as usize] = [
    0,
    0,
    4_294_967_295,
    2_642_245,
    65_535,
    7_131,
    1_625,
    565,
    255,
    138,
    84,
    56,
    40,
    30,
    23,
    19,
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
];

/// Bit masks over `n mod 32` of residues that cannot be `k`-th powers, indexed
/// by `k`. Same table as MPU `util.c` (`_rootmask32`).
const ROOTMASK32: [u32; 41] = [
    0x0000_0000,
    0x0000_0000,
    0xfdfc_fdec,
    0x5455_5454,
    0xfffc_fffc,
    0x5555_5554,
    0xfdfd_fdfc,
    0x5555_5554,
    0xffff_fffc,
    0x5555_5554,
    0xfdfd_fdfc,
    0x5555_5554,
    0xfffd_fffc,
    0xd555_5556,
    0xfdfd_fdfc,
    0xf57d_57d6,
    0xffff_fffc,
    0xffff_d556,
    0xfdfd_fdfe,
    0xd57f_fffe,
    0xfffd_fffc,
    0xffd7_ff7e,
    0xfdfd_fdfe,
    0xffff_d7fe,
    0xffff_fffc,
    0xffff_ffd6,
    0xfdff_fdfe,
    0xd7ff_fffe,
    0xfffd_fffe,
    0xfff7_fffe,
    0xfdff_fffe,
    0xffff_f7fe,
    0xffff_fffc,
    0xffff_fff6,
    0xffff_fdfe,
    0xf7ff_fffe,
    0xfffd_fffe,
    0xfff7_fffe,
    0xfdff_fffe,
    0xffff_f7fe,
    0xffff_fffc,
];

/// Integer square root: the largest `r` with `r * r <= n`.
///
/// ```
/// use math_prime_util::sqrtint;
/// assert_eq!(sqrtint(1524155677489), 1234567);
/// assert_eq!(sqrtint(u64::MAX), 4294967295);
/// ```
#[inline]
pub fn sqrtint(n: u64) -> u64 {
    // MPU: r = sqrt((double)n) + 1e-6, then correct downwards once. The f64
    // estimate is within 1 of the truth for every u64, so one correction
    // in each direction makes it exact under every rounding mode.
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    let mut r = (n as f64).sqrt() as u64;
    if r > 4_294_967_295 {
        r = 4_294_967_295;
    }
    while r.checked_mul(r).is_none_or(|sq| sq > n) {
        r -= 1;
    }
    while (r + 1).checked_mul(r + 1).is_some_and(|sq| sq <= n) {
        r += 1;
    }
    r
}

/// `true` if `n` is a perfect square. Returns the root through `is_square_root`
/// when it is wanted.
///
/// ```
/// use math_prime_util::is_square;
/// assert!(is_square(0));
/// assert!(is_square(1524155677489));
/// assert!(!is_square(1524155677490));
/// ```
#[inline]
pub fn is_square(n: u64) -> bool {
    is_square_root(n).is_some()
}

/// The square root of `n` if `n` is a perfect square.
///
/// ```
/// use math_prime_util::int::is_square_root;
/// assert_eq!(is_square_root(49), Some(7));
/// assert_eq!(is_square_root(50), None);
/// ```
#[inline]
pub fn is_square_root(n: u64) -> Option<u64> {
    // Fast filter from MPU: rejects 95% of non-squares by the residue mod 64.
    if (1u64 << (n & 63)) & 0xfdfd_fded_fdfc_fdec != 0 {
        return None;
    }
    let r = sqrtint(n);
    (r * r == n).then_some(r)
}

/// Integer cube root: the largest `r` with `r^3 <= n`.
#[inline]
fn icbrt(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    let mut r = (n as f64).cbrt() as u64;
    if r > 2_642_245 {
        r = 2_642_245;
    }
    while r * r * r > n {
        r -= 1;
    }
    while r < 2_642_245 && (r + 1) * (r + 1) * (r + 1) <= n {
        r += 1;
    }
    r
}

/// `b^e` without overflow checks; callers guarantee it fits.
#[inline]
fn ipow(mut b: u64, mut e: u32) -> u64 {
    let mut p = 1u64;
    while e != 0 {
        if e & 1 != 0 {
            p = p.wrapping_mul(b);
        }
        e >>= 1;
        if e != 0 {
            b = b.wrapping_mul(b);
        }
    }
    p
}

/// `k`-th root for `3 <= k <= 40` where the result fits in a `u32`:
/// floating-point estimate, then exact correction against `ROOT_MAX`.
fn irootn(n: u64, k: u32) -> u64 {
    let max = u64::from(ROOT_MAX[k as usize]);
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    let mut r = (n as f64).powf(1.0 / f64::from(k)) as u64;
    if r > max {
        r = max;
    }
    while r > 0 && ipow(r, k) > n {
        r -= 1;
    }
    while r < max && ipow(r + 1, k) <= n {
        r += 1;
    }
    r
}

/// Integer `k`-th root: the largest `r` with `r^k <= n`.
///
/// # Panics
///
/// Panics if `k == 0`, as MPU's `rootint` dies there.
///
/// ```
/// use math_prime_util::rootint;
/// assert_eq!(rootint(15625, 3), 25);
/// assert_eq!(rootint(4544344858450091408, 6), 1286);
/// assert_eq!(rootint(4544344858450091409, 6), 1287);
/// assert_eq!(rootint(u64::MAX, 63), 2);
/// ```
pub fn rootint(n: u64, k: u32) -> u64 {
    assert!(k != 0, "rootint: k must be at least 1");
    if n <= 1 {
        return n;
    }
    match k {
        1 => n,
        2 => sqrtint(n),
        3 => icbrt(n),
        _ if k >= 64 || n >> k == 0 => 1,
        _ if k > MAX_POW3 => 1 + u64::from(k < 64),
        _ if k >= 32 => 2 + u64::from(n >= ipow(3, k)),
        _ => irootn(n, k),
    }
}

/// Ceiling `k`-th root: the smallest `r` with `r^k >= n`.
///
/// # Panics
///
/// Panics if `k == 0`.
///
/// ```
/// use math_prime_util::crootint;
/// assert_eq!(crootint(15625, 3), 25);
/// assert_eq!(crootint(15626, 3), 26);
/// ```
pub fn crootint(n: u64, k: u32) -> u64 {
    assert!(k != 0, "crootint: k must be at least 1");
    if n <= 1 || k == 1 {
        return n;
    }
    let r = rootint(n, k);
    r + u64::from(powint(r, k).is_none_or(|p| p < n))
}

/// `n^k`, or `None` if the result does not fit in a `u64`.
///
/// Same convention as MPU's `ipowsafe` (which returns `UV_MAX` for overflow)
/// and as `u64::checked_pow`. `0^0 == 1`.
///
/// ```
/// use math_prime_util::powint;
/// assert_eq!(powint(3, 40), Some(12157665459056928801));
/// assert_eq!(powint(3, 41), None);
/// assert_eq!(powint(0, 0), Some(1));
/// ```
#[inline]
pub fn powint(n: u64, k: u32) -> Option<u64> {
    if n == 0 {
        return Some(u64::from(k == 0));
    }
    if n == 1 || k == 0 {
        return Some(1);
    }
    if k == 1 {
        return Some(n);
    }
    if k <= MAX_POW3 {
        return (n <= u64::from(ROOT_MAX[k as usize])).then(|| ipow(n, k));
    }
    n.checked_pow(k)
}

/// Floor of the base-`b` logarithm of `n`: the largest `e` with `b^e <= n`.
///
/// # Panics
///
/// Panics if `n == 0` or `b < 2`, like `u64::ilog`.
///
/// ```
/// use math_prime_util::logint;
/// assert_eq!(logint(19284098234, 16), 8);
/// assert_eq!(logint(58, u64::MAX), 0);
/// assert_eq!(logint(1000, 10), 3);
/// ```
pub fn logint(n: u64, b: u64) -> u32 {
    assert!(n != 0, "logint: n must be at least 1");
    assert!(b >= 2, "logint: base must be at least 2");
    n.ilog(b)
}

/// `true` if `n` is a perfect `k`-th power (`n == r^k` for some `r`).
///
/// With `k == 0`, returns `true` if `n` is a perfect power for *any* `k >= 2`
/// (MPU's `is_power(n)` returns the maximal `k`; see [`is_power_exp`]).
///
/// ```
/// use math_prime_util::is_power;
/// assert!(is_power(815730721, 8));
/// assert!(!is_power(815730722, 8));
/// assert!(is_power(1024, 0));
/// assert!(!is_power(1025, 0));
/// ```
pub fn is_power(n: u64, k: u32) -> bool {
    if k == 0 {
        return is_power_exp(n) != 0;
    }
    is_power_root(n, k).is_some()
}

/// The `k`-th root of `n` if `n` is a perfect `k`-th power.
///
/// # Panics
///
/// Panics if `k == 0`.
pub fn is_power_root(n: u64, k: u32) -> Option<u64> {
    assert!(k != 0, "is_power_root: k must be at least 1");
    if n < 2 || k == 1 {
        return Some(n);
    }
    if k > MAX_POW3 {
        return (k < 64 && n == 1u64 << k).then_some(2);
    }
    if k == 2 {
        return is_square_root(n);
    }
    #[allow(clippy::cast_possible_truncation)]
    if (1u32 << (n & 31)) & ROOTMASK32[k as usize] != 0 {
        return None;
    }
    if k == 3 {
        let r = n % 117;
        if (r.wrapping_mul(833_230_740) & r.wrapping_mul(120_676_722) & 0x3081_106b) != 0 {
            return None;
        }
        let r = icbrt(n);
        return (r * r * r == n).then_some(r);
    }
    let r = irootn(n, k);
    (ipow(r, k) == n).then_some(r)
}

/// The largest `k >= 2` such that `n` is a perfect `k`-th power, or `0` if
/// `n` is not a perfect power. Matches MPU's one-argument `is_power(n)`.
///
/// `0` and `1` return `0`, as in MPU.
///
/// ```
/// use math_prime_util::int::is_power_exp;
/// assert_eq!(is_power_exp(1024), 10);
/// assert_eq!(is_power_exp(36), 2);
/// assert_eq!(is_power_exp(37), 0);
/// ```
pub fn is_power_exp(n: u64) -> u32 {
    if n < 4 {
        return 0;
    }
    // MPU: find any root via the prime exponents 2,3,5,...; recurse on the
    // root to find the maximal exponent.
    let mut kmax = 0u32;
    let mut m = n;
    loop {
        let mut found = 0u32;
        let lim = m.ilog2();
        for k in [2u32, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
            if k > lim {
                break;
            }
            if let Some(r) = is_power_root(m, k) {
                m = r;
                found = k;
                break;
            }
        }
        if found == 0 {
            return kmax;
        }
        kmax = if kmax == 0 { found } else { kmax * found };
    }
}

/// Greatest common divisor. `gcd(0, 0) == 0`.
///
/// ```
/// use math_prime_util::gcd;
/// assert_eq!(gcd(12, 18), 6);
/// assert_eq!(gcd(0, 5), 5);
/// ```
#[inline]
pub fn gcd(mut x: u64, mut y: u64) -> u64 {
    // Binary gcd (the FLINT `gcdz` that MPU selects when ctz is fast).
    if x == 0 {
        return y;
    }
    if y == 0 {
        return x;
    }
    let shift = (x | y).trailing_zeros();
    x >>= x.trailing_zeros();
    loop {
        y >>= y.trailing_zeros();
        if x > y {
            core::mem::swap(&mut x, &mut y);
        }
        y -= x;
        if y == 0 {
            return x << shift;
        }
    }
}

/// Least common multiple, or `None` if it does not fit in a `u64`.
/// `lcm(0, y) == Some(0)`.
///
/// ```
/// use math_prime_util::lcm;
/// assert_eq!(lcm(4, 6), Some(12));
/// assert_eq!(lcm(u64::MAX, 2), None);
/// ```
#[inline]
pub fn lcm(x: u64, y: u64) -> Option<u64> {
    if x == 0 || y == 0 {
        return Some(0);
    }
    x.checked_mul(y / gcd(x, y))
}

/// Extended Euclidean algorithm: returns `(g, u, v)` with `g = gcd(a, b)` and
/// `a*u + b*v == g`, as MPU's `gcdext` does on signed values.
///
/// ```
/// use math_prime_util::gcdext;
/// assert_eq!(gcdext(240, 46), (2, -9, 47));
/// assert_eq!(gcdext(0, 0), (0, 0, 0));
/// ```
pub fn gcdext(a: i64, b: i64) -> (i64, i64, i64) {
    // (s, t, r) are the running coefficients and remainder; (u, v, g) the
    // previous ones, which hold the answer when r reaches 0.
    let (mut s, mut u) = (0i64, 1i64);
    let (mut t, mut v) = (1i64, 0i64);
    let (mut r, mut g) = (b, a);
    if a == 0 && b == 0 {
        u = 0;
        t = 0;
    }
    while r != 0 {
        let q = g / r;
        (g, r) = (r, g - q * r);
        (u, s) = (s, u - q * s);
        (v, t) = (t, v - q * t);
    }
    if g < 0 {
        (g, u, v) = (-g, -u, -v);
    }
    (g, u, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sqrt_edges() {
        for n in [
            0u64,
            1,
            2,
            3,
            4,
            15,
            16,
            17,
            u64::from(u32::MAX),
            1 << 62,
            u64::MAX,
        ] {
            let r = sqrtint(n);
            assert!(r * r <= n, "{n}");
            assert!((r + 1).checked_mul(r + 1).is_none_or(|s| s > n), "{n}");
        }
    }

    #[test]
    fn root_tables_consistent() {
        for k in 2..=MAX_POW3 {
            let m = u64::from(ROOT_MAX[k as usize]);
            assert!(m.checked_pow(k).is_some(), "k={k}");
            assert!((m + 1).checked_pow(k).is_none(), "k={k}");
        }
    }

    #[test]
    #[cfg_attr(miri, ignore = "brute-force loop, too slow under Miri")]
    fn rootint_brute() {
        for k in 2..=70u32 {
            for n in (0..2000u64).chain([u64::MAX, u64::MAX - 1, 1 << 63]) {
                let r = rootint(n, k);
                assert!(
                    r.checked_pow(k).is_some_and(|p| p <= n),
                    "n={n} k={k} r={r}"
                );
                assert!(
                    (r + 1).checked_pow(k).is_none_or(|p| p > n),
                    "n={n} k={k} r={r}"
                );
            }
        }
    }

    #[test]
    #[cfg_attr(miri, ignore = "brute-force loop, too slow under Miri")]
    fn is_power_brute() {
        for n in 0..5000u64 {
            for k in 1..=13u32 {
                let expect = (0..=n)
                    .take_while(|r| r.checked_pow(k).is_some_and(|p| p <= n))
                    .any(|r| r.pow(k) == n);
                assert_eq!(is_power(n, k), expect, "n={n} k={k}");
            }
        }
    }

    #[test]
    #[cfg_attr(miri, ignore = "brute-force loop, too slow under Miri")]
    fn gcd_brute() {
        fn slow(mut a: u64, mut b: u64) -> u64 {
            while b != 0 {
                (a, b) = (b, a % b);
            }
            a
        }
        for a in 0..200u64 {
            for b in 0..200u64 {
                assert_eq!(gcd(a, b), slow(a, b));
            }
        }
        assert_eq!(gcd(u64::MAX, u64::MAX - 1), 1);
    }
}

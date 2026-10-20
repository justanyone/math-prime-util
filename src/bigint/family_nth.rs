//! The `n`th almost prime, omega prime and powerful number, and the
//! powerful numbers of an interval, beyond the 64-bit range: the `BigUint`
//! forms of the functions of [`crate::families`] with the same names.
//!
//! Port of MPU 0.75 `PP.pm` `nth_almost_prime`, `nth_omega_prime`,
//! `nth_powerful` and `powerful_numbers`, the pure Perl path that MPU takes
//! when a value does not fit a native word. Where the native function
//! answers (the index fits `u64` and so does the result) the `BigUint` form
//! returns the native answer, so below `2^64` the two agree.
//!
//! Above, the `n`th value is the least `x` whose exact count reaches `n`,
//! found by interpolation and bisection on the count. The counts follow
//! `PP.pm`: almost primes first take out the powers of two that every member
//! must have (`_kap_reduce_nth`, `_kap_reduce_count`), and the counts then
//! enumerate products of small primes with a native prime count at each
//! leaf (`_kapc_count`, `_omega_prime_count_rec`, `_powerful_count_recurse`).
//! The run time grows with the number of such products, as in MPU's Perl
//! path: a large `k` with a small index takes milliseconds
//! (`nth_almost_prime(64, 10^6)`, `nth_powerful(100, 60)`), an index just
//! past the native range for a mid-size `k` takes seconds
//! (`nth_omega_prime(15, 299179)`), and a small `k` with a large index does
//! not finish. A count that would need a prime count beyond `2^64` is not
//! tried: the `n`th functions return `None` there, where MPU's Perl path
//! does not return.
//!
//! `nth_semiprime` and `nth_prime_power` have no `BigUint` form: past the
//! native range MPU finds them from exact semiprime and prime power counts,
//! which need prime counts near or past `2^64`, and its Perl path does not
//! return. [`nth_almost_prime`] with `k = 2` and [`nth_omega_prime`] with
//! `k = 1`, which are those two functions, return `None` there.
//!
//! ```
//! use math_prime_util::bigint::family_nth::{nth_almost_prime, nth_omega_prime};
//! use num_bigint::BigUint;
//! let two64 = BigUint::from(1_u8) << 64_usize;
//! assert_eq!(nth_almost_prime(64, &BigUint::from(1_u8)), Some(two64));
//! assert_eq!(
//!     nth_omega_prime(16, &BigUint::from(1_u8)).map(|v| v.to_string()),
//!     Some("32589158477190044730".into())
//! );
//! ```
// Sieve indices fit `usize`; a double only starts the `u128` root search.
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]

use super::arith::rootint;
use super::counts::nth_prime;
use super::family_bounds::{nth_almost_prime_lower, nth_almost_prime_upper};
use super::sequences::pn_primorial;
use num_bigint::BigUint;

/// `MPU_MAX_SEMI_PRIME_IDX`: the native `nth_semiprime` stops at this index.
const MAX_SEMI_PRIME_IDX: u64 = 1_701_748_900_850_019_777;
/// `MPU_MAX_PRIME_IDX`: the native `nth_prime_power` stops at this index.
const MAX_PRIME_IDX: u64 = 425_656_284_035_217_743;
/// The largest square-free sieve held in memory; above it each test factors.
const SIEVE_LIMIT: u64 = 1 << 24;

/// OEIS A078843 as in `PP.pm` `_kap_reduce_nth`: `A078843[k]` counts the
/// `k`-almost primes up to `3^k`. Below that index the `n`th `k`-almost
/// prime is even (an odd one is at least `3^k`), twice the `n`th
/// `(k - 1)`-almost prime.
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
    140_187,
    216_625,
    334_527,
    516_126,
    795_632,
    1_225_641,
    1_886_570,
    2_901_796,
    4_460_359,
    6_851_532,
    10_518_476,
    16_138_642,
    24_748_319,
    37_932_129,
    58_110_457,
    88_981_343,
    136_192_537,
    208_364_721,
    318_653_143,
    487_128_905,
    744_398_307,
    1_137_129_971,
    1_736_461_477,
    2_650_785_552,
    4_045_250_962,
    6_171_386_419,
    9_412_197_641,
    14_350_773_978,
    21_874_583_987,
    33_334_053_149,
    50_783_701_654,
    77_348_521_640,
    117_780_873_397,
    179_306_456_282,
    272_909_472_119,
    415_284_741_506,
];

fn big(v: u64) -> BigUint {
    BigUint::from(v)
}

fn small(n: &BigUint) -> Option<u64> {
    u64::try_from(n).ok()
}

/// Square-free test for `1..=limit`, sieved when the limit is small.
struct SquareFree(Vec<bool>);

impl SquareFree {
    fn new(limit: u64) -> Self {
        if limit > SIEVE_LIMIT {
            return Self(Vec::new());
        }
        let size = limit as usize;
        let mut flags = vec![true; size + 1];
        let mut p = 2;
        while p * p <= size {
            if flags[p] {
                for j in (p * p..=size).step_by(p * p) {
                    flags[j] = false;
                }
            }
            p += 1;
        }
        Self(flags)
    }

    fn test(&self, v: u64) -> bool {
        self.0
            .get(v as usize)
            .copied()
            .unwrap_or_else(|| crate::is_square_free(v))
    }
}

/// Lists the members of a family in `lo..=hi`, in increasing order.
type Members<'a> = &'a dyn Fn(&BigUint, &BigUint) -> Vec<BigUint>;

/// A search that has at most this many members left lists them.
const WALK: u64 = 64;

/// The least `x` in `(lo, hi]` with `count(x) >= n`, given
/// `count(lo) < n` (pass its value if known). `hi` doubles until it
/// reaches `n`. The search interpolates on the counts, as MPU's
/// `inverse_interpolate.c` does, and bisects after a step that does not
/// halve the interval. With `members`, it lists the last few members.
fn least_reaching(
    mut lo: BigUint,
    mut hi: BigUint,
    count_lo: Option<BigUint>,
    n: &BigUint,
    count: impl Fn(&BigUint) -> Option<BigUint>,
    members: Option<Members<'_>>,
) -> Option<BigUint> {
    let mut count_lo = match count_lo {
        Some(c) => c,
        None => count(&lo)?,
    };
    if hi <= lo {
        hi = &lo + 1_u8;
    }
    let mut count_hi = count(&hi)?;
    while count_hi < *n {
        lo.clone_from(&hi);
        count_lo = count_hi;
        hi <<= 1_usize;
        count_hi = count(&hi)?;
    }
    let mut bisect = false;
    while &hi - &lo > big(1) {
        if let Some(list) = members {
            if &count_hi - &count_lo <= big(WALK) {
                let rank = small(&(n - &count_lo))?;
                return list(&(&lo + 1_u8), &hi).into_iter().nth(rank as usize - 1);
            }
        }
        let width = &hi - &lo;
        let mid = if bisect {
            (&lo + &hi) >> 1_usize
        } else {
            let guess = &lo + &width * (n - &count_lo) / (&count_hi - &count_lo);
            guess.clamp(&lo + 1_u8, &hi - 1_u8)
        };
        let c = count(&mid)?;
        if c >= *n {
            hi = mid;
            count_hi = c;
        } else {
            lo = mid;
            count_lo = c;
        }
        bisect = (&hi - &lo) * 2_u8 > width;
    }
    Some(hi)
}

/// `PP.pm` `_kap_reduce_nth`: the power of two `r` that every one of the
/// first `n` `k`-almost primes has, beyond the `(k - r)`-almost primes.
fn kap_reduce_nth(k: u32, n: &BigUint) -> u32 {
    if k <= 1 {
        return 0;
    }
    let last = (A078843.len() - 1) as u32;
    let mut r = 0;
    if k > last {
        if *n >= big(A078843[last as usize]) {
            return 0;
        }
        r = k - last;
    }
    while *n < big(A078843[(k - r) as usize]) {
        r += 1;
    }
    r
}

/// The count of `k`-almost primes at most `y` whose prime factors are all at
/// least `lo`, for `y` in `u64` and `k >= 1`.
fn kap_native(y: u64, lo: u64, k: u32) -> u64 {
    if k == 1 {
        return if y < lo {
            0
        } else {
            crate::prime_count(y) - crate::prime_count(lo - 1)
        };
    }
    if lo == 2 {
        return crate::almost_prime_count(u64::from(k), y);
    }
    if k == 2 {
        return crate::primes_in(lo..=crate::sqrtint(y))
            .map(|p| crate::prime_count(y / p) - crate::prime_count(p) + 1)
            .sum();
    }
    crate::primes_in(lo..=crate::rootint(y, k))
        .map(|p| kap_native(y / p, p, k - 1))
        .sum()
}

/// `PP.pm` `_kapc_count`: as [`kap_native`] for any `y`; `None` when a prime
/// count past `2^64` would be needed. With no bound on the factors
/// (`lo = 2`), `_kap_reduce_count` first takes out the factors of two that
/// every member has: below `3^k` each `k`-almost prime is even.
fn kap_rec(y: &BigUint, lo: u64, k: u32) -> Option<BigUint> {
    let (mut y, mut k) = (y.clone(), k);
    if lo == 2 {
        let mut pow3 = BigUint::from(3_u8).pow(k);
        while k > 0 && y < pow3 {
            y >>= 1_usize;
            k -= 1;
            pow3 /= 3_u8;
        }
        if k == 0 {
            return Some(big(u64::from(y.bits() != 0)));
        }
    }
    if let Some(y) = small(&y) {
        return Some(big(kap_native(y, lo, k)));
    }
    if k <= 2 {
        return None;
    }
    let hi = small(&rootint(&y, k))?;
    let mut sum = BigUint::default();
    for p in crate::primes_in(lo..=hi) {
        sum += kap_rec(&(&y / p), p, k - 1)?;
    }
    Some(sum)
}

/// `PP.pm` `almost_prime_count`: the exact count of `k`-almost primes at
/// most `x`.
fn kap_count(k: u32, x: &BigUint) -> Option<BigUint> {
    if k == 0 {
        return Some(big(u64::from(x.bits() != 0)));
    }
    if (x >> k as usize).bits() == 0 {
        return Some(BigUint::default());
    }
    kap_rec(x, 2, k)
}

/// Return the `n`th integer with `k` prime factors, counted with
/// multiplicity.
///
/// Port of MPU 0.75 `PP.pm` `nth_almost_prime`. The native
/// [`nth_almost_prime`](crate::nth_almost_prime) answers where its index
/// and result fit `u64`. Above, the reduction `_kap_reduce_nth` writes the
/// value as `2^r` times an almost prime with `k - r` factors when the index
/// is small for `k`; otherwise a search on the exact count finds it,
/// between [`nth_almost_prime_lower`] and [`nth_almost_prime_upper`].
/// `k = 1` is [`nth_prime`].
///
/// Returns `None` for `n = 0`, for `k = 0` with `n > 1` (MPU returns
/// `undef`), and where a prime count past `2^64` would be needed, which
/// includes `k = 2` past the native semiprime range.
///
/// ```
/// use math_prime_util::bigint::family_nth::nth_almost_prime;
/// use num_bigint::BigUint;
/// assert_eq!(nth_almost_prime(3, &BigUint::from(10_u8)), Some(BigUint::from(45_u8)));
/// let v = nth_almost_prime(70, &BigUint::from(100_u8)).expect("value");
/// assert_eq!(v.to_string(), "42796446251006159749120");
/// ```
#[must_use]
pub fn nth_almost_prime(k: u32, n: &BigUint) -> Option<BigUint> {
    if n.bits() == 0 {
        return None;
    }
    if *n == big(1) {
        return Some(big(1) << k as usize);
    }
    match k {
        0 => return None,
        1 => return nth_prime(n),
        2 => {
            return small(n)
                .filter(|&m| m <= MAX_SEMI_PRIME_IDX)
                .and_then(crate::nth_semiprime)
                .map(big);
        }
        _ => {}
    }
    if let Some(m) = small(n) {
        if k < 64 {
            if let Some(v) = crate::nth_almost_prime(k, m) {
                return Some(big(v));
            }
        }
        if m < 8 {
            let base = [0_u64, 8, 12, 18, 20, 27, 28, 30][m as usize];
            return Some(big(base) << (k - 3) as usize);
        }
    }
    let r = kap_reduce_nth(k, n);
    if r > 0 {
        return nth_almost_prime(k - r, n).map(|v| v << r as usize);
    }
    // The native form had no answer, so the value is past u64::MAX (its count
    // there is below n). Start from the lower bound when its count is below n.
    let count = |x: &BigUint| kap_count(k, x);
    let (mut lo, mut count_lo) = (big(u64::MAX), None);
    if let Some(lower) = nth_almost_prime_lower(k, n) {
        if lower > lo {
            let below = lower - 1_u8;
            let c = count(&below)?;
            if c < *n {
                (lo, count_lo) = (below, Some(c));
            }
        }
    }
    let hi = nth_almost_prime_upper(k, n).unwrap_or_default();
    least_reaching(lo, hi, count_lo, n, count, None)
}

/// `PP.pm` `_omega_prime_count_rec`: integers at most `n` that are `m`
/// times `k` more distinct primes, from `p` up (the `j`th prime).
fn omega_rec(k: u32, n: &BigUint, m: &BigUint, mut p: u64, s: u64, mut j: u64) -> Option<BigUint> {
    let mut count = BigUint::default();
    while p <= s {
        let r = crate::next_prime(p)?;
        let mut t = m * p;
        while t <= *n {
            let w = n / &t;
            if k == 2 {
                if big(r) > w {
                    break;
                }
                let w = small(&w)?;
                count += crate::prime_count(w) - j;
                let mut q = r;
                while q <= w {
                    let mut u = &t * q * q;
                    if u > *n {
                        break;
                    }
                    while u <= *n {
                        count += 1_u8;
                        u *= q;
                    }
                    q = crate::next_prime(q)?;
                }
            } else {
                let s2 = rootint(&w, k - 1);
                if big(r) > s2 {
                    break;
                }
                count += omega_rec(k - 1, n, &t, r, small(&s2)?, j + 1)?;
            }
            t *= p;
        }
        p = r;
        j += 1;
    }
    Some(count)
}

/// `PP.pm` `omega_prime_count`: the exact count of integers at most `x`
/// with `k` distinct prime factors.
fn omega_count(k: u32, x: &BigUint) -> Option<BigUint> {
    if let Some(v) = small(x) {
        return Some(big(crate::omega_prime_count(k, v)));
    }
    if k <= 1 {
        return (k == 0).then(|| big(1));
    }
    omega_rec(k, x, &big(1), 2, small(&rootint(x, k))?, 1)
}

/// Return the `n`th integer with `k` distinct prime factors.
///
/// Port of MPU 0.75 `PP.pm` `nth_omega_prime`. The native
/// [`nth_omega_prime`](crate::nth_omega_prime) answers where its index and
/// result fit `u64`. Above, a search from `pn_primorial(k)` on the
/// exact count (`_omega_prime_count_rec`) finds the value. `k = 1` is
/// `nth_prime_power`.
///
/// Returns `None` for `n = 0`, for `k = 0` with `n > 1` (MPU returns
/// `undef`), and where a prime count past `2^64` would be needed, which
/// includes `k = 1` past the native prime power range.
///
/// ```
/// use math_prime_util::bigint::family_nth::nth_omega_prime;
/// use num_bigint::BigUint;
/// assert_eq!(nth_omega_prime(2, &BigUint::from(1000_u16)), Some(BigUint::from(2102_u16)));
/// let v = nth_omega_prime(16, &BigUint::from(2_u8)).expect("value");
/// assert_eq!(v.to_string(), "36278497172720993190");
/// ```
#[must_use]
pub fn nth_omega_prime(k: u32, n: &BigUint) -> Option<BigUint> {
    if n.bits() == 0 {
        return None;
    }
    if *n == big(1) {
        return Some(pn_primorial(&big(u64::from(k))));
    }
    match k {
        0 => return None,
        1 => {
            return small(n)
                .filter(|&m| m <= MAX_PRIME_IDX)
                .and_then(crate::nth_prime_power)
                .map(big);
        }
        _ => {}
    }
    if let Some(v) = small(n).and_then(|m| crate::nth_omega_prime(k, m)) {
        return Some(big(v));
    }
    // The native form had no answer, so the value is past u64::MAX (its count
    // there is below n), and past pn_primorial(k), the first member.
    let lo = big(u64::MAX).max(pn_primorial(&big(u64::from(k))));
    let hi = &lo << 1_usize;
    least_reaching(lo, hi, None, n, |x| omega_count(k, x), None)
}

/// The arithmetic of the powerful number recursions: `u128` where the
/// interval fits it (fast), `BigUint` past it. Every product they form is at
/// most the high end, so `u128` does not overflow.
trait Wide: Clone + Ord {
    fn of(v: u64) -> Self;
    /// `floor(self^(1/r))`.
    fn root(&self, r: u32) -> Self;
    /// `floor(self / d)`.
    fn quot(&self, d: &Self) -> Self;
    /// `ceil(self / d)`.
    fn cquot(&self, d: &Self) -> Self;
    /// `self * v^r`.
    fn times_pow(&self, v: &Self, r: u32) -> Self;
    fn succ(&self) -> Self;
    fn rem_small(&self, v: u64) -> u64;
    fn word(&self) -> Option<u64>;
    fn wide(&self) -> BigUint;
}

impl Wide for u128 {
    fn of(v: u64) -> Self {
        Self::from(v)
    }
    fn root(&self, r: u32) -> Self {
        let n = *self;
        match r {
            1 => n,
            2 => n.isqrt(),
            _ => {
                // A double estimate is within one of the root; step to it.
                let fits = |x: Self| x.checked_pow(r).is_some_and(|p| p <= n);
                let mut x = (n as f64).powf(1.0 / f64::from(r)) as Self;
                while fits(x + 1) {
                    x += 1;
                }
                while x > 0 && !fits(x) {
                    x -= 1;
                }
                x
            }
        }
    }
    fn quot(&self, d: &Self) -> Self {
        self / d
    }
    fn cquot(&self, d: &Self) -> Self {
        self.div_ceil(*d)
    }
    fn times_pow(&self, v: &Self, r: u32) -> Self {
        self * v.pow(r)
    }
    fn succ(&self) -> Self {
        self + 1
    }
    fn rem_small(&self, v: u64) -> u64 {
        (self % Self::from(v)) as u64
    }
    fn word(&self) -> Option<u64> {
        u64::try_from(*self).ok()
    }
    fn wide(&self) -> BigUint {
        BigUint::from(*self)
    }
}

impl Wide for BigUint {
    fn of(v: u64) -> Self {
        big(v)
    }
    fn root(&self, r: u32) -> Self {
        rootint(self, r)
    }
    fn quot(&self, d: &Self) -> Self {
        self / d
    }
    fn cquot(&self, d: &Self) -> Self {
        let q = self / d;
        if &q * d == *self { q } else { q + 1_u8 }
    }
    fn times_pow(&self, v: &Self, r: u32) -> Self {
        self * v.pow(r)
    }
    fn succ(&self) -> Self {
        self + 1_u8
    }
    fn rem_small(&self, v: u64) -> u64 {
        small(&(self % v)).unwrap_or(0)
    }
    fn word(&self) -> Option<u64> {
        small(self)
    }
    fn wide(&self) -> BigUint {
        self.clone()
    }
}

/// Return `true` when `gcd(m, v) = 1`.
fn coprime<W: Wide>(m: &W, v: u64) -> bool {
    crate::gcd(m.rem_small(v), v) == 1
}

/// `PP.pm` `_powerful_count_recurse` (MPU `powerful.c` `_pcr`): the
/// `k`-powerful integers at most `n` whose square-free parts of exponents
/// above `r` are fixed in `m`.
fn powerful_rec<W: Wide + std::iter::Sum>(n: &W, k: u32, sf: &SquareFree, m: &W, r: u32) -> W {
    let limit = n.quot(m).root(r);
    if r <= k {
        return limit;
    }
    let top = limit.word().unwrap_or(u64::MAX);
    (1..=top)
        .filter(|&i| sf.test(i) && coprime(m, i))
        .map(|i| {
            let mi = m.times_pow(&W::of(i), r);
            if r - 1 == k {
                n.quot(&mi).root(k)
            } else {
                powerful_rec(n, k, sf, &mi, r - 1)
            }
        })
        .sum()
}

/// The exact count of `k`-powerful integers at most `n`, for `k >= 2`.
fn powerful_count(n: &BigUint, k: u32) -> BigUint {
    if let Some(v) = small(n) {
        return big(crate::powerful_count(v, k));
    }
    let sf = SquareFree::new(small(&rootint(n, k + 1)).unwrap_or(u64::MAX));
    match u128::try_from(n) {
        Ok(n) => powerful_rec(&n, k, &sf, &1, 2 * k - 1).wide(),
        Err(_) => powerful_rec(n, k, &sf, &big(1), 2 * k - 1),
    }
}

/// Return the `n`th `k`-powerful integer: every prime factor appears at
/// least `k` times. One is the first.
///
/// Port of MPU 0.75 `PP.pm` `nth_powerful`. The native
/// [`nth_powerful`](crate::nth_powerful) answers where its index and result
/// fit `u64`. Above, a search with the exact count (`powerful.c`
/// `powerful_count`) narrows the value down to a few members, which
/// [`powerful_numbers`] then lists. `k <= 1` returns `n`, as in MPU.
///
/// Returns `None` for `n = 0`, where MPU returns `undef`. MPU's Perl path
/// dies for `k = 2` and `n >= 2^64` (a `NaN` from its estimate); the port
/// runs the same search there.
///
/// ```
/// use math_prime_util::bigint::family_nth::nth_powerful;
/// use num_bigint::BigUint;
/// assert_eq!(nth_powerful(&BigUint::from(4_u8), 2), Some(BigUint::from(9_u8)));
/// let v = nth_powerful(&BigUint::from(100_u8), 60).expect("value");
/// assert_eq!(v.to_string(), "11972515182562019788602740026717047105681");
/// ```
#[must_use]
pub fn nth_powerful(n: &BigUint, k: u32) -> Option<BigUint> {
    if n.bits() == 0 {
        return None;
    }
    if k <= 1 || *n == big(1) {
        return Some(n.clone());
    }
    if let Some(v) = small(n).and_then(|m| crate::nth_powerful(m, k)) {
        return Some(big(v));
    }
    if *n <= big(3) {
        return Some(big(1) << (k as usize + small(n)? as usize - 2));
    }
    // Past the native range: above u64::MAX, and above 2^k - 1 (only 1 is
    // k-powerful below 2^k).
    let lo = big(u64::MAX).max((big(1) << k as usize) - 1_u8);
    let hi = &lo << 1_usize;
    let list = |a: &BigUint, b: &BigUint| powerful_numbers(a, b, k);
    least_reaching(lo, hi, None, n, |x| Some(powerful_count(x, k)), Some(&list))
}

/// `PP.pm` `_pcg`: push the `k`-powerful integers of `lo..=hi` whose
/// square-free parts of exponents above `r` are fixed in `m`.
fn powerful_gen<W: Wide>(lo: &W, hi: &W, k: u32, sf: &SquareFree, m: &W, r: u32, out: &mut Vec<W>) {
    let end = hi.quot(m).root(r);
    if r <= k {
        let mut v = if lo > m {
            let lom = lo.cquot(m);
            let root = lom.root(r);
            if W::of(1).times_pow(&root, r) == lom {
                root
            } else {
                root.succ()
            }
        } else {
            W::of(1)
        };
        while v <= end {
            out.push(m.times_pow(&v, r));
            v = v.succ();
        }
        return;
    }
    let end = end.word().unwrap_or(u64::MAX);
    for v in (1..=end).filter(|&v| sf.test(v) && coprime(m, v)) {
        powerful_gen(lo, hi, k, sf, &m.times_pow(&W::of(v), r), r - 1, out);
    }
}

/// List the `k`-powerful integers in the inclusive interval `lo..=hi`, in
/// increasing order. `k <= 1` lists every integer of the interval.
///
/// Port of MPU 0.75 `PP.pm` `powerful_numbers` and `_pcg`. The native
/// [`powerful_numbers`](crate::powerful_numbers) answers where `hi` fits
/// `u64`. Above, the generation of `_pcg` runs on `u128` or `BigUint`. Its
/// cost grows with `hi^(1/(k+1))` and with the length of the list, not with
/// the length of the interval: a short interval near `2^64` with `k = 2`
/// takes a cube root's worth of steps. MPU's Perl path dies for most such
/// intervals (it multiplies two native integers into a float); the port
/// computes them.
///
/// ```
/// use math_prime_util::bigint::family_nth::powerful_numbers;
/// use num_bigint::BigUint;
/// let two64 = BigUint::from(1_u8) << 64_usize;
/// let hi = &two64 + 100_u8;
/// assert_eq!(powerful_numbers(&(&two64 - 100_u8), &hi, 2), vec![two64]);
/// ```
#[must_use]
pub fn powerful_numbers(lo: &BigUint, hi: &BigUint, k: u32) -> Vec<BigUint> {
    let lo = lo.max(&big(1)).clone();
    if *hi < lo {
        return Vec::new();
    }
    if let Some(h) = small(hi) {
        let l = small(&lo).unwrap_or(h);
        return crate::powerful_numbers(l, h, k)
            .into_iter()
            .map(big)
            .collect();
    }
    if k <= 1 {
        let mut out = Vec::new();
        let mut v = lo;
        while v <= *hi {
            out.push(v.clone());
            v += 1_u8;
        }
        return out;
    }
    if u64::from(k) >= hi.bits() {
        return if lo == big(1) {
            vec![big(1)]
        } else {
            Vec::new()
        };
    }
    let sf = SquareFree::new(small(&rootint(hi, k + 1)).unwrap_or(u64::MAX));
    let mut out = Vec::new();
    if let (Ok(l), Ok(h)) = (u128::try_from(&lo), u128::try_from(hi)) {
        let mut words = Vec::new();
        powerful_gen(&l, &h, k, &sf, &1, 2 * k - 1, &mut words);
        out.extend(words.iter().map(Wide::wide));
    } else {
        powerful_gen(&lo, hi, k, &sf, &big(1), 2 * k - 1, &mut out);
    }
    out.sort_unstable();
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reduction_table_matches_the_native_rows() {
        // Below A078843[k + 1] the nth (k + 1)-almost prime is twice the nth
        // k-almost prime; at A078843[k + 1] it is not.
        for k in 3..20_u32 {
            let n = A078843[k as usize + 1];
            let twice = |n| crate::nth_almost_prime(k, n).map(|v| v * 2);
            assert_eq!(
                crate::nth_almost_prime(k + 1, n - 1),
                twice(n - 1),
                "k = {k}"
            );
            assert_ne!(crate::nth_almost_prime(k + 1, n), twice(n), "k = {k}");
            assert_eq!(
                kap_reduce_nth(k + 1, &big(n - 1)),
                1 + kap_reduce_nth(k, &big(n - 1))
            );
        }
        // A078843[8] = 64 <= 100 < A078843[9] = 103.
        assert_eq!(kap_reduce_nth(70, &big(100)), 62);
        assert_eq!(kap_reduce_nth(70, &big(A078843[60])), 0);
        assert_eq!(kap_reduce_nth(1, &big(5)), 0);
    }

    #[test]
    fn counts_agree_with_native_below_two_to_64() {
        for x in [0_u64, 1, 7, 8, 100, 1000, 65_536, 1_000_003] {
            let b = big(x);
            for k in 0..6 {
                assert_eq!(
                    kap_count(k, &b),
                    Some(big(crate::almost_prime_count(u64::from(k), x)))
                );
                assert_eq!(
                    omega_count(k, &b),
                    Some(big(crate::omega_prime_count(k, x)))
                );
            }
            for k in 2..6 {
                assert_eq!(powerful_count(&b, k), big(crate::powerful_count(x, k)));
            }
        }
    }

    #[test]
    fn restricted_counts_split_the_prime_factors() {
        // kap_native(y, lo, k) counts products of k primes >= lo.
        // 3 * (3 5 7 11 13 17 19 23 29 31), 5 * (5 7 11 13 17 19), 7 * (7 11 13).
        assert_eq!(kap_native(100, 3, 2), 19);
        assert_eq!(kap_native(30, 3, 1), 9);
        assert_eq!(kap_native(2, 3, 1), 0);
        let y = 1_000_000_u64;
        let mut by_hand = 0;
        for p in crate::primes_in(3..=crate::rootint(y, 3)) {
            by_hand += kap_native(y / p, p, 2);
        }
        assert_eq!(kap_native(y, 3, 3), by_hand);
        assert_eq!(
            kap_native(y, 2, 3),
            kap_native(y / 2, 2, 2) + kap_native(y, 3, 3)
        );
    }

    #[test]
    fn big_counts_need_small_leaves() {
        // A semiprime count past 2^64 needs pi(x / 2) past 2^64.
        let two65 = big(1) << 65_usize;
        assert_eq!(kap_rec(&two65, 2, 2), None);
        assert_eq!(omega_count(2, &two65), None);
        assert_eq!(omega_count(1, &two65), None);
        assert_eq!(omega_count(0, &two65), Some(big(1)));
        assert_eq!(kap_count(0, &two65), Some(big(1)));
        assert_eq!(kap_count(70, &two65), Some(big(0)));
    }

    #[test]
    fn square_free_sieve_and_fallback_agree() {
        let sieve = SquareFree::new(1000);
        let none = SquareFree::new(SIEVE_LIMIT + 1);
        for v in 1..=1000 {
            assert_eq!(sieve.test(v), crate::is_square_free(v), "{v}");
            assert_eq!(none.test(v), crate::is_square_free(v), "{v}");
        }
        assert!(sieve.test(1001));
        assert!(!sieve.test(1003 * 1003));
    }

    #[test]
    fn least_reaching_widens_and_bisects() {
        let id = |x: &BigUint| Some(x.clone());
        assert_eq!(
            least_reaching(big(0), big(0), None, &big(37), id, None),
            Some(big(37))
        );
        assert_eq!(
            least_reaching(big(5), big(6), None, &big(6), id, None),
            Some(big(6))
        );
        assert_eq!(
            least_reaching(big(5), big(8), None, &big(7), |_| None, None),
            None
        );
        // Interpolation on a count of slope 1/3 lands on the answer.
        let third = |x: &BigUint| Some(x / 3_u8);
        for n in [1_u64, 2, 1000, 123_456_789] {
            let want = big(3 * n);
            assert_eq!(
                least_reaching(big(0), big(7), Some(big(0)), &big(n), third, None),
                Some(want)
            );
        }
    }
}

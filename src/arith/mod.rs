//! Totients and multiplicative arithmetic from MPU 0.75.
//! Divisor sums, discriminants, and arithmetic in residue rings.

mod sopfr;
pub use sopfr::sopfr;
mod sopf;
pub use sopf::sopf;
mod kronecker;
pub use kronecker::kronecker;
mod chinese;
pub use chinese::chinese;
mod qnr;
pub use qnr::qnr;
mod is_qr;
pub use is_qr::is_qr;
mod is_fundamental;
pub use is_fundamental::is_fundamental;
mod ramanujan_sum;
pub use ramanujan_sum::ramanujan_sum;
mod valuation;
pub use valuation::valuation;

use crate::{gcd, invmod, is_power, is_prime, is_square, powmod, rootint, sqrtint};
use std::collections::HashMap;

fn factors(mut n: u64) -> Vec<(u64, u32)> {
    fn split(n: u64, out: &mut Vec<u64>) {
        if n <= 1 {
            return;
        }
        if is_prime(n) != 0 {
            out.push(n);
            return;
        }
        // MPU factor.c::prho_factor: batches of 64 Floyd steps and backtracking.
        let (mut u, mut v, mut c) = (7_u64, 7_u64, 1_u64);
        loop {
            let step = |x: u64| {
                u64::try_from((u128::from(x) * u128::from(x) + u128::from(c)) % u128::from(n))
                    .unwrap()
            };
            let (old_u, old_v) = (u, v);
            let mut product = 1;
            for _ in 0..64 {
                u = step(u);
                v = step(step(v));
                product =
                    u64::try_from(u128::from(product) * u128::from(u.abs_diff(v)) % u128::from(n))
                        .unwrap();
            }
            let mut d = gcd(product, n);
            if d == 1 {
                continue;
            }
            if d == n {
                u = old_u;
                v = old_v;
                for _ in 0..=64 {
                    u = step(u);
                    v = step(step(v));
                    d = gcd(u.abs_diff(v), n);
                    if d != 1 {
                        break;
                    }
                }
            }
            if d > 1 && d < n {
                split(d, out);
                split(n / d, out);
                return;
            }
            u = u64::try_from((u128::from(u) + 2) % u128::from(n)).unwrap();
            v = u;
            c += 2;
        }
    }
    if n < 2 {
        return Vec::new();
    }
    let mut flat = Vec::new();
    for p in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        while n % p == 0 {
            flat.push(p);
            n /= p;
        }
    }
    split(n, &mut flat);
    flat.sort_unstable();
    let mut result: Vec<(u64, u32)> = Vec::new();
    for p in flat {
        if let Some((q, e)) = result.last_mut() {
            if *q == p {
                *e += 1;
                continue;
            }
        }
        result.push((p, 1));
    }
    result
}

fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    for p in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43] {
        while n > 1 && n % p == 0 {
            factors.push(p);
            n /= p;
        }
    }
    let mut pending = vec![n];
    while let Some(n) = pending.pop() {
        if n < 2 {
            continue;
        }
        if is_prime(n) != 0 {
            factors.push(n);
            continue;
        }
        let mut a = 1;
        let divisor = loop {
            if let Some(d) = brent(n, a) {
                break d;
            }
            a += 1;
        };
        pending.push(divisor);
        pending.push(n / divisor);
    }
    factors.sort_unstable();
    factors
}

#[allow(clippy::cast_possible_truncation)] // Remainders modulo u64 fit u64.
fn brent(n: u64, mut a: u64) -> Option<u64> {
    let step =
        |x: u64, a: u64| ((u128::from(x) * u128::from(x) + u128::from(a)) % u128::from(n)) as u64;
    let inner = if n <= 4_000_000_000 { 32 } else { 160 };
    let (mut r, mut xi, mut xm) = (1_u64, 1, 1);
    let mut rounds = 500_000;
    let mut fails = 6;
    while rounds > 0 {
        let mut left = r.min(rounds);
        let mut saved = xi;
        let mut f = 1;
        while left > 0 {
            let count = left.min(inner);
            saved = xi;
            left -= count;
            rounds -= count;
            xi = step(xi, a);
            let mut m = xi.abs_diff(xm);
            for _ in 1..count {
                xi = step(xi, a);
                m = (u128::from(m) * u128::from(xi.abs_diff(xm)) % u128::from(n)) as u64;
            }
            f = gcd(m, n);
            if f != 1 {
                break;
            }
        }
        if f == 1 {
            r *= 2;
            xm = xi;
            continue;
        }
        if f == n {
            xi = saved;
            loop {
                xi = step(xi, a);
                f = gcd(xi.abs_diff(xm), n);
                if f != 1 || r == 0 {
                    break;
                }
                r -= 1;
            }
        }
        if f == n || f == 0 {
            if fails == 0 {
                break;
            }
            fails -= 1;
            xm = ((u128::from(xm) + 11) % u128::from(n)) as u64;
            xi = xm;
            a += 1;
        } else {
            return Some(f);
        }
    }
    None
}

fn factor_powers(n: u64) -> Vec<(u64, u32)> {
    let mut result: Vec<(u64, u32)> = Vec::new();
    for p in prime_factors(n) {
        if let Some(last) = result.last_mut() {
            if last.0 == p {
                last.1 += 1;
                continue;
            }
        }
        result.push((p, 1));
    }
    result
}

/// Return Euler's totient. Port of `totients.c::totient` and `_totient_fac`.
/// ```
/// assert_eq!(math_prime_util::euler_phi(36), 12);
/// ```
pub fn euler_phi(n: u64) -> u64 {
    factors(n).into_iter().fold(n, |v, (p, _)| v / p * (p - 1))
}

fn range_len(lo: u64, hi: u64) -> usize {
    usize::try_from(
        hi.checked_sub(lo)
            .and_then(|v| v.checked_add(1))
            .expect("invalid or oversized range"),
    )
    .expect("range fits usize")
}

/// Return totients on an inclusive interval. Port of `totients.c::range_totient`.
/// # Panics
/// Panics if the interval is reversed or too large to allocate.
/// ```
/// assert_eq!(math_prime_util::euler_phi_range(0, 4), [0,1,1,2,2]);
/// ```
pub fn euler_phi_range(lo: u64, hi: u64) -> Vec<u64> {
    let len = range_len(lo, hi);
    if lo == 0 {
        let mut t = vec![0; len];
        if hi == 0 {
            return t;
        }
        t[1] = 1;
        let mut primes = Vec::new();
        for i in (1..=hi / 2).step_by(2) {
            let ix = usize::try_from(i).unwrap();
            if t[ix] == 0 {
                t[ix] = i - 1;
                if i <= sqrtint(hi) {
                    primes.push(i);
                }
            }
            let v = t[ix];
            for &p in &primes {
                if p > hi / i {
                    break;
                }
                let j = usize::try_from(i * p).unwrap();
                if i % p == 0 {
                    t[j] = v * p;
                    break;
                }
                t[j] = v * (p - 1);
            }
            t[ix * 2] = v;
            if i < hi / 2 {
                t[ix * 2 + 2] = t[ix + 1] * 2;
            }
        }
        for i in (1..len).step_by(2) {
            if t[i] == 0 {
                t[i] = u64::try_from(i).unwrap() - 1;
            }
        }
        t
    } else {
        // Ranged factoring: sieve small factors, then factor each remaining cofactor.
        let mut rem: Vec<u64> = (lo..=hi).collect();
        let mut t = rem.clone();
        for p in crate::sieve::primes(2, sqrtint(hi).min(1_000_000)) {
            let offset = (p - lo % p) % p;
            if offset > hi - lo {
                continue;
            }
            for i in (usize::try_from(offset).unwrap()..len).step_by(usize::try_from(p).unwrap()) {
                t[i] = t[i] / p * (p - 1);
                while rem[i] % p == 0 {
                    rem[i] /= p;
                }
            }
        }
        for (r, v) in rem.into_iter().zip(&mut t) {
            for (p, _) in factors(r) {
                *v = *v / p * (p - 1);
            }
        }
        t
    }
}

/// Return the Jordan totient, or `None` on overflow.
/// Port of `totients.c::jordan_totient`; checked powers also cover the Perl fallback.
/// ```
/// assert_eq!(math_prime_util::jordan_totient(2, 12), Some(96));
/// ```
pub fn jordan_totient(k: u64, n: u64) -> Option<u64> {
    if k == 0 || n <= 1 {
        return Some(u64::from(n == 1));
    }
    if n == 2 {
        // 2^k - 1 is computed in u128 so that k = 64 (result u64::MAX) does not overflow.
        let v = 2_u128.checked_pow(u32::try_from(k).ok()?)? - 1;
        return u64::try_from(v).ok();
    }
    let k = u32::try_from(k).ok()?;
    let mut v = 1_u64;
    for (p, e) in factors(n) {
        let pk = p.checked_pow(k)?;
        v = v.checked_mul(pk - 1)?;
        for _ in 1..e {
            v = v.checked_mul(pk)?;
        }
    }
    Some(v)
}

/// Return Carmichael's lambda. Port of `totients.c::carmichael_lambda`.
/// ```
/// assert_eq!(math_prime_util::carmichael_lambda(24), 2);
/// ```
#[allow(clippy::missing_panics_doc)] // The index is bounded by n < 8.
pub fn carmichael_lambda(n: u64) -> u64 {
    if n < 8 {
        return [0, 1, 1, 2, 2, 4, 2, 6][usize::try_from(n).unwrap()];
    }
    factors(n).into_iter().fold(1, |v, (p, e)| {
        let mut term = p.pow(e - 1) * (p - 1);
        if p == 2 && e > 2 {
            term /= 2;
        }
        v / gcd(v, term) * term
    })
}

/// Return the Moebius function. Port of `moebius.c::moebius`.
/// ```
/// assert_eq!(math_prime_util::moebius(30), -1);
/// ```
#[allow(clippy::missing_panics_doc)] // Both conversions are bounded by the small table.
pub fn moebius(n: u64) -> i8 {
    const SMALL: [u32; 16] = [
        2_703_565_065,
        23_406_865,
        620_863_913,
        1_630_114_197,
        157_354_249,
        2_844_895_525,
        2_166_423_889,
        363_177_345,
        2_835_441_929,
        2_709_852_521,
        1_095_049_497,
        92_897_577,
        1_772_687_649,
        162_113_833,
        160_497_957,
        689_538_385,
    ];
    if n < 256 {
        return i8::try_from((SMALL[usize::try_from(n / 16).unwrap()] >> (2 * (n % 16))) & 3)
            .unwrap()
            - 1;
    }
    for p in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n % (p * p) == 0 {
            return 0;
        }
    }
    let f = factors(n);
    if f.iter().any(|&(_, e)| e > 1) {
        0
    } else if f.len() % 2 == 0 {
        1
    } else {
        -1
    }
}

/// Return Moebius values on an inclusive interval.
/// Port of `moebius.c::range_moebius`, `_sieve_moebius`, and `_moebius_by_hand`.
/// # Panics
/// Panics if the interval is reversed or too large to allocate.
/// ```
/// assert_eq!(math_prime_util::moebius_range(1, 4), [1,-1,-1,0]);
/// ```
pub fn moebius_range(lo: u64, hi: u64) -> Vec<i8> {
    let len = range_len(lo, hi);
    let width = hi - lo;
    if hi < 100 || width < 10 || (hi > 1 << 25 && width < rootint(hi, 3) / 4) {
        return (lo..=hi).map(moebius).collect();
    }
    let mut mu = vec![0_u8; len];
    let (mut logp, mut nextlog) = (1_u8, 3_u64);
    let root = sqrtint(hi);
    for p in crate::sieve::primes(2, root + u64::from(root * root != hi)) {
        if p > nextlog {
            logp += 2;
            nextlog = (nextlog - 1) * 4 + 1;
        }
        let start = ((p - lo % p) % p).max(if lo == 0 { p } else { 0 });
        for i in (usize::try_from(start).unwrap()..len).step_by(usize::try_from(p).unwrap()) {
            mu[i] = mu[i].wrapping_add(logp);
        }
        let p2 = p * p;
        let start = ((p2 - lo % p2) % p2).max(if lo == 0 { p2 } else { 0 });
        for i in (usize::try_from(start).unwrap()..len).step_by(usize::try_from(p2).unwrap()) {
            mu[i] = 128;
        }
    }
    mu.into_iter()
        .enumerate()
        .map(|(i, a)| {
            let n = lo + u64::try_from(i).unwrap();
            if n == 0 || a & 128 != 0 {
                0
            } else {
                let sign = if a & 1 == 0 { 1 } else { -1 };
                if u32::from(a) >= n.ilog2() {
                    sign
                } else {
                    -sign
                }
            }
        })
        .collect()
}

mod mertens;
pub use mertens::mertens;
mod liouville;
pub use liouville::liouville;
mod sumliouville;
pub use sumliouville::sumliouville;
mod exp_mangoldt;
pub use exp_mangoldt::exp_mangoldt;

/// Sum Euler's totient through `n`, or return `None` on native overflow.
/// Port of `totients.c::sumtotient` and `_sumt`.
/// ```
/// assert_eq!(math_prime_util::sumtotient(10), Some(32));
/// ```
pub fn sumtotient(n: u64) -> Option<u64> {
    fn sumt(n: u64, prefix: &[u64], cache: &mut HashMap<u64, u64>) -> u64 {
        if let Ok(i) = usize::try_from(n) {
            if let Some(&v) = prefix.get(i) {
                return v;
            }
        }
        if let Some(&v) = cache.get(&n) {
            return v;
        }
        let s = sqrtint(n);
        let lim = n / (s + 1);
        let mut v = u128::from(n) * u128::from(n + 1) / 2 - u128::from(n - n / 2);
        for k in 2..=lim {
            v -= u128::from(sumt(n / k, prefix, cache));
            v -= u128::from(n / k - n / (k + 1)) * u128::from(sumt(k, prefix, cache));
        }
        if s > lim {
            v -= u128::from(n / s - n / (s + 1)) * u128::from(sumt(s, prefix, cache));
        }
        let v = u64::try_from(v).unwrap();
        cache.insert(n, v);
        v
    }
    if n > 7_790_208_950 {
        return None;
    }
    let mut prefix = euler_phi_range(0, rootint(n, 3).pow(2).max(2));
    for i in 2..prefix.len() {
        prefix[i] += prefix[i - 1];
    }
    Some(sumt(n, &prefix, &mut HashMap::new()))
}

/// Return Dedekind's psi, or `None` on overflow.
/// Port of `totients.c::dedekind_psi`.
/// ```
/// assert_eq!(math_prime_util::dedekind_psi(12), Some(24));
/// ```
pub fn dedekind_psi(n: u64) -> Option<u64> {
    if n == 0 {
        return Some(0);
    }
    let mut v = 1_u64;
    for (p, e) in factors(n) {
        v = v
            .checked_mul(p.checked_add(1)?)?
            .checked_mul(p.pow(e - 1))?;
    }
    Some(v)
}

fn divisors(n: u64) -> Vec<u64> {
    let mut d = vec![1];
    for (p, e) in factors(n) {
        let len = d.len();
        let mut power = 1;
        for _ in 0..e {
            power *= p;
            for i in 0..len {
                d.push(d[i] * power);
            }
        }
    }
    d.sort_unstable();
    d
}

/// List all native integers with totient `n`.
/// Port of `totients.c::inverse_totient_list`; returns an empty list if none exist.
/// ```
/// assert_eq!(math_prime_util::inverse_totient(4), [5,8,10,12]);
/// ```
pub fn inverse_totient(n: u64) -> Vec<u64> {
    if n == 1 {
        return vec![1, 2];
    }
    if n == 0 || n % 2 == 1 {
        return Vec::new();
    }
    let divs = divisors(n);
    let mut sets: HashMap<u64, Vec<u64>> = HashMap::from([(1, vec![1])]);
    for &d in divs.iter().rev() {
        let Some(p) = d.checked_add(1) else {
            continue;
        };
        if is_prime(p) == 0 {
            continue;
        }
        let (mut dp, mut pp) = (d, p);
        let mut additions: HashMap<u64, Vec<u64>> = HashMap::new();
        loop {
            let nd = n / dp;
            for &d2 in &divs {
                if d2 > nd {
                    break;
                }
                if nd % d2 != 0 || (d == 1 && d2 * dp != n) {
                    continue;
                }
                if let Some(vals) = sets.get(&d2) {
                    for &v in vals {
                        if let Some(v) = v.checked_mul(pp) {
                            additions.entry(d2 * dp).or_default().push(v);
                        }
                    }
                }
            }
            if nd % p != 0 {
                break;
            }
            dp *= p;
            let Some(next) = pp.checked_mul(p) else {
                break;
            };
            pp = next;
        }
        for (k, v) in additions {
            sets.entry(k).or_default().extend(v);
        }
    }
    let mut result = sets.remove(&n).unwrap_or_default();
    result.sort_unstable();
    result.dedup();
    result
}

fn totpred(n: u64, maxd: u64) -> bool {
    if n & 1 == 1 {
        return false;
    }
    if n.is_power_of_two() {
        return true;
    }
    let n = n / 2;
    if n < maxd
        && n.checked_mul(2)
            .and_then(|v| v.checked_add(1))
            .is_some_and(|p| is_prime(p) != 0)
    {
        return true;
    }
    for d in divisors(n) {
        if d >= maxd {
            break;
        }
        let Some(p) = d.checked_mul(2).and_then(|v| v.checked_add(1)) else {
            continue;
        };
        if is_prime(p) == 0 {
            continue;
        }
        let mut r = n / d;
        loop {
            if r == p || totpred(r, d) {
                return true;
            }
            if r % p != 0 {
                break;
            }
            r /= p;
        }
    }
    false
}

/// Test whether `n` is a totient. Port of `totients.c::is_totient` and `_totpred`.
/// ```
/// assert!(math_prime_util::is_totient(12));
/// ```
pub fn is_totient(n: u64) -> bool {
    if n == 0 || n & 1 == 1 {
        n == 1
    } else {
        totpred(n, n)
    }
}

fn sigma(n: u64, k: u64) -> Option<u128> {
    if n <= 1 {
        return Some(u128::from(n));
    }
    let mut product = 1_u128;
    for (p, e) in factor_powers(n) {
        if k == 0 {
            product *= u128::from(e + 1);
            continue;
        }
        let pk = u128::from(p).checked_pow(u32::try_from(k).ok()?)?;
        let (mut power, mut sum) = (1_u128, 1_u128);
        for _ in 0..e {
            power = power.checked_mul(pk)?;
            sum = sum.checked_add(power)?;
        }
        product = product.checked_mul(sum)?;
    }
    Some(product)
}

/// Return the sum of the k-th powers of the divisors of n.
/// Return `None` if the result exceeds u64.
///
/// Port of MPU 0.75 `factor.c::divisor_sum`.
///
/// ```
/// assert_eq!(math_prime_util::divisor_sum(12, 1), Some(28));
/// ```
pub fn divisor_sum(n: u64, k: u64) -> Option<u64> {
    u64::try_from(sigma(n, k)?).ok()
}

const TAU: &[i64] = &[
    0,
    1,
    -24,
    252,
    -1472,
    4830,
    -6048,
    -16_744,
    84_480,
    -113_643,
    -115_920,
    534_612,
    -370_944,
    -577_738,
    401_856,
    1_217_160,
    987_136,
    -6_905_934,
    2_727_432,
    10_661_420,
    -7_109_760,
    -4_219_488,
    -12_830_688,
    18_643_272,
    21_288_960,
    -25_499_225,
    13_865_712,
    -73_279_080,
    24_647_168,
    128_406_630,
    -29_211_840,
    -52_843_168,
    -196_706_304,
    134_722_224,
    165_742_416,
    -80_873_520,
    167_282_496,
    -182_213_314,
    -255_874_080,
    -145_589_976,
    408_038_400,
    308_120_442,
    101_267_712,
    -17_125_708,
    -786_948_864,
    -548_895_690,
    -447_438_528,
];

/// Return the native Ramanujan tau value.
/// Return `None` if the native calculation overflows.
///
/// Port of MPU 0.75 `util.c::ramanujan_tau` and `PP.pm::_tauprime`, `_taupower`.
///
/// ```
/// assert_eq!(math_prime_util::ramanujan_tau(2), Some(-24));
/// ```
pub fn ramanujan_tau(n: u64) -> Option<i64> {
    if let Some(&value) = TAU.get(usize::try_from(n).ok()?) {
        return Some(value);
    }
    let mut result = 1_i128;
    for (p, e) in factor_powers(n) {
        let tp = tau_prime(p)?;
        let p11 = i128::from(p).checked_pow(11)?;
        let (mut previous, mut current) = (1_i128, tp);
        for _ in 1..e {
            let next = tp
                .checked_mul(current)?
                .checked_sub(p11.checked_mul(previous)?)?;
            (previous, current) = (current, next);
        }
        result = result.checked_mul(current)?;
    }
    i64::try_from(result).ok()
}

// MPU PP.pm::_tauprime and _taupower supply values beyond util.c's table.
fn tau_prime(p: u64) -> Option<i128> {
    if let Some(&value) = TAU.get(usize::try_from(p).ok()?) {
        return Some(i128::from(value));
    }
    let x = i128::from(p);
    let leading = 28_i128
        .checked_mul(x.checked_pow(6)?)?
        .checked_sub(28_i128.checked_mul(x.checked_pow(5)?)?)?
        .checked_sub(90_i128.checked_mul(x.checked_pow(4)?)?)?
        .checked_sub(35_i128.checked_mul(x.checked_pow(3)?)?)?
        .checked_sub(1)?;
    let mut sum = 0_i128;
    for t in 1..=crate::sqrtint(p) {
        let t2 = t * t;
        let v = p - t2;
        let h = match v % 4 {
            0 => 2 * class_number(v) + class_number(4 * v),
            3 => (if v % 8 == 3 { 6 } else { 4 }) * class_number(v),
            _ => class_number(4 * v),
        };
        let t2 = i128::from(t2);
        let term = t2
            .checked_pow(3)?
            .checked_mul(4 * t2 * t2 - 9 * x * t2 + 7 * x * x)?
            .checked_mul(h)?;
        sum = sum.checked_add(term)?;
    }
    leading.checked_sub(32_i128.checked_mul(sum / 3)?)
}
// util.c::hclassno and _count_class_div, in the bounded tau-prime path.
fn class_number(n: u64) -> i128 {
    if n == 0 {
        return -1;
    }
    if matches!(n % 4, 1 | 2) {
        return 0;
    }
    if n == 3 {
        return 4;
    }
    let mut b = n & 1;
    let mut b2 = (n + 1) >> 2;
    let square = is_square(b2);
    let mut h = divisor_sum(b2, 0).unwrap_or(0) >> 1;
    if b == 1 {
        h = 1 + u64::from(square) + 2 * (h - 1);
    }
    b += 2;
    loop {
        b2 = (n + b * b) >> 2;
        if 3 * b2 >= n {
            break;
        }
        let limit = crate::sqrtint(b2) - u64::from(is_square(b2));
        let divisors = (b + 1..=limit).filter(|d| b2 % d == 0).count();
        h += u64::from(b2 % b == 0)
            + u64::from(is_square(b2))
            + 2 * u64::try_from(divisors).unwrap_or(0);
        b += 2;
    }
    let correction = if b2 * 3 == n {
        4
    } else if square && n % 2 == 0 {
        6
    } else {
        0
    };
    i128::from(12 * h + correction)
}

/// Return the sum of the proper divisors of n.
/// Return `None` if the result exceeds u64.
///
/// Port of MPU 0.75 `factor.c::aliquot_sum`.
///
/// ```
/// assert_eq!(math_prime_util::aliquot_sum(12), Some(16));
/// ```
pub fn aliquot_sum(n: u64) -> Option<u64> {
    if n <= 1 {
        return Some(0);
    }
    u64::try_from(sigma(n, 1)? - u128::from(n)).ok()
}

/// Return the aliquot sum minus n as a native signed word.
/// MPU converts the difference to i64 with wraparound.
/// Return `None` if the aliquot sum exceeds u64.
///
/// Port of MPU 0.75 `XS.xs::abundance` and `factor.c::aliquot_sum`.
///
/// ```
/// assert_eq!(math_prime_util::abundance(12), Some(4));
/// ```
pub fn abundance(n: u64) -> Option<i64> {
    let sum = aliquot_sum(n)?;
    Some(i64::from_ne_bytes(sum.wrapping_sub(n).to_ne_bytes()))
}

fn kronecker_unsigned(a: u64, mut b: u64) -> i32 {
    if b == 0 {
        return i32::from(a == 1);
    }
    let mut s = 1;
    let r = b.trailing_zeros();
    if r > 0 {
        if a % 2 == 0 {
            return 0;
        }
        if r % 2 == 1 && matches!(a % 8, 3 | 5) {
            s = -s;
        }
        b >>= r;
    }
    kronecker_odd(a, b, s)
}
fn kronecker_odd(mut a: u64, mut b: u64, mut s: i32) -> i32 {
    while a != 0 {
        let r = a.trailing_zeros();
        if r % 2 == 1 && matches!(b % 8, 3 | 5) {
            s = -s;
        }
        a >>= r;
        if a & b & 2 != 0 {
            s = -s;
        }
        (a, b) = (b % a, a);
    }
    if b == 1 { s } else { 0 }
}

/// Return the least solution and the combined modulus.
/// Return `None` for a zero modulus, inconsistent inputs, or modulus overflow.
/// An empty input gives `(0, 0)`.
///
/// Port of MPU 0.75 `util.c::chinese and _simple_chinese`.
///
/// ```
/// assert_eq!(math_prime_util::chinese2(&[(2, 3), (3, 5)]), Some((8, 15)));
/// ```
pub fn chinese2(congruences: &[(u64, u64)]) -> Option<(u64, u64)> {
    if congruences.is_empty() {
        return Some((0, 0));
    }
    // With two or more pairs, every input modulus must stay in the signed range (native C path).
    if congruences.len() > 1 && congruences.iter().any(|&(_, m)| m > u64::MAX >> 1) {
        return None;
    }
    let mut pairs = congruences.to_vec();
    pairs.sort_unstable_by_key(|a| std::cmp::Reverse(a.1));
    let (mut r, mut m) = pairs[0];
    if m == 0 {
        return None;
    }
    r %= m;
    for &(a, n) in &pairs[1..] {
        if n == 0 {
            return None;
        }
        let g = gcd(m, n);
        if r % g != a % g {
            return None;
        }
        let ng = n / g;
        let modulus = m.checked_mul(ng)?;
        let delta = (i128::from(a) - i128::from(r)) / i128::from(g);
        let delta = u64::try_from(delta.rem_euclid(i128::from(ng))).ok()?;
        let t = crate::mulmod(delta, invmod(m / g, ng)?, ng)?;
        r = u64::try_from((u128::from(r) + u128::from(m) * u128::from(t)) % u128::from(modulus))
            .ok()?;
        m = modulus;
    }
    Some((r, m))
}

fn lambda(n: u64) -> u64 {
    let mut result = 1;
    for (p, e) in factor_powers(n) {
        let term = if p == 2 && e > 2 {
            1 << (e - 2)
        } else {
            (p - 1) * p.pow(e - 1)
        };
        result = result / gcd(result, term) * term;
    }
    result
}

/// Return the multiplicative order of a modulo n.
/// Return `None` if n is zero or a is not a unit modulo n.
///
/// Port of MPU 0.75 `util.c::znorder`.
///
/// ```
/// assert_eq!(math_prime_util::znorder(2, 7), Some(3));
/// ```
pub fn znorder(a: u64, n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    if n == 1 {
        return Some(1);
    }
    if gcd(a, n) != 1 {
        return None;
    }
    let mut k = lambda(n);
    for (p, e) in factor_powers(k) {
        k /= p.pow(e);
        let mut a1 = powmod(a, k, n)?;
        while a1 != 1 {
            k *= p;
            a1 = powmod(a1, p, n)?;
        }
    }
    Some(k)
}

/// Return the least primitive root modulo n.
/// Return `None` if n is zero or no root exists.
///
/// Port of MPU 0.75 `util.c::znprimroot`.
///
/// ```
/// assert_eq!(math_prime_util::znprimroot(7), Some(3));
/// ```
pub fn znprimroot(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    if n <= 4 {
        return Some(n - 1);
    }
    if n % 4 == 0 {
        return None;
    }
    let even = n % 2 == 0;
    let odd = if even { n / 2 } else { n };
    let factors = factor_powers(odd);
    if factors.len() != 1 {
        return None;
    }
    let (p, e) = factors[0];
    if p == 3 && even {
        return Some(5);
    }
    let phi = p - 1;
    let phif = factor_powers(phi);
    for a in 2..p {
        if (even && a % 2 == 0) || matches!(a, 4 | 8 | 9) {
            continue;
        }
        if kronecker_unsigned(a, p) != -1 {
            continue;
        }
        if phif
            .iter()
            .skip(1)
            .any(|&(q, _)| powmod(a, phi / q, p) == Some(1))
        {
            continue;
        }
        if e == 1 || powmod(a, phi, p * p) != Some(1) {
            return Some(a);
        }
    }
    None
}

/// Test whether a is a primitive root modulo n.
/// Return `None` if n is zero.
///
/// Port of MPU 0.75 `util.c::is_primitive_root`.
///
/// ```
/// assert_eq!(math_prime_util::is_primitive_root(3, 7), Some(true));
/// ```
pub fn is_primitive_root(a: u64, n: u64) -> Option<bool> {
    if n == 0 {
        return None;
    }
    if n == 1 {
        return Some(true);
    }
    let a = a % n;
    if a <= 1 {
        return Some(a == 1 && n <= 2);
    }
    if n <= 4 {
        return Some(a == n - 1);
    }
    if n % 4 == 0 || (n % 2 == 0 && a % 2 == 0) || is_square(a) || gcd(a, n) != 1 {
        return Some(false);
    }
    let odd = if n % 2 == 0 { n / 2 } else { n };
    let f = factor_powers(odd);
    if f.len() != 1 {
        return Some(false);
    }
    let (p, e) = f[0];
    if e > 1 && powmod(a, p - 1, p * p) == Some(1) {
        return Some(false);
    }
    if kronecker_unsigned(a, p) != -1 {
        return Some(false);
    }
    let phi = p - 1;
    if is_power(a, 3) && gcd(3, phi) != 1 {
        return Some(false);
    }
    Some(
        factor_powers(phi)
            .iter()
            .all(|&(q, _)| powmod(a, phi / q, p) != Some(1)),
    )
}

mod inverse_sigma0;
pub use inverse_sigma0::*;


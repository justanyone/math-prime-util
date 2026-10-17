//! Native modular roots from MPU 0.75 `rootmod.c`.
use super::{addmod_u, invmod, mulmod_u, powmod_u};
use crate::{gcd, is_prime, sqrtint};

/// Divide `a` by `b` modulo `n`. Return `None` if `b` has no inverse.
/// Port of MPU `util.c`, `divmod`.
///
/// ```
/// assert_eq!(math_prime_util::divmod(6, 2, 7), Some(3));
/// assert_eq!(math_prime_util::divmod(6, 2, 8), None);
/// ```
pub fn divmod(a: u64, b: u64, n: u64) -> Option<u64> {
    invmod(b, n).map(|v| mulmod_u(a, v, n))
}

fn sub(a: u64, b: u64, n: u64) -> u64 {
    addmod_u(a % n, n - b % n, n)
}

fn gcddiv(a: u64, b: u64, n: u64) -> u64 {
    let g = gcd(a, b);
    if g == 0 { return 0; }
    divmod(a / g, b / g, n).unwrap_or(0)
}

// Factor only after the primality test, as in MPU factorint.
fn factors(mut n: u64) -> Vec<(u64, u32)> {
    let mut out = Vec::new();
    let mut p = 2;
    while n > 1 {
        if is_prime(n) != 0 { out.push((n, 1)); break; }
        if n % p == 0 {
            let mut e = 0;
            while n % p == 0 { n /= p; e += 1; }
            out.push((p, e));
        }
        p = if p == 2 { 3 } else { p + 2 };
    }
    out
}

// rootmod.c, _sqrtmod_prime (native arithmetic path).
fn sqrt_prime(a: u64, p: u64) -> u64 {
    if a <= 1 || p == 2 { return a; }
    if p % 4 == 3 { return powmod_u(a, (p >> 2) + 1, p); }
    if p % 8 == 5 {
        let a2 = addmod_u(a, a, p);
        let alpha = powmod_u(a2, (p - 5) >> 3, p);
        let beta = mulmod_u(a2, mulmod_u(alpha, alpha, p), p);
        return mulmod_u(alpha, mulmod_u(a, sub(beta, 1, p), p), p);
    }
    if p % 16 == 9 {
        let a2 = addmod_u(a, a, p);
        let mut alpha = powmod_u(a2, (p - 9) >> 4, p);
        let mut beta = mulmod_u(a2, mulmod_u(alpha, alpha, p), p);
        let mut d = 1;
        if mulmod_u(beta, beta, p) != p - 1 {
            d = 3;
            while powmod_u(d, (p - 1) / 2, p) != p - 1 { d += 2; }
            alpha = mulmod_u(alpha, powmod_u(d, (p - 9) >> 3, p), p);
            beta = mulmod_u(a2, mulmod_u(mulmod_u(d, d, p), mulmod_u(alpha, alpha, p), p), p);
        }
        return mulmod_u(alpha, mulmod_u(a, mulmod_u(d, sub(beta, 1, p), p), p), p);
    }
    if powmod_u(a, (p - 1) / 2, p) != 1 { return 0; }
    let mut q = p - 1;
    let mut r = q.trailing_zeros();
    q >>= r;
    let mut t = 3;
    while powmod_u(t, (p - 1) / 2, p) != p - 1 { t += 2; }
    let mut z = powmod_u(t, q, p);
    let mut b = powmod_u(a, q, p);
    let mut x = powmod_u(a, q.div_ceil(2), p);
    while b != 1 {
        t = b;
        let mut m = 0;
        while m < r && t != 1 { t = mulmod_u(t, t, p); m += 1; }
        if m >= r { break; }
        t = powmod_u(z, 1 << (r - m - 1), p);
        x = mulmod_u(x, t, p);
        z = mulmod_u(t, t, p);
        b = mulmod_u(b, z, p);
        r = m;
    }
    x
}

// rootmod.c, _sqrtmod_prime_power.
fn sqrt_power(a: u64, p: u64, e: u32) -> Option<u64> {
    if e == 0 { return Some(0); }
    let n = p.pow(e);
    let a = a % n;
    if e == 1 {
        let r = sqrt_prime(a, p);
        return (mulmod_u(r, r, p) == a).then_some(r.min(p - r));
    }
    if a == 0 { return Some(0); }
    if a % (p * p) == 0 { return sqrt_power(a / (p * p), p, e - 2).map(|r| r * p); }
    if a % p == 0 { return None; }
    let ered = if p > 2 || e < 5 { e.div_ceil(2) } else { (e + 3) / 2 };
    let s = sqrt_power(a, p, ered)?;
    let np = if p == 2 { n.checked_mul(p).unwrap_or(n) } else { n };
    let r = addmod_u(s, gcddiv(sub(a, mulmod_u(s, s, np), np), addmod_u(s, s, np), n), n);
    let r = r.min(n - r);
    (mulmod_u(r, r, n) == a).then_some(r)
}

fn crt(r: u64, n: u64, s: u64, m: u64) -> u64 {
    let t = mulmod_u(invmod(n, m).unwrap_or(0), sub(s, r, m), m);
    r + n * t
}

/// Return a square root modulo `n`, or `None` if none exists.
/// Port of MPU `rootmod.c`, `sqrtmod` and `_sqrtmod_composite`.
///
/// ```
/// assert_eq!(math_prime_util::sqrtmod(10, 13), Some(6));
/// assert_eq!(math_prime_util::sqrtmod(3, 7), None);
/// ```
pub fn sqrtmod(a: u64, n: u64) -> Option<u64> {
    if n == 0 { return None; }
    let a = a % n;
    if a <= 1 || n <= 2 { return Some(a); }
    let root = sqrtint(a);
    if root * root == a { return Some(root); }
    let mut r = 0;
    let mut modulus = 1;
    for (p, e) in factors(n) {
        let m = p.pow(e);
        r = crt(r, modulus, sqrt_power(a, p, e)?, m);
        modulus *= m;
    }
    let r = r.min(n - r);
    (mulmod_u(r, r, n) == a).then_some(r)
}

const MAX_ROOTS: usize = 600_000_000;
fn push_root(roots: &mut Vec<u64>, r: u64) {
    assert!(roots.len() < MAX_ROOTS, "Maximum returned roots exceeded");
    roots.push(r);
}

// rootmod.c, _allsqrtmodpk (Hugo van der Sanden's algorithm).
fn all_sqrt_power(a: u64, p: u64, k: u32) -> Vec<u64> {
    let pk = p.pow(k);
    let a = a % pk;
    let mut roots = Vec::new();
    if a % p == 0 {
        if a == 0 {
            let low = p.pow(k / 2);
            let high = p.pow(k.div_ceil(2));
            assert!(low <= MAX_ROOTS as u64, "Maximum returned roots exceeded");
            return (0..low).map(|i| high * i).collect();
        }
        if (a / p) % p != 0 { return roots; }
        for r in all_sqrt_power(a / p / p, p, k - 2) {
            for j in 0..p { push_root(&mut roots, r * p + j * (pk / p)); }
        }
        return roots;
    }
    if let Some(q) = sqrt_power(a, p, k) {
        roots.push(q);
        if p != 2 || k > 1 { roots.push(pk - q); }
        if p == 2 && k > 2 {
            let q2 = mulmod_u(q, pk / p - 1, pk);
            roots.extend([q2, pk - q2]);
        }
    }
    roots
}

/// Return all square roots modulo `n`, in ascending order.
/// Port of MPU `rootmod.c`, `allsqrtmod` and `_allsqrtmodpk`.
///
/// # Panics
/// Panics if the number of roots exceeds MPU's limit of 600 million.
///
/// ```
/// assert_eq!(math_prime_util::allsqrtmod(1, 8), vec![1, 3, 5, 7]);
/// ```
pub fn allsqrtmod(a: u64, n: u64) -> Vec<u64> {
    if n == 0 { return Vec::new(); }
    if n <= 2 { return vec![a % n]; }
    let mut roots = vec![0];
    let mut modulus = 1;
    for (p, e) in factors(n) {
        let m = p.pow(e);
        let other = all_sqrt_power(a, p, e);
        let mut next = Vec::new();
        for r in roots { for &s in &other { push_root(&mut next, crt(r, modulus, s, m)); } }
        roots = next;
        if roots.is_empty() { break; }
        modulus *= m;
    }
    roots.sort_unstable();
    roots
}

fn corn_one(u: u64, d: u64, p: u64) -> Option<(u64, u64)> {
    let mut a = p;
    let mut b = u.max(p - u);
    let limit = sqrtint(p);
    while b > limit { (a, b) = (b, a % b); }
    let t = p - b * b;
    if t % d != 0 || t == 0 { return None; }
    let y = sqrtint(t / d);
    (y * y == t / d).then_some((b, y))
}

/// Solve `x*x + d*y*y = n` in nonnegative integers.
/// Port of MPU `util.c`, `cornacchia`, `corn_one`, and `corn_all`.
///
/// ```
/// assert_eq!(math_prime_util::cornacchia(5, 29), Some((3, 2)));
/// ```
pub fn cornacchia(d: u64, n: u64) -> Option<(u64, u64)> {
    if n == 0 { return Some((0, 0)); }
    if d == 0 {
        let x = sqrtint(n);
        return (x * x == n).then_some((x, 0));
    }
    let negd = sub(0, d, n);
    if is_prime(n) != 0 { return corn_one(sqrtmod(negd, n)?, d, n); }
    if n >> 53 != 0 {
        let roots = allsqrtmod(negd, n);
        for &r in &roots[..roots.len() / 2] {
            if let Some(pair) = corn_one(r, d, n) { return Some(pair); }
        }
    }
    for y in 0..=sqrtint(n / d) {
        let t = n - d * y * y;
        let x = sqrtint(t);
        if x * x == t { return Some((x, y)); }
    }
    None
}

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
    if g == 0 {
        return 0;
    }
    divmod(a / g, b / g, n).unwrap_or(0)
}

// Factor only after the primality test, as in MPU factorint.
fn factors(mut n: u64) -> Vec<(u64, u32)> {
    let mut out = Vec::new();
    if n > 1 && is_prime(n) != 0 {
        return vec![(n, 1)];
    }
    let mut p = 2;
    while n > 1 {
        if n % p == 0 {
            let mut e = 0;
            while n % p == 0 {
                n /= p;
                e += 1;
            }
            out.push((p, e));
            if n > 1 && is_prime(n) != 0 {
                out.push((n, 1));
                break;
            }
        }
        p = if p == 2 { 3 } else { p + 2 };
    }
    out
}

// rootmod.c, _sqrtmod_prime (native arithmetic path).
fn sqrt_prime(a: u64, p: u64) -> u64 {
    if a <= 1 || p == 2 {
        return a;
    }
    if p % 4 == 3 {
        return powmod_u(a, (p >> 2) + 1, p);
    }
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
            while powmod_u(d, (p - 1) / 2, p) != p - 1 {
                d += 2;
            }
            alpha = mulmod_u(alpha, powmod_u(d, (p - 9) >> 3, p), p);
            beta = mulmod_u(
                a2,
                mulmod_u(mulmod_u(d, d, p), mulmod_u(alpha, alpha, p), p),
                p,
            );
        }
        return mulmod_u(alpha, mulmod_u(a, mulmod_u(d, sub(beta, 1, p), p), p), p);
    }
    if powmod_u(a, (p - 1) / 2, p) != 1 {
        return 0;
    }
    let mut q = p - 1;
    let mut r = q.trailing_zeros();
    q >>= r;
    let mut t = 3;
    while powmod_u(t, (p - 1) / 2, p) != p - 1 {
        t += 2;
    }
    let mut z = powmod_u(t, q, p);
    let mut b = powmod_u(a, q, p);
    let mut x = powmod_u(a, q.div_ceil(2), p);
    while b != 1 {
        t = b;
        let mut m = 0;
        while m < r && t != 1 {
            t = mulmod_u(t, t, p);
            m += 1;
        }
        if m >= r {
            break;
        }
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
    if e == 0 {
        return Some(0);
    }
    let n = p.pow(e);
    if e == 1 {
        let a = a % p;
        let r = sqrt_prime(a, p);
        return (mulmod_u(r, r, p) == a).then_some(r.min(p - r));
    }
    if a % n == 0 {
        return Some(0);
    }
    if a % (p * p) == 0 {
        return sqrt_power(a / (p * p), p, e - 2).map(|r| r * p);
    }
    if a % p == 0 {
        return None;
    }
    let ered = if p > 2 || e < 5 {
        e.div_ceil(2)
    } else {
        e.div_ceil(2) + 1
    };
    let s = sqrt_power(a, p, ered)?;
    let np = if p == 2 {
        n.checked_mul(p).unwrap_or(n)
    } else {
        n
    };
    let r = addmod_u(
        s,
        gcddiv(sub(a, mulmod_u(s, s, np), np), addmod_u(s, s, np), n),
        n,
    );
    let r = r.min(n - r);
    (mulmod_u(r, r, n) == a % n).then_some(r)
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
    if n == 0 {
        return None;
    }
    let a = a % n;
    if a <= 1 || n <= 2 {
        return Some(a);
    }
    let root = sqrtint(a);
    if root * root == a {
        return Some(root);
    }
    let mut r = 0;
    let mut modulus = 1;
    for (p, e) in factors(n) {
        let m = p.pow(e);
        r = crt(r, modulus, sqrt_power(a, p, e)?, m);
        modulus *= m;
    }
    let r = r.min(n - r);
    (mulmod_u(r, r, n) == a % n).then_some(r)
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
        if (a / p) % p != 0 {
            return roots;
        }
        for r in all_sqrt_power(a / p / p, p, k - 2) {
            for j in 0..p {
                push_root(&mut roots, r * p + j * (pk / p));
            }
        }
        return roots;
    }
    if let Some(q) = sqrt_power(a, p, k) {
        roots.push(q);
        if p != 2 || k > 1 {
            roots.push(pk - q);
        }
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
    if n == 0 {
        return Vec::new();
    }
    if n <= 2 {
        return vec![a % n];
    }
    let mut roots = vec![0];
    let mut modulus = 1;
    for (p, e) in factors(n) {
        let m = p.pow(e);
        let other = all_sqrt_power(a, p, e);
        let mut next = Vec::new();
        for r in roots {
            for &s in &other {
                push_root(&mut next, crt(r, modulus, s, m));
            }
        }
        roots = next;
        if roots.is_empty() {
            break;
        }
        modulus *= m;
    }
    roots.sort_unstable();
    roots
}

fn corn_one(u: u64, d: u64, p: u64) -> Option<(u64, u64)> {
    let mut a = p;
    let mut b = u.max(p - u);
    let limit = sqrtint(p);
    while b > limit {
        (a, b) = (b, a % b);
    }
    let t = p - b * b;
    if t % d != 0 || t == 0 {
        return None;
    }
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
    if n == 0 {
        return Some((0, 0));
    }
    if d == 0 {
        let x = sqrtint(n);
        return (x * x == n).then_some((x, 0));
    }
    let negd = sub(0, d, n);
    if is_prime(n) != 0 {
        return corn_one(sqrtmod(negd, n)?, d, n);
    }
    if n >> 53 != 0 {
        let roots = allsqrtmod(negd, n);
        for &r in &roots[..roots.len() / 2] {
            if let Some(pair) = corn_one(r, d, n) {
                return Some(pair);
            }
        }
    }
    for y in 0..=sqrtint(n / d) {
        let t = n - d * y * y;
        let x = sqrtint(t);
        if x * x == t {
            return Some((x, y));
        }
    }
    None
}

// rootmod.c, _find_ts_generator and _ts_rootmod.
fn ts_root(a: u64, k: u64, p: u64) -> u64 {
    let mut r = p - 1;
    let mut e = 0;
    while r % k == 0 {
        r /= k;
        e += 1;
    }
    let mut x = powmod_u(a, invmod(k % r, r).unwrap_or(0), p);
    let mut aa = mulmod_u(powmod_u(x, k, p), invmod(a, p).unwrap_or(0), p);
    if aa == 1 {
        return x;
    }
    let mut generator = 2;
    let (mut y, mut m) = loop {
        let y = powmod_u(generator, r, p);
        let m = powmod_u(y, k.pow(e - 1), p);
        if m != 1 {
            break (y, m);
        }
        generator += 1;
    };
    while aa != 1 {
        let mut l = 1;
        let mut t = aa;
        let mut z = t;
        while t != 1 {
            if l >= e {
                return 0;
            }
            z = t;
            t = powmod_u(t, k, p);
            l += 1;
        }
        let mut log = 0;
        let mut v = 1;
        while v != z && log < k {
            v = mulmod_u(v, m, p);
            log += 1;
        }
        if log == k {
            return 0;
        }
        let kz = if log == 0 { 0 } else { k - log };
        m = powmod_u(m, kz, p);
        t = powmod_u(y, kz * k.pow(e - l), p);
        e = l - 1;
        x = mulmod_u(x, t, p);
        y = powmod_u(t, k, p);
        if y <= 1 {
            return 0;
        }
        aa = mulmod_u(aa, y, p);
    }
    x
}

// rootmod.c, _rootmod_prime_splitk.
fn root_prime(mut a: u64, k: u64, p: u64) -> u64 {
    a %= p;
    if a <= 1 || p == 2 {
        return a;
    }
    if k == 2 {
        return sqrt_prime(a, p);
    }
    let g = gcd(k, p - 1);
    for (f, e) in factors(g) {
        for _ in 0..e {
            a = ts_root(a, f, p);
            if a == 0 {
                return 0;
            }
        }
    }
    if g != k {
        let pg = (p - 1) / g;
        a = powmod_u(a, invmod((k / g) % pg, pg).unwrap_or(0), p);
    }
    a
}

fn hensel(r: u64, a: u64, k: u64, n: u64) -> Option<u64> {
    let f = sub(powmod_u(r, k, n), a, n);
    if f == 0 {
        return Some(r);
    }
    let fp = mulmod_u(k, powmod_u(r, k - 1, n), n);
    let d = divmod(f, fp, n)?;
    (d != 0).then(|| sub(r, d, n))
}

/// Return a `k`th root modulo `n`, or `None` if none exists.
/// Port of MPU `rootmod.c`, `rootmod` and `_rootmod_composite1`.
///
/// ```
/// assert_eq!(math_prime_util::rootmod(8, 3, 13), Some(2));
/// assert_eq!(math_prime_util::rootmod(2, 0, 7), None);
/// ```
pub fn rootmod(a: u64, k: u64, n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let a = a % n;
    let r = if n <= 2 || a <= 1 {
        a
    } else if k <= 1 {
        if k == 0 { 1 } else { a }
    } else if k < 64 && crate::is_power(a, u32::try_from(k).unwrap_or(0)) {
        crate::rootint(a, u32::try_from(k).unwrap_or(0))
    } else {
        let mut result = 0;
        let mut modulus = 1;
        for (p, e) in factors(n) {
            let mut r = root_prime(a, k, p);
            if powmod_u(r, k, p) != a % p {
                return None;
            }
            let mut pe = p;
            for _ in 2..=e {
                pe *= p;
                if let Some(next) = hensel(r, a, k, pe) {
                    r = next;
                } else {
                    let m = pe / p / p;
                    let next = (1..p).find_map(|t| hensel(r + t * m, a, k, pe));
                    if let Some(next) = next {
                        r = next;
                    } else {
                        let base = r % p;
                        r = (0..pe / p)
                            .map(|t| base + t * p)
                            .find(|&v| powmod_u(v, k, pe) == a % pe)?;
                    }
                }
            }
            result = crt(result, modulus, r, pe);
            modulus *= pe;
        }
        result
    };
    let r = if k == 2 { r.min(n - r) } else { r };
    (powmod_u(r, k, n) == a).then_some(r)
}

// rootmod.c, _ts_prime. Returns one root and a root of unity.
fn ts_prime(a: u64, k: u64, p: u64) -> (u64, u64) {
    let mut r = p - 1;
    while r % k == 0 {
        r /= k;
    }
    let mut ke = (p - 1) / r;
    let mut x = powmod_u(a, invmod(k % r, r).unwrap_or(0), p);
    let mut b = mulmod_u(powmod_u(x, k, p), invmod(a, p).unwrap_or(0), p);
    let mut candidate = 2;
    let (mut t, y) = loop {
        let t = powmod_u(candidate, r, p);
        let y = powmod_u(t, ke / k, p);
        if y != 1 {
            break (t, y);
        }
        candidate += 1;
    };
    while ke != k {
        ke /= k;
        let tt = t;
        t = powmod_u(t, k, p);
        let mut aa = powmod_u(b, ke / k, p);
        while aa != 1 {
            x = mulmod_u(x, tt, p);
            b = mulmod_u(b, t, p);
            aa = mulmod_u(aa, y, p);
        }
    }
    (x, t)
}

fn all_prime(a: u64, k: u64, p: u64) -> Vec<u64> {
    let a = a % p;
    if p == 2 || a == 0 {
        return vec![a];
    }
    let g = gcd(k, p - 1);
    if g == 1 {
        return vec![powmod_u(a, invmod(k % (p - 1), p - 1).unwrap_or(0), p)];
    }
    if powmod_u(a, (p - 1) / g, p) != 1 {
        return Vec::new();
    }
    let (r, z) = ts_prime(a, k, p);
    let mut roots = Vec::new();
    let mut v = r;
    for _ in 0..k {
        push_root(&mut roots, v);
        v = mulmod_u(v, z, p);
    }
    roots
}

// rootmod.c, _allrootmod_prime_power.
fn all_root_power(a: u64, k: u64, p: u64, e: u32) -> Vec<u64> {
    if e == 1 {
        return all_prime(a, k, p);
    }
    let n = p.pow(e);
    let mut roots = Vec::new();
    if a % n == 0 {
        let t = u64::from(e - 1) / k + 1;
        let t = u32::try_from(t).unwrap_or(e);
        let step = p.pow(t);
        let count = p.pow(e - t);
        assert!(count <= MAX_ROOTS as u64, "Maximum returned roots exceeded");
        return (0..count).map(|i| mulmod_u(i, step, n)).collect();
    }
    let pk = u32::try_from(k).ok().and_then(|k| p.checked_pow(k));
    if let Some(pk) = pk.filter(|&pk| a % pk == 0) {
        // Divisibility by p^k implies k < e here.
        let k32 = u32::try_from(k).unwrap_or(e);
        let pe1 = p.pow(k32 - 1);
        let pek = p.pow(e - k32 + 1);
        let other = all_root_power(a / pk, k, p, e - k32);
        for r in other {
            for j in 0..pe1 {
                push_root(
                    &mut roots,
                    addmod_u(mulmod_u(r, p, n), mulmod_u(j, pek, n), n),
                );
            }
        }
    } else if a % p != 0 {
        let np = n.checked_mul(p).unwrap_or(n);
        let ered = if p > 2 || e < 5 {
            e.div_ceil(2)
        } else {
            e.div_ceil(2) + 1
        };
        let other = all_root_power(a, k, p, ered);
        if k != p {
            for s in other {
                let t = powmod_u(s, k - 1, n);
                let r = addmod_u(
                    s,
                    gcddiv(sub(a, mulmod_u(t, s, n), n), mulmod_u(k, t, n), n),
                    n,
                );
                push_root(&mut roots, r);
            }
        } else {
            for s in other {
                let t = powmod_u(s, k - 1, np);
                let r = addmod_u(
                    s,
                    gcddiv(sub(a, mulmod_u(t, s, np), np), mulmod_u(k, t, np), n),
                    n,
                );
                if powmod_u(r, k, n) == a % n {
                    for i in 0..k {
                        push_root(
                            &mut roots,
                            mulmod_u(r, addmod_u(mulmod_u(i, n / p, n), 1, n), n),
                        );
                    }
                }
            }
            roots.sort_unstable();
            roots.dedup();
        }
    }
    roots
}

// rootmod.c, _allrootmod_kprime and _rootmod_cprod.
fn all_kprime(a: u64, k: u64, n: u64) -> Vec<u64> {
    let mut roots = vec![0];
    let mut modulus = 1;
    for (p, e) in factors(n) {
        let pe = p.pow(e);
        let local = all_root_power(a, k, p, e);
        let mut next = Vec::new();
        for r in roots {
            for &s in &local {
                push_root(&mut next, crt(r, modulus, s, pe));
            }
        }
        roots = next;
        modulus *= pe;
        if roots.is_empty() {
            break;
        }
    }
    roots
}

/// Return all `k`th roots modulo `n`, in ascending order.
/// Port of MPU `rootmod.c`, `allrootmod`, `_allrootmod_prime`, and `_ts_prime`.
///
/// # Panics
/// Panics if the number of roots exceeds MPU's limit of 600 million.
///
/// ```
/// assert_eq!(math_prime_util::allrootmod(1, 3, 7), vec![1, 2, 4]);
/// ```
pub fn allrootmod(a: u64, k: u64, n: u64) -> Vec<u64> {
    if n == 0 {
        return Vec::new();
    }
    let a = a % n;
    if n == 1 {
        return vec![0];
    }
    if k == 0 {
        if a != 1 {
            return Vec::new();
        }
        assert!(n <= MAX_ROOTS as u64, "Maximum returned roots exceeded");
        return (0..n).collect();
    }
    if n == 2 || k == 1 {
        return vec![a];
    }
    if k == 2 {
        return allsqrtmod(a, n);
    }
    let mut roots = vec![a];
    for (f, e) in factors(k) {
        for _ in 0..e {
            roots = roots
                .into_iter()
                .flat_map(|r| all_kprime(r, f, n))
                .collect();
            assert!(roots.len() <= MAX_ROOTS, "Maximum returned roots exceeded");
            if roots.is_empty() {
                return roots;
            }
        }
    }
    roots.sort_unstable();
    roots.dedup();
    roots
}

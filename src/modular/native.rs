use super::{addmod_u, invmod, mulmod_u, powmod_u};
use crate::{gcd, is_prime, sqrtint};

// reference/mpu/factor.c: pbrent_factor, non-Montgomery path.
#[allow(clippy::cast_possible_truncation)] // Each remainder is below the u64 modulus.
fn brent(n: u64, mut a: u64) -> Option<u64> {
    let step =
        |x: u64, a: u64| ((u128::from(x) * u128::from(x) + u128::from(a)) % u128::from(n)) as u64;
    let inner = if n <= 4_000_000_000 { 32 } else { 160 };
    let (mut r, mut f, mut xi, mut xm) = (1u64, 1, 1, 1);
    let mut rounds = 500_000;
    let mut fails = 6;
    while rounds > 0 {
        let mut left = r.min(rounds);
        let mut saved = xi;
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
        if f == 0 || f == n {
            if fails == 0 {
                break;
            }
            fails -= 1;
            xm = ((u128::from(xm) + 11) % u128::from(n)) as u64;
            xi = xm;
            a += 1;
            continue;
        }
        return Some(f);
    }
    None
}

fn factors(mut n: u64) -> Vec<u64> {
    let mut out = Vec::new();
    for p in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        while n % p == 0 && n > 1 {
            out.push(p);
            n /= p;
        }
    }
    let mut stack = vec![n];
    while let Some(n) = stack.pop() {
        if n <= 1 {
            continue;
        }
        if is_prime(n) != 0 {
            out.push(n);
            continue;
        }
        let mut a = 1;
        let d = loop {
            if let Some(d) = brent(n, a) {
                break d;
            }
            a += 2;
        };
        stack.extend([d, n / d]);
    }
    out.sort_unstable();
    out
}

fn factor_exp(n: u64) -> Vec<(u64, u32)> {
    let mut out = Vec::new();
    for p in factors(n) {
        if let Some((q, e)) = out.last_mut() {
            if *q == p {
                *e += 1;
                continue;
            }
        }
        out.push((p, 1));
    }
    out
}

// reference/mpu/util.c: znorder, with Euler's phi as an order multiple.
fn order(g: u64, n: u64) -> u64 {
    let mut k = n;
    for (p, _) in factor_exp(n) {
        k = k / p * (p - 1);
    }
    for (p, _) in factor_exp(k) {
        while k % p == 0 && powmod_u(g, k / p, n) == 1 {
            k /= p;
        }
    }
    k
}

// Coprime CRT, reference/mpu/util.c: chinese.
fn crt(parts: &[(u64, u64)]) -> u64 {
    let (mut x, mut m) = (0, 1);
    for &(a, n) in parts {
        let d = super::submod(a, x % n, n).unwrap();
        let t = mulmod_u(d, invmod(m % n, n).unwrap(), n);
        x += m * t;
        m *= n;
    }
    x
}

/// Return `n! mod m`. Return `None` for a zero modulus.
///
/// Port of `reference/mpu/factmod.c`: `factorialmod` and `_facmod`.
/// ```
/// use math_prime_util::factorialmod;
/// assert_eq!(factorialmod(10, 101), Some(72));
/// ```
pub fn factorialmod(n: u64, m: u64) -> Option<u64> {
    if m == 0 {
        return None;
    }
    if n >= m || m == 1 {
        return Some(0);
    }
    if n <= 1 {
        return Some(1);
    }
    let prime = is_prime(m) != 0;
    let d = if prime && n > m / 2 { m - n - 1 } else { n };
    if d < 2 {
        return Some(if d == 0 { m - 1 } else { 1 });
    }
    if d > 100
        && !prime
        && factor_exp(m)
            .iter()
            .all(|&(p, e)| valuation(n, p) >= u64::from(e))
    {
        return Some(0);
    }
    let mut r = 1;
    if d < 1000 {
        for i in 2..=d {
            r = mulmod_u(r, i, m);
            if r == 0 {
                break;
            }
        }
    } else {
        // _facmod's prime valuation product, using the crate's segmented sieve.
        for p in crate::primes_in(2..=d) {
            r = mulmod_u(r, powmod_u(p, valuation(d, p), m), m);
        }
    }
    if d != n && r != 0 {
        if d % 2 == 0 {
            r = m - r;
        }
        return invmod(r, m);
    }
    Some(r)
}

fn valuation(mut n: u64, p: u64) -> u64 {
    let mut e = 0;
    while n >= p {
        n /= p;
        e += n;
    }
    e
}

// reference/mpu/factmod.c: _factorialmod_without_prime.
fn unit_factorial(mut n: u64, p: u64, m: u64) -> u64 {
    let mut r = 1;
    if n >= m {
        if (n / m) % 2 == 1 && (p > 2 || m == 4) {
            r = m - 1;
        }
        n %= m;
    }
    for i in 2..=n {
        if i % p != 0 {
            r = mulmod_u(r, i, m);
        }
    }
    r
}

fn unit_factorial_powers(mut n: u64, p: u64, m: u64) -> u64 {
    let mut r = 1;
    while n > 1 {
        r = mulmod_u(r, unit_factorial(n, p, m), m);
        n /= p;
    }
    r
}

// reference/mpu/factmod.c: _binomial_mod_prime_power.
fn binomial_prime_power(n: u64, mut k: u64, p: u64, e: u32) -> u64 {
    if k > n {
        return 0;
    }
    if k == 0 || k == n {
        return 1;
    }
    k = k.min(n - k);
    let b = valuation(n, p) - valuation(k, p) - valuation(n - k, p);
    if b >= u64::from(e) {
        return 0;
    }
    let m = p.pow(e);
    let mut den = unit_factorial_powers(k, p, m);
    let mut num = 1;
    if k >= m {
        num = unit_factorial_powers(n, p, m);
        den = mulmod_u(den, unit_factorial_powers(n - k, p, m), m);
    } else {
        for i in n - k + 1..=n {
            let mut v = i;
            while v % p == 0 {
                v /= p;
            }
            num = mulmod_u(num, v, m);
        }
    }
    mulmod_u(
        mulmod_u(num, invmod(den, m).unwrap(), m),
        powmod_u(p, b, m),
        m,
    )
}

fn binomial_lucas(mut n: u64, mut k: u64, p: u64) -> u64 {
    let mut r = 1;
    while n > 0 || k > 0 {
        r = mulmod_u(r, binomial_prime_power(n % p, k % p, p, 1), p);
        if r == 0 {
            break;
        }
        n /= p;
        k /= p;
    }
    r
}

// reference/mpu/factmod.c: _binomial_lucas_mod_prime_power (Granville).
fn binomial_granville(n: u64, k: u64, p: u64, mut q: u32) -> u64 {
    let m = p.pow(q);
    let (mut nn, mut kk, mut rr) = (n, k, n - k);
    let mut carries = Vec::new();
    let mut carry = 0;
    while nn > 0 {
        carry = u64::from(nn % p < kk % p + carry);
        carries.push(carry);
        nn /= p;
        kk /= p;
        rr /= p;
    }
    for i in (1..carries.len()).rev() {
        carries[i - 1] += carries[i];
    }
    if carries[0] >= u64::from(q) {
        return 0;
    }
    q -= u32::try_from(carries[0]).unwrap();
    let m1 = p.pow(q);
    let mut r =
        if (p > 2 || q < 3) && (q as usize) < carries.len() && carries[q as usize - 1] % 2 == 1 {
            m - 1
        } else {
            1
        };
    r = mulmod_u(r, powmod_u(p, carries[0], m), m);
    (nn, kk, rr) = (n, k, n - k);
    while nn > 0 {
        let num = unit_factorial(nn % m1, p, m);
        let den = mulmod_u(
            unit_factorial(kk % m1, p, m),
            unit_factorial(rr % m1, p, m),
            m,
        );
        r = mulmod_u(r, mulmod_u(num, invmod(den, m).unwrap(), m), m);
        nn /= p;
        kk /= p;
        rr /= p;
    }
    r
}

/// Return `binomial(n, k) mod m`. Return `None` for a zero modulus.
///
/// Port of `reference/mpu/factmod.c`: `binomialmod`, with Lucas and CRT.
/// ```
/// use math_prime_util::binomialmod;
/// assert_eq!(binomialmod(10, 3, 12), Some(0));
/// ```
pub fn binomialmod(n: u64, k: u64, m: u64) -> Option<u64> {
    if m == 0 {
        return None;
    }
    if m == 1 {
        return Some(0);
    }
    if k == 0 || k >= n {
        return Some(u64::from(k == 0 || k == n));
    }
    if m == 2 {
        return Some(u64::from(k & !n == 0));
    }
    if is_prime(m) != 0 {
        return Some(binomial_lucas(n, k, m));
    }
    let parts: Vec<_> = factor_exp(m)
        .into_iter()
        .map(|(p, e)| {
            let pm = p.pow(e);
            let b = if e == 1 {
                binomial_lucas(n, k, p)
            } else if k.min(n - k) <= 100_000 && k.min(n - k) < pm {
                binomial_prime_power(n, k, p, e)
            } else {
                binomial_granville(n, k, p, e)
            };
            (b, pm)
        })
        .collect();
    Some(crt(&parts))
}

// reference/mpu/znlog.c: dlp_bsgs, interleaved baby and giant steps.
fn bsgs(a: u64, g: u64, p: u64, n: u64) -> Option<u64> {
    use std::collections::hash_map::Entry;
    let maxm = sqrtint(n);
    let m = ((maxm + 1) / 2).min(40_000_000).max(1);
    let mut table = std::collections::HashMap::new();
    let aa = mulmod_u(a, a, p);
    let gm = powmod_u(g, m, p);
    let (mut s, mut t) = (mulmod_u(a, g, p), gm);
    table.insert(a, 0);
    for i in 1..=m {
        match table.entry(s) {
            Entry::Occupied(e) => {
                let x = super::submod(mulmod_u(*e.get(), m, n), i, n).unwrap();
                return (powmod_u(g, x, p) == a).then_some(x);
            }
            Entry::Vacant(e) => {
                e.insert(i);
            }
        }
        s = mulmod_u(s, g, p);
        if s == aa {
            return Some(i + 1);
        }
        match table.entry(t) {
            Entry::Occupied(e) => {
                let x = super::submod(mulmod_u(i, m, n), *e.get(), n).unwrap();
                return (powmod_u(g, x, p) == a).then_some(x);
            }
            Entry::Vacant(e) => {
                e.insert(i);
            }
        }
        t = mulmod_u(t, gm, p);
    }
    for i in m + 1..=n.div_ceil(m) {
        if let Some(&j) = table.get(&t) {
            let x = super::submod(mulmod_u(i, m, n), j, n).unwrap();
            if powmod_u(g, x, p) == a {
                return Some(x);
            }
        }
        t = mulmod_u(t, gm, p);
    }
    None
}

fn log_solve(a: u64, g: u64, p: u64, n: u64) -> Option<u64> {
    if a == 1 {
        return Some(0);
    }
    let mut t = g;
    for k in 1..=n.min(10_000) {
        if t == a {
            return Some(k);
        }
        t = mulmod_u(t, g, p);
        if t == g {
            return None;
        }
    }
    if n <= 10_000 || powmod_u(a, n, p) != 1 {
        return None;
    }
    bsgs(a, g, p, n)
}

// reference/mpu/znlog.c: znlog_prime_power and znlog_ph.
fn pohlig_hellman(a: u64, g: u64, p: u64, n: u64) -> Option<u64> {
    let mut parts = Vec::new();
    for (q, e) in factor_exp(n) {
        let pe = q.pow(e);
        let delta = powmod_u(a, n / pe, p);
        let gamma = powmod_u(g, n / pe, p);
        let gamma0 = powmod_u(gamma, q.pow(e - 1), p);
        let (mut k, mut pj) = (0, 1);
        for j in 0..e {
            let t = if k == 0 {
                delta
            } else {
                mulmod_u(delta, powmod_u(gamma, pe - k, p), p)
            };
            let h = powmod_u(t, q.pow(e - 1 - j), p);
            let x = log_solve(h, gamma0, p, q)?;
            k += (x % q) * pj;
            if j != e - 1 {
                pj *= q;
            }
        }
        if powmod_u(gamma, k, p) != delta {
            return None;
        }
        parts.push((k, pe));
    }
    let x = crt(&parts);
    (powmod_u(g, x, p) == a).then_some(x)
}

/// Find the least `k` such that `g^k mod n == a`. Return `None` if absent.
///
/// Port of `reference/mpu/znlog.c`: `znlog`, `znlog_non_coprime`,
/// `dlp_bsgs`, and `znlog_ph`.
/// ```
/// use math_prime_util::znlog;
/// assert_eq!(znlog(5, 2, 1019), Some(10));
/// assert_eq!(znlog(2, 4, 17), None);
/// ```
pub fn znlog(a: u64, g: u64, n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    if n == 1 {
        return Some(0);
    }
    let (a, g) = (a % n, g % n);
    if a == 1 {
        return Some(0);
    }
    if g == 0 {
        return (a == 0).then_some(1);
    }
    let mut d = gcd(g, n);
    if d != 1 {
        let (mut nr, mut ar, mut gp, mut acc, mut k) = (n, a, 1, 1, 0);
        while d > 1 {
            if gp == a {
                return Some(k);
            }
            if ar % d != 0 {
                return None;
            }
            nr /= d;
            ar /= d;
            acc = if nr > 1 { mulmod_u(acc, g / d, nr) } else { 0 };
            gp = mulmod_u(gp, g, n);
            k += 1;
            d = gcd(g, nr);
        }
        if gp == a {
            return Some(k);
        }
        if nr == 1 {
            return None;
        }
        let target = mulmod_u(ar % nr, invmod(acc, nr)?, nr);
        k += znlog(target, g % nr, nr)?;
        return (powmod_u(g, k, n) == a).then_some(k);
    }
    let ord = order(g, n);
    if powmod_u(a, ord, n) != 1 {
        return None;
    }
    if is_prime(ord) != 0 {
        log_solve(a, g, n, ord)
    } else {
        pohlig_hellman(a, g, n, ord)
    }
}

// Fibonacci doubling supplies lucasumod(1,-1,k,n) without signed moduli.
fn fibonacci(k: u128, n: u64) -> (u64, u64) {
    if k == 0 {
        return (0, 1 % n);
    }
    let (a, b) = fibonacci(k / 2, n);
    let c = mulmod_u(a, super::submod(addmod_u(b, b, n), a, n).unwrap(), n);
    let d = addmod_u(mulmod_u(a, a, n), mulmod_u(b, b, n), n);
    if k % 2 == 0 {
        (c, d)
    } else {
        (d, addmod_u(c, d, n))
    }
}

/// Return the period of the Fibonacci sequence modulo `n`.
///
/// Port of `reference/mpu/util.c`: `_pisano_prime_power` and `pisano_period`.
/// The result uses `u128` since a native modulus can have a larger period.
/// ```
/// use math_prime_util::pisano_period;
/// assert_eq!(pisano_period(10), 60);
/// ```
pub fn pisano_period(n: u64) -> u128 {
    if n <= 1 {
        return u128::from(n);
    }
    let mut k = 1u128;
    for (p, e) in factor_exp(n) {
        let base = match p {
            2 => 3,
            3 => 8,
            5 => 20,
            7 => 16,
            _ if p < 300 => {
                let (mut a, mut b, mut j) = (1, 1, 1);
                while a != 0 || b != 1 {
                    (a, b) = (b, addmod_u(a, b, p));
                    j += 1;
                }
                j
            }
            _ => {
                let mut j = if p % 5 == 1 || p % 5 == 4 {
                    p - 1
                } else {
                    p + 1
                };
                for (q, e) in factor_exp(j) {
                    for _ in 0..e {
                        if fibonacci(u128::from(j / q), p).0 != 0 {
                            break;
                        }
                        j /= q;
                    }
                }
                j
            }
        };
        let part = u128::from(base) * u128::from(p).pow(e - 1);
        let (mut x, mut y) = (k, part);
        while y != 0 {
            (x, y) = (y, x % y);
        }
        k = k / x * part;
    }
    let mut r = k;
    while fibonacci(r - 1, n).0 != 1 {
        r += k;
    }
    r
}

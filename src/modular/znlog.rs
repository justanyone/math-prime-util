use super::native::{crt, factor_exp, order};
use super::{addmod_u, invmod, mulmod_u, powmod_u};
use crate::{gcd, is_prime, sqrtint};

// reference/mpu/znlog.c: dlp_bsgs, interleaved baby and giant steps.
fn bsgs(a: u64, g: u64, p: u64, n: u64) -> Option<u64> {
    use std::collections::hash_map::Entry;
    let maxm = sqrtint(n);
    let m = maxm.div_ceil(2).clamp(1, 40_000_000);
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
    let mut bound = n.div_ceil(m);
    if m < maxm && bound > 8 * m {
        bound = 8 * m;
    }
    for i in m + 1..=bound {
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

// reference/mpu/znlog.c: dlp_prho, non-Montgomery path.
fn log_rho(a: u64, g: u64, p: u64, n: u64, maxrounds: u64) -> Option<u64> {
    let cycle = |(u, v, w): (u64, u64, u64)| match u % 3 {
        0 => (mulmod_u(u, u, p), addmod_u(v, v, n), addmod_u(w, w, n)),
        1 => (mulmod_u(u, a, p), addmod_u(v, 1, n), w),
        _ => (mulmod_u(u, g, p), v, addmod_u(w, 1, n)),
    };
    for restart in 0..=4 {
        let mut slow = (powmod_u(g, restart, p), 0, restart % n);
        let mut fast = cycle(slow);
        let (mut power, mut lam, mut round) = (1, 1, 1);
        while slow.0 != fast.0 && round < maxrounds.min(n) {
            if power == lam {
                slow = fast;
                power *= 2;
                lam = 0;
            }
            fast = cycle(fast);
            lam += 1;
            round += 1;
        }
        if slow.0 == fast.0 {
            let r1 = super::submod(slow.1, fast.1, n).unwrap();
            if r1 == 0 {
                continue;
            }
            let r2 = super::submod(fast.2, slow.2, n).unwrap();
            let common = gcd(r1, n);
            if r2 % common != 0 {
                return None;
            }
            let modulus = n / common;
            let x = mulmod_u(r2 / common, invmod(r1 / common, modulus)?, modulus);
            for j in 0..common {
                let k = x + j * modulus;
                if powmod_u(g, k, p) == a {
                    return Some(k);
                }
            }
            return None;
        }
    }
    None
}

fn log_solve(a: u64, g: u64, p: u64, n: u64) -> Option<u64> {
    if a == 1 {
        return Some(0);
    }
    if n <= 20_000 {
        if let Some(k) = log_rho(a, g, p, n, sqrtint(n) * 4) {
            return Some(k);
        }
    }
    let mut t = g;
    for k in 1..=n.min(20_000) {
        if t == a {
            return Some(k);
        }
        t = mulmod_u(t, g, p);
        if t == g {
            return None;
        }
    }
    if n <= 20_000 || powmod_u(a, n, p) != 1 {
        return None;
    }
    let result = if sqrtint(n).div_ceil(2) > 1_000_000 {
        log_rho(a, g, p, n, 2_000_000_000)
    } else {
        bsgs(a, g, p, n)
    };
    if result.is_some() {
        return result;
    }
    let mut t = g;
    for k in 1..p {
        if t == a {
            return Some(k);
        }
        t = mulmod_u(t, g, p);
        if t == g {
            break;
        }
    }
    None
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bsgs_and_rho_paths() {
        let (g, p, n) = (25, 1_000_000_007, 500_000_003);
        let a = powmod_u(g, 1_234_567, p);
        assert_eq!(bsgs(a, g, p, n), Some(1_234_567));
        assert_eq!(log_solve(a, g, p, n), Some(1_234_567));
        for k in [1, 2, 3, 17, 123, 508] {
            let a = powmod_u(4, k, 1019);
            let x = log_rho(a, 4, 1019, 509, 509).unwrap();
            assert_eq!(powmod_u(4, x, 1019), a);
        }
        assert_eq!(bsgs(3, 4, 17, 8), None);
        assert_eq!(log_rho(3, 4, 17, 8, 8), None);
    }
}

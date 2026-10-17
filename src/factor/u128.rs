//! Portable building blocks from MPU `factor128.c`.

fn add(a: u128, b: u128, n: u128) -> u128 {
    if a >= n - b { a - (n - b) } else { a + b }
}

fn sub(a: u128, b: u128, n: u128) -> u128 {
    if a >= b { a - b } else { n - (b - a) }
}

fn mul(mut a: u128, mut b: u128, n: u128) -> u128 {
    if let Some(product) = a.checked_mul(b) {
        return product % n;
    }
    let mut result = 0;
    while b != 0 {
        if b & 1 != 0 {
            result = add(result, a, n);
        }
        b >>= 1;
        a = add(a, a, n);
    }
    result
}

fn half(a: u128, n: u128) -> u128 {
    (a >> 1) + if a & 1 == 0 { 0 } else { (n >> 1) + 1 }
}

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn jacobi(mut a: u128, mut n: u128) -> i8 {
    let mut sign = 1;
    while a != 0 {
        let shift = a.trailing_zeros();
        a >>= shift;
        if shift & 1 != 0 && matches!(n % 8, 3 | 5) {
            sign = -sign;
        }
        if a % 4 == 3 && n % 4 == 3 {
            sign = -sign;
        }
        (a, n) = (n % a, a);
    }
    if n == 1 { sign } else { 0 }
}

fn square(n: u128) -> bool {
    let mut lo = 0;
    let mut hi = 1u128 << 64;
    while hi - lo > 1 {
        let mid = lo + (hi - lo) / 2;
        if mid <= n / mid {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    lo * lo == n
}

fn miller_rabin(n: u128) -> bool {
    let s = (n - 1).trailing_zeros();
    let mut d = (n - 1) >> s;
    let mut a = 2;
    let mut x = 1;
    while d != 0 {
        if d & 1 != 0 {
            x = mul(x, a, n);
        }
        a = mul(a, a, n);
        d >>= 1;
    }
    if x == 1 || x == n - 1 {
        return true;
    }
    for _ in 1..s {
        x = mul(x, x, n);
        if x == n - 1 {
            return true;
        }
    }
    false
}

// MPU factor128.c: select_extra_strong_parameters128, is_extra_strong_lucas128.
fn lucas(n: u128) -> bool {
    let mut p = 3u128;
    loop {
        let j = jacobi(p * p - 4, n);
        if j == -1 {
            break;
        }
        if j == 0 {
            let g = gcd(p * p - 4, n);
            if g != 1 && g != n {
                return false;
            }
        }
        if p == 23 && square(n) {
            return false;
        }
        p += 1;
        assert!(p <= 65535, "Lucas parameter exceeds 65535");
    }
    let discriminant = p * p - 4;
    let s = (n + 1).trailing_zeros();
    let d = (n + 1) >> s;
    let mut u = 1;
    let mut v = p;
    for bit in (0..d.ilog2()).rev() {
        u = mul(u, v, n);
        v = sub(mul(v, v, n), 2, n);
        if d & (1u128 << bit) != 0 {
            let next_u = half(add(mul(u, p, n), v, n), n);
            v = half(add(mul(v, p, n), mul(u, discriminant, n), n), n);
            u = next_u;
        }
    }
    if u == 0 && (v == 2 || v == n - 2) {
        return true;
    }
    for r in 1..s {
        if v == 0 {
            return true;
        }
        if r + 1 < s {
            v = sub(mul(v, v, n), 2, n);
        }
    }
    false
}

/// Tests primality on the u128 path of `is_prime`.
///
/// Port of MPU `factor128.c`, `is_prime128` and `is_bpsw128`.
/// Returns 2 for a proven u64 prime, 1 for a larger probable prime, or 0.
/// The larger path uses base-2 Miller–Rabin and extra-strong Lucas tests.
///
/// # Panics
/// Panics if the Lucas parameter exceeds 65535, as in MPU.
///
/// ```
/// use math_prime_util::is_prime_u128;
/// assert_eq!(is_prime_u128((1u128 << 127) - 1), 1);
/// assert_eq!(is_prime_u128(100), 0);
/// ```
pub fn is_prime_u128(n: u128) -> u8 {
    if let Ok(small) = u64::try_from(n) {
        return crate::is_prime(small);
    }
    if n & 1 == 0 || n == u128::MAX {
        return 0;
    }
    for p in 3u128..=2011 {
        if crate::is_prime(u64::try_from(p).expect("small prime")) != 0 && n % p == 0 {
            return 0;
        }
    }
    u8::from(miller_rabin(n) && lucas(n))
}

// MPU factor128.c: brent128. Uses ordinary residues and portable products.
fn brent(n: u128, c: u128, max_rounds: u64) -> Option<u128> {
    let step = |v| add(mul(v, v, n), c, n);
    let mut y = 2;
    let mut q = 1;
    let mut r = 1u64;
    let (mut x, mut saved);
    let mut p;
    loop {
        x = y;
        for _ in 0..r {
            y = step(y);
        }
        let mut k = 0;
        loop {
            saved = y;
            for _ in 0..128.min(r - k) {
                y = step(y);
                q = mul(q, x.abs_diff(y), n);
            }
            p = gcd(q, n);
            k += 128;
            if k >= r || p != 1 {
                break;
            }
        }
        if r > max_rounds {
            return None;
        }
        r *= 2;
        if p != 1 {
            break;
        }
    }
    if p == n {
        loop {
            saved = step(saved);
            p = gcd(x.abs_diff(saved), n);
            if p != 1 {
                break;
            }
        }
    }
    (p != n).then_some(p)
}

/// Factors an integer on the u128 path of `factor`.
///
/// Ports trial division and the split stack from MPU `factor128.c`,
/// `factorintp128`, with its `brent128` splitter. Factors are sorted.
/// Zero returns `[0]`. One returns an empty vector.
/// Larger prime factors pass MPU's BPSW probable-prime test.
/// The p-1, SQUFOF, ECM and SIQS stages are not yet ported.
///
/// # Errors
/// Returns the unsplit cofactor if four bounded Brent attempts fail.
///
/// # Panics
/// Panics if the Lucas parameter exceeds 65535, as in MPU.
///
/// ```
/// use math_prime_util::factor_u128;
/// assert_eq!(factor_u128(12), Ok(vec![2, 2, 3]));
/// ```
pub fn factor_u128(mut n: u128) -> Result<Vec<u128>, u128> {
    if n < 2 {
        return Ok(if n == 0 { vec![0] } else { vec![] });
    }
    let mut factors = Vec::new();
    for p in 2u128..=2011 {
        if p * p > n {
            break;
        }
        if crate::is_prime(u64::try_from(p).expect("small prime")) == 0 {
            continue;
        }
        while n % p == 0 {
            factors.push(p);
            n /= p;
        }
    }
    let mut stack = vec![n];
    while let Some(t) = stack.pop() {
        if t == 1 {
            continue;
        }
        if is_prime_u128(t) != 0 {
            factors.push(t);
            continue;
        }
        let f = (1..=4).find_map(|c| brent(t, c, 262_144)).ok_or(t)?;
        stack.push(f);
        stack.push(t / f);
    }
    factors.sort_unstable();
    Ok(factors)
}

/// Factors a u64 integer into sorted prime factors.
///
/// Uses the trial division and split stack from MPU `factor128.c`,
/// `factorintp128`. See [`factor_u128`] for the u128 path and current limits.
///
/// # Errors
/// Returns the unsplit cofactor if the bounded splitter fails.
///
/// # Panics
/// Panics if the Lucas parameter exceeds 65535, as in MPU.
///
/// ```
/// use math_prime_util::factor::factor;
/// assert_eq!(factor(12), Ok(vec![2, 2, 3]));
/// ```
pub fn factor(n: u64) -> Result<Vec<u64>, u128> {
    factor_u128(u128::from(n)).map(|factors| {
        factors
            .into_iter()
            .map(|p| u64::try_from(p).expect("factor fits input"))
            .collect()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arithmetic_edges() {
        let n = u128::MAX - 158;
        assert_eq!(add(n - 1, n - 1, n), n - 2);
        assert_eq!(sub(0, 1, n), n - 1);
        assert_eq!(mul(n - 1, n - 1, n), 1);
        assert_eq!(half(n - 2, n), n - 1);
        assert_eq!(half(4, n), 2);
        assert!(square(0));
        assert!(square(u128::from(u64::MAX).pow(2)));
        assert!(!square(u128::MAX));
        assert_eq!(jacobi(0, 3), 0);
        assert_eq!(jacobi(3, 7), -1);
        assert!(!miller_rabin(2047 * 3));
        assert!(miller_rabin(2047));
        assert!(!lucas(2047));
        assert!(!lucas(10009u128.pow(2)));
        for n in [10007, 10009, 65537, 104_729] {
            assert!(lucas(n));
        }
    }

    #[test]
    fn bounded_splitter() {
        assert_eq!(brent(10007 * 10009, 1, 0), None);
        assert_eq!(brent(13, 1, 32), None);
        let n = 10007 * 10009;
        let f = brent(n, 1, 1024).unwrap();
        assert!(f > 1 && f < n && n % f == 0);
        assert_eq!(factor(1), Ok(vec![]));
        assert_eq!(factor(0), Ok(vec![0]));
    }
}

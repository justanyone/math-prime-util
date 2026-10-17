use crate::{gcd, is_prime, mulmod};

/// Counts prime factors with multiplicity. Zero has one factor; one has none.
///
/// Port of `prime_bigomega` in MPU `factor.c`.
///
/// ```
/// assert_eq!(math_prime_util::prime_bigomega(360), 6);
/// ```
pub fn prime_bigomega(n: u64) -> u32 {
    exponents(n).into_iter().sum()
}

// Private dependency until the sibling factor/factor_exp port is available.
// Uses the trial division and recursive splitting of factor.c, factor.
pub(super) fn exponents(mut n: u64) -> Vec<u32> {
    if n < 2 {
        return if n == 0 { vec![1] } else { Vec::new() };
    }
    let mut factors = Vec::new();
    for p in [2, 3, 5] {
        while n % p == 0 {
            factors.push(p);
            n /= p;
        }
    }
    split(n, &mut factors);
    factors.sort_unstable();
    let mut result = Vec::new();
    let mut previous = 0;
    for p in factors {
        if p == previous {
            *result.last_mut().expect("a previous factor exists") += 1;
        } else {
            result.push(1);
            previous = p;
        }
    }
    result
}

fn split(n: u64, factors: &mut Vec<u64>) {
    if n == 1 {
        return;
    }
    if is_prime(n) != 0 {
        factors.push(n);
        return;
    }
    let f = rho(n);
    split(f, factors);
    split(n / f, factors);
}

// Port of factor.c, prho_factor. Restart with another polynomial on failure.
fn rho(n: u64) -> u64 {
    let mut a = 1;
    loop {
        let mut u = 7;
        let mut v = 7;
        for _ in 0..4_000_000 {
            u = step(u, a, n);
            v = step(step(v, a, n), a, n);
            let f = gcd(u.abs_diff(v), n);
            if f == n {
                break;
            }
            if f > 1 {
                return f;
            }
        }
        a += 1;
    }
}

fn step(x: u64, a: u64, n: u64) -> u64 {
    let square = mulmod(x, x, n).expect("positive modulus");
    crate::addmod(square, a, n).expect("positive modulus")
}

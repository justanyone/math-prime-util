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
pub(crate) fn sorted_factors(mut n: u64) -> Vec<u64> {
    if n < 2 {
        return if n == 0 { vec![0] } else { Vec::new() };
    }
    let mut factors = Vec::new();
    for p in [2, 3, 5] {
        while n % p == 0 {
            factors.push(p);
            n /= p;
        }
    }
    for p in crate::primes_in(7..=421) {
        while n % p == 0 {
            factors.push(p);
            n /= p;
        }
    }
    split(n, &mut factors);
    factors.sort_unstable();
    factors
}

pub(super) fn exponents(n: u64) -> Vec<u32> {
    if n == 0 {
        return vec![1];
    }
    let factors = sorted_factors(n);
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
    let power = crate::int::is_power_exp(n);
    if power > 1 {
        let root = crate::rootint(n, power);
        let mut root_factors = Vec::new();
        split(root, &mut root_factors);
        for _ in 0..power {
            factors.extend_from_slice(&root_factors);
        }
        return;
    }
    let mut seed = 7;
    let f = loop {
        if let Some(f) = rho(n, seed) {
            break f;
        }
        seed += 2;
    };
    split(f, factors);
    split(n / f, factors);
}

// Port of factor.c, prho_factor: 64-step batches and three polynomial retries.
fn rho(n: u64, seed: u64) -> Option<u64> {
    let mut u = seed;
    let mut v = seed;
    let mut a = 1;
    let mut fails = 3;
    for _ in 0..62_500 {
        let mut m = 1;
        let (old_u, old_v) = (u, v);
        for _ in 0..64 {
            u = step(u, a, n);
            v = step(step(v, a, n), a, n);
            m = mulmod(m, u.abs_diff(v), n).expect("positive modulus");
        }
        let mut f = gcd(m, n);
        if f == 1 {
            continue;
        }
        if f == n {
            u = old_u;
            v = old_v;
            for _ in 0..=64 {
                u = step(u, a, n);
                v = step(step(v, a, n), a, n);
                f = gcd(u.abs_diff(v), n);
                if f != 1 {
                    break;
                }
            }
        }
        if f == 0 || f == n {
            if fails == 0 {
                break;
            }
            fails -= 1;
            u = crate::addmod(u, 2, n).expect("positive modulus");
            v = u;
            a += 2;
        } else {
            return Some(f);
        }
    }
    None
}

fn step(x: u64, a: u64, n: u64) -> u64 {
    let square = mulmod(x, x, n).expect("positive modulus");
    crate::addmod(square, a, n).expect("positive modulus")
}

#[cfg(test)]
mod tests {
    use super::{exponents, rho, step};

    #[test]
    fn rho_backtracking_and_retry_limit() {
        // A prime makes every nontrivial gcd equal n. Exhaust all retries.
        assert_eq!(rho(7, 7), None);
        for n in [49, 77, 121, 143, 10_403, 1_000_036_000_099] {
            let f = rho(n, 7).expect("these composites split within the C bound");
            assert!(f > 1 && f < n);
            assert_eq!(n % f, 0);
        }
    }

    #[test]
    fn modular_step_avoids_overflow() {
        let n = u64::MAX - 58;
        let x = n - 1;
        assert_eq!(step(x, 7, n), 8);
    }

    #[test]
    fn large_perfect_power_and_mixed_exponents() {
        assert_eq!(exponents(1_000_003_u64.pow(3)), vec![3]);
        assert_eq!(exponents(2_u64.pow(2) * 3_u64.pow(5) * 7), vec![2, 5, 1]);
    }
}

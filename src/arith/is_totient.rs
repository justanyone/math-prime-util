use super::divisors;
use crate::is_prime;

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

/// Test whether `n` is a totient.
/// Port of `reference/mpu/totients.c::is_totient` and `_totpred`.
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

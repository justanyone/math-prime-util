use super::divisors;
use crate::is_prime;
use std::collections::HashMap;

/// List all native integers with totient `n`.
/// Port of `reference/mpu/totients.c::inverse_totient_list`.
/// Return an empty list if no native integer has this totient.
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
    if is_prime(n / 2) != 0 {
        if is_prime(n + 1) == 0 {
            return Vec::new();
        }
        if n >= 10 {
            let mut result = vec![n + 1];
            if let Some(second) = (n + 1).checked_mul(2) {
                result.push(second);
            }
            return result;
        }
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

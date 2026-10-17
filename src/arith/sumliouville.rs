use super::{liouville::SMALL, mertens::Mertens};
use crate::sqrtint;

// MPU moebius.c::liouville_array, used only through 96.
fn liouville_array(n: u64) -> Vec<i8> {
    let hi = n.max(15);
    let mut values = vec![-1; usize::try_from(hi + 1).unwrap()];
    values[..16].copy_from_slice(&SMALL);
    let mut a = 16;
    while a <= hi {
        let b = (2 * a - 1).min(hi);
        for p in crate::sieve::primes(2, sqrtint(b)) {
            for k in (2 * p..=b).step_by(usize::try_from(p).unwrap()) {
                if k >= a {
                    values[usize::try_from(k).unwrap()] = -values[usize::try_from(k / p).unwrap()];
                }
            }
        }
        a = b + 1;
    }
    values
}

/// Return the sum of Liouville values from one through `n`.
///
/// Port of MPU 0.75 `moebius.c::sumliouville` and `liouville_array`.
///
/// ```
/// assert_eq!(math_prime_util::sumliouville(10), 0);
/// assert_eq!(math_prime_util::sumliouville(0), 0);
/// ```
pub fn sumliouville(n: u64) -> i64 {
    if n <= 96 {
        return liouville_array(n)[1..=usize::try_from(n).unwrap()].iter().map(|&v| i64::from(v)).sum();
    }
    let mut ctx = Mertens::new(n);
    let root = sqrtint(n);
    let mut sum = ctx.get(n);
    let mut k = 2;
    while k <= root {
        let quotient = n / (k * k);
        if quotient == 1 { break; }
        sum += ctx.get(quotient);
        k += 1;
    }
    sum + i64::try_from(root + 1 - k).unwrap()
}

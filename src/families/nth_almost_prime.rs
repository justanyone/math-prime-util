use super::almost_prime_count_upper::{max_count, max_nth, reduce_count};
use super::nth_almost_prime_lower::{inverse, reduction, small};
use crate::{prime_count, primes_in, rootint, sqrtint};

// MPU almost_primes.c: _cs and _final_sum. Prime products are nondecreasing.
fn recursive_count(n: u64, lo: u64, k: u32) -> u64 {
    if k == 2 {
        let mut index = prime_count(lo) - 1;
        return primes_in(lo..=sqrtint(n))
            .map(|p| {
                let value = prime_count(n / p) - index;
                index += 1;
                value
            })
            .sum();
    }
    primes_in(lo..=rootint(n, k))
        .map(|p| recursive_count(n / p, p, k - 1))
        .sum()
}
pub(super) fn exact_count(k: u32, n: u64) -> u64 {
    if k == 0 {
        return u64::from(n >= 1);
    }
    if k >= 64 || n >> k == 0 {
        return 0;
    }
    let (k, n) = reduce_count(k, n);
    if n >= max_nth(k) {
        return max_count(k);
    }
    match k {
        0 => n,
        1 => prime_count(n),
        2 => crate::semiprime_count(n),
        _ => recursive_count(n, 2, k),
    }
}
/// Return the `n`th integer with `k` prime factors, with multiplicity.
///
/// Port of MPU 0.75 `almost_primes.c`, `nth_almost_prime`, `_cs`, and
/// `_final_sum`. Reduce powers of two, then invert the exact count.
/// Return `None` for index zero or a value outside the native range.
///
/// ```
/// assert_eq!(math_prime_util::nth_almost_prime(3, 10), Some(45));
/// assert_eq!(math_prime_util::nth_almost_prime(3, 0), None);
/// ```
#[must_use]
pub fn nth_almost_prime(k: u32, n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    if k == 0 {
        return (n == 1).then_some(1);
    }
    if k == 1 {
        return crate::nth_prime(n);
    }
    if k == 2 {
        return crate::nth_semiprime(n);
    }
    let max = max_count(k);
    if n >= max {
        return (n == max && max > 0).then(|| max_nth(k));
    }
    if n < 8 {
        return Some(small(k, n));
    }
    let r = reduction(k, n);
    if r > 0 {
        return nth_almost_prime(k - r, n).map(|v| v << r);
    }
    if k == 3 || k == 4 {
        return Some(interpolate_approx(k, n));
    }
    let lo = super::nth_almost_prime_lower(k, n)?;
    let mut hi = inverse(lo, 0, n, |x| exact_count(k, x));
    while !crate::is_almost_prime(u64::from(k), hi) {
        hi -= 1;
    }
    Some(hi)
}

// MPU inverse_interpolate.c: interpolate_with_approx, tolerance 20000.
fn interpolate_approx(k: u32, n: u64) -> u64 {
    let nth = |v| super::nth_almost_prime_approx(k, v).unwrap_or(0);
    let original = nth(n);
    let mut guess = original;
    let mut count = 0;
    let mut min = 0;
    let mut max = u64::MAX;
    let mut iterations = 0;
    for _ in 2..20 {
        iterations += 1;
        count = exact_count(k, guess);
        if n.abs_diff(count) < 20000 {
            break;
        }
        let estimate = nth(count);
        if count <= n && guess > min {
            min = guess;
        }
        if count >= n && guess < max {
            max = guess;
        }
        guess = if original >= estimate {
            guess.saturating_add(original - estimate)
        } else {
            guess.saturating_sub(estimate - original)
        };
        if guess <= min || guess >= max {
            let span = max - min;
            guess = if span <= 1 {
                if guess <= min { max } else { min }
            } else if 19999 >= span {
                min + span / 2
            } else if guess <= min {
                min + 19999
            } else {
                max - 19999
            };
        }
    }
    if iterations == 18 {
        count = exact_count(k, guess);
    }
    if count < n {
        while count < n {
            guess += 1;
            while !crate::is_almost_prime(u64::from(k), guess) {
                guess += 1;
            }
            count += 1;
        }
    } else {
        while !crate::is_almost_prime(u64::from(k), guess) {
            guess -= 1;
        }
        while count > n {
            guess -= 1;
            while !crate::is_almost_prime(u64::from(k), guess) {
                guess -= 1;
            }
            count -= 1;
        }
    }
    guess
}

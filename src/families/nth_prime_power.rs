use super::nth_prime_power_approx::{MAX_INDEX, MAX_PRIME, count, nth_prime_power_approx};
use crate::{is_prime, rootint};

// MPU prime_powers.c: prime_power; test the prime root of a perfect power.
fn is_power(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if is_prime(n) != 0 {
        return true;
    }
    (2..=n.ilog2()).any(|k| {
        let r = rootint(n, k);
        r.checked_pow(k) == Some(n) && is_prime(r) != 0
    })
}

/// Return the `n`th prime power. Index one is 2.
/// Returns `None` for zero. Indices above MPU's native prime limit return
/// the largest native prime, as in the C implementation.
/// Port of MPU 0.75 `prime_powers.c`, `nth_prime_power`, and
/// `inverse_interpolate.c`, `interpolate_with_approx` (tolerance 800).
/// ```
/// assert_eq!(math_prime_util::nth_prime_power(10), Some(16));
/// assert_eq!(math_prime_util::nth_prime_power(0), None);
/// ```
pub fn nth_prime_power(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    if n <= 7 {
        return Some(n + 1 + n / 5);
    }
    if n > MAX_INDEX {
        return Some(MAX_PRIME);
    }
    let approx = nth_prime_power_approx(n)?;
    let mut guess = approx;
    let (mut min, mut max) = (0, u64::MAX);
    let mut rank = 0;
    for _ in 2..20 {
        rank = count(guess, 0);
        if rank.abs_diff(n) < 800 {
            break;
        }
        let at_rank = nth_prime_power_approx(rank)?;
        if rank <= n && guess > min {
            min = guess;
        }
        if rank >= n && guess < max {
            max = guess;
        }
        guess = if approx >= at_rank {
            guess.saturating_add(approx - at_rank)
        } else {
            guess.saturating_sub(at_rank - approx)
        };
        if guess <= min || guess >= max {
            let span = max - min;
            guess = if span <= 1 {
                if guess <= min { max } else { min }
            } else if 799 >= span {
                min + span / 2
            } else if guess <= min {
                min + 799
            } else {
                max - 799
            };
        }
    }
    // Recount if the last iteration changed the guess.
    if rank.abs_diff(n) >= 800 {
        rank = count(guess, 0);
    }
    if rank < n {
        while rank < n {
            guess += 1;
            while !is_power(guess) {
                guess += 1;
            }
            rank += 1;
        }
    } else {
        while !is_power(guess) {
            guess -= 1;
        }
        while rank > n {
            guess -= 1;
            while !is_power(guess) {
                guess -= 1;
            }
            rank -= 1;
        }
    }
    Some(guess)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn membership_and_native_limit() {
        for n in [0, 1, 6, 12, 36, 100] {
            assert!(!is_power(n));
        }
        for n in [2, 3, 4, 8, 9, 25, 27, 49, 64, 81] {
            assert!(is_power(n));
        }
        assert_eq!(nth_prime_power(MAX_INDEX + 1), Some(MAX_PRIME));
        assert_eq!(nth_prime_power_approx(MAX_INDEX + 1), None);
        assert_eq!(super::super::nth_prime_power_lower(MAX_INDEX + 1), None);
        assert_eq!(super::super::nth_prime_power_upper(MAX_INDEX + 1), None);
    }
}

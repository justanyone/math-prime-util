//! Nth semiprime from MPU 0.75 `semi_primes.c`.

use super::nth_semiprime_approx::nth_semiprime_approx;
use crate::{is_semiprime, rootint, semi_primes, semiprime_count};

/// Small semiprimes, as in `_semiprimelist` of MPU `semi_primes.c`.
/// Entry 0 is a placeholder: there is no zeroth semiprime.
pub(crate) const SEMIPRIME_LIST: [u32; 191] = [
    0, 4, 6, 9, 10, 14, 15, 21, 22, 25, 26, 33, 34, 35, 38, 39, 46, 49, 51, 55, 57, 58, 62, 65, 69,
    74, 77, 82, 85, 86, 87, 91, 93, 94, 95, 106, 111, 115, 118, 119, 121, 122, 123, 129, 133, 134,
    141, 142, 143, 145, 146, 155, 158, 159, 161, 166, 169, 177, 178, 183, 185, 187, 194, 201, 202,
    203, 205, 206, 209, 213, 214, 215, 217, 218, 219, 221, 226, 235, 237, 247, 249, 253, 254, 259,
    262, 265, 267, 274, 278, 287, 289, 291, 295, 298, 299, 301, 302, 303, 305, 309, 314, 319, 321,
    323, 326, 327, 329, 334, 335, 339, 341, 346, 355, 358, 361, 362, 365, 371, 377, 381, 382, 386,
    391, 393, 394, 395, 398, 403, 407, 411, 413, 415, 417, 422, 427, 437, 445, 446, 447, 451, 453,
    454, 458, 466, 469, 471, 473, 478, 481, 482, 485, 489, 493, 497, 501, 502, 505, 511, 514, 515,
    517, 519, 526, 527, 529, 533, 535, 537, 538, 542, 543, 545, 551, 553, 554, 559, 562, 565, 566,
    573, 579, 581, 583, 586, 589, 591, 597, 611, 614, 622, 623,
];

/// Largest semiprime that MPU 0.75 returns (`MPU_MAX_SEMI_PRIME`, 64-bit).
pub(crate) const MAX_SEMI_PRIME: u64 = 18_446_744_073_709_551_601;
/// Index of the largest semiprime (`MPU_MAX_SEMI_PRIME_IDX`, 64-bit).
pub(crate) const MAX_SEMI_PRIME_IDX: u64 = 1_701_748_900_850_019_777;

/// Distance (in semiprimes) below which MPU walks instead of sieving
/// (`SP_SIEVE_THRESH` in `semi_primes.c`).
const SP_SIEVE_THRESH: u64 = 100;

/// `nth_semiprime_approx` for an index that is known to be positive.
fn approx(n: u64) -> u64 {
    nth_semiprime_approx(n).unwrap_or(0)
}

/// The first semiprime above `n` (`_next_semiprime` in MPU `semi_primes.c`).
fn next_semiprime(mut n: u64) -> u64 {
    loop {
        n += 1;
        if is_semiprime(n) {
            return n;
        }
    }
}

/// The last semiprime below `n` (`_prev_semiprime` in MPU `semi_primes.c`).
fn prev_semiprime(mut n: u64) -> u64 {
    loop {
        n -= 1;
        if is_semiprime(n) {
            return n;
        }
    }
}

/// Returns the `n`-th semiprime (1-based), or `None` when `n` is 0.
///
/// Port of `nth_semiprime` in MPU `semi_primes.c`. Small indices come from
/// the table. Past the table, [`nth_semiprime_approx`] gives a guess that
/// successive interpolations against the exact [`semiprime_count`] bring
/// within `16 * cbrt(n)` semiprimes; a sieve ([`semi_primes`]) then covers
/// the remaining distance in blocks, and a short walk over [`is_semiprime`]
/// lands on the exact index. An index past the largest 64-bit semiprime
/// index returns `Some(0)`, as MPU does.
///
/// # Examples
///
/// ```
/// use math_prime_util::nth_semiprime;
///
/// assert_eq!(nth_semiprime(1), Some(4));
/// assert_eq!(nth_semiprime(5), Some(14));
/// assert_eq!(nth_semiprime(1234), Some(4497));
/// assert_eq!(nth_semiprime(0), None);
/// ```
#[must_use]
pub fn nth_semiprime(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    if let Ok(idx) = usize::try_from(n) {
        if idx < SEMIPRIME_LIST.len() {
            return Some(u64::from(SEMIPRIME_LIST[idx]));
        }
    }
    // Overflow boundary of `nth_semiprime` in MPU `semi_primes.c`.
    if n >= MAX_SEMI_PRIME_IDX {
        return Some(if n == MAX_SEMI_PRIME_IDX {
            MAX_SEMI_PRIME
        } else {
            0
        });
    }

    let mut guess = approx(n);
    // Use exact counts until within this many semiprimes.
    let sptol = 16 * rootint(n, 3);
    let (mut ming, mut maxg) = (0u64, u64::MAX);
    let mut spcnt = 0u64;
    // Successive interpolations until the difference is small enough.
    for _ in 2..20 {
        while !is_semiprime(guess) {
            guess += 1;
        }
        spcnt = semiprime_count(guess);
        if n.abs_diff(spcnt) < sptol {
            break;
        }
        let an = approx(n);
        let ac = approx(spcnt);
        // Keep the previous guesses as bounds so a new guess cannot overshoot.
        if spcnt <= n && guess > ming {
            ming = guess;
        }
        if spcnt >= n && guess < maxg {
            maxg = guess;
        }
        if an >= ac {
            guess = guess.saturating_add(an - ac).min(MAX_SEMI_PRIME);
        } else {
            guess = guess.saturating_sub(ac - an);
        }
        if guess <= ming {
            guess = ming + sptol - 1;
        }
        if guess >= maxg {
            guess = maxg - sptol + 1;
        }
    }

    // With far enough to go, sieve for semiprimes in blocks. A block that
    // cannot fit in the domain is MPU's `return 0`.
    if n > spcnt && n - spcnt > SP_SIEVE_THRESH {
        let Some(moved) = sieve_forward(n, guess, spcnt) else {
            return Some(0);
        };
        (guess, spcnt) = moved;
    } else if n < spcnt && spcnt - n > SP_SIEVE_THRESH {
        let Some(moved) = sieve_backward(n, guess, spcnt) else {
            return Some(0);
        };
        (guess, spcnt) = moved;
    }

    // Finally, walk over semiprimes to the exact spot.
    while spcnt > n {
        guess = prev_semiprime(guess);
        spcnt -= 1;
    }
    while spcnt < n {
        guess = next_semiprime(guess);
        spcnt += 1;
    }
    Some(guess)
}

/// Width of the next sieve block from the estimated distance `hi - lo`,
/// as computed in MPU `nth_semiprime`: 10 percent and 100 of slack, capped.
fn block_range(hi: u64, lo: u64) -> u64 {
    let range = hi.saturating_sub(lo);
    if range > 113_636_272 {
        125_000_000
    } else {
        range + range / 10 + 100
    }
}

/// Sieve forwards from the semiprime `guess` (number `spcnt`) until the
/// count reaches or passes `n`; returns the new guess and count. `None`
/// stands for MPU's `return 0` when no block fits below the maximum.
fn sieve_forward(n: u64, mut guess: u64, mut spcnt: u64) -> Option<(u64, u64)> {
    while n > spcnt {
        let range = block_range(approx(n), approx(spcnt)).min(MAX_SEMI_PRIME - guess);
        if range == 0 {
            return None;
        }
        let end = guess + range;
        let block = semi_primes(guess + 1, end);
        if block.is_empty() {
            guess = end;
            continue;
        }
        let count = block.len() as u64;
        if spcnt + count <= n {
            guess = block[block.len() - 1];
            spcnt += count;
        } else {
            for &value in &block {
                if spcnt >= n {
                    break;
                }
                guess = value;
                spcnt += 1;
            }
        }
    }
    Some((guess, spcnt))
}

/// Sieve backwards from the semiprime `guess` (number `spcnt`) until the
/// count reaches or passes `n`; returns the new guess and count. `None`
/// stands for MPU's `return 0` when no block fits above zero.
fn sieve_backward(n: u64, mut guess: u64, mut spcnt: u64) -> Option<(u64, u64)> {
    while n < spcnt {
        let range = block_range(approx(spcnt), approx(n)).min(guess);
        if range == 0 {
            return None;
        }
        let beg = guess - range;
        let block = semi_primes(beg, guess - 1);
        if block.is_empty() {
            guess = beg;
            continue;
        }
        let count = block.len() as u64;
        if spcnt - count >= n {
            guess = block[0];
            spcnt -= count;
        } else {
            for &value in block.iter().rev() {
                if n >= spcnt {
                    break;
                }
                guess = value;
                spcnt -= 1;
            }
        }
    }
    Some((guess, spcnt))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_semiprimes() {
        assert_eq!(nth_semiprime(0), None);
        assert_eq!(nth_semiprime(1), Some(4));
        assert_eq!(nth_semiprime(2), Some(6));
        assert_eq!(nth_semiprime(3), Some(9));
    }

    #[test]
    fn past_table() {
        // 623 is the last table entry; 626 = 2 * 313 is the next semiprime.
        assert_eq!(nth_semiprime(190), Some(623));
        assert_eq!(nth_semiprime(191), Some(626));
        assert_eq!(next_semiprime(623), 626);
        assert_eq!(prev_semiprime(626), 623);
        // MPU 0.75 values (tests/fixtures/families.tsv).
        assert_eq!(nth_semiprime(1234), Some(4497));
        assert_eq!(nth_semiprime(12345), Some(51019));
        assert_eq!(nth_semiprime(123_456), Some(573_355));
    }

    #[test]
    fn count_round_trip() {
        for n in [200u64, 500, 999, 2000, 5000, 20_000, 70_000] {
            let s = nth_semiprime(n).unwrap();
            assert!(is_semiprime(s), "{s} is not a semiprime");
            assert_eq!(semiprime_count(s), n);
        }
    }

    #[test]
    fn block_ranges() {
        assert_eq!(block_range(100, 100), 100);
        assert_eq!(block_range(100, 200), 100);
        assert_eq!(block_range(1100, 100), 1200);
        assert_eq!(block_range(200_000_000, 0), 125_000_000);
    }

    #[test]
    fn overflow_boundary() {
        // Returns before any scan, so an unguarded scan would not reach it.
        assert_eq!(nth_semiprime(MAX_SEMI_PRIME_IDX), Some(MAX_SEMI_PRIME));
        assert_eq!(nth_semiprime(MAX_SEMI_PRIME_IDX + 1), Some(0));
        assert_eq!(nth_semiprime(u64::MAX), Some(0));
    }
}

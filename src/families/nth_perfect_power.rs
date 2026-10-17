use super::perfect_power_count_approx::count;
use crate::rootint;

const MAX_POWER: u64 = 18_446_744_065_119_617_025;
const MAX_INDEX: u64 = 4_297_615_581;

// MPU perfect_powers.c, nth_perfect_power_approx. Rust casts saturate at u64::MAX.
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
fn approximate(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    if n >= MAX_INDEX {
        return MAX_POWER;
    }
    let n = n as f64;
    let mut pp =
        n.powf(2.0) + (13.0 / 3.0) * n.powf(4.0 / 3.0) + (32.0 / 15.0) * n.powf(16.0 / 15.0);
    pp += -2.0 * n.powf(5.0 / 3.0) - 2.0 * n.powf(7.0 / 5.0) - 2.0 * n.powf(9.0 / 7.0)
        + 2.0 * n.powf(12.0 / 10.0);
    pp += -2.0 * n.powf(13.0 / 11.0) - 2.0 * n.powf(15.0 / 13.0)
        + 2.0 * n.powf(16.0 / 14.0)
        + 2.0 * n.powf(17.0 / 15.0);
    pp -= 0.48 * n.powf(19.0 / 17.0);
    pp -= 1.5;
    pp as u64
}

// MPU inverse_interpolate.c, interpolate_with_approx, with tolerance 1000.
fn interpolate(n: u64) -> (u64, u64) {
    let approx = approximate(n);
    let mut guess = approx;
    let (mut min, mut max) = (0, u64::MAX);
    for _ in 2..20 {
        let c = count(guess);
        if n.abs_diff(c) < 1000 {
            return (guess, c);
        }
        let approx_count = approximate(c);
        if c <= n && guess > min {
            min = guess;
        }
        if c >= n && guess < max {
            max = guess;
        }
        guess = if approx >= approx_count {
            guess.saturating_add(approx - approx_count)
        } else {
            guess.saturating_sub(approx_count - approx)
        };
        if guess <= min || guess >= max {
            let span = max - min;
            let below = guess <= min;
            guess = if span <= 1 {
                if below { max } else { min }
            } else if 999 >= span {
                min + (span >> 1)
            } else if below {
                min + 999
            } else {
                max - 999
            };
        }
    }
    (guess, count(guess))
}

// MPU perfect_powers.c, _prev_perfect_power, for positive powers.
fn previous(n: u64) -> u64 {
    if n <= 4 {
        return u64::from(n > 1);
    }
    if n <= 8 {
        return 4;
    }
    let mut best = 8;
    for k in 2..=n.ilog2() {
        let r = rootint(n, k);
        let mut c = r.pow(k);
        if c >= n {
            c = (r - 1).pow(k);
        }
        if c > best && c < n {
            best = c;
        }
    }
    best
}

// MPU perfect_powers.c, _next_perfect_power, for positive powers.
fn next(n: u64) -> u64 {
    if n == 0 {
        return 1;
    }
    if n == 1 {
        return 4;
    }
    if n >= MAX_POWER {
        return 0;
    }
    let mut best = (rootint(n, 2) + 1).checked_pow(2).unwrap_or(u64::MAX);
    for k in 3..=1 + n.ilog2() {
        let c = (rootint(n, k) + 1).checked_pow(k).unwrap_or(u64::MAX);
        if c != u64::MAX && c < best && c > n {
            best = c;
        }
    }
    if best == u64::MAX { 0 } else { best }
}

/// Return the `n`th positive perfect power, with 1 at index 1.
///
/// Index 0 returns `None`. Indices at or above 4,297,615,581 return the largest
/// perfect power that fits in `u64`, as in MPU.
/// Port of `nth_perfect_power` in MPU 0.75 `perfect_powers.c`, using
/// `interpolate_with_approx` from `inverse_interpolate.c`.
///
/// ```
/// use math_prime_util::nth_perfect_power;
/// assert_eq!(nth_perfect_power(6), Some(25));
/// assert_eq!(nth_perfect_power(0), None);
/// ```
#[must_use]
pub fn nth_perfect_power(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    if n == 1 {
        return Some(1);
    }
    if n >= MAX_INDEX {
        return Some(MAX_POWER);
    }
    let (g, mut c) = interpolate(n);
    let mut g = g.min(MAX_POWER);
    if c >= n {
        g = previous(g + 1);
        while c > n {
            g = previous(g);
            c -= 1;
        }
    } else {
        while c < n {
            g = next(g);
            c += 1;
        }
    }
    Some(g)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn helper_boundaries() {
        assert_eq!(approximate(0), 0);
        assert_eq!(approximate(1), 1);
        assert_eq!(approximate(MAX_INDEX), MAX_POWER);
        assert_eq!(previous(1), 0);
        assert_eq!(previous(4), 1);
        assert_eq!(previous(8), 4);
        assert_eq!(previous(9), 8);
        assert_eq!(next(0), 1);
        assert_eq!(next(1), 4);
        assert_eq!(next(2), 4);
        assert_eq!(next(MAX_POWER - 1), MAX_POWER);
        assert_eq!(next(MAX_POWER), 0);
    }

    #[test]
    fn large_indices_and_saturation() {
        for n in [1_000_000, 100_000_000, MAX_INDEX - 1] {
            let p = nth_perfect_power(n).unwrap();
            assert_eq!(count(p), n);
            assert_eq!(count(p - 1), n - 1);
        }
        assert_eq!(nth_perfect_power(MAX_INDEX), Some(MAX_POWER));
        assert_eq!(nth_perfect_power(u64::MAX), Some(MAX_POWER));
    }
}

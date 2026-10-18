use crate::{allsqrtmod, is_prime, is_square_root, sqrtint, sqrtmod};

/// Signed Kronecker symbol `(a/b)` for unsigned arguments.
///
/// Same recurrence as MPU `util.c::kronecker_uu`.
fn kronecker_uu(a: u64, mut b: u64) -> i32 {
    if b == 0 {
        return i32::from(a == 1);
    }
    let mut sign = 1;
    let zeros = b.trailing_zeros();
    if zeros > 0 {
        if a % 2 == 0 {
            return 0;
        }
        if zeros % 2 == 1 && matches!(a % 8, 3 | 5) {
            sign = -sign;
        }
        b >>= zeros;
    }
    kronecker_odd(a, b, sign)
}

fn kronecker_odd(mut a: u64, mut b: u64, mut sign: i32) -> i32 {
    while a != 0 {
        let zeros = a.trailing_zeros();
        if zeros % 2 == 1 && matches!(b % 8, 3 | 5) {
            sign = -sign;
        }
        a >>= zeros;
        if a & b & 2 != 0 {
            sign = -sign;
        }
        (a, b) = (b % a, a);
    }
    if b == 1 { sign } else { 0 }
}

/// One Cornacchia step from a square root of `-d` modulo `p`.
///
/// Algorithm 2.3.12 of Cohen and Pomerance, as in MPU `util.c::corn_one`.
fn corn_one(u: u64, d: u64, p: u64) -> Option<(u64, u64)> {
    let mut a = p;
    // The two roots are `u` and `p - u`; keep the larger one.
    let mut b = u.max(p - u);
    let limit = sqrtint(p);
    while b > limit {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    // `b <= isqrt(p)`, so `b * b` fits in `u64` and is at most `p`.
    let remainder = p - b * b;
    if remainder % d != 0 {
        return None;
    }
    let scaled = remainder / d;
    if scaled == 0 {
        return None;
    }
    is_square_root(scaled).map(|y| (b, y))
}

/// Cornacchia-Smith over the first half of the square roots of `-d` modulo `p`.
///
/// MPU `util.c::corn_all`. Roots come in `±` pairs; the larger partner is
/// selected inside [`corn_one`].
fn corn_all(neg_d: u64, d: u64, p: u64) -> Option<(u64, u64)> {
    let roots = allsqrtmod(neg_d, p);
    for &root in roots.iter().take(roots.len() / 2) {
        if let Some(solution) = corn_one(root, d, p) {
            return Some(solution);
        }
    }
    None
}

/// Solves `x^2 + d*y^2 = p` for `(x, y)`, or `None` if there is no solution.
///
/// Port of MPU 0.75 `util.c::cornacchia` (`corn_one`, `corn_all`). For a prime
/// `p` the Kronecker symbol `(-d/p)` rejects impossible cases before any
/// search. A composite `p >= 2^53` tries the square roots of `-d` modulo `p`
/// before the bounded scan.
///
/// # Panics
/// Panics when that composite root path asks [`allsqrtmod`] for more than
/// MPU's limit of 600 million roots.
///
/// ```
/// use math_prime_util::cornacchia;
/// assert_eq!(cornacchia(1, 5), Some((2, 1)));
/// assert_eq!(cornacchia(1, 3), None);
/// ```
pub fn cornacchia(d: u64, p: u64) -> Option<(u64, u64)> {
    if p == 0 {
        return Some((0, 0));
    }
    if d == 0 {
        return is_square_root(p).map(|root| (root, 0));
    }
    let reduced = d % p;
    let neg_d = if reduced == 0 { 0 } else { p - reduced };
    if is_prime(p) != 0 {
        if kronecker_uu(neg_d, p) == -1 {
            return None;
        }
        return corn_one(sqrtmod(neg_d, p)?, d, p);
    }
    // `(p >> 31) >> 22` in the C source: `p >= 2^53`.
    if p >= 1_u64 << 53 && kronecker_uu(neg_d, p) != -1 {
        if let Some(solution) = corn_all(neg_d, d, p) {
            return Some(solution);
        }
    }
    let lim_u = sqrtint(p / d);
    for u in 0..=lim_u {
        let Some(square) = u.checked_mul(u) else {
            break;
        };
        let Some(scaled) = d.checked_mul(square) else {
            break;
        };
        if scaled > p {
            break;
        }
        if let Some(root) = is_square_root(p - scaled) {
            return Some((root, u));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{corn_one, cornacchia, kronecker_uu};
    use std::sync::mpsc;
    use std::time::Duration;

    #[test]
    fn cornacchia_cases() {
        assert_eq!(cornacchia(0, 0), Some((0, 0)));
        assert_eq!(cornacchia(0, 13), None);
        assert_eq!(cornacchia(0, 16), Some((4, 0)));
        assert_eq!(cornacchia(1, 5), Some((2, 1)));
        assert_eq!(cornacchia(1, 3), None);
        assert_eq!(cornacchia(3, 23), None);
        assert_eq!(cornacchia(3, 25), Some((5, 0)));
    }

    #[test]
    fn kronecker_uu_edges() {
        assert_eq!(kronecker_uu(0, 0), 0);
        assert_eq!(kronecker_uu(1, 0), 1);
        assert_eq!(kronecker_uu(2, 4), 0);
        assert_eq!(kronecker_uu(3, 2), -1);
        assert_eq!(kronecker_uu(1, 1), 1);
    }

    #[test]
    fn corn_one_rejects_a_non_solution() {
        assert_eq!(corn_one(1, 5, 17), None);
    }

    /// `2^61 - 1` is prime and `3 (mod 4)`. The scan would examine about
    /// `2^30.5` candidates; the Kronecker test returns `None` at once.
    #[test]
    fn rejects_mersenne_prime_without_scanning() {
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            let _ = tx.send(cornacchia(1, (1_u64 << 61) - 1));
        });
        assert_eq!(rx.recv_timeout(Duration::from_secs(2)), Ok(None));
    }

    /// `p = 134217757 * 134217869` (`>= 2^53`). The `u = 0` scan finds
    /// `(132396848, 22033973)`. `corn_all` returns the other representation.
    #[test]
    fn large_composite_uses_square_root_path() {
        let p = 18_014_421_326_499_833_u64;
        assert_eq!(cornacchia(1, p), Some((114_145_427, 70_606_252)));
    }

    /// Prime above `i64::MAX`, `1 (mod 4)`, so the unsigned Kronecker path
    /// must accept it and `corn_one` recovers the representation.
    #[test]
    fn prime_above_i64_max_has_a_representation() {
        let p = 18_446_744_073_709_551_557_u64;
        assert_eq!(cornacchia(1, p), Some((3_995_190_446, 1_576_450_879)));
    }
}

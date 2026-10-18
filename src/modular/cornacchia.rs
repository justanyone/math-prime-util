use crate::{is_square_root, sqrtint};

/// Solves `x^2 + d*y^2 = p` for `(x, y)`, or `None` if there is no solution.
///
/// Ported from MPU's `util.c` (`cornacchia`). For `p == 0` it returns
/// `(0, 0)`. For `d == 0` it returns `(sqrt(p), 0)` when `p` is a square.
/// Otherwise it scans `u` from `0` to `isqrt(p / d)` and returns the first
/// `(x, u)` with `p - d*u^2` a perfect square. The scan finds a solution
/// when one exists, but the pair it returns may differ from MPU's pair.
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
    let limu = sqrtint(p / d);
    (0..=limu).find_map(|u| {
        let t = p - d * u * u;
        is_square_root(t).map(|root| (root, u))
    })
}

#[cfg(test)]
mod tests {
    use super::cornacchia;

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
}

/// Sum `floor((a*i + b) / m)` for `i` in `0..n`.
///
/// Port of `util.c`'s `floor_sum` in MPU 0.75. Uses the same Euclidean
/// reduction, with `u128` intermediates. Returns `None` if the sum exceeds
/// `u64::MAX`.
///
/// # Panics
/// Panics if `m` is zero.
///
/// # Examples
/// ```
/// use math_prime_util::floor_sum;
/// assert_eq!(floor_sum(4, 10, 6, 3), Some(3));
/// ```
pub fn floor_sum(n: u64, m: u64, a: u64, b: u64) -> Option<u64> {
    assert!(m != 0, "floor_sum: divide by zero");
    if n == 0 {
        return Some(0);
    }
    let (mut n, mut m, mut a, mut b) = (u128::from(n), u128::from(m), u128::from(a), u128::from(b));
    let limit = u128::from(u64::MAX);
    let mut sum = 0;
    loop {
        if a >= m {
            let y = n * (n - 1) / 2;
            let w = a / m;
            a %= m;
            if y > (limit - sum) / w {
                return None;
            }
            sum += y * w;
        }
        if b >= m {
            let w = b / m;
            b %= m;
            if n > (limit - sum) / w {
                return None;
            }
            sum += n * w;
        }
        let ymax = a * n + b;
        if ymax < m {
            break;
        }
        n = ymax / m;
        b = ymax % m;
        std::mem::swap(&mut m, &mut a);
    }
    u64::try_from(sum).ok()
}

#[cfg(test)]
mod tests {
    use super::floor_sum;

    #[test]
    fn boundaries() {
        assert_eq!(floor_sum(0, 1, u64::MAX, u64::MAX), Some(0));
        assert_eq!(floor_sum(1, 1, u64::MAX, u64::MAX), Some(u64::MAX));
        assert_eq!(floor_sum(2, 1, 0, u64::MAX), None);
        assert_eq!(floor_sum(u64::MAX, 1, 1, 0), None);
        assert_eq!(floor_sum(2, u64::MAX, u64::MAX - 1, 2), Some(1));
        assert_eq!(floor_sum(4, 10, 6, 3), Some(3));
        assert_eq!(floor_sum(10, 7, 0, 15), Some(20));
    }

    #[test]
    #[should_panic(expected = "divide by zero")]
    fn zero_modulus() {
        floor_sum(0, 0, 0, 0);
    }
}

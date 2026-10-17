use crate::{gcd, is_prime, is_square};

/// Return whether `n` is a sum of `k` squares.
///
/// Zero is allowed as a square. In Perl, `k` defaults to 2. Pass `2` for that call.
/// `k == 0` is true only for `n == 0`. `k == 1` is the square test.
/// Every `n` is a sum of 4 or more squares.
///
/// Port of `is_sum_of_squares` in MPU 0.75 `XS.xs`.
/// `k == 2` follows `is_sum_of_two_squares` in `util.c`.
/// `k == 3` follows `is_sum_of_three_squares` in `util.c`.
///
/// ```
/// use math_prime_util::is_sum_of_squares;
/// assert!(is_sum_of_squares(5, 2));
/// assert!(!is_sum_of_squares(3, 2));
/// assert!(is_sum_of_squares(7, 4));
/// ```
pub fn is_sum_of_squares(n: u64, k: u64) -> bool {
    if k == 0 {
        return n == 0;
    }
    if k == 1 {
        return is_square(n);
    }
    if k > 3 || n < 3 {
        return true;
    }
    if k == 3 {
        return is_sum_of_three_squares(n);
    }
    is_sum_of_two_squares(n)
}

/// `util.c` `is_sum_of_three_squares`: true unless `n = 4^a * (8b + 7)`.
fn is_sum_of_three_squares(n: u64) -> bool {
    let tz = n.trailing_zeros();
    (tz & 1) == 1 || ((n >> tz) & 7) != 7
}

/// `util.c` `is_sum_of_two_squares`.
fn is_sum_of_two_squares(mut n: u64) -> bool {
    n >>= n.trailing_zeros();
    if n % 4 == 3 {
        return false;
    }
    for prime in [3, 7, 11, 19, 23, 31] {
        let mut exp = 0u32;
        while n % prime == 0 {
            n /= prime;
            exp += 1;
        }
        if exp % 2 == 1 {
            return false;
        }
    }
    !odd_exponent_3_mod_4(n)
}

fn odd_exponent_3_mod_4(n: u64) -> bool {
    if n == 1 {
        return false;
    }
    let mut factors = Vec::new();
    push_factors(n, &mut factors);
    factors.sort_unstable();
    let mut index = 0;
    while index < factors.len() {
        let prime = factors[index];
        let mut exp = 0u32;
        while index < factors.len() && factors[index] == prime {
            exp += 1;
            index += 1;
        }
        if prime % 4 == 3 && exp % 2 == 1 {
            return true;
        }
    }
    false
}

fn push_factors(n: u64, factors: &mut Vec<u64>) {
    let mut pending = vec![n];
    while let Some(value) = pending.pop() {
        if value == 1 {
            continue;
        }
        if is_prime(value) != 0 {
            factors.push(value);
            continue;
        }
        let divisor = pollard(value);
        pending.push(divisor);
        pending.push(value / divisor);
    }
}

fn pollard(n: u64) -> u64 {
    let mut add = 1u64;
    loop {
        let divisor = rho(n, add);
        if divisor != n {
            return divisor;
        }
        add += 1;
    }
}

fn rho(n: u64, add: u64) -> u64 {
    let mut x = 2u64;
    let mut y = 2u64;
    let mut divisor = 1u64;
    while divisor == 1 {
        x = step(x, add, n);
        y = step(step(y, add, n), add, n);
        divisor = gcd(x.abs_diff(y), n);
    }
    divisor
}

#[allow(clippy::cast_possible_truncation)]
fn step(x: u64, add: u64, n: u64) -> u64 {
    ((u128::from(x) * u128::from(x) + u128::from(add)) % u128::from(n)) as u64
}

#[cfg(test)]
mod tests {
    use super::is_sum_of_squares;
    use crate::is_prime;

    #[test]
    fn bounds_and_small_counts() {
        assert!(is_sum_of_squares(0, 0));
        assert!(!is_sum_of_squares(1, 0));
        assert!(is_sum_of_squares(0, 1));
        assert!(is_sum_of_squares(1, 1));
        assert!(!is_sum_of_squares(2, 1));
        assert!(is_sum_of_squares(0, 2));
        assert!(is_sum_of_squares(2, 2));
        assert!(!is_sum_of_squares(3, 2));
        assert!(is_sum_of_squares(4, 2));
        assert!(!is_sum_of_squares(7, 2));
        assert!(is_sum_of_squares(6, 3));
        assert!(!is_sum_of_squares(7, 3));
        assert!(!is_sum_of_squares(28, 3));
        assert!(is_sum_of_squares(7, 4));
        assert!(is_sum_of_squares(7, 5));
        assert!(is_sum_of_squares(u64::MAX, u64::MAX));
        assert!(!is_sum_of_squares(u64::MAX, 2));
        assert!(is_sum_of_squares(18_446_744_073_709_551_557, 2));
    }

    #[test]
    fn large_semiprime_uses_pollard() {
        let p = 100_003u64;
        let q = 100_019u64;
        assert_eq!(is_prime(p), 2);
        assert_eq!(is_prime(q), 2);
        assert_eq!(p % 4, 3);
        assert_eq!(q % 4, 3);
        assert!(!is_sum_of_squares(p * q, 2));
        assert!(is_sum_of_squares(p * p, 2));
        assert!(is_sum_of_squares(53, 2));
    }

    #[test]
    fn repeat_and_threads_agree() {
        let samples = [(0u64, 0u64), (7, 3), (65, 2), (p_q(), 2), (u64::MAX, 2)];
        let once: Vec<_> = samples
            .iter()
            .map(|&(n, k)| is_sum_of_squares(n, k))
            .collect();
        assert_eq!(
            once,
            samples
                .iter()
                .map(|&(n, k)| is_sum_of_squares(n, k))
                .collect::<Vec<_>>()
        );
        let threads: Vec<_> = (0..4)
            .map(|_| {
                std::thread::spawn(move || {
                    samples
                        .iter()
                        .map(|&(n, k)| is_sum_of_squares(n, k))
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        for thread in threads {
            assert_eq!(thread.join().unwrap(), once);
        }
    }

    fn p_q() -> u64 {
        100_003 * 100_019
    }
}

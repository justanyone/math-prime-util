/// Test whether `n` is the area of a rational right triangle.
///
/// Port of `congruent_numbers.c`, `is_congruent_number`,
/// `remove_square_part`, and `_is_congruent_number_tunnell`, MPU 0.75.
/// This port uses the ACK filter and Tunnell's count. The other C factor
/// filters are not yet ported, so large inputs can take much longer.
/// Tunnell's test assumes the weak Birch and Swinnerton-Dyer conjecture.
///
/// ```
/// use math_prime_util::is_congruent_number;
/// assert!(is_congruent_number(6));
/// assert!(!is_congruent_number(3));
/// ```
pub fn is_congruent_number(mut n: u64) -> bool {
    if n < 5 {
        return false;
    }
    let mut squarefree = 1;
    let mut prime = 2;
    while prime <= n / prime {
        let mut odd = false;
        while n % prime == 0 {
            n /= prime;
            odd = !odd;
        }
        if odd {
            squarefree *= prime;
        }
        prime += if prime == 2 { 1 } else { 2 };
    }
    n *= squarefree;
    if n < 13 {
        return (5..=7).contains(&n);
    }
    // Alter, Curtz, and Kubota filter from _is_congruent_number_filter1.
    if n % 8 >= 5 {
        return true;
    }
    tunnell(n)
}

fn tunnell(n: u64) -> bool {
    let mut solutions = [0u128; 2];
    let odd = n % 2 == 1;
    let target = if odd { n } else { n / 2 };
    for z in 0..=crate::sqrtint(target / 8) {
        let remainder = target - 8 * z * z;
        let mut count = 0u128;
        if odd {
            for y in 0..=crate::sqrtint(remainder / 2) {
                if crate::is_square(remainder - 2 * y * y) {
                    count += 1 << (1 + u32::from(y > 0) + u32::from(z > 0));
                }
            }
        } else {
            for x in (1..=crate::sqrtint(remainder)).step_by(2) {
                let y = remainder - x * x;
                if crate::is_square(y) {
                    count += 1 << (1 + u32::from(y > 0) + u32::from(z > 0));
                }
            }
        }
        solutions[usize::from(z % 2 != 0)] += count;
    }
    solutions[0] == solutions[1]
}

#[cfg(test)]
mod tests {
    use super::tunnell;

    /// Number of integer triples with `a*x^2 + b*y^2 + c*z^2 == m`.
    fn count(a: i64, b: i64, c: i64, m: i64) -> i64 {
        let mut total = 0;
        let bound = |k: i64| (0..=m).take_while(|t| k * t * t <= m).last().unwrap_or(0);
        for x in -bound(a)..=bound(a) {
            for y in -bound(b)..=bound(b) {
                for z in -bound(c)..=bound(c) {
                    if a * x * x + b * y * y + c * z * z == m {
                        total += 1;
                    }
                }
            }
        }
        total
    }

    /// Tunnell's criterion straight from the theorem, for squarefree `n`.
    fn reference(n: i64) -> bool {
        if n % 2 == 1 {
            count(1, 2, 8, n) == 2 * count(1, 2, 32, n)
        } else {
            count(4, 1, 8, n / 2) == 2 * count(4, 1, 32, n / 2)
        }
    }

    fn squarefree(n: u64) -> bool {
        (2..=n).take_while(|d| d * d <= n).all(|d| n % (d * d) != 0)
    }

    #[test]
    fn tunnell_matches_the_theorem() {
        for n in 1..=250_u64 {
            if squarefree(n) {
                assert_eq!(tunnell(n), reference(i64::try_from(n).unwrap()), "n={n}");
            }
        }
    }

    #[test]
    fn tunnell_known_values() {
        for n in [
            5_u64, 6, 7, 13, 14, 15, 21, 22, 23, 29, 30, 31, 34, 37, 38, 39,
        ] {
            assert!(tunnell(n), "{n}");
        }
        for n in [1_u64, 2, 3, 10, 11, 17, 19, 25, 26, 33, 35] {
            assert!(!tunnell(n), "{n}");
        }
    }
}

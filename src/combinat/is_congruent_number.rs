/// Test whether `n` is the area of a rational right triangle.
///
/// Port of `congruent_numbers.c`, `is_congruent_number`,
/// `remove_square_part`, and `_is_congruent_number_tunnell`, MPU 0.75.
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

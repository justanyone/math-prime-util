/// Returns the reduced harmonic sum `1 + 1/2 + ... + 1/n`.
///
/// Returns `None` when the numerator exceeds `i128` or the denominator exceeds
/// `u128`. Port of MPU 0.75 `util.c`, `harmfrac` and `_harmonic_split`, with
/// wider arithmetic and common denominator factors cancelled before products.
///
/// ```
/// assert_eq!(math_prime_util::harmfrac(3), Some((11, 6)));
/// assert_eq!(math_prime_util::harmfrac(0), Some((0, 1)));
/// ```
pub fn harmfrac(n: u64) -> Option<(i128, u128)> {
    // H_89 exceeds i128. Later positive sums cannot fit this representation.
    if n >= 89 {
        return None;
    }
    if n == 0 {
        return Some((0, 1));
    }
    let (num, den) = harmonic_split(1, u128::from(n) + 1)?;
    Some((i128::try_from(num).ok()?, den))
}

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn harmonic_split(a: u128, b: u128) -> Option<(u128, u128)> {
    let (num, den) = match b - a {
        1 => return Some((1, a)),
        2 => (2 * a + 1, a * (a + 1)),
        _ => {
            let mid = (a + b) / 2;
            let (p, q) = harmonic_split(a, mid)?;
            let (r, s) = harmonic_split(mid, b)?;
            let g = gcd(q, s);
            let num = p.checked_mul(s / g)?.checked_add(r.checked_mul(q / g)?)?;
            (num, (q / g).checked_mul(s)?)
        }
    };
    let g = gcd(num, den);
    Some((num / g, den / g))
}

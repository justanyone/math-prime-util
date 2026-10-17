use super::random_maurer_prime_with_cert::nbit_prime;
use super::urandomm;

/// Returns a semiprime with `bits` bits and an even split of factor bits.
/// Ports `random_prime.c`: `random_semiprime`, including its small tables.
/// Returns `None` above 64 bits until bigint support is available.
/// # Panics
/// Panics if `bits < 4`, as in MPU's XS wrapper, or if OS entropy fails.
/// ```
/// math_prime_util::srand(15);
/// assert_eq!(math_prime_util::random_semiprime(4), Some(9));
/// assert_eq!(math_prime_util::random_semiprime(65), None);
/// ```
pub fn random_semiprime(bits: u32) -> Option<u64> {
    assert!(bits >= 4, "random_semiprime, bits must be >= 4");
    if bits > 64 {
        return None;
    }
    const SMALL: [u64; 14] = [35, 35, 49, 65, 77, 91, 143, 143, 169, 299, 319, 341, 377, 403];
    match bits {
        4 => return Some(9),
        5 => return Some(21),
        6..=9 => {
            let start = 3 * usize::try_from(bits - 6).unwrap();
            let range = if bits == 9 { 5 } else { 3 };
            return Some(SMALL[start + usize::try_from(urandomm(range)).unwrap()]);
        }
        _ => {}
    }
    let min = 1_u64 << (bits - 1);
    let max = min + (min - 1);
    loop {
        let n = nbit_prime(bits / 2) * nbit_prime(bits - bits / 2);
        if (min..=max).contains(&n) {
            return Some(n);
        }
    }
}

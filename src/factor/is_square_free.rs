use super::is_semiprime::factors;

/// Return whether no prime square divides `n`. Zero is not square free.
///
/// Port of MPU 0.75 `util.c::is_square_free` and
/// `factor.c::factoredp_is_square_free` (`factor.h::factored_is_square_free`).
///
/// ```
/// use math_prime_util::is_square_free;
/// assert!(is_square_free(30));
/// assert!(!is_square_free(12));
/// ```
pub fn is_square_free(n: u64) -> bool {
    const TABLE: [u32; 8] = [
        3_840_601_326,
        1_856_556_782,
        3_941_394_158,
        2_362_371_810,
        3_970_362_990,
        3_471_729_898,
        4_008_603_310,
        3_938_642_668,
    ];
    if n < 256 {
        return TABLE[(n >> 5) as usize] & (1 << (n % 32)) != 0;
    }
    for p in [2, 3, 5, 7, 11, 13] {
        if n % (p * p) == 0 {
            return false;
        }
    }
    for p in [17, 19, 23, 29, 31, 37] {
        if p * p > n {
            return true;
        }
        if n % (p * p) == 0 {
            return false;
        }
    }
    let f = factors(n);
    !f.windows(2).any(|pair| pair[0] == pair[1])
}

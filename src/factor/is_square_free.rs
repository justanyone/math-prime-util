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
        3840601326, 1856556782, 3941394158, 2362371810, 3970362990, 3471729898, 4008603310,
        3938642668,
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

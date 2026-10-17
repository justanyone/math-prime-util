//! Perrin pseudoprime test from MPU 0.75 `primality.c`.
//!
//! The filter tables are copied from `_perrindata` and `_perrinmask`.
use super::{signed_jacobi, sub};
use crate::modular::{addmod_u, mulmod_u};

/// Modular filter rows: `(divisor, period, offset into PERRIN_MASK)`.
/// Port of `_perrindata` in MPU `primality.c`.
const PERRIN_DATA: [(u16, u16, u16); 19] = [
    (2, 7, 0),
    (3, 13, 1),
    (4, 14, 2),
    (5, 24, 3),
    (7, 48, 4),
    (9, 39, 6),
    (11, 120, 8),
    (13, 183, 12),
    (17, 288, 18),
    (19, 180, 27),
    (23, 22, 33),
    (25, 120, 34),
    (29, 871, 38),
    (31, 993, 66),
    (37, 1368, 98),
    (41, 1723, 141),
    (43, 231, 195),
    (47, 2257, 203),
    (223, 111, 274),
];

/// Bit masks for the `PERRIN_DATA` filters. Port of `_perrinmask` in MPU `primality.c`.
const PERRIN_MASK: &[u32] = &[
    22, 523, 514, 65890, 8519810, 130, 4259842, 0, 526338, 2147483904, 1644233728, 1, 8194, 1073774592, 1024, 134221824, 128, 512, 181250, 2048, 0, 1, 134217736, 1049600, 524545, 2147500288, 0, 524290, 536870912, 32768, 33554432, 2048, 0, 2, 2, 256, 65536, 64, 536875010, 32768, 256, 64, 0, 32, 1073741824, 0, 1048576, 1048832, 371200000, 0, 0, 536887552, 32, 2147487744, 2097152, 32768, 1024, 0, 1024, 536870912, 128, 512, 0, 0, 512, 0, 2147483650, 45312, 128, 0, 8388640, 0, 8388608, 8388608, 0, 2048, 4096, 92800000, 262144, 0, 65536, 4, 0, 4, 4, 4194304, 8388608, 1075838976, 536870956, 0, 134217728, 8192, 0, 8192, 8192, 0, 2, 0, 268435458, 134223392, 1073741824, 268435968, 2097152, 67108864, 0, 8192, 1073741840, 0, 0, 128, 0, 0, 512, 1450000, 8, 131136, 536870928, 0, 4, 2097152, 4096, 64, 0, 32768, 0, 0, 131072, 371200000, 2048, 33570816, 4096, 32, 1024, 536870912, 1048576, 16384, 0, 8388608, 0, 0, 0, 2, 512, 0, 128, 0, 134217728, 2, 32, 0, 0, 0, 0, 8192, 0, 1073742080, 536870912, 0, 4096, 16777216, 526336, 32, 0, 65536, 33554448, 708, 67108864, 2048, 0, 0, 536870912, 0, 536870912, 33554432, 33554432, 2147483648, 512, 64, 0, 1074003968, 512, 0, 524288, 0, 0, 0, 67108864, 524288, 1048576, 0, 131076, 0, 33554432, 131072, 0, 2, 8390656, 16384, 16777216, 134217744, 0, 131104, 0, 2, 32768, 0, 0, 0, 1450000, 32768, 0, 0, 0, 0, 0, 16, 0, 1024, 16400, 1048576, 32, 1024, 0, 260, 536870912, 269484032, 0, 16384, 0, 524290, 0, 0, 512, 65536, 0, 0, 0, 134217732, 0, 67108880, 536887296, 0, 0, 32, 0, 65568, 0, 524288, 2147483648, 0, 4096, 4096, 134217984, 268500992, 0, 33554432, 131072, 0, 0, 0, 16777216, 0, 0, 0, 0, 0, 524288, 0, 0, 67108864, 0, 0, 2, 0, 2, 32, 1024, 0
];

/// Perrin signature `S` of `n` by the doubling rule of Adams and Shanks (1982).
/// Port of `calc_perrin_sig` in MPU `primality.c`, in plain `u128` arithmetic.
fn perrin_signature(n: u64) -> [u64; 6] {
    let mut s = [1, n - 1, 3, 3, 0, 2];
    if n <= 1 {
        return s;
    }
    let bits = 64 - n.leading_zeros();
    for k in (0..bits - 1).rev() {
        let sq = |x: u64| mulmod_u(x, x, n);
        let t0 = sub(sub(sq(s[0]), s[5], n), s[5], n);
        let t1 = sub(sub(sq(s[1]), s[4], n), s[4], n);
        let t2 = sub(sub(sq(s[2]), s[3], n), s[3], n);
        let t3 = sub(sub(sq(s[3]), s[2], n), s[2], n);
        let t4 = sub(sub(sq(s[4]), s[1], n), s[1], n);
        let t5 = sub(sub(sq(s[5]), s[0], n), s[0], n);
        let t01 = sub(t2, t1, n);
        let t34 = sub(t5, t4, n);
        let t45 = addmod_u(t34, t3, n);
        s = if (n >> k) & 1 == 1 {
            [t0, t01, t1, t4, t45, t5]
        } else {
            [t01, t1, addmod_u(t01, t0, n), t34, t4, t45]
        };
    }
    s
}

/// Tests whether `n` divides the Perrin number `P(n)`.
/// `restricted` 0 is the plain test, 1 adds `P(-n) = -1`,
/// 2 adds the Adams-Shanks signature test, and 3 adds the Arno/Grantham
/// rejection of the Jacobi zero case. Larger values also reject even `n`.
///
/// Port of MPU `primality.c`, `is_perrin_pseudoprime`.
///
/// ```
/// use math_prime_util::is_perrin_pseudoprime;
/// assert!(is_perrin_pseudoprime(271_441, 0));
/// assert!(!is_perrin_pseudoprime(10, 0));
/// ```
pub fn is_perrin_pseudoprime(n: u64, restricted: u32) -> bool {
    if n < 3 {
        return n >= 2;
    }
    if n % 2 == 0 && restricted > 2 {
        return false;
    }
    let n32 = (n % 10920) as u32;
    if n32 % 2 == 0 && (22 >> (n32 % 7)) & 1 == 0 {
        return false;
    }
    if n32 % 3 == 0 && (523 >> (n32 % 13)) & 1 == 0 {
        return false;
    }
    if n32 % 5 == 0 && (65890 >> (n32 % 24)) & 1 == 0 {
        return false;
    }
    if n32 % 4 == 0 && (514 >> (n32 % 14)) & 1 == 0 {
        return false;
    }
    for &(div, period, offset) in &PERRIN_DATA[4..] {
        if n % u64::from(div) == 0 {
            let mod_p = n % u64::from(period);
            let word = PERRIN_MASK[usize::from(offset) + (mod_p / 32) as usize];
            if (word >> (mod_p % 32)) & 1 == 0 {
                return false;
            }
        }
    }
    let s = perrin_signature(n);
    if s[4] != 0 {
        return false;
    }
    if restricted == 0 {
        return true;
    }
    if s[1] != n - 1 {
        return false;
    }
    if restricted == 1 {
        return true;
    }
    let jacobi = signed_jacobi(-23, n);
    if jacobi == -1 {
        let b = s[2];
        let b2 = mulmod_u(b, b, n);
        let a = sub(addmod_u(1, mulmod_u(b, 3, n), n), b2, n);
        let c = sub(mulmod_u(b2, 3, n), 2, n);
        return s[0] == a
            && s[2] == b
            && s[3] == b
            && s[5] == c
            && b != 3
            && sub(mulmod_u(b2, b, n), b, n) == 1;
    }
    if jacobi == 0 && n != 23 && restricted > 2 {
        return false;
    }
    if s[0] == 1 && s[2] == 3 && s[3] == 3 && s[5] == 2 {
        return true;
    }
    s[0] == 0
        && s[5] == n - 1
        && s[2] != s[3]
        && addmod_u(s[2], s[3], n) == n - 3
        && mulmod_u(sub(s[2], s[3], n), sub(s[2], s[3], n), n) == n - (23 % n)
}

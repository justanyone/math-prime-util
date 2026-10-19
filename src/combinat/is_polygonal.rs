/// Test whether `n` is a polygonal number with `k` sides.
///
/// Port of `util.c`, `polygonal_root`, from MPU 0.75.
/// Use the square discriminant and the integral root test. A discriminant
/// beyond 128 bits leaves at most three candidate roots, which are tested
/// directly.
///
/// # Panics
/// Panics if `k < 3`.
///
/// ```
/// use math_prime_util::is_polygonal;
/// assert!(is_polygonal(15, 3));
/// assert!(!is_polygonal(14, 3));
/// ```
pub fn is_polygonal(n: u64, k: u64) -> bool {
    assert!(k >= 3, "is_polygonal root < 3");
    if n <= 1 {
        return true;
    }
    if k == 4 {
        return crate::is_square(n);
    }
    // k > n cannot produce a polygonal number after the first one.
    if k > n {
        return false;
    }
    let k = u128::from(k);
    let offset = k.abs_diff(4);
    let Some(d) = (8 * k - 16)
        .checked_mul(u128::from(n))
        .and_then(|d| d.checked_add(offset * offset))
    else {
        // The discriminant exceeds 128 bits only when `9 (k - 2) n >= 2^128`,
        // so with `k <= n < 2^64` the root `r` has `r (r - 1) <= 2n / (k - 2) < 18`:
        // test the candidates `r = 2, 3, 4` directly.
        let n = u128::from(n);
        return (2..=4).any(|r| r * ((k - 2) * (r - 1) + 2) / 2 == n);
    };
    let root = sqrt_u128(d);
    if root * root != d {
        return false;
    }
    let numerator = if k == 3 { root - 1 } else { root + k - 4 };
    numerator % (2 * k - 4) == 0
}

fn sqrt_u128(n: u128) -> u128 {
    if n < 2 {
        return n;
    }
    let mut x = 1u128 << ((128 - n.leading_zeros()).div_ceil(2));
    loop {
        // x and n/x are at most 2^64 after initialization.
        #[allow(clippy::manual_midpoint)] // midpoint is newer than MSRV 1.85.
        let next = (x + n / x) / 2;
        if next >= x {
            return x;
        }
        x = next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sqrt_u128_small_and_boundary_values() {
        for (n, root) in [
            (2_u128, 1),
            (3, 1),
            (4, 2),
            (5, 2),
            (8, 2),
            (9, 3),
            (15, 3),
            (16, 4),
        ] {
            assert_eq!(sqrt_u128(n), root, "n={n}");
        }
        for r in [1_u128 << 40, (1 << 63) + 12_345, u128::from(u64::MAX)] {
            assert_eq!(sqrt_u128(r * r), r);
            assert_eq!(sqrt_u128(r * r - 1), r - 1);
            assert_eq!(sqrt_u128(r * r + 1), r);
        }
    }

    #[test]
    fn large_discriminants() {
        assert_eq!(sqrt_u128(0), 0);
        assert_eq!(sqrt_u128(1), 1);
        assert_eq!(sqrt_u128(u128::MAX), u128::from(u64::MAX));
        assert!(is_polygonal(u64::MAX, u64::MAX));
        assert!(!is_polygonal(u64::MAX - 1, u64::MAX));
        assert!(is_polygonal(u64::MAX - 1, u64::MAX - 1));
        let k = u64::MAX / 3;
        assert!(is_polygonal(3 * k - 3, k));
        assert!(!is_polygonal(3 * k - 2, k));
        // The 4th k-gonal number is 6k - 8; the discriminant overflows u128.
        let k = u64::MAX / 6;
        assert!(is_polygonal(6 * k - 8, k));
        assert!(!is_polygonal(6 * k - 7, k));
        assert!(!is_polygonal(6 * k - 9, k));
    }
}

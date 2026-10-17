/// Test whether `n` is a polygonal number with `k` sides.
///
/// Port of `util.c`, `polygonal_root`, from MPU 0.75.
/// Use the square discriminant and the integral root test.
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
        // The discriminant can exceed u128 when k is large. In this
        // range the second polygonal number is k and the third exceeds u64.
        return u128::from(n) == k;
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
    fn large_discriminants() {
        assert_eq!(sqrt_u128(0), 0);
        assert_eq!(sqrt_u128(1), 1);
        assert_eq!(sqrt_u128(u128::MAX), u128::from(u64::MAX));
        assert!(is_polygonal(u64::MAX, u64::MAX));
        assert!(!is_polygonal(u64::MAX - 1, u64::MAX));
        assert!(is_polygonal(u64::MAX - 1, u64::MAX - 1));
    }
}

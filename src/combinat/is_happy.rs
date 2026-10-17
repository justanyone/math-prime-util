/// Returns the happy height, or zero if the digit-power sequence cycles.
///
/// The height of one is one. Use base 10 and exponent 2 for ordinary happy
/// numbers. Port of `happy_height`, `sum_of_digits`, and
/// `sum_of_squared_digits` in MPU 0.75 `util.c`.
///
/// # Panics
///
/// Panics if `base` is outside `2..=36` or `exponent` exceeds 10.
///
/// ```
/// assert_eq!(math_prime_util::is_happy(19, 10, 2), 5);
/// assert_eq!(math_prime_util::is_happy(2, 10, 2), 0);
/// ```
pub fn is_happy(mut n: u64, base: u32, exponent: u32) -> u64 {
    assert!((2..=36).contains(&base), "invalid base");
    assert!(exponent <= 10, "invalid exponent");
    if base == 10 && exponent == 2 {
        const HEIGHTS: [u8; 101] = [
            0, 1, 0, 0, 0, 0, 0, 6, 0, 0, 2, 0, 0, 3, 0, 0, 0, 0, 0, 5, 0, 0, 0, 4, 0, 0, 0, 0, 4,
            0, 0, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 4, 0, 0, 0, 3,
            0, 0, 0, 0, 5, 0, 0, 5, 0, 0, 4, 0, 0, 2,
        ];
        let mut height = 0;
        while n > 100 {
            n = digit_power_sum(n, 10, 2);
            height += 1;
        }
        let tail = u64::from(HEIGHTS[usize::try_from(n).expect("n <= 100")]);
        return if tail == 0 { 0 } else { height + tail };
    }
    let mut height = 1_u64;
    let mut checkpoint = 0;
    while n > 1 && n != checkpoint {
        if height.is_power_of_two() {
            checkpoint = n;
        }
        n = digit_power_sum(n, base, exponent);
        height += 1;
    }
    if n == 1 { height } else { 0 }
}

fn digit_power_sum(mut n: u64, base: u32, exponent: u32) -> u64 {
    let base = u64::from(base);
    let mut sum = 0;
    while n != 0 {
        let digit = n % base;
        sum += match exponent {
            0 => 1,
            1 => digit,
            2 => digit * digit,
            _ => digit.pow(exponent),
        };
        n /= base;
    }
    // At most 64 digits, each at most 35^10, so the sum fits u64.
    sum
}

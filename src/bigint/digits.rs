//! Arbitrary precision digit operations from MPU 0.75.

use num_bigint::BigUint;

/// Return digits with optional padding or truncation.
/// Port of MPU 0.75 `util.c::to_digit_array`.
///
/// # Panics
/// Panics if the base is less than two.
///
/// ```
/// use math_prime_util::bigint::digits::todigits;
/// assert_eq!(todigits(&123_u32.into(), 10, None), vec![1, 2, 3]);
/// ```
pub fn todigits(n: &BigUint, base: u64, length: Option<usize>) -> Vec<u64> {
    assert!(base >= 2);
    let mut n = n.clone();
    let mut digits = Vec::new();
    while n != BigUint::ZERO && length.is_none_or(|len| digits.len() < len) {
        digits.push(u64::try_from(&n % base).expect("digit fits base"));
        n /= base;
    }
    if let Some(len) = length {
        digits.resize(len, 0);
    }
    digits.reverse();
    digits
}

/// Return lowercase digits.
/// Port of MPU 0.75 `util.c::to_digit_string`.
///
/// # Panics
/// Panics if the base is outside 2 through 36.
///
/// ```
/// use math_prime_util::bigint::digits::todigitstring;
/// assert_eq!(todigitstring(&255_u32.into(), 16, None), "ff");
/// ```
pub fn todigitstring(n: &BigUint, base: u64, length: Option<usize>) -> String {
    assert!((2..=36).contains(&base));
    todigits(n, base, length)
        .into_iter()
        .map(|d| {
            char::from(
                b"0123456789abcdefghijklmnopqrstuvwxyz"
                    [usize::try_from(d).expect("digit fits usize")],
            )
        })
        .collect()
}

/// Combine digits without a size limit.
/// Port of MPU 0.75 `util.c::from_digit_to_UV`.
///
/// # Panics
/// Panics if the base is less than two or a digit is too large.
///
/// ```
/// use math_prime_util::bigint::digits::fromdigits;
/// assert_eq!(fromdigits(&[1, 2, 3], 10), Some(123_u32.into()));
/// ```
pub fn fromdigits(digits: &[u64], base: u64) -> Option<BigUint> {
    assert!(base >= 2);
    Some(digits.iter().fold(BigUint::ZERO, |n, &d| {
        assert!(d < base, "digit must be smaller than base");
        n * base + d
    }))
}

/// Reverse the digits.
/// Port of MPU 0.75 `util.c::to_digit_array` and `from_digit_to_UV`.
///
/// # Panics
/// Panics if the base is less than two.
///
/// ```
/// use math_prime_util::bigint::digits::reverse_digits;
/// assert_eq!(reverse_digits(&120_u32.into(), 10), Some(21_u32.into()));
/// ```
pub fn reverse_digits(n: &BigUint, base: u64) -> Option<BigUint> {
    let mut digits = todigits(n, base, None);
    digits.reverse();
    fromdigits(&digits, base)
}

/// Test whether the digits read the same in both directions.
/// Port of MPU 0.75 `XS.xs::is_palindrome`.
///
/// # Panics
/// Panics if the base is less than two.
///
/// ```
/// assert!(math_prime_util::bigint::digits::is_palindrome(&121_u32.into(), 10));
/// ```
pub fn is_palindrome(n: &BigUint, base: u64) -> bool {
    let digits = todigits(n, base, None);
    digits.iter().eq(digits.iter().rev())
}

/// Test whether a positive number is divisible by its digit sum.
/// Port of MPU 0.75 `XS.xs::is_harshad`.
///
/// # Panics
/// Panics if the base is less than two.
///
/// ```
/// assert!(math_prime_util::bigint::digits::is_harshad(&18_u32.into(), 10));
/// ```
pub fn is_harshad(n: &BigUint, base: u64) -> bool {
    let sum: BigUint = todigits(n, base, None).into_iter().map(BigUint::from).sum();
    n != &BigUint::ZERO && n % sum == BigUint::ZERO
}

/// Return the iterated digit sum.
/// Port of MPU 0.75 `XS.xs::digital_root`.
///
/// # Panics
/// Panics if the base is less than two.
///
/// ```
/// assert_eq!(math_prime_util::bigint::digits::digital_root(&493_u32.into(), 10), 7_u32.into());
/// ```
pub fn digital_root(n: &BigUint, base: u64) -> BigUint {
    assert!(base >= 2);
    if n == &BigUint::ZERO {
        BigUint::ZERO
    } else {
        (n - 1_u32) % (base - 1) + 1_u32
    }
}

/// Return the iterated digit product.
/// Port of MPU 0.75 `XS.xs::mult_digital_root`.
///
/// # Panics
/// Panics if the base is less than two.
///
/// ```
/// assert_eq!(math_prime_util::bigint::digits::mult_digital_root(&77_u32.into(), 10), 8_u32.into());
/// ```
pub fn mult_digital_root(n: &BigUint, base: u64) -> BigUint {
    assert!(base >= 2);
    let mut n = n.clone();
    let b = BigUint::from(base);
    while n >= b {
        n = todigits(&n, base, None)
            .into_iter()
            .map(BigUint::from)
            .product();
    }
    n
}

/// Return the canonical Fibonacci representation.
/// Port of MPU 0.75 `util.c::to_zeckendorf`.
///
/// ```
/// assert_eq!(math_prime_util::bigint::digits::tozeckendorf(&6_u32.into()), "1001");
/// ```
pub fn tozeckendorf(n: &BigUint) -> String {
    if n == &BigUint::ZERO {
        return "0".into();
    }
    let (mut a, mut b, mut c) = (BigUint::ZERO, BigUint::from(1_u32), BigUint::from(1_u32));
    let mut count = 0;
    while &c <= n {
        let next = &b + &c;
        (a, b, c) = (b, c, next);
        count += 1;
    }
    let mut n = n.clone();
    let mut out = String::new();
    for _ in 0..count {
        let previous = &b - &a;
        (a, b, c) = (previous, a, b);
        out.push(if c <= n {
            n -= &c;
            '1'
        } else {
            '0'
        });
    }
    out
}

/// Read canonical Fibonacci digits without a size limit.
/// Port of MPU 0.75 `util.c::validate_zeckendorf` and `from_zeckendorf`.
///
/// # Panics
/// Panics if the text is not canonical.
///
/// ```
/// assert_eq!(math_prime_util::bigint::digits::fromzeckendorf("1001"), Some(6_u32.into()));
/// ```
pub fn fromzeckendorf(text: &str) -> Option<BigUint> {
    assert!(
        text == "0"
            || (text.starts_with('1')
                && !text.contains("11")
                && text.bytes().all(|b| b == b'0' || b == b'1'))
    );
    let (mut a, mut b, mut n) = (BigUint::from(1_u32), BigUint::from(2_u32), BigUint::ZERO);
    for d in text.bytes().rev() {
        if d == b'1' {
            n += &a;
        }
        let next = &a + &b;
        (a, b) = (b, next);
    }
    Some(n)
}

/// Return the happy height, or zero if the sequence cycles.
/// Port of MPU 0.75 `util.c::happy_height`.
///
/// # Panics
/// Panics if the base is outside 2 through 36 or the exponent exceeds 10.
///
/// ```
/// assert_eq!(math_prime_util::bigint::digits::is_happy(&19_u32.into(), 10, 2), 5);
/// ```
pub fn is_happy(n: &BigUint, base: u32, exponent: u32) -> u64 {
    assert!((2..=36).contains(&base), "invalid base");
    assert!(exponent <= 10, "invalid exponent");
    let mut n = n.clone();
    let mut height = 1_u64;
    let mut checkpoint = BigUint::ZERO;
    while n > BigUint::from(1_u32) && n != checkpoint {
        if height.is_power_of_two() {
            checkpoint = n.clone();
        }
        n = todigits(&n, u64::from(base), None)
            .into_iter()
            .map(|d| BigUint::from(d).pow(exponent))
            .sum();
        height += 1;
    }
    if n == BigUint::from(1_u32) { height } else { 0 }
}

/// Parse an integer and truncate its fractional part toward zero.
/// Uses MPU 0.75 `PP.pm::toint`, the fallback for `XS.xs::toint`.
/// Return `None` for a negative result or an exponent outside addressable memory.
///
/// # Panics
/// Panics if the text is not a valid number.
///
/// ```
/// assert_eq!(math_prime_util::bigint::digits::toint("1.5e2"), Some(150_u32.into()));
/// ```
pub fn toint(text: &str) -> Option<BigUint> {
    if text.is_empty() {
        return Some(BigUint::ZERO);
    }
    let text = text.trim();
    assert!(!text.is_empty(), "invalid number");
    let negative = text.starts_with('-');
    let text = text.strip_prefix(['+', '-']).unwrap_or(text);
    let (base, text) = if text.starts_with("0x") || text.starts_with("0X") {
        (16, &text[2..])
    } else if text.starts_with("0b") || text.starts_with("0B") {
        (2, &text[2..])
    } else if text.starts_with("0o") || text.starts_with("0O") {
        (8, &text[2..])
    } else {
        (10, text)
    };
    let bytes = text.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b == b'_' {
            assert!(
                i > 0
                    && i + 1 < bytes.len()
                    && char::from(bytes[i - 1]).is_digit(base)
                    && char::from(bytes[i + 1]).is_digit(base),
                "invalid underscore"
            );
        }
    }
    let text = text.replace('_', "");
    let digits = if base == 10 {
        let mut exponent_parts = text.split(['e', 'E']);
        let mantissa = exponent_parts.next().unwrap_or("");
        let exponent = exponent_parts.next().map_or(0_i128, |s| {
            assert!(!s.is_empty(), "invalid exponent");
            s.parse().expect("invalid exponent")
        });
        assert!(exponent_parts.next().is_none(), "invalid exponent");
        let mut parts = mantissa.split('.');
        let integer = parts.next().unwrap_or("");
        let fraction = parts.next().unwrap_or("");
        assert!(parts.next().is_none(), "invalid decimal");
        let mut digits = format!("{integer}{fraction}");
        assert!(
            !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()),
            "invalid number"
        );
        let shift = exponent.checked_sub(i128::try_from(fraction.len()).ok()?)?;
        if shift < 0 {
            let trim = usize::try_from(shift.unsigned_abs()).unwrap_or(usize::MAX);
            digits.truncate(digits.len().saturating_sub(trim));
        } else {
            let significant = digits.trim_start_matches('0');
            if significant.is_empty() {
                return Some(BigUint::ZERO);
            }
            digits = significant.to_owned();
            digits.extend(std::iter::repeat_n('0', usize::try_from(shift).ok()?));
        }
        digits
    } else {
        assert!(
            !text.is_empty() && text.chars().all(|c| c.is_digit(base)),
            "invalid number"
        );
        text
    };
    let magnitude = digits
        .chars()
        .try_fold(BigUint::ZERO, |n, c| Some(n * base + c.to_digit(base)?))?;
    if negative && magnitude != BigUint::ZERO {
        None
    } else {
        Some(magnitude)
    }
}

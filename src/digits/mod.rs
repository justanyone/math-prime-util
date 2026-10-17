//! Digit conversion and checked integer arithmetic from MPU.
//!
//! Fixed width arithmetic returns `None` when the result is out of range.

/// Return digits with optional padding or truncation.
///
/// Port of MPU `util.c: to_digit_array`.
///
/// # Panics
///
/// Panics if the base is less than two.
///
/// ```
/// use math_prime_util::todigits;
/// assert_eq!(todigits(123, 10, None), vec![1,2,3]);
/// ```
pub fn todigits(n: i128, base: u64, length: Option<usize>) -> Vec<u64> {
    assert!(base >= 2);
    let mut n = n.unsigned_abs();
    let b = u128::from(base);
    let mut digits = Vec::new();
    while n != 0 && length.is_none_or(|len| digits.len() < len) {
        digits.push(u64::try_from(n % b).expect("digit fits base"));
        n /= b;
    }
    if let Some(len) = length {
        digits.resize(len, 0);
    }
    digits.reverse();
    digits
}

/// Return lowercase digits.
///
/// Port of MPU `util.c: to_digit_string`.
///
/// # Panics
///
/// Panics if the base is outside 2 through 36.
///
/// ```
/// use math_prime_util::todigitstring;
/// assert_eq!(todigitstring(255,16,None), "ff");
/// ```
pub fn todigitstring(n: i128, base: u64, length: Option<usize>) -> String {
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

/// Combine digits. Return `None` on overflow.
///
/// Port of MPU `util.c: from_digit_to_UV`.
///
/// # Panics
///
/// Panics if the base is less than two or a digit is too large.
///
/// ```
/// use math_prime_util::fromdigits;
/// assert_eq!(fromdigits(&[1,2,3],10),Some(123));
/// ```
pub fn fromdigits(digits: &[u64], base: u64) -> Option<u64> {
    assert!(base >= 2);
    digits.iter().try_fold(0_u64, |n, &d| {
        assert!(d < base, "digit must be smaller than base");
        n.checked_mul(base)?.checked_add(d)
    })
}

/// Sum valid digit characters.
///
/// Port of MPU `XS.xs: sumdigits`.
///
/// # Panics
///
/// Panics if the base is outside 2 through 36.
///
/// ```
/// use math_prime_util::sumdigits;
/// assert_eq!(sumdigits("0xff",None),30);
/// ```
pub fn sumdigits(text: &str, base: Option<u32>) -> u64 {
    let (text, base) = match base {
        Some(b) => (text, b),
        None => text.strip_prefix("0x").map_or_else(
            || text.strip_prefix("0b").map_or((text, 10), |s| (s, 2)),
            |s| (s, 16),
        ),
    };
    assert!((2..=36).contains(&base), "base must be in 2..=36");
    text.chars()
        .filter_map(|c| c.to_digit(base))
        .map(u64::from)
        .sum()
}

/// Reverse absolute digits. Return `None` on overflow.
///
/// Port of MPU `strops.c: strint_reverse_digits`.
///
/// # Panics
///
/// Panics if the base is less than two.
///
/// ```
/// use math_prime_util::reverse_digits;
/// assert_eq!(reverse_digits(120,10),Some(21));
/// ```
pub fn reverse_digits(n: i128, base: u64) -> Option<u128> {
    let digits = todigits(n, base, None);
    digits.iter().rev().try_fold(0_u128, |v, &d| {
        v.checked_mul(u128::from(base))?.checked_add(u128::from(d))
    })
}

/// Test whether a nonnegative integer reads the same in both directions.
///
/// Port of MPU `XS.xs: is_palindrome`.
///
/// # Panics
///
/// Panics if the number is negative or the base is less than two.
///
/// ```
/// use math_prime_util::is_palindrome;
/// assert!(is_palindrome(121, 10));
/// ```
pub fn is_palindrome(n: i128, base: u64) -> bool {
    assert!(n >= 0, "n must be nonnegative");
    let d = todigits(n, base, None);
    d.iter().eq(d.iter().rev())
}

/// Test whether a positive integer is divisible by its digit sum.
///
/// Port of MPU `XS.xs: is_harshad`.
///
/// # Panics
///
/// Panics if the base is less than two.
///
/// ```
/// use math_prime_util::is_harshad;
/// assert!(is_harshad(18, 10));
/// ```
pub fn is_harshad(n: i128, base: u64) -> bool {
    let d = todigits(n, base, None);
    n > 0 && n.unsigned_abs() % d.into_iter().map(u128::from).sum::<u128>() == 0
}

/// Return the iterated digit sum.
///
/// Port of MPU `XS.xs: digital_root`.
///
/// # Panics
///
/// Panics if the number is negative or the base is less than two.
///
/// ```
/// use math_prime_util::digital_root;
/// assert_eq!(digital_root(493,10),7);
/// ```
pub fn digital_root(n: i128, base: u64) -> u128 {
    assert!(n >= 0 && base >= 2);
    let n = n.unsigned_abs();
    if n == 0 {
        0
    } else {
        1 + (n - 1) % u128::from(base - 1)
    }
}

/// Return the iterated digit product.
///
/// Port of MPU `XS.xs: mult_digital_root`.
///
/// # Panics
///
/// Panics if the number is negative or the base is less than two.
///
/// ```
/// use math_prime_util::mult_digital_root;
/// assert_eq!(mult_digital_root(77,10),8);
/// ```
pub fn mult_digital_root(n: i128, base: u64) -> u128 {
    assert!(n >= 0 && base >= 2);
    let mut n = n.unsigned_abs();
    let b = u128::from(base);
    while n >= b {
        let mut product = 1;
        while n > 0 {
            product *= n % b;
            n /= b;
        }
        n = product;
    }
    n
}

/// Return the canonical Fibonacci representation.
///
/// Port of MPU `util.c: to_zeckendorf`.
///
/// ```
/// use math_prime_util::tozeckendorf;
/// assert_eq!(tozeckendorf(6), "1001");
/// ```
pub fn tozeckendorf(n: u64) -> String {
    if n == 0 {
        return "0".into();
    }
    let (mut a, mut b, mut c) = (0_u128, 1_u128, 1_u128);
    let mut count = 0;
    while c <= u128::from(n) {
        (a, b, c) = (b, c, b + c);
        count += 1;
    }
    let mut n = u128::from(n);
    let mut out = String::new();
    for _ in 0..count {
        (a, b, c) = (b - a, a, b);
        out.push(if c <= n {
            n -= c;
            '1'
        } else {
            '0'
        });
    }
    out
}

/// Read canonical Fibonacci digits. Return `None` on overflow.
///
/// Port of MPU `util.c: validate_zeckendorf, from_zeckendorf`.
///
/// # Panics
///
/// Panics if the text is not canonical.
///
/// ```
/// use math_prime_util::fromzeckendorf;
/// assert_eq!(fromzeckendorf("1001"),Some(6));
/// ```
pub fn fromzeckendorf(text: &str) -> Option<u64> {
    assert!(
        text == "0"
            || (text.starts_with('1')
                && !text.contains("11")
                && text.bytes().all(|b| b == b'0' || b == b'1'))
    );
    let (mut a, mut b, mut n) = (1_u128, 2_u128, 0_u128);
    for d in text.bytes().rev() {
        if d == b'1' {
            n = n.checked_add(a)?;
        }
        (a, b) = (b, a.checked_add(b)?);
    }
    u64::try_from(n).ok()
}

/// Count set bits in the absolute value.
///
/// Port of MPU `XS.xs: hammingweight (popcnt)`.
///
/// ```
/// use math_prime_util::hammingweight;
/// assert_eq!(hammingweight(-7), 3);
/// ```
pub fn hammingweight(n: i128) -> u32 {
    n.unsigned_abs().count_ones()
}

/// Return the checked integer result.
///
/// Port of MPU `XS.xs: absint`.
///
/// ```
/// use math_prime_util::absint;
/// assert_eq!(absint(-4),Some(4));
/// ```
pub fn absint(n: i128) -> Option<i128> {
    n.checked_abs()
}

/// Return the checked integer result.
///
/// Port of MPU `XS.xs: negint`.
///
/// ```
/// use math_prime_util::negint;
/// assert_eq!(negint(4),Some(-4));
/// ```
pub fn negint(n: i128) -> Option<i128> {
    n.checked_neg()
}

/// Return the checked integer result.
///
/// Port of MPU `XS.xs: add1int`.
///
/// ```
/// use math_prime_util::add1int;
/// assert_eq!(add1int(4),Some(5));
/// ```
pub fn add1int(n: i128) -> Option<i128> {
    n.checked_add(1)
}

/// Return the checked integer result.
///
/// Port of MPU `XS.xs: sub1int`.
///
/// ```
/// use math_prime_util::sub1int;
/// assert_eq!(sub1int(4),Some(3));
/// ```
pub fn sub1int(n: i128) -> Option<i128> {
    n.checked_sub(1)
}

/// Return minus one, zero, or one.
///
/// Port of MPU `XS.xs: signint`.
///
/// ```
/// use math_prime_util::signint;
/// assert_eq!(signint(-7), -1);
/// ```
pub fn signint(n: i128) -> i128 {
    n.signum()
}

/// Compare two integers.
///
/// Port of MPU `XS.xs: cmpint`.
///
/// ```
/// use math_prime_util::cmpint;
/// assert_eq!(cmpint(2,3),-1);
/// ```
pub fn cmpint(a: i128, b: i128) -> i8 {
    match a.cmp(&b) {
        core::cmp::Ordering::Less => -1,
        core::cmp::Ordering::Equal => 0,
        core::cmp::Ordering::Greater => 1,
    }
}

/// Return the checked integer result.
///
/// Port of MPU `strops.c: strint_add_s`.
///
/// ```
/// use math_prime_util::addint;
/// assert_eq!(addint(2, 3), Some(5));
/// ```
pub fn addint(a: i128, b: i128) -> Option<i128> {
    a.checked_add(b)
}

/// Return the checked integer result.
///
/// Port of MPU `strops.c: strint_add_s`.
///
/// ```
/// use math_prime_util::subint;
/// assert_eq!(subint(2, 3), Some(-1));
/// ```
pub fn subint(a: i128, b: i128) -> Option<i128> {
    a.checked_sub(b)
}

/// Return the checked integer result.
///
/// Port of MPU `strops.c: strint_mul`.
///
/// ```
/// use math_prime_util::mulint;
/// assert_eq!(mulint(2, 3), Some(6));
/// ```
pub fn mulint(a: i128, b: i128) -> Option<i128> {
    a.checked_mul(b)
}

/// Multiply then combine. Return `None` on overflow.
///
/// Port of MPU `strops.c: strint_muladd_s`.
///
/// ```
/// use math_prime_util::muladdint;
/// assert_eq!(muladdint(2,3,4),Some(10));
/// ```
pub fn muladdint(a: i128, b: i128, c: i128) -> Option<i128> {
    mulcombine(a, b, c, false)
}

/// Multiply then combine. Return `None` on overflow.
///
/// Port of MPU `strops.c: strint_muladd_s`.
///
/// ```
/// use math_prime_util::mulsubint;
/// assert_eq!(mulsubint(2,3,4),Some(2));
/// ```
pub fn mulsubint(a: i128, b: i128, c: i128) -> Option<i128> {
    mulcombine(a, b, c, true)
}

/// Return quotient and remainder. Return `None` on overflow.
///
/// Port of MPU `util.c: tdivrem`.
///
/// # Panics
///
/// Panics if the divisor is zero.
///
/// ```
/// use math_prime_util::tdivrem;
/// assert_eq!(tdivrem(-7,3),Some((-2,-1)));
/// ```
pub fn tdivrem(a: i128, b: i128) -> Option<(i128, i128)> {
    assert!(b != 0);
    #[allow(unused_mut)]
    let (mut q, mut r) = (a.checked_div(b)?, a.checked_rem(b)?);

    Some((q, r))
}

/// Return quotient and remainder. Return `None` on overflow.
///
/// Port of MPU `util.c: fdivrem`.
///
/// # Panics
///
/// Panics if the divisor is zero.
///
/// ```
/// use math_prime_util::fdivrem;
/// assert_eq!(fdivrem(-7,3),Some((-3,2)));
/// ```
pub fn fdivrem(a: i128, b: i128) -> Option<(i128, i128)> {
    assert!(b != 0);
    #[allow(unused_mut)]
    let (mut q, mut r) = (a.checked_div(b)?, a.checked_rem(b)?);
    if r != 0 && (r < 0) != (b < 0) {
        q = q.checked_sub(1)?;
        r = r.checked_add(b)?;
    }
    Some((q, r))
}

/// Return quotient and remainder. Return `None` on overflow.
///
/// Port of MPU `util.c: cdivrem`.
///
/// # Panics
///
/// Panics if the divisor is zero.
///
/// ```
/// use math_prime_util::cdivrem;
/// assert_eq!(cdivrem(-7,3),Some((-2,-1)));
/// ```
pub fn cdivrem(a: i128, b: i128) -> Option<(i128, i128)> {
    assert!(b != 0);
    #[allow(unused_mut)]
    let (mut q, mut r) = (a.checked_div(b)?, a.checked_rem(b)?);
    if r != 0 && (a < 0) == (b < 0) {
        q = q.checked_add(1)?;
        r = r.checked_sub(b)?;
    }
    Some((q, r))
}

/// Return quotient and remainder. Return `None` on overflow.
///
/// Port of MPU `util.c: edivrem`.
///
/// # Panics
///
/// Panics if the divisor is zero.
///
/// ```
/// use math_prime_util::divrem;
/// assert_eq!(divrem(-7,3),Some((-3,2)));
/// ```
pub fn divrem(a: i128, b: i128) -> Option<(i128, i128)> {
    assert!(b != 0);
    #[allow(unused_mut)]
    let (mut q, mut r) = (a.checked_div(b)?, a.checked_rem(b)?);
    if r < 0 {
        if b > 0 {
            q -= 1;
            r += b;
        } else {
            q += 1;
            r -= b;
        }
    }
    Some((q, r))
}

/// Return the checked division result.
///
/// Port of MPU `strops.c: strint_divint`.
///
/// # Panics
///
/// Panics if the divisor is zero.
///
/// ```
/// use math_prime_util::divint;
/// assert_eq!(divint(-7, 3), Some(-3));
/// ```
pub fn divint(a: i128, b: i128) -> Option<i128> {
    fdivrem(a, b).map(|v| v.0)
}

/// Return the checked division result.
///
/// Port of MPU `strops.c: strint_modint`.
///
/// # Panics
///
/// Panics if the divisor is zero.
///
/// ```
/// use math_prime_util::modint;
/// assert_eq!(modint(-7, 3), Some(2));
/// ```
pub fn modint(a: i128, b: i128) -> Option<i128> {
    fdivrem(a, b).map(|v| v.1)
}

/// Return the checked division result.
///
/// Port of MPU `strops.c: strint_cdivint`.
///
/// # Panics
///
/// Panics if the divisor is zero.
///
/// ```
/// use math_prime_util::cdivint;
/// assert_eq!(cdivint(-7, 3), Some(-2));
/// ```
pub fn cdivint(a: i128, b: i128) -> Option<i128> {
    cdivrem(a, b).map(|v| v.0)
}

/// Shift by a signed count. A negative count reverses the direction. Return `None` on overflow.
///
/// Port of MPU `strops.c: strint_lshiftint`.
///
/// ```
/// use math_prime_util::lshiftint;
/// assert_eq!(lshiftint(8, 1), Some(16));
/// ```
pub fn lshiftint(n: i128, k: i128) -> Option<i128> {
    if k < 0 {
        return rshiftint(n, k.saturating_neg());
    }
    if n == 0 {
        return Some(0);
    }
    if k >= 128 {
        return None;
    }
    let k = u32::try_from(k).ok()?;
    let result = n.checked_shl(k)?;
    if result >> k == n { Some(result) } else { None }
}

/// Shift by a signed count. A negative count reverses the direction. Return `None` on overflow.
///
/// Port of MPU `strops.c: strint_rshiftint`.
///
/// ```
/// use math_prime_util::rshiftint;
/// assert_eq!(rshiftint(8, 1), Some(4));
/// ```
pub fn rshiftint(n: i128, k: i128) -> Option<i128> {
    if k < 0 {
        return lshiftint(n, k.saturating_neg());
    }
    if k >= 128 {
        return Some(0);
    }
    let magnitude = n.unsigned_abs() >> u32::try_from(k).ok()?;
    if n < 0 {
        if magnitude == 1_u128 << 127 {
            Some(i128::MIN)
        } else {
            i128::try_from(magnitude).ok()?.checked_neg()
        }
    } else {
        i128::try_from(magnitude).ok()
    }
}

/// Shift by a signed count. A negative count reverses the direction. Return `None` on overflow.
///
/// Port of MPU `strops.c: strint_rashiftint`.
///
/// ```
/// use math_prime_util::rashiftint;
/// assert_eq!(rashiftint(8, 1), Some(4));
/// ```
pub fn rashiftint(n: i128, k: i128) -> Option<i128> {
    if k < 0 {
        return lshiftint(n, k.saturating_neg());
    }
    if k >= 128 {
        return Some(if n < 0 { -1 } else { 0 });
    }
    Some(n >> u32::try_from(k).ok()?)
}

/// Test integer parity.
///
/// Port of MPU `XS.xs: is_odd`.
///
/// ```
/// use math_prime_util::is_odd;
/// assert!(is_odd(3));
/// ```
pub fn is_odd(n: i128) -> bool {
    n % 2 != 0
}

/// Test integer parity.
///
/// Port of MPU `XS.xs: is_even`.
///
/// ```
/// use math_prime_util::is_even;
/// assert!(is_even(4));
/// ```
pub fn is_even(n: i128) -> bool {
    n % 2 == 0
}

/// Test divisibility. Zero divides only zero.
///
/// Port of MPU `XS.xs: is_divisible`.
///
/// ```
/// use math_prime_util::is_divisible;
/// assert!(is_divisible(12, -3));
/// ```
pub fn is_divisible(n: i128, d: i128) -> bool {
    d == 0 && n == 0 || d != 0 && n.unsigned_abs() % d.unsigned_abs() == 0
}

/// Test congruence. A zero modulus tests equality.
///
/// Port of MPU `XS.xs: is_congruent`.
///
/// ```
/// use math_prime_util::is_congruent;
/// assert!(is_congruent(-1, 5, 3));
/// ```
pub fn is_congruent(n: i128, c: i128, d: i128) -> bool {
    if d == 0 {
        return n == c;
    }
    let d = d.unsigned_abs();
    let residue = |v: i128| {
        let r = v.unsigned_abs() % d;
        if v < 0 && r != 0 { d - r } else { r }
    };
    residue(n) == residue(c)
}

/// Parse a number and truncate toward zero. Return `None` on overflow.
///
/// Port of MPU `xs_internal.c: _normalize_toint_string` and `XS.xs: toint`.
///
/// # Panics
///
/// Panics if the text is not a number.
///
/// ```
/// use math_prime_util::toint;
/// assert_eq!(toint("1.5e2"), Some(150));
/// ```
pub fn toint(text: &str) -> Option<i128> {
    if text.is_empty() {
        return Some(0);
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
                return Some(0);
            }
            if shift > 39 {
                return None;
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
    let magnitude = digits.chars().try_fold(0_u128, |n, c| {
        n.checked_mul(u128::from(base))?
            .checked_add(u128::from(c.to_digit(base)?))
    })?;
    if negative && magnitude == 1_u128 << 127 {
        Some(i128::MIN)
    } else {
        let n = i128::try_from(magnitude).ok()?;
        if negative { n.checked_neg() } else { Some(n) }
    }
}

fn mulcombine(a: i128, b: i128, c: i128, subtract: bool) -> Option<i128> {
    let product = a.unsigned_abs().checked_mul(b.unsigned_abs())?;
    let product_negative = (a < 0) != (b < 0);
    let c_negative = (c < 0) != subtract;
    let c = c.unsigned_abs();
    let (magnitude, negative) = if product_negative == c_negative {
        (product.checked_add(c)?, product_negative)
    } else if product >= c {
        (product - c, product_negative)
    } else {
        (c - product, c_negative)
    };
    if negative && magnitude == 1_u128 << 127 {
        Some(i128::MIN)
    } else {
        let n = i128::try_from(magnitude).ok()?;
        if negative { n.checked_neg() } else { Some(n) }
    }
}

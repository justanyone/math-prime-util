//! Continued fractions and rational enumeration.
//!
//! The MPU entry points are in `rational.c` and `rational.h`.

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

/// Expand a rational as a simple continued fraction.
/// Reference: MPU `rational.c`, `contfrac`.
/// # Panics
/// Panics if the denominator is not positive or the numerator is `i128::MIN`.
/// ```
/// assert_eq!(math_prime_util::contfrac(67, 29), vec![2, 3, 4, 2]);
/// ```
pub fn contfrac(mut numerator: i128, mut denominator: i128) -> Vec<i128> {
    assert!(denominator > 0, "denominator must be positive");
    if numerator < 0 {
        let mut terms = contfrac(
            numerator.checked_neg().expect("numerator overflow"),
            denominator,
        );
        if terms.len() == 1 {
            terms[0] = -terms[0];
        } else {
            terms[0] = -terms[0] - 1;
            if terms[1] == 1 {
                terms.remove(1);
                terms[1] += 1;
            } else {
                terms[1] -= 1;
                terms.insert(1, 1);
            }
        }
        return terms;
    }
    let mut terms = Vec::new();
    while denominator != 0 {
        terms.push(numerator.div_euclid(denominator));
        (numerator, denominator) = (denominator, numerator.rem_euclid(denominator));
    }
    terms
}

/// Return the convergents of a continued fraction.
/// Reference: MPU `rational.c`, `convergents`.
/// # Panics
/// Panics if a term after the first is not positive or arithmetic overflows.
/// ```
/// assert_eq!(math_prime_util::convergents(&[0, 3]), vec![(0, 1), (1, 3)]);
/// ```
pub fn convergents(terms: &[i128]) -> Vec<(i128, i128)> {
    let (mut p0, mut p1, mut q0, mut q1) = (0_i128, 1_i128, 1_i128, 0_i128);
    let mut result = Vec::with_capacity(terms.len());
    for (i, &a) in terms.iter().enumerate() {
        assert!(i == 0 || a > 0, "continued fraction terms must be positive");
        let p = a
            .checked_mul(p1)
            .and_then(|v| v.checked_add(p0))
            .expect("numerator overflow");
        let q = a
            .checked_mul(q1)
            .and_then(|v| v.checked_add(q0))
            .expect("denominator overflow");
        result.push((p, q));
        (p0, p1, q0, q1) = (p1, p, q1, q);
    }
    result
}

/// Evaluate a continued fraction. An empty list means zero.
/// Reference: MPU `rational.c`, `from_contfrac`.
/// # Panics
/// Panics for invalid terms or arithmetic overflow.
/// ```
/// assert_eq!(math_prime_util::from_contfrac(&[2, 3, 4, 2]), (67, 29));
/// ```
pub fn from_contfrac(terms: &[i128]) -> (i128, i128) {
    convergents(terms).last().copied().unwrap_or((0, 1))
}

/// Find the closest rational with denominator at most `limit`.
/// Reference: MPU `rational.c`, `bestrational`.
/// # Panics
/// Panics if the value is not finite, the limit is zero, or the result overflows.
/// ```
/// assert_eq!(math_prime_util::bestrational(3.14159, 100), (311, 99));
/// ```
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
pub fn bestrational(value: f64, limit: u64) -> (i128, u64) {
    assert!(
        value.is_finite() && limit > 0,
        "finite value and positive limit required"
    );
    let x = value.abs();
    assert!(x < i128::MAX as f64, "numerator overflow");
    let (mut p0, mut q0, mut p1, mut q1) = (0_i128, 1_u64, 1_i128, 0_u64);
    let mut rem = x;
    loop {
        let a = rem.floor();
        if q1 != 0
            && (a > limit as f64
                || q1
                    .checked_mul(a as u64)
                    .and_then(|v| v.checked_add(q0))
                    .is_none_or(|q| q > limit))
        {
            break;
        }
        let (p2, q2) = (p0 + a as i128 * p1, q0 + a as u64 * q1);
        (p0, q0, p1, q1) = (p1, q1, p2, q2);
        let fraction = rem - a;
        if fraction < 1e-15 {
            break;
        }
        rem = 1.0 / fraction;
    }
    if q1 == 0 {
        return ((x.round() as i128) * if value < 0.0 { -1 } else { 1 }, 1);
    }
    let k = (limit - q0) / q1;
    let (p2, q2) = (p0 + i128::from(k) * p1, q0 + k * q1);
    let (p, q) = if q2 != 0 && (p2 as f64 / q2 as f64 - x).abs() < (p1 as f64 / q1 as f64 - x).abs()
    {
        (p2, q2)
    } else {
        (p1, q1)
    };
    (if value < 0.0 { -p } else { p }, q)
}

fn validate(a: u128, b: u128) {
    assert!(
        a > 0 && b > 0 && gcd(a, b) == 1,
        "positive reduced fraction required"
    );
}

/// Return the next fraction in Calkin-Wilf order.
/// Reference: MPU `rational.c`, `next_calkin_wilf`.
/// # Panics
/// Panics for a nonpositive or unreduced fraction, or overflow.
/// ```
/// assert_eq!(math_prime_util::next_calkin_wilf(1, 2), (2, 1));
/// ```
pub fn next_calkin_wilf(a: u128, b: u128) -> (u128, u128) {
    validate(a, b);
    let r = a % b;
    let denominator = a
        .checked_sub(r)
        .and_then(|v| v.checked_add(b - r))
        .expect("fraction overflow");
    (b, denominator)
}

fn tree_index(mut a: u128, mut b: u128, reverse: bool) -> u128 {
    validate(a, b);
    let mut bits = Vec::new();
    while a != b {
        let right = a > b;
        let count = if right { (a - 1) / b } else { (b - 1) / a };
        assert!(
            count < 128 && bits.len() + usize::try_from(count).unwrap() < 128,
            "index overflow"
        );
        for _ in 0..count {
            bits.push(right);
        }
        if right {
            a -= count * b;
        } else {
            b -= count * a;
        }
    }
    if !reverse {
        bits.reverse();
    }
    bits.into_iter()
        .fold(1, |n, bit| (n << 1) | u128::from(bit))
}

/// Return the Calkin-Wilf index of a reduced positive fraction.
/// Reference: MPU `rational.c`, `calkin_wilf_n`.
/// # Panics
/// Panics for invalid fractions or indices that exceed `u128`.
/// ```
/// assert_eq!(math_prime_util::calkin_wilf_n(3, 2), 5);
/// ```
pub fn calkin_wilf_n(a: u128, b: u128) -> u128 {
    tree_index(a, b, false)
}

/// Return the Stern-Brocot breadth-first index.
/// Reference: MPU `rational.c`, `stern_brocot_n`.
/// # Panics
/// Panics for invalid fractions or indices that exceed `u128`.
/// ```
/// assert_eq!(math_prime_util::stern_brocot_n(3, 2), 6);
/// ```
pub fn stern_brocot_n(a: u128, b: u128) -> u128 {
    tree_index(a, b, true)
}

/// Return a fraction by its Calkin-Wilf index.
/// Reference: MPU `rational.c`, `nth_calkin_wilf`.
/// # Panics
/// Panics if the index is zero.
/// ```
/// assert_eq!(math_prime_util::nth_calkin_wilf(5), (3, 2));
/// ```
pub fn nth_calkin_wilf(n: u128) -> (u128, u128) {
    assert!(n > 0, "index must be positive");
    let (mut a, mut b) = (1, 1);
    for bit in (0..n.ilog2()).rev() {
        if n & (1 << bit) == 0 {
            b += a;
        } else {
            a += b;
        }
    }
    (a, b)
}

/// Return a fraction by its Stern-Brocot breadth-first index.
/// Reference: MPU `rational.c`, `nth_stern_brocot`.
/// # Panics
/// Panics if the index is zero.
/// ```
/// assert_eq!(math_prime_util::nth_stern_brocot(6), (3, 2));
/// ```
pub fn nth_stern_brocot(n: u128) -> (u128, u128) {
    assert!(n > 0, "index must be positive");
    let depth = n.ilog2();
    let mut reversed = 1_u128;
    for bit in 0..depth {
        reversed = (reversed << 1) | ((n >> bit) & 1);
    }
    nth_calkin_wilf(reversed)
}

/// Return the next fraction in Stern-Brocot breadth-first order.
/// Reference: MPU `rational.c`, `next_stern_brocot`.
/// # Panics
/// Panics for invalid fractions or an index overflow.
/// ```
/// assert_eq!(math_prime_util::next_stern_brocot(1, 2), (2, 1));
/// ```
pub fn next_stern_brocot(a: u128, b: u128) -> (u128, u128) {
    nth_stern_brocot(stern_brocot_n(a, b).checked_add(1).expect("index overflow"))
}

/// Return a term of Stern's diatomic sequence.
/// Reference: MPU `rational.c`, `nth_stern_diatomic`.
/// ```
/// assert_eq!(math_prime_util::nth_stern_diatomic(5), 3);
/// ```
pub fn nth_stern_diatomic(n: u128) -> u128 {
    if n == 0 { 0 } else { nth_calkin_wilf(n).0 }
}

/// Return the Farey sequence of order `n`.
/// Reference: MPU `rational.c`, `farey`.
/// # Panics
/// Panics if `n` is zero.
/// ```
/// assert_eq!(math_prime_util::farey(2), vec![(0, 1), (1, 2), (1, 1)]);
/// ```
pub fn farey(n: u64) -> Vec<(u64, u64)> {
    assert!(n > 0, "order must be positive");
    let (mut a, mut b, mut c, mut d) = (0_u64, 1_u64, 1_u64, n);
    let mut result = vec![(a, b)];
    loop {
        result.push((c, d));
        if c == 1 && d == 1 {
            break;
        }
        let k = (u128::from(n) + u128::from(b)) / u128::from(d);
        let e = u64::try_from(k * u128::from(c) - u128::from(a)).unwrap();
        let f = u64::try_from(k * u128::from(d) - u128::from(b)).unwrap();
        (a, b, c, d) = (c, d, e, f);
    }
    result
}

/// Return the first Farey fraction greater than `a/b`.
/// Reference: MPU `rational.c`, `next_farey`.
/// # Panics
/// Panics if the order or denominator is zero.
/// ```
/// assert_eq!(math_prime_util::next_farey(5, 1, 3), Some((2, 5)));
/// ```
pub fn next_farey(n: u64, a: u64, b: u64) -> Option<(u64, u64)> {
    assert!(n > 0 && b > 0, "positive order and denominator required");
    if a >= b {
        return None;
    }
    let (mut p, mut q) = (1, 1);
    for d in 1..=n {
        let c = u64::try_from(u128::from(a) * u128::from(d) / u128::from(b)).unwrap() + 1;
        if u128::from(c) * u128::from(q) < u128::from(p) * u128::from(d) {
            (p, q) = (c, d);
        }
    }
    Some((p, q))
}

/// Return the insertion rank of `a/b` in the Farey sequence of order `n`.
/// Reference: MPU `rational.c`, `farey_rank`.
/// # Panics
/// Panics for zero order or denominator, or if the order cannot fit in memory.
/// ```
/// assert_eq!(math_prime_util::farey_rank(5, 1, 2), 5);
/// ```
pub fn farey_rank(n: u64, a: u64, b: u64) -> u128 {
    assert!(n > 0 && b > 0, "positive order and denominator required");
    let adjustment = u128::from(
        a > b || (a > 0 && u128::from(b) / gcd(u128::from(a), u128::from(b)) > u128::from(n)),
    );
    let a = a.min(b);
    let size = usize::try_from(n).expect("order exceeds address space");
    let mut counts = vec![0_u128; size + 1];
    for (d, count) in counts.iter_mut().enumerate().skip(1) {
        *count = u128::from(a) * d as u128 / u128::from(b);
    }
    let mut rank = 0;
    for d in 1..=size {
        let count = counts[d];
        rank += count;
        for multiple in (2 * d..=size).step_by(d) {
            counts[multiple] -= count;
        }
    }
    rank + adjustment
}

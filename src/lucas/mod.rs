//! Lucas sequences with signed parameters and bounded exact results.
//!
//! Exact values use `i128` so both signs of a `u64` magnitude fit.

fn bounded(value: i128) -> Option<i128> {
    (value.unsigned_abs() <= u128::from(u64::MAX)).then_some(value)
}

fn power(mut base: i128, mut exponent: u64) -> Option<i128> {
    let mut value = 1_i128;
    while exponent != 0 {
        if exponent & 1 != 0 {
            value = value.checked_mul(base)?;
        }
        exponent >>= 1;
        if exponent != 0 {
            base = base.checked_mul(base)?;
        }
    }
    Some(value)
}

// Binary doubling keeps U_k and U_(k+1), without division by two.
fn exact_pair(p: i128, q: i128, k: u64) -> Option<(i128, i128)> {
    if k == 0 {
        return Some((0, 1));
    }
    let (u, w) = exact_pair(p, q, k / 2)?;
    let a = u.checked_mul(w.checked_mul(2)?.checked_sub(p.checked_mul(u)?)?)?;
    let b = w
        .checked_mul(w)?
        .checked_sub(q.checked_mul(u.checked_mul(u)?)?)?;
    if k & 1 == 0 {
        Some((a, b))
    } else {
        Some((b, p.checked_mul(b)?.checked_sub(q.checked_mul(a)?)?))
    }
}

fn exact(p: i64, q: i64, k: u64) -> (Option<i128>, Option<i128>) {
    let (p, q) = (i128::from(p), i128::from(q));
    if k == 0 {
        return (Some(0), Some(2));
    }
    if p == 0 {
        return if k & 1 == 0 {
            (
                Some(0),
                power(-q, k / 2)
                    .and_then(|v| v.checked_mul(2))
                    .and_then(bounded),
            )
        } else {
            (power(-q, k / 2).and_then(bounded), Some(0))
        };
    }
    if q == 0 {
        return (
            power(p, k - 1).and_then(bounded),
            power(p, k).and_then(bounded),
        );
    }
    let Some((u, w)) = exact_pair(p, q, k) else {
        return (None, None);
    };
    let v = w
        .checked_mul(2)
        .and_then(|w| p.checked_mul(u).and_then(|pu| w.checked_sub(pu)));
    (bounded(u), v.and_then(bounded))
}

fn residue(value: i64, n: u64) -> u64 {
    u64::try_from(i128::from(value).rem_euclid(i128::from(n))).expect("residue fits u64")
}

fn mul(a: u64, b: u64, n: u64) -> u64 {
    u64::try_from(u128::from(a) * u128::from(b) % u128::from(n)).expect("residue fits u64")
}

fn sub(a: u64, b: u64, n: u64) -> u64 {
    if a >= b { a - b } else { n - (b - a) }
}

fn add(a: u64, b: u64, n: u64) -> u64 {
    u64::try_from((u128::from(a) + u128::from(b)) % u128::from(n)).expect("residue fits u64")
}

fn modular_pair(p: u64, q: u64, k: u64, n: u64) -> (u64, u64) {
    if k == 0 {
        return (0, 1 % n);
    }
    let (u, w) = modular_pair(p, q, k / 2, n);
    let a = mul(u, sub(add(w, w, n), mul(p, u, n), n), n);
    let b = sub(mul(w, w, n), mul(q, mul(u, u, n), n), n);
    if k & 1 == 0 {
        (a, b)
    } else {
        (b, sub(mul(p, b, n), mul(q, a, n), n))
    }
}

/// Return `(U_k, V_k, Q^k)` modulo `n`.
///
/// Reference: MPU `lucas_seq.c`, `lucas_sequence`.
///
/// # Panics
/// Panics if `n` is zero.
///
/// ```
/// assert_eq!(math_prime_util::lucas_sequence(17, 1, -1, 5), (5, 11, 16));
/// ```
pub fn lucas_sequence(n: u64, p: i64, q: i64, k: u64) -> (u64, u64, u64) {
    assert!(n != 0, "modulus must be positive");
    let (p, mut q) = (residue(p, n), residue(q, n));
    let (u, w) = modular_pair(p, q, k, n);
    let v = sub(add(w, w, n), mul(p, u, n), n);
    let mut exponent = k;
    let mut qk = 1 % n;
    while exponent != 0 {
        if exponent & 1 != 0 {
            qk = mul(qk, q, n);
        }
        q = mul(q, q, n);
        exponent >>= 1;
    }
    (u, v, qk)
}

/// Return `U_k`, or `None` if its magnitude exceeds `u64::MAX`.
/// Reference: MPU `lucas_seq.c`, `lucasu`.
/// ```
/// assert_eq!(math_prime_util::lucasu(1, -1, 10), Some(55));
/// ```
pub fn lucasu(p: i64, q: i64, k: u64) -> Option<i128> {
    exact(p, q, k).0
}

/// Return `V_k`, or `None` if its magnitude exceeds `u64::MAX`.
/// Reference: MPU `lucas_seq.c`, `lucasv`.
/// ```
/// assert_eq!(math_prime_util::lucasv(1, -1, 10), Some(123));
/// ```
pub fn lucasv(p: i64, q: i64, k: u64) -> Option<i128> {
    exact(p, q, k).1
}

/// Return both exact values. Each component has its own overflow result.
/// Reference: MPU `lucas_seq.c`, `lucasuv`.
/// ```
/// assert_eq!(math_prime_util::lucasuv(1, -1, 10), (Some(55), Some(123)));
/// ```
pub fn lucasuv(p: i64, q: i64, k: u64) -> (Option<i128>, Option<i128>) {
    exact(p, q, k)
}

/// Return `U_k` modulo `n`. Return `None` for a zero modulus.
/// Reference: MPU `lucas_seq.c`, `lucasumod`.
/// ```
/// assert_eq!(math_prime_util::lucasumod(1, -1, 10, 17), Some(4));
/// ```
pub fn lucasumod(p: i64, q: i64, k: u64, n: u64) -> Option<u64> {
    lucasuvmod(p, q, k, n).map(|pair| pair.0)
}

/// Return `V_k` modulo `n`. Return `None` for a zero modulus.
/// Reference: MPU `lucas_seq.c`, `lucasvmod`.
/// ```
/// assert_eq!(math_prime_util::lucasvmod(1, -1, 10, 17), Some(4));
/// ```
pub fn lucasvmod(p: i64, q: i64, k: u64, n: u64) -> Option<u64> {
    lucasuvmod(p, q, k, n).map(|pair| pair.1)
}

/// Return `(U_k, V_k)` modulo `n`. Return `None` for a zero modulus.
/// Reference: MPU `lucas_seq.c`, `lucasuvmod`.
/// ```
/// assert_eq!(math_prime_util::lucasuvmod(1, -1, 10, 17), Some((4, 4)));
/// ```
pub fn lucasuvmod(p: i64, q: i64, k: u64, n: u64) -> Option<(u64, u64)> {
    if n == 0 {
        return None;
    }
    let (p, q) = (residue(p, n), residue(q, n));
    let (u, w) = modular_pair(p, q, k, n);
    Some((u, sub(add(w, w, n), mul(p, u, n), n)))
}

/// Return the Fibonacci number at a signed index, with a `u64` magnitude bound.
/// Reference: MPU `lucas_seq.c`, `fibonacci`.
/// ```
/// assert_eq!(math_prime_util::fibonacci(-10), Some(-55));
/// ```
pub fn fibonacci(k: i64) -> Option<i128> {
    let value = lucasu(1, -1, k.unsigned_abs())?;
    Some(if k < 0 && k & 1 == 0 { -value } else { value })
}

/// Return the Lucas number at a signed index, with a `u64` magnitude bound.
/// Reference: MPU `lucas_seq.c`, `lucas_number`.
/// ```
/// assert_eq!(math_prime_util::lucas_number(-3), Some(-4));
/// ```
pub fn lucas_number(k: i64) -> Option<i128> {
    let value = lucasv(1, -1, k.unsigned_abs())?;
    Some(if k < 0 && k & 1 != 0 { -value } else { value })
}

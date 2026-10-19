//! Exact combinatorial products and permutation ranks.

use num_bigint::BigUint;

use crate::PermutationError;

/// Returns the product of `m` descending integers from `n`.
///
/// Port of MPU 0.75 `util.c::falling_factorial`, without word overflow.
/// The empty product is one. If `m > n`, the result is zero.
///
/// ```
/// use math_prime_util::bigint::combinat::falling_factorial;
/// use num_bigint::BigUint;
/// assert_eq!(falling_factorial(&5_u8.into(), &3_u8.into()), BigUint::from(60_u8));
/// ```
#[must_use]
pub fn falling_factorial(n: &BigUint, m: &BigUint) -> BigUint {
    if m == &BigUint::ZERO {
        return BigUint::from(1_u8);
    }
    if m > n {
        return BigUint::ZERO;
    }
    let mut result = n.clone();
    let mut i = BigUint::from(1_u8);
    while &i < m {
        result *= n - &i;
        i += 1_u8;
    }
    result
}

/// Returns the product of `m` ascending integers from `n`.
///
/// Port of MPU 0.75 `util.c::rising_factorial`, without word overflow.
///
/// ```
/// use math_prime_util::bigint::combinat::rising_factorial;
/// use num_bigint::BigUint;
/// assert_eq!(rising_factorial(&5_u8.into(), &3_u8.into()), BigUint::from(210_u16));
/// ```
#[must_use]
pub fn rising_factorial(n: &BigUint, m: &BigUint) -> BigUint {
    if m == &BigUint::ZERO {
        return BigUint::from(1_u8);
    }
    falling_factorial(&(n + m - 1_u8), m)
}

/// Returns `n * (n-k) * (n-2k) * ...`.
///
/// Port of MPU 0.75 `util.c::multifactorial`, without word overflow.
///
/// # Panics
/// Panics if `k` is zero, as the native function does.
///
/// ```
/// use math_prime_util::bigint::combinat::multifactorial;
/// use num_bigint::BigUint;
/// assert_eq!(multifactorial(&9_u8.into(), &2_u8.into()), BigUint::from(945_u16));
/// ```
#[must_use]
pub fn multifactorial(n: &BigUint, k: &BigUint) -> BigUint {
    assert!(*k != BigUint::ZERO, "multifactorial: k must be positive");
    let mut result = BigUint::from(1_u8);
    let mut i = n.clone();
    while i != BigUint::ZERO {
        result *= &i;
        if &i <= k {
            break;
        }
        i -= k;
    }
    result
}

/// Returns the permutation of `0..n` at zero-based lexicographic `rank`.
///
/// Port of MPU 0.75 `util.c::num_to_perm`. The rank wraps modulo `n!`.
/// The factorial digits follow `PP.pm::numtoperm` after native overflow.
///
/// ```
/// use math_prime_util::bigint::combinat::numtoperm;
/// assert_eq!(numtoperm(5, &15_u8.into()), [0, 3, 2, 4, 1]);
/// ```
#[must_use]
pub fn numtoperm(n: usize, rank: &BigUint) -> Vec<usize> {
    let mut result: Vec<_> = (0..n).collect();
    if n < 2 {
        return result;
    }
    let mut f = (2..n).fold(BigUint::from(1_u8), |a, b| a * b);
    let mut rank = rank % (&f * n);
    for i in 0..n - 1 {
        // rank < (n-i)!, so this digit is a valid slice offset.
        let digit = &rank / &f;
        let offset = usize::try_from(&digit).unwrap_or(0);
        rank %= &f;
        f /= n - i - 1;
        result[i..=i + offset].rotate_right(1);
    }
    result
}

/// Returns the zero-based lexicographic rank of a permutation.
///
/// Port of MPU 0.75 `util.c::perm_to_num`, with `PP.pm::permtonum`
/// validation and exact factorial arithmetic.
///
/// # Errors
/// Returns [`PermutationError::Invalid`] for duplicate or out-of-range entries.
///
/// ```
/// use math_prime_util::bigint::combinat::permtonum;
/// assert_eq!(permtonum(&[2_u8.into(), 1_u8.into(), 0_u8.into()]), Ok(5_u8.into()));
/// ```
pub fn permtonum(perm: &[BigUint]) -> Result<BigUint, PermutationError> {
    let n = perm.len();
    let mut seen = vec![false; n];
    for value in perm {
        let index = usize::try_from(value).map_err(|_| PermutationError::Invalid)?;
        if index >= n || seen[index] {
            return Err(PermutationError::Invalid);
        }
        seen[index] = true;
    }
    let mut rank = BigUint::ZERO;
    let mut f = (2..n).fold(BigUint::from(1_u8), |a, b| a * b);
    for i in 0..n.saturating_sub(1) {
        let smaller = perm[i + 1..].iter().filter(|v| *v < &perm[i]).count();
        rank += &f * smaller;
        f /= n - i - 1;
    }
    Ok(rank)
}

fn to_usize(n: &BigUint) -> Option<usize> {
    usize::try_from(n).ok()
}

/// Returns the Bell number `B(n)`, or `None` if `n` does not fit a `usize`.
///
/// Port of MPU 0.75 `PP.pm::bell_number` using the Bell triangle.
///
/// ```
/// use math_prime_util::bigint::combinat::bell_number;
/// use num_bigint::BigUint;
/// assert_eq!(bell_number(&10_u8.into()), Some(BigUint::from(115_975_u32)));
/// ```
#[must_use]
pub fn bell_number(n: &BigUint) -> Option<BigUint> {
    let n = to_usize(n)?;
    let mut row = vec![BigUint::from(1_u8)];
    for _ in 0..n {
        let mut next = Vec::with_capacity(row.len() + 1);
        next.push(row[row.len() - 1].clone());
        for v in &row {
            let s = &next[next.len() - 1] + v;
            next.push(s);
        }
        row = next;
    }
    Some(row[0].clone())
}

/// Returns the Catalan number `C(n)`, or `None` if `n` does not fit a `usize`.
///
/// Port of MPU 0.75 `util.c::catalan_number`, with the product recurrence.
///
/// ```
/// use math_prime_util::bigint::combinat::catalan_number;
/// use num_bigint::BigUint;
/// assert_eq!(catalan_number(&10_u8.into()), Some(BigUint::from(16_796_u32)));
/// ```
#[must_use]
pub fn catalan_number(n: &BigUint) -> Option<BigUint> {
    let n = to_usize(n)?;
    let mut c = BigUint::from(1_u8);
    for i in 1..=n {
        c = c * (4 * i - 2) / (i + 1);
    }
    Some(c)
}

/// Returns the number of derangements of `n` items.
///
/// Port of MPU 0.75 `PP.pm::subfactorial`.
///
/// ```
/// use math_prime_util::bigint::combinat::subfactorial;
/// use num_bigint::BigUint;
/// assert_eq!(subfactorial(&5_u8.into()), Some(BigUint::from(44_u8)));
/// ```
#[must_use]
pub fn subfactorial(n: &BigUint) -> Option<BigUint> {
    let n = to_usize(n)?;
    let (mut a, mut b) = (BigUint::from(1_u8), BigUint::ZERO);
    if n == 0 {
        return Some(a);
    }
    for i in 2..=n {
        let next = (&a + &b) * (i - 1);
        a = std::mem::replace(&mut b, next);
    }
    Some(b)
}

/// Returns the Fubini (ordered Bell) number of `n`.
///
/// Port of MPU 0.75 `PP.pm::fubini`, with the binomial recurrence.
///
/// ```
/// use math_prime_util::bigint::combinat::fubini;
/// use num_bigint::BigUint;
/// assert_eq!(fubini(&4_u8.into()), Some(BigUint::from(75_u8)));
/// ```
#[must_use]
pub fn fubini(n: &BigUint) -> Option<BigUint> {
    let n = to_usize(n)?;
    let mut a: Vec<BigUint> = vec![BigUint::from(1_u8)];
    for i in 1..=n {
        let mut binom = BigUint::from(1_u8);
        let mut sum = BigUint::ZERO;
        for k in 1..=i {
            binom = binom * (i - k + 1) / k;
            sum += &binom * &a[i - k];
        }
        a.push(sum);
    }
    a.pop()
}

/// Returns the number of partitions of `n`.
///
/// Port of MPU 0.75 `PP.pm::partitions`, with Euler's pentagonal recurrence.
///
/// ```
/// use math_prime_util::bigint::combinat::partitions;
/// use num_bigint::BigUint;
/// assert_eq!(partitions(&100_u8.into()), Some(BigUint::from(190_569_292_u64)));
/// ```
#[must_use]
pub fn partitions(n: &BigUint) -> Option<BigUint> {
    let n = to_usize(n)?;
    let mut p: Vec<BigUint> = vec![BigUint::from(1_u8)];
    for i in 1..=n {
        let (mut plus, mut minus) = (BigUint::ZERO, BigUint::ZERO);
        let mut k = 1_usize;
        while k * (3 * k - 1) / 2 <= i {
            let t = &p[i - k * (3 * k - 1) / 2];
            if k % 2 == 1 {
                plus += t;
            } else {
                minus += t;
            }
            let g = k * (3 * k + 1) / 2;
            if g <= i {
                if k % 2 == 1 {
                    plus += &p[i - g];
                } else {
                    minus += &p[i - g];
                }
            }
            k += 1;
        }
        p.push(plus - minus);
    }
    p.pop()
}

/// Returns the number of partitions of `n` into distinct parts.
///
/// Port of MPU 0.75 `PP.pm::partitionsq`, with a distinct-part count table.
///
/// ```
/// use math_prime_util::bigint::combinat::partitionsq;
/// use num_bigint::BigUint;
/// assert_eq!(partitionsq(&10_u8.into()), Some(BigUint::from(10_u8)));
/// ```
#[must_use]
pub fn partitionsq(n: &BigUint) -> Option<BigUint> {
    let n = to_usize(n)?;
    let mut q = vec![BigUint::ZERO; n + 1];
    q[0] = BigUint::from(1_u8);
    for part in 1..=n {
        for i in (part..=n).rev() {
            let t = q[i - part].clone();
            q[i] += t;
        }
    }
    q.pop()
}

/// Returns a Stirling number of kind 1 (unsigned), kind 2, or a Lah number
/// (kind 3). Kind 1 is the unsigned magnitude.
///
/// Port of MPU 0.75 `PP.pm::stirling`, with recurrences on exact integers.
/// Returns `None` if `n` does not fit a `usize`.
///
/// # Panics
/// Panics if `kind` is not 1, 2 or 3, as the native function does.
///
/// ```
/// use math_prime_util::bigint::combinat::stirling;
/// use num_bigint::BigUint;
/// assert_eq!(stirling(&4_u8.into(), &2_u8.into(), 1), Some(BigUint::from(11_u8)));
/// assert_eq!(stirling(&4_u8.into(), &2_u8.into(), 2), Some(BigUint::from(7_u8)));
/// assert_eq!(stirling(&4_u8.into(), &2_u8.into(), 3), Some(BigUint::from(36_u8)));
/// ```
#[must_use]
pub fn stirling(n: &BigUint, m: &BigUint, kind: u64) -> Option<BigUint> {
    assert!((1..=3).contains(&kind), "stirling: type must be 1, 2, or 3");
    let n = to_usize(n)?;
    let Some(m) = to_usize(m) else {
        return Some(BigUint::ZERO);
    };
    if m == n {
        return Some(BigUint::from(1_u8));
    }
    if n == 0 || m == 0 || m > n {
        return Some(BigUint::ZERO);
    }
    if kind == 3 {
        let lah = &binomial(n, m) * &binomial(n - 1, m - 1) * factorial(n - m);
        return Some(lah);
    }
    // row[j] holds S(i, j) for the current i.
    let mut row = vec![BigUint::ZERO; m + 1];
    row[0] = BigUint::from(1_u8);
    for i in 1..=n {
        for j in (1..=m.min(i)).rev() {
            let scale = if kind == 1 { i - 1 } else { j };
            let t = &row[j] * scale + &row[j - 1];
            row[j] = t;
        }
        row[0] = BigUint::ZERO;
    }
    row.pop()
}

fn factorial(n: usize) -> BigUint {
    (2..=n).fold(BigUint::from(1_u8), |a, b| a * b)
}

fn binomial(n: usize, k: usize) -> BigUint {
    let mut b = BigUint::from(1_u8);
    for i in 1..=k {
        b = b * (n - k + i) / i;
    }
    b
}

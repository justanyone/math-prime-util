use super::binomial;

/// Returns a signed Stirling number (kind 1), a Stirling number of the
/// second kind (kind 2), or a Lah number (kind 3).
///
/// Port of `stirling1`, `stirling2`, and `stirling3` in MPU 0.75 `util.c`.
/// The first-kind overflow fallback uses the direct recurrence from
/// `lib/Math/Prime/Util/PP.pm`, `stirling`, as MPU does after C overflow.
/// The second-kind fallback uses its corresponding positive recurrence.
/// These use binomial sums and factorial products. Intermediates use
/// 128 bits. Return `None` if the magnitude exceeds `u64`.
///
/// # Panics
///
/// Panics if `kind` is not 1, 2, or 3, as in MPU's `XS.xs` wrapper.
///
/// ```
/// assert_eq!(math_prime_util::stirling(4, 2, 1), Some(11));
/// assert_eq!(math_prime_util::stirling(4, 2, 2), Some(7));
/// assert_eq!(math_prime_util::stirling(4, 2, 3), Some(36));
/// ```
pub fn stirling(n: u64, m: u64, kind: u64) -> Option<i128> {
    assert!((1..=3).contains(&kind), "stirling: type must be 1, 2, or 3");
    if m == n {
        return Some(1);
    }
    if n == 0 || m == 0 || m > n {
        return Some(0);
    }
    let value = match kind {
        1 => first(n, m).or_else(|| direct(n, m, 1))?,
        2 => second(n, m).or_else(|| direct(n, m, 2))?,
        _ => {
            let a = u128::from(binomial(n, m)?);
            let b = u128::from(binomial(n - 1, m - 1)?);
            i128::try_from(a.checked_mul(b)?.checked_mul(factorial(n - m)?)?).ok()?
        }
    };
    (value.unsigned_abs() <= u128::from(u64::MAX)).then_some(value)
}

fn factorial(n: u64) -> Option<u128> {
    if n > 34 {
        return None;
    }
    Some((2..=n).map(u128::from).product())
}

fn second(n: u64, m: u64) -> Option<i128> {
    if m == 1 {
        return Some(1);
    }
    if m == n - 1 {
        return binomial(n, 2).map(i128::from);
    }
    let f = i128::try_from(factorial(m)?).ok()?;
    let mut sum = 0_i128;
    for j in 1..=m {
        let mut term = i128::from(binomial(m, j)?);
        for _ in 0..n {
            term = term.checked_mul(i128::from(j))?;
        }
        sum = if (m - j) % 2 == 1 {
            sum.checked_sub(term)?
        } else {
            sum.checked_add(term)?
        };
    }
    Some(sum / f)
}

fn first(n: u64, m: u64) -> Option<i128> {
    if m == 1 {
        let f = i128::try_from(factorial(n - 1)?).ok()?;
        return Some(if n % 2 == 1 { f } else { -f });
    }
    let mut sum = 0_i128;
    for k in 1..=n - m {
        let a = i128::from(binomial(k.checked_add(n - 1)?, n - m + k)?);
        let b = i128::from(binomial(n.checked_add(n - m)?, n - m - k)?);
        let s = second(n - m + k, k)?;
        let term = a.checked_mul(b)?.checked_mul(s)?;
        sum = if k % 2 == 1 {
            sum.checked_sub(term)?
        } else {
            sum.checked_add(term)?
        };
    }
    Some(sum)
}

// MPU's positive first-kind PP recurrence, indexed by the deficit n - m.
// The second-kind recurrence replaces the factor n - 1 with m.
// Capping is exact for the native range because every term is positive.
fn direct(n: u64, m: u64, kind: u64) -> Option<i128> {
    let deficit = n - m;
    // With m >= 2, S(m + d, m) >= S(d + 2, 2) = 2^(d+1) - 1.
    // First-kind magnitudes are at least as large. The m == 1 cases
    // have already been handled by the native formulas.
    if deficit >= 64 {
        return None;
    }
    let d = usize::try_from(deficit).ok()?;
    let limit = u128::from(u64::MAX) + 1;
    let mut row = [0_u128; 64];
    row[0] = 1;
    for i in 2..=n {
        let top = usize::try_from((i - 1).min(deficit)).ok()?;
        for j in (1..=top).rev() {
            let factor = if kind == 1 {
                i - 1
            } else {
                i - u64::try_from(j).ok()?
            };
            row[j] = row[j]
                .saturating_add(u128::from(factor).saturating_mul(row[j - 1]))
                .min(limit);
        }
        if row[d] == limit {
            return None;
        }
    }
    let magnitude = i128::from(u64::try_from(row[d]).ok()?);
    Some(if kind == 1 && deficit % 2 == 1 {
        -magnitude
    } else {
        magnitude
    })
}

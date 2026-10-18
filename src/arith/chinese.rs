use crate::{gcd, invmod};

/// Solve simultaneous congruences. Each pair is (residue, modulus).
/// Return the least solution. An empty input gives zero.
/// Return `None` for a zero modulus, inconsistent inputs, or native overflow.
///
/// Port of MPU 0.75 `util.c::chinese` and `_simple_chinese`.
/// The signed path uses extended Euclid. The overflow path requires coprime moduli.
///
/// ```
/// assert_eq!(math_prime_util::chinese(&[(2, 3), (3, 5)]), Some(8));
/// assert_eq!(math_prime_util::chinese(&[(0, 2), (1, 2)]), None);
/// ```
pub fn chinese(congruences: &[(u64, u64)]) -> Option<u64> {
    if congruences.is_empty() {
        return Some(0);
    }
    let mut pairs = congruences.to_vec();
    // Keep the Shell sort gaps from util.c::chinese.
    for gap in [7983, 3548, 1577, 701, 301, 132, 57, 23, 10, 4, 1] {
        for i in gap..pairs.len() {
            let pair = pairs[i];
            let mut j = i;
            while j >= gap && pairs[j - gap].1 < pair.1 {
                pairs[j] = pairs[j - gap];
                j -= gap;
            }
            pairs[j] = pair;
        }
    }
    if pairs.last()?.1 == 0 {
        return None;
    }
    let (a, mut lcm) = pairs[0];
    let signed_max = u64::MAX >> 1;
    if lcm > signed_max {
        return simple_chinese(&pairs);
    }
    let mut sum = a % lcm;
    for &(a, n) in &pairs[1..] {
        let (g, u, v) = extended_gcd(lcm, n);
        if sum % g != a % g {
            return None;
        }
        let (s, t) = (n / g, lcm / g);
        if s > signed_max / lcm {
            return simple_chinese(&pairs);
        }
        lcm *= s;
        let u = u64::try_from(u.rem_euclid(i128::from(lcm))).ok()?;
        let v = u64::try_from(v.rem_euclid(i128::from(lcm))).ok()?;
        let vs = product(v, s, lcm);
        let ut = product(u, t, lcm);
        sum = u64::try_from(
            (u128::from(product(vs, sum, lcm)) + u128::from(product(ut, a, lcm))) % u128::from(lcm),
        )
        .ok()?;
    }
    Some(sum)
}

// Port of util.c::gcdext. i128 holds the signed intermediate coefficients.
fn extended_gcd(a: u64, b: u64) -> (u64, i128, i128) {
    let (mut r, mut old_r) = (i128::from(b), i128::from(a));
    let (mut s, mut old_s) = (0, 1);
    let (mut t, mut old_t) = (1, 0);
    while r != 0 {
        let q = old_r / r;
        (r, old_r) = (old_r - q * r, r);
        (s, old_s) = (old_s - q * s, s);
        (t, old_t) = (old_t - q * t, t);
    }
    (crate::gcd(a, b), old_s, old_t)
}

#[allow(clippy::cast_possible_truncation)] // The residue is below a u64 modulus.
fn product(a: u64, b: u64, modulus: u64) -> u64 {
    (u128::from(a) * u128::from(b) % u128::from(modulus)) as u64
}

// Port of util.c::_simple_chinese, including the native product bound.
fn simple_chinese(pairs: &[(u64, u64)]) -> Option<u64> {
    let mut lcm = 1_u64;
    for &(_, n) in pairs {
        if gcd(lcm, n) != 1 {
            return None;
        }
        lcm = lcm.checked_mul(n)?;
    }
    let mut sum = 0_u64;
    for &(a, n) in pairs {
        let p = lcm / n;
        let inverse = invmod(p, n)?;
        if inverse == 0 {
            return None;
        }
        let term = product(p, product(a, inverse, lcm), lcm);
        sum = u64::try_from((u128::from(sum) + u128::from(term)) % u128::from(lcm)).ok()?;
    }
    Some(sum)
}

use crate::{addmod, gcd, invmod, mulmod};

/// Return the least solution and the combined modulus.
/// Return `None` for inconsistent inputs, a zero modulus, or overflow.
///
/// Port of MPU 0.75 `util.c::chinese` and `_simple_chinese`.
/// The C fallback requires coprime moduli when the signed path overflows.
/// An empty input returns `(0, 0)`.
///
/// ```
/// assert_eq!(math_prime_util::chinese2(&[(2, 3), (3, 5)]), Some((8, 15)));
/// ```
pub fn chinese2(congruences: &[(u64, u64)]) -> Option<(u64, u64)> {
    if congruences.is_empty() {
        return Some((0, 0));
    }
    let mut pairs = congruences.to_vec();
    // MPU's Shell sort orders moduli from largest to smallest.
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
    let (mut sum, mut lcm) = pairs[0];
    if lcm > (u64::MAX >> 1) {
        return simple_chinese(&pairs);
    }
    sum %= lcm;
    for &(a, n) in &pairs[1..] {
        let (g, u, v) = extended_gcd(lcm, n);
        if sum % g != a % g {
            return None;
        }
        let (s, t) = (n / g, lcm / g);
        if s > (u64::MAX >> 1) / lcm {
            return simple_chinese(&pairs);
        }
        lcm *= s;
        let u = u64::try_from(u.rem_euclid(i128::from(lcm))).ok()?;
        let v = u64::try_from(v.rem_euclid(i128::from(lcm))).ok()?;
        let vs = mulmod(v, s, lcm)?;
        let ut = mulmod(u, t, lcm)?;
        sum = addmod(mulmod(vs, sum, lcm)?, mulmod(ut, a, lcm)?, lcm)?;
    }
    Some((sum, lcm))
}

fn extended_gcd(a: u64, b: u64) -> (u64, i128, i128) {
    let (mut old_r, mut r) = (i128::from(a), i128::from(b));
    let (mut old_u, mut u) = (1, 0);
    let (mut old_v, mut v) = (0, 1);
    while r != 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - q * r);
        (old_u, u) = (u, old_u - q * u);
        (old_v, v) = (v, old_v - q * v);
    }
    // The gcd of two u64 values fits u64.
    (u64::try_from(old_r).unwrap(), old_u, old_v)
}

fn simple_chinese(pairs: &[(u64, u64)]) -> Option<(u64, u64)> {
    let mut lcm = 1_u64;
    for &(_, n) in pairs {
        if gcd(lcm, n) != 1 {
            return None;
        }
        lcm = lcm.checked_mul(n)?;
    }
    let mut sum = 0;
    for &(a, n) in pairs {
        let p = lcm / n;
        let inverse = invmod(p, n)?;
        if inverse == 0 {
            return None;
        }
        let term = mulmod(p, mulmod(a, inverse, lcm)?, lcm)?;
        sum = addmod(sum, term, lcm)?;
    }
    Some((sum, lcm))
}

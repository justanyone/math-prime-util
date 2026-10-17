//! Continued fractions, convergents, best rational approximations, the
//! Calkin-Wilf and Stern-Brocot trees, and Farey sequences.
//!
//! Ported from MPU's `rational.c` and `rational.h` (MPU 0.75).

/// Continued fraction terms of `num / den`, by the Euclidean algorithm.
///
/// Same method as MPU `rational.c` `contfrac`. The last remainder (the
/// gcd) is dropped.
///
/// # Examples
///
/// ```
/// use math_prime_util::contfrac;
///
/// assert_eq!(contfrac(415, 93), vec![4, 2, 6, 7]);
/// ```
#[must_use]
pub fn contfrac(num: u64, den: u64) -> Vec<u64> {
    let (mut n, mut d) = (num, den);
    let mut cf = Vec::new();
    while d > 0 {
        let q = n / d;
        let r = n - q * d;
        n = d;
        d = r;
        cf.push(q);
    }
    cf
}

/// Numerator and denominator of the value of a continued fraction.
///
/// Walks the terms from the back, as MPU does for `from_contfrac`. Returns
/// `None` for an empty list or when the value does not fit in `u64`.
///
/// # Examples
///
/// ```
/// use math_prime_util::from_contfrac;
///
/// assert_eq!(from_contfrac(&[4, 2, 6, 7]), Some((415, 93)));
/// ```
#[must_use]
pub fn from_contfrac(cf: &[u64]) -> Option<(u64, u64)> {
    let (&last, rest) = cf.split_last()?;
    let (mut p, mut q) = (last, 1u64);
    for &a in rest.iter().rev() {
        let np = a.checked_mul(p)?.checked_add(q)?;
        q = p;
        p = np;
    }
    Some((p, q))
}

/// The convergents `p_i / q_i` of a continued fraction, as `(p, q)` pairs.
///
/// Same method and bounds as MPU `rational.c` `convergents`: the length
/// must be from 1 to 128, and the walk stops with `None` on overflow.
///
/// # Examples
///
/// ```
/// use math_prime_util::convergents;
///
/// assert_eq!(
///     convergents(&[0, 2, 1, 3]),
///     Some(vec![(0, 1), (1, 2), (1, 3), (4, 11)])
/// );
/// ```
#[must_use]
pub fn convergents(cfrac: &[u64]) -> Option<Vec<(u64, u64)>> {
    if cfrac.is_empty() || cfrac.len() > 128 {
        return None;
    }
    let (mut a0, mut a1, mut b0, mut b1) = (1u64, cfrac[0], 0u64, 1u64);
    let mut out = vec![(a1, b1)];
    for &a in &cfrac[1..] {
        if a == 0 || u64::MAX / a < a1 || u64::MAX / a < b1 {
            return None;
        }
        let an = a * a1;
        let bn = a * b1;
        if u64::MAX - an < a0 || u64::MAX - bn < b0 {
            return None;
        }
        let (an, bn) = (an + a0, bn + b0);
        a0 = a1;
        a1 = an;
        b0 = b1;
        b1 = bn;
        out.push((a1, b1));
    }
    Some(out)
}

/// The best rational `n / d` with `d <= dbound` close to `x`.
///
/// Same method as MPU `rational.c` `bestrational`: continued fraction
/// expansion of `x`, with the best semiconvergent at the bound. Returns
/// `None` when `dbound` is 0, when `|x|` is not below `u64::MAX`, or when
/// an intermediate value overflows.
///
/// # Examples
///
/// ```
/// use math_prime_util::bestrational;
///
/// assert_eq!(bestrational(3.14159265358979, 38), Some((22, 7)));
/// ```
#[must_use]
pub fn bestrational(x: f64, dbound: u64) -> Option<(u64, u64)> {
    let xabs = x.abs();
    if dbound == 0 || !(xabs < u64::MAX as f64) {
        return None;
    }
    let (mut p0, mut q0) = (1u64, 0u64);
    let mut p1 = xabs as u64;
    let mut q1 = 1u64;
    let mut rem = xabs - p1 as f64;
    while rem > 0.0 {
        let invrem = 1.0 / rem;
        if !(invrem < u64::MAX as f64) {
            return None;
        }
        let a = invrem as u64;
        rem = invrem - a as f64;
        if a == 0 {
            return None;
        }
        let qlimit = (dbound - q0) / q1;
        if a > qlimit {
            if qlimit >= 1 {
                if p1 != 0 && qlimit > (u64::MAX - p0) / p1 {
                    return None;
                }
                let ps = qlimit * p1 + p0;
                let qs = qlimit * q1 + q0;
                let mut pserr = ps as f64 - xabs * qs as f64;
                let mut p1err = p1 as f64 - xabs * q1 as f64;
                if pserr < 0.0 {
                    pserr = -pserr;
                }
                if p1err < 0.0 {
                    p1err = -p1err;
                }
                if pserr * (q1 as f64) < p1err * (qs as f64) {
                    p1 = ps;
                    q1 = qs;
                }
            }
            break;
        }
        let q2 = a * q1 + q0;
        if u64::MAX / a < p1 {
            return None;
        }
        let mut p2 = a * p1;
        if u64::MAX - p2 < p0 {
            return None;
        }
        p2 += p0;
        p0 = p1;
        q0 = q1;
        p1 = p2;
        q1 = q2;
    }
    Some((p1, q1))
}

/// The Calkin-Wilf successor of the reduced fraction `num / den`.
///
/// Same formula as MPU `rational.c` `next_calkin_wilf`. Returns `None`
/// for a zero argument, a fraction that is not reduced, or overflow.
///
/// # Examples
///
/// ```
/// use math_prime_util::next_calkin_wilf;
///
/// assert_eq!(next_calkin_wilf(1, 1), Some((1, 2)));
/// assert_eq!(next_calkin_wilf(1, 2), Some((2, 1)));
/// ```
#[must_use]
pub fn next_calkin_wilf(num: u64, den: u64) -> Option<(u64, u64)> {
    if num == 0 || den == 0 || gcd(num, den) != 1 {
        return None;
    }
    let (n, d) = (num, den);
    let nd_new = if n < d {
        d - n
    } else if d == 1 {
        n.checked_add(1)?
    } else {
        let nd = n % d;
        let nr = n - nd;
        let dr = d - nd;
        nr.checked_add(dr)?
    };
    Some((d, nd_new))
}

/// The Stern-Brocot successor of the reduced fraction `num / den`.
///
/// Same method as MPU `rational.c` `next_stern_brocot`: at the end of a
/// row, move to the start of the next row. Otherwise take the node after
/// `n(F)` in the tree. Returns `None` for a zero argument, a fraction that
/// is not reduced, or overflow.
///
/// # Examples
///
/// ```
/// use math_prime_util::next_stern_brocot;
///
/// assert_eq!(next_stern_brocot(1, 1), Some((1, 2)));
/// assert_eq!(next_stern_brocot(1, 2), Some((2, 1)));
/// ```
#[must_use]
pub fn next_stern_brocot(num: u64, den: u64) -> Option<(u64, u64)> {
    if num == 0 || den == 0 || gcd(num, den) != 1 {
        return None;
    }
    if den == 1 {
        if num == u64::MAX {
            return None;
        }
        return Some((1, num + 1));
    }
    let index = stern_brocot_n(num, den)?;
    if index == u64::MAX {
        return None;
    }
    Some(nth_stern_brocot(index + 1))
}

/// Index of `num / den` in the Calkin-Wilf tree, from its continued fraction.
///
/// Same method as MPU `rational.c` `calkin_wilf_n`. Returns `None` when
/// the index is 0 or does not fit in `u64`.
///
/// # Panics
///
/// Panics when `num / den` is not reduced, as MPU does (`croak`).
///
/// # Examples
///
/// ```
/// use math_prime_util::calkin_wilf_n;
///
/// assert_eq!(calkin_wilf_n(3, 2), Some(5));
/// assert_eq!(calkin_wilf_n(1, 1), Some(1));
/// ```
#[must_use]
pub fn calkin_wilf_n(num: u64, den: u64) -> Option<u64> {
    let mut cf = contfrac(num, den);
    let rem = gcd(num, den);
    assert!(rem == 1, "Rational must be reduced");
    let last = cf.last_mut()?;
    if *last == 0 {
        return None;
    }
    *last -= 1;
    let mut n: u64 = 0;
    let mut shift: u32 = 0;
    let mut bit_on = true;
    for &term in &cf {
        if shift + term as u32 >= u64::BITS {
            return None;
        }
        if bit_on {
            for bit in 0..term {
                n |= 1u64 << (shift + bit as u32);
            }
        }
        shift += term as u32;
        bit_on = !bit_on;
    }
    n |= 1u64 << shift;
    Some(n)
}

/// Index of `num / den` in the Stern-Brocot tree.
///
/// Same method as MPU `rational.c` `stern_brocot_n`: reverse the bits of
/// the Calkin-Wilf index. Returns `None` where `calkin_wilf_n` does.
///
/// # Panics
///
/// Panics when `num / den` is not reduced.
///
/// # Examples
///
/// ```
/// use math_prime_util::stern_brocot_n;
///
/// assert_eq!(stern_brocot_n(1, 1), Some(1));
/// assert_eq!(stern_brocot_n(2, 3), Some(5));
/// ```
#[must_use]
pub fn stern_brocot_n(num: u64, den: u64) -> Option<u64> {
    let mut m = calkin_wilf_n(num, den)?;
    let mut n: u64 = 1;
    while m > 1 {
        n = (n << 1) | (m & 1);
        m >>= 1;
    }
    Some(n)
}

/// The `n`-th fraction of the Calkin-Wilf tree.
///
/// Same method as MPU `rational.c` `nth_calkin_wilf`.
///
/// # Panics
///
/// Panics when `n` is 0, as MPU does for a bad argument.
///
/// # Examples
///
/// ```
/// use math_prime_util::nth_calkin_wilf;
///
/// assert_eq!(nth_calkin_wilf(1), (1, 1));
/// assert_eq!(nth_calkin_wilf(4), (1, 3));
/// ```
#[must_use]
pub fn nth_calkin_wilf(n: u64) -> (u64, u64) {
    assert!(n > 0, "nth_calkin_wilf: n must be positive");
    let mut b = n.ilog2();
    let (mut p, mut q) = (0u64, 1u64);
    loop {
        if (n >> b) & 1 == 1 {
            p += q;
        } else {
            q += p;
        }
        if b == 0 {
            break;
        }
        b -= 1;
    }
    (p, q)
}

/// The `n`-th fraction of the Stern-Brocot tree.
///
/// Same method as MPU `rational.c` `nth_stern_brocot`.
///
/// # Panics
///
/// Panics when `n` is 0, as MPU does for a bad argument.
///
/// # Examples
///
/// ```
/// use math_prime_util::nth_stern_brocot;
///
/// assert_eq!(nth_stern_brocot(1), (1, 1));
/// assert_eq!(nth_stern_brocot(5), (2, 3));
/// ```
#[must_use]
pub fn nth_stern_brocot(mut n: u64) -> (u64, u64) {
    assert!(n > 0, "nth_stern_brocot: n must be positive");
    let (mut p, mut q) = (1u64, 1u64);
    while n > 1 {
        if n & 1 == 1 {
            p += q;
        } else {
            q += p;
        }
        n >>= 1;
    }
    (p, q)
}

/// The Stern diatomic sequence value `s(n)`.
///
/// Same method as MPU `rational.c` `nth_stern_diatomic`.
///
/// # Examples
///
/// ```
/// use math_prime_util::nth_stern_diatomic;
///
/// assert_eq!(nth_stern_diatomic(0), 0);
/// assert_eq!(nth_stern_diatomic(6), 2);
/// ```
#[must_use]
pub fn nth_stern_diatomic(mut n: u64) -> u64 {
    let (mut p, mut q) = (0u64, 1u64);
    while n > 0 {
        if n & 1 == 1 {
            p += q;
        } else {
            q += p;
        }
        n >>= 1;
    }
    p
}

/// Greatest common divisor, by the Euclidean algorithm.
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

/// Next Farey fraction after `p / q` in the order `F_n`.
///
/// Same method as MPU `rational.c` `next_farey`: the extended Euclidean
/// algorithm gives the neighbour. Returns `None` when `p / q` is not
/// below 1, when `q > n`, or when no successor exists in `F_n`.
///
/// # Panics
///
/// Panics when the extended gcd fails, as MPU does (`croak`). Not reached
/// for valid input.
///
/// # Examples
///
/// ```
/// use math_prime_util::next_farey;
///
/// assert_eq!(next_farey(5, 1, 3), Some((2, 5)));
/// assert_eq!(next_farey(3, 1, 1), None);
/// ```
#[must_use]
pub fn next_farey(n: u32, p: u32, q: u32) -> Option<(u32, u32)> {
    if n == 0 || p >= q {
        return None;
    }
    let g = gcd(u64::from(p), u64::from(q));
    let uvp = u64::from(p) / g;
    let uvq = u64::from(q) / g;
    if uvq > u64::from(n) || uvq > i64::MAX as u64 {
        return None;
    }
    let (ivu, ivv) = ext_gcd_coeffs(uvp as i64, uvq as i64);
    let (r, tail) = if ivu <= 0 {
        ((-ivu) as u64, ivv as u64)
    } else {
        let vabs = (ivv as u64).wrapping_neg();
        if vabs > uvp {
            return None;
        }
        (uvq - ivu as u64, uvp - vabs)
    };
    let h = (u64::from(n) - r) / uvq;
    let nq = h * uvq + r;
    let np = h * uvp + tail;
    if np > nq || nq > u64::from(n) {
        return None;
    }
    Some((np as u32, nq as u32))
}

/// Coefficients `(x, y)` with `x * a + y * b = 1`, for coprime `a, b >= 1`.
fn ext_gcd_coeffs(a: i64, b: i64) -> (i64, i64) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1i64, 0i64);
    while r != 0 {
        let qt = old_r / r;
        (old_r, r) = (r, old_r - qt * r);
        (old_s, s) = (s, old_s - qt * s);
    }
    (old_s, (old_r - old_s * a) / b)
}

/// Rank of `p / q` in the Farey sequence `F_n`: the count of its terms up
/// to `p / q`, starting at 1 for `0 / 1`.
///
/// Same method as MPU `rational.c` `farey_rank`: the totient sieve over
/// `2..=n`. Returns `u64::MAX` where MPU does, on overflow or bad input.
///
/// # Examples
///
/// ```
/// use math_prime_util::farey_rank;
///
/// assert_eq!(farey_rank(5, 1, 2), 6);
/// assert_eq!(farey_rank(1, 0, 1), 0);
/// ```
#[must_use]
pub fn farey_rank(n: u32, p: u32, q: u32) -> u64 {
    if n == 0 || q == 0 || p == 0 {
        return 0;
    }
    if n == u32::MAX {
        return u64::MAX;
    }
    let g = gcd(u64::from(p), u64::from(q));
    let (p, q) = (u64::from(p) / g, u64::from(q) / g);
    if u64::from(n) > u64::MAX / p {
        return u64::MAX;
    }
    let n = n as usize;
    let mut count = vec![0u64; n + 1];
    for i in 2..=n {
        count[i] = (i as u64 * p - 1) / q;
    }
    let mut sum: u64 = 1;
    for i in 2..=n {
        let icount = count[i];
        let mut j = i;
        while j <= n - i {
            count[j + i] -= icount;
            j += i;
        }
        if icount >= u64::MAX - sum {
            return u64::MAX;
        }
        sum += icount;
    }
    sum
}

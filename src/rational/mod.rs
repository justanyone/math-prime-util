//! Continued fractions, convergents, best rational approximations, the
//! Calkin-Wilf and Stern-Brocot trees, and Farey sequences.
//!
//! Ported from MPU's `rational.c` and `rational.h` (MPU 0.75).

#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::cast_possible_wrap
)]

/// Continued fraction terms of `num / den`, by the Euclidean algorithm.
///
/// Same method as MPU `rational.c` `contfrac`. The last remainder (the
/// gcd) is dropped.
///
/// # Panics
///
/// Panics if the denominator is zero.
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
    assert!(den > 0, "contfrac: denominator must be positive");
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
/// Uses MPU `XS.xs` `from_contfrac` and `rational.c` `convergents`.
/// Returns `None` when the value does not fit in `u64`.
///
/// # Panics
///
/// Panics if a term after the first is zero.
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
    assert!(
        cf.iter().skip(1).all(|&a| a > 0),
        "terms after the first must be positive"
    );
    if cf.is_empty() {
        return Some((0, 1));
    }
    convergents(cf)?.last().copied()
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
/// # Panics
///
/// Panics if the denominator bound is zero.
///
/// # Examples
///
/// ```
/// use math_prime_util::bestrational;
///
/// assert_eq!(bestrational(3.14159265358979, 38), Some((22, 7)));
/// ```
#[must_use]
pub fn bestrational(x: f64, dbound: u64) -> Option<(i128, u64)> {
    assert!(
        dbound > 0,
        "bestrational: denominator bound must be positive"
    );
    let xabs = x.abs();
    if dbound == 0 || !xabs.is_finite() || xabs >= u64::MAX as f64 {
        return None;
    }
    let (mut p0, mut q0) = (1u64, 0u64);
    let mut p1 = xabs as u64;
    let mut q1 = 1u64;
    let mut rem = xabs - p1 as f64;
    while rem > 0.0 {
        let invrem = 1.0 / rem;
        if !invrem.is_finite() || invrem >= u64::MAX as f64 {
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
    Some((
        if x.is_sign_negative() {
            -i128::from(p1)
        } else {
            i128::from(p1)
        },
        q1,
    ))
}

/// The Calkin-Wilf successor of the reduced fraction `num / den`.
///
/// Same formula as MPU `rational.c` `next_calkin_wilf`. Returns `None`
/// on overflow.
///
/// # Panics
///
/// Panics if either argument is zero or the fraction is not reduced.
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
    assert!(
        num > 0 && den > 0 && gcd(num, den) == 1,
        "rational must be positive and reduced"
    );
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
/// is not reduced. Returns `None` on overflow.
///
/// # Panics
///
/// Panics if either argument is zero or the fraction is not reduced.
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
    assert!(
        num > 0 && den > 0 && gcd(num, den) == 1,
        "rational must be positive and reduced"
    );
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
    assert!(num > 0 && den > 0, "rational must be positive");
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
        if term >= u64::from(u64::BITS - shift) {
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

/// Counts fractions strictly below `p / q` in the Farey sequence `F_n`.
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
    if p > q {
        return farey_len(n);
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

/// Returns the number of terms in the Farey sequence of order `n`.
///
/// Uses MPU `rational.c` `farey_length` and the divisor grouping in
/// `totients.c` `_sumtotient_direct`.
///
/// # Panics
///
/// Panics if `n` is zero.
///
/// ```
/// assert_eq!(math_prime_util::farey_len(5), 11);
/// ```
#[must_use]
pub fn farey_len(n: u32) -> u64 {
    assert!(n > 0, "farey: order must be positive");
    if n <= 2 {
        return u64::from(n) + 1;
    }
    let n = u64::from(n);
    let root = crate::sqrtint(n) as usize;
    let flag = n < root as u64 * (root as u64 + 1);
    let mut large: Vec<u64> = (0..=root as u64).collect();
    for p in 2..=root {
        if large[p] == p as u64 {
            for j in (p..=root).step_by(p) {
                large[j] -= large[j] / p as u64;
            }
        }
    }
    let mut small = vec![0u64; root + 1];
    small[1] = 1;
    for i in 2..=root {
        small[i] = small[i - 1] + large[i];
    }
    if flag {
        large[root] = small[root];
    }
    for i in (1..=root - usize::from(flag)).rev() {
        let m = n / i as u64;
        let r = crate::sqrtint(m) as usize;
        let mut sum = (m + 1) / 2 * (m | 1) - (m - m / 2);
        let mut k = 2;
        let mut j = k * i;
        while j <= root {
            sum -= large[j];
            sum -= (m / k as u64 - m / (k + 1) as u64) * small[k];
            k += 1;
            j += i;
        }
        while k <= r {
            sum -= small[(m / k as u64) as usize];
            sum -= (m / k as u64 - m / (k + 1) as u64) * small[k];
            k += 1;
        }
        if m < r as u64 * (r as u64 + 1) {
            sum += small[r];
        }
        large[i] = sum;
    }
    1 + large[1]
}

/// Returns all terms of the Farey sequence of order `n`.
///
/// Uses the Haros recurrence in MPU `rational.c` `farey_array`.
///
/// # Panics
///
/// Panics if `n` is zero or the sequence cannot fit in memory.
///
/// ```
/// assert_eq!(math_prime_util::farey(2), vec![(0, 1), (1, 2), (1, 1)]);
/// ```
#[must_use]
pub fn farey(n: u32) -> Vec<(u32, u32)> {
    assert!(n > 0, "farey: order must be positive");
    let len = farey_len(n);
    assert!(len < u64::from(u32::MAX), "farey: sequence too large");
    let mut out = Vec::with_capacity(usize::try_from(len).expect("farey: sequence too large"));
    let (mut p0, mut q0, mut p1, mut q1) = (0u64, 1u64, 1u64, u64::from(n));
    for _ in 0..len {
        out.push((p0 as u32, q0 as u32));
        let j = (q0 + u64::from(n)) / q1;
        (p0, p1) = (p1, j * p1 - p0);
        (q0, q1) = (q1, j * q1 - q0);
    }
    out
}

/// Returns the term at zero-based index `k` in the Farey sequence.
///
/// Uses the rank search and short walk in MPU `rational.c` `kth_farey`
/// and `_walk_to_k`. Returns `None` if the index is outside the sequence.
///
/// ```
/// assert_eq!(math_prime_util::farey_nth(5, 3), Some((1, 3)));
/// ```
#[must_use]
pub fn farey_nth(n: u32, k: u64) -> Option<(u32, u32)> {
    if n == 0 {
        return None;
    }
    if k < 2 {
        return Some(if k == 0 { (0, 1) } else { (1, n) });
    }
    if n < 2 {
        return None;
    }
    let (mut lo, mut hi, mut count) = (1, n, 1);
    if n >= 5 {
        let inc = ((u64::from(n) + 8191) >> 13) as u32;
        let nf = f64::from(n);
        let len = 1.0 + (0.304 * nf * nf + 0.29 * nf + 0.95);
        let estimate = k as f64 * (nf / len);
        let mut guess = if estimate <= f64::from(lo) {
            lo + 1
        } else if estimate >= f64::from(hi) {
            hi - 1
        } else {
            estimate as u32
        };
        let mut rank = 0;
        if lo < hi {
            rank = farey_rank(n, guess, n);
            if rank <= k {
                lo = guess;
                count = rank;
            } else {
                hi = guess - 1;
            }
        }
        guess = if rank <= k {
            if hi - inc < guess { hi } else { guess + inc }
        } else if lo + inc + 1 > guess {
            lo
        } else {
            guess - 1 - inc
        };
        if lo < hi && guess > lo && guess < hi {
            rank = farey_rank(n, guess, n);
            if rank <= k {
                lo = guess;
                count = rank;
            } else {
                hi = guess - 1;
            }
        }
    }
    while lo < hi {
        let mid = lo + ((hi - lo + 1) >> 1);
        let rank = farey_rank(n, mid, n);
        if rank <= k {
            lo = mid;
            count = rank;
        } else {
            hi = mid - 1;
        }
    }
    if lo == n {
        return (k == count).then_some((1, 1));
    }
    let g = gcd(u64::from(lo), u64::from(n)) as u32;
    let (mut p0, mut q0) = (lo / g, n / g);
    let mut remaining = k - count;
    if remaining == 0 {
        return Some((p0, q0));
    }
    let (mut p1, mut q1) = next_farey(n, p0, q0)?;
    remaining -= 1;
    for _ in 0..remaining {
        let j = (u64::from(q0) + u64::from(n)) / u64::from(q1);
        let p2 = j * u64::from(p1) - u64::from(p0);
        let q2 = j * u64::from(q1) - u64::from(q0);
        (p0, p1) = (p1, u32::try_from(p2).ok()?);
        (q0, q1) = (q1, u32::try_from(q2).ok()?);
    }
    Some((p1, q1))
}

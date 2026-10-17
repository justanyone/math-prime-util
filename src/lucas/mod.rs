//! Lucas sequences and Fibonacci numbers.
//!
//! Ports `reference/mpu/lucas_seq.c` (MPU 0.75). The modular functions use
//! the same methods as MPU: the Crandall-style ladder for odd moduli and the
//! `alt_lucas_seq` ladder for even moduli. `P` and `Q` are `i64`, as in MPU.

/// `(a + b) mod n`, for `a, b < n`.
fn addmod(a: u64, b: u64, n: u64) -> u64 {
    ((u128::from(a) + u128::from(b)) % u128::from(n)) as u64
}

/// `(a - b) mod n`, for `a, b < n`.
fn submod(a: u64, b: u64, n: u64) -> u64 {
    ((u128::from(a) + u128::from(n) - u128::from(b)) % u128::from(n)) as u64
}

/// `(a * b) mod n`.
fn mulmod(a: u64, b: u64, n: u64) -> u64 {
    ((u128::from(a) * u128::from(b)) % u128::from(n)) as u64
}

/// `(a * b + c) mod n`.
fn muladdmod(a: u64, b: u64, c: u64, n: u64) -> u64 {
    ((u128::from(a) * u128::from(b) + u128::from(c)) % u128::from(n)) as u64
}

/// `(a * b - c) mod n`, for `c < n`.
fn mulsubmod(a: u64, b: u64, c: u64, n: u64) -> u64 {
    submod(mulmod(a, b, n), c, n)
}

/// `a^k mod n`, by square and multiply.
fn powmod(a: u64, mut k: u64, n: u64) -> u64 {
    let mut base = a % n;
    let mut acc = 1 % n;
    while k > 0 {
        if k & 1 == 1 {
            acc = mulmod(acc, base, n);
        }
        base = mulmod(base, base, n);
        k >>= 1;
    }
    acc
}

/// Reduces a signed value into `0..n`, as MPU's `ivmod` does.
fn ivmod(a: i64, n: u64) -> u64 {
    let m = a.unsigned_abs() % n;
    if a >= 0 || m == 0 {
        m
    } else {
        n - m
    }
}

/// `a / 2 mod n`, or `None` when 2 has no inverse and `a` is odd.
///
/// Same result as MPU's `divmod(a, 2, n)`, with `0` mapped to `None`.
fn half_mod(a: u64, n: u64) -> Option<u64> {
    if a % 2 == 0 {
        Some(a / 2)
    } else if n % 2 == 1 {
        Some((a / 2) + (n / 2) + 1)
    } else {
        None
    }
}

/// Inverse of `a` mod `n`, or `None` when `gcd(a, n) != 1`.
fn inv_mod(a: u64, n: u64) -> Option<u64> {
    let (mut r0, mut r1) = (i128::from(n), i128::from(a % n));
    let (mut t0, mut t1) = (0_i128, 1_i128);
    while r1 != 0 {
        let q = r0 / r1;
        (r0, r1) = (r1, r0 - q * r1);
        (t0, t1) = (t1, t0 - q * t1);
    }
    if r0 != 1 {
        return None;
    }
    Some(t0.rem_euclid(i128::from(n)) as u64)
}

/// Halves `x` mod odd `n` in the way MPU does: `(x + n) / 2` when `x` is odd.
fn halve_odd(x: u64, n: u64) -> u64 {
    if x & 1 == 1 {
        (n >> 1) + (x >> 1) + 1
    } else {
        x >> 1
    }
}

/// Floor of log2(k), for `k > 0`.
fn top_bit(k: u64) -> u32 {
    63 - k.leading_zeros()
}

/// Port of `alt_lucas_seq` in `reference/mpu/lucas_seq.c`.
///
/// Works for any modulus `n >= 2`, including even `n`.
/// Returns `(U_k, V_k) mod n`.
fn alt_lucas_seq(n: u64, pmod: u64, qmod: u64, k: u64) -> (u64, u64) {
    let mut uh: u64 = 1;
    let mut vl: u64 = 2;
    let mut vh: u64 = pmod;
    let mut ql: u64 = 1;
    let mut qh: u64 = 1;
    let s = k.trailing_zeros();
    let m = top_bit(k);

    if pmod == 1 && qmod == n - 1 {
        let mut sl: i64 = 1;
        let mut sh: i64 = 1;
        for j in (s + 1..=m).rev() {
            sl *= sh;
            ql = if sl == 1 { 1 } else { n - 1 };
            if (k >> j) & 1 == 1 {
                sh = -sl;
                uh = mulmod(uh, vh, n);
                vl = submod(mulmod(vh, vl, n), ql, n);
                let c = if sh == 1 { 2 } else { n - 2 };
                vh = submod(mulmod(vh, vh, n), c, n);
            } else {
                sh = sl;
                uh = submod(mulmod(uh, vl, n), ql, n);
                vh = submod(mulmod(vh, vl, n), ql, n);
                let c = if sl == 1 { 2 } else { n - 2 };
                vl = submod(mulmod(vl, vl, n), c, n);
            }
        }
        sl *= sh;
        ql = if sl == 1 { 1 } else { n - 1 };
        uh = submod(mulmod(uh, vl, n), ql, n);
        vl = submod(mulmod(vh, vl, n), ql, n);
        for j in 0..s {
            uh = mulmod(uh, vl, n);
            let c = if j > 0 { 2 } else { n - 2 };
            vl = submod(mulmod(vl, vl, n), c, n);
        }
        return (uh, vl);
    }

    for j in (s + 1..=m).rev() {
        ql = mulmod(ql, qh, n);
        if (k >> j) & 1 == 1 {
            qh = mulmod(ql, qmod, n);
            uh = mulmod(uh, vh, n);
            vl = submod(mulmod(vh, vl, n), mulmod(pmod, ql, n), n);
            vh = submod(mulmod(vh, vh, n), addmod(qh, qh, n), n);
        } else {
            qh = ql;
            uh = submod(mulmod(uh, vl, n), ql, n);
            vh = submod(mulmod(vh, vl, n), mulmod(pmod, ql, n), n);
            vl = submod(mulmod(vl, vl, n), addmod(ql, ql, n), n);
        }
    }
    ql = mulmod(ql, qh, n);
    qh = mulmod(ql, qmod, n);
    uh = submod(mulmod(uh, vl, n), ql, n);
    vl = submod(mulmod(vh, vl, n), mulmod(pmod, ql, n), n);
    ql = mulmod(ql, qh, n);
    for _ in 0..s {
        uh = mulmod(uh, vl, n);
        vl = submod(mulmod(vl, vl, n), addmod(ql, ql, n), n);
        ql = mulmod(ql, ql, n);
    }
    (uh, vl)
}

/// Returns `(U_k, V_k) mod n` for reduced `p`, `q` (both `< n`), `k > 0`, `n >= 2`.
///
/// Branch order follows `lucasuvmod` in `reference/mpu/lucas_seq.c`.
fn uvmod_core(p: u64, q: u64, k: u64, n: u64) -> (u64, u64) {
    let d = submod(mulmod(p, p, n), mulmod(4, q, n), n);
    if d == 0 {
        if let Some(b) = half_mod(p, n).filter(|&b| b != 0) {
            let u = mulmod(k % n, powmod(b, k - 1, n), n);
            let v = mulmod(2, powmod(b, k, n), n);
            return (u, v);
        }
    }

    let b = top_bit(k);
    let mut u: u64 = 1;
    let mut v: u64 = p;

    if q == 1 && d != 0 {
        if let Some(inv_d) = inv_mod(d, n) {
            // Only 2 mulmods per bit.
            u = mulsubmod(p, p, 2, n);
            for bit in (0..b).rev() {
                let t = mulsubmod(u, v, p, n);
                if (k >> bit) & 1 == 1 {
                    v = t;
                    u = mulsubmod(u, u, 2, n);
                } else {
                    u = t;
                    v = mulsubmod(v, v, 2, n);
                }
            }
            u = addmod(u, u, n);
            u = submod(u, mulmod(v, p, n), n);
            u = mulmod(u, inv_d, n);
            return (u, v);
        }
    }

    if p == 1 && q == n - 1 {
        return alt_lucas_seq(n, p, q, k);
    }

    if n % 2 == 1 && (q == 1 || q == n - 1) {
        let mut qs = q == 1;
        for bit in (0..b).rev() {
            u = mulmod(u, v, n);
            let c = if qs { n - 2 } else { 2 };
            v = muladdmod(v, v, c, n);
            qs = true;
            if (k >> bit) & 1 == 1 {
                let t2 = mulmod(u, d, n);
                if p != 1 {
                    u = mulmod(u, p, n);
                }
                u = addmod(u, v, n);
                u = halve_odd(u, n);
                if p != 1 {
                    v = mulmod(v, p, n);
                }
                v = addmod(v, t2, n);
                v = halve_odd(v, n);
                qs = q == 1;
            }
        }
        return (u, v);
    }

    if n % 2 == 1 {
        let mut qk = q;
        for bit in (0..b).rev() {
            u = mulmod(u, v, n);
            v = mulsubmod(v, v, addmod(qk, qk, n), n);
            qk = mulmod(qk, qk, n);
            if (k >> bit) & 1 == 1 {
                let t2 = mulmod(u, d, n);
                u = muladdmod(u, p, v, n);
                u = halve_odd(u, n);
                v = muladdmod(v, p, t2, n);
                v = halve_odd(v, n);
                qk = mulmod(qk, q, n);
            }
        }
        return (u, v);
    }

    alt_lucas_seq(n, p, q, k)
}

/// Lucas sequences `U_k(P, Q)` and `V_k(P, Q)` mod `n`, and `Q^k mod n`.
///
/// Port of `lucas_seq` in `reference/mpu/lucas_seq.c` (MPU 0.75).
/// Returns `(U_k, V_k, Q^k)`, all reduced mod `n`.
///
/// # Panics
///
/// Panics when `n` is 0 (MPU dies with "modulus n must be > 0").
///
/// # Examples
///
/// ```
/// // Fibonacci: P = 1, Q = -1. U_17 = 1597 and V_17 = 2207.
/// let (u, v, qk) = math_prime_util::lucas_sequence(10_000, 1, -1, 17);
/// assert_eq!((u, v), (1597, 2207));
/// assert_eq!(qk, 1); // (-1)^17 mod 10000 = 9999, so this line checks Q^k below
/// ```
pub fn lucas_sequence(n: u64, p: i64, q: i64, k: u64) -> (u64, u64, u64) {
    assert!(n > 0, "lucas_sequence: modulus n must be > 0");
    if n == 1 {
        return (0, 0, 0);
    }
    let (pm, qm) = (ivmod(p, n), ivmod(q, n));
    let (u, v) = lucasuvmod(p, q, k, n);
    (u, v, powmod(qm, k, n))
}

/// `(U_k(P, Q), V_k(P, Q)) mod n`, with `P` and `Q` reduced first.
///
/// Port of `lucasuvmod` in `reference/mpu/lucas_seq.c` (MPU 0.75).
///
/// # Panics
///
/// Panics when `n` is 0.
///
/// # Examples
///
/// ```
/// // U_5 = 5 and V_5 = 11 for P = 1, Q = -1.
/// assert_eq!(math_prime_util::lucasuvmod(1, -1, 5, 1000), (5, 11));
/// ```
pub fn lucasuvmod(p: i64, q: i64, k: u64, n: u64) -> (u64, u64) {
    assert!(n > 0, "lucasuvmod: modulus n must be > 0");
    if n == 1 {
        return (0, 0);
    }
    if k == 0 {
        return (0, 2 % n);
    }
    uvmod_core(ivmod(p, n), ivmod(q, n), k, n)
}

/// `U_k(P, Q) mod n`.
///
/// Port of `lucasumod` in `reference/mpu/lucas_seq.c` (MPU 0.75).
///
/// # Panics
///
/// Panics when `n` is 0.
///
/// # Examples
///
/// ```
/// // U_10 = 55 for the Fibonacci pair, reduced mod 7 gives 6.
/// assert_eq!(math_prime_util::lucasumod(1, -1, 10, 7), 6);
/// ```
pub fn lucasumod(p: i64, q: i64, k: u64, n: u64) -> u64 {
    lucasuvmod(p, q, k, n).0
}

/// `V_k(P, Q) mod n`.
///
/// Port of `lucasvmod` in `reference/mpu/lucas_seq.c` (MPU 0.75). The C
/// version has a V-only ladder for each branch. This port takes the V value
/// from the same ladder as `lucasuvmod`, which gives the same residues.
///
/// # Panics
///
/// Panics when `n` is 0.
///
/// # Examples
///
/// ```
/// // V_10 = 123 for the Fibonacci pair, reduced mod 1000 gives 123.
/// assert_eq!(math_prime_util::lucasvmod(1, -1, 10, 1000), 123);
/// ```
pub fn lucasvmod(p: i64, q: i64, k: u64, n: u64) -> u64 {
    lucasuvmod(p, q, k, n).1
}

/// Exact `(U_k, V_k)` for `P`, `Q` as `i64`, or `None` when a value
/// does not fit in `i64`.
///
/// Port of `lucasuv` in `reference/mpu/lucas_seq.c` (MPU 0.75). MPU
/// stops when an operand is at or above `2^31` in magnitude. This port does the
/// same and then checks that the results fit `i64`.
///
/// # Examples
///
/// ```
/// // Fibonacci pair: U_10 = 55, V_10 = 123.
/// assert_eq!(math_prime_util::lucasuv(1, -1, 10), Some((55, 123)));
/// ```
pub fn lucasuv(p: i64, q: i64, k: u64) -> Option<(i64, i64)> {
    if k == 0 {
        return Some((0, 2));
    }
    let lim: i128 = 1 << 31;
    let over = |v: i128| v.abs() >= lim;
    let (p, q) = (i128::from(p), i128::from(q));
    if over(p) || over(q) {
        return None;
    }
    let (mut uh, mut vl, mut vh, mut ql, mut qh): (i128, i128, i128, i128, i128) = (1, 2, p, 1, 1);
    let s = k.trailing_zeros();
    let m = top_bit(k);
    for j in (s + 1..=m).rev() {
        if [uh, vh, vl, ql, qh].iter().any(|&x| over(x)) {
            return None;
        }
        ql *= qh;
        if over(ql) {
            return None;
        }
        if (k >> j) & 1 == 1 {
            qh = ql * q;
            if over(qh) {
                return None;
            }
            uh *= vh;
            vl = vh * vl - p * ql;
            vh = vh * vh - 2 * qh;
        } else {
            qh = ql;
            uh = uh * vl - ql;
            vh = vh * vl - p * ql;
            vl = vl * vl - 2 * ql;
        }
    }
    if over(ql) || over(qh) {
        return None;
    }
    ql *= qh;
    if over(ql) {
        return None;
    }
    qh = ql * q;
    if [uh, vh, vl, ql, qh].iter().any(|&x| over(x)) {
        return None;
    }
    uh = uh * vl - ql;
    vl = vh * vl - p * ql;
    ql *= qh;
    for _ in 0..s {
        if [uh, vl, ql].iter().any(|&x| over(x)) {
            return None;
        }
        uh *= vl;
        vl = vl * vl - 2 * ql;
        ql *= ql;
    }
    Some((i64::try_from(uh).ok()?, i64::try_from(vl).ok()?))
}

/// Exact `U_k(P, Q)`, or `None` when it does not fit in `i64`.
///
/// Port of the `U` half of `lucasuv` in `reference/mpu/lucas_seq.c` (MPU 0.75).
///
/// # Examples
///
/// ```
/// assert_eq!(math_prime_util::lucasu(1, -1, 10), Some(55));
/// ```
pub fn lucasu(p: i64, q: i64, k: u64) -> Option<i64> {
    lucasuv(p, q, k).map(|(u, _)| u)
}

/// Exact `V_k(P, Q)`, or `None` when it does not fit in `i64`.
///
/// Port of the `V` half of `lucasuv` in `reference/mpu/lucas_seq.c` (MPU 0.75).
///
/// # Examples
///
/// ```
/// assert_eq!(math_prime_util::lucasv(1, -1, 10), Some(123));
/// ```
pub fn lucasv(p: i64, q: i64, k: u64) -> Option<i64> {
    lucasuv(p, q, k).map(|(_, v)| v)
}

/// Computes `F(k)` and `F(k+1)` by doubling, or `None` on overflow.
///
/// Port of `_calc_fib` in `reference/mpu/lucas_seq.c` (MPU 0.75). Valid for
/// `k < 93`, the largest index whose pair fits `u64`.
fn calc_fib(k: u64) -> Option<(u64, u64)> {
    if k >= 93 {
        return None;
    }
    if k <= 1 {
        return Some((k, 1));
    }
    let (mut a, mut b) = (1_u64, 1_u64);
    for j in (0..top_bit(k)).rev() {
        let c = a * (2 * b - a);
        let d = a * a + b * b;
        if (k >> j) & 1 == 1 {
            a = d;
            b = c + d;
        } else {
            a = c;
            b = d;
        }
    }
    Some((a, b))
}

/// The Fibonacci number `F(k)`, or `None` when it does not fit in `u64`.
///
/// Port of `fibonacci_number` in `reference/mpu/lucas_seq.c` (MPU 0.75).
///
/// # Examples
///
/// ```
/// assert_eq!(math_prime_util::fibonacci(10), Some(55));
/// assert_eq!(math_prime_util::fibonacci(93), None);
/// ```
pub fn fibonacci(k: u64) -> Option<u64> {
    if k <= 1 {
        return Some(k);
    }
    calc_fib(k - 1).map(|(_, b)| b)
}

/// The Lucas number `L(k) = F(k-1) + F(k+1)`, or `None` when it does not fit in `u64`.
///
/// Port of `lucas_number` in `reference/mpu/lucas_seq.c` (MPU 0.75).
///
/// # Examples
///
/// ```
/// assert_eq!(math_prime_util::lucas_number(0), Some(2));
/// assert_eq!(math_prime_util::lucas_number(10), Some(123));
/// ```
pub fn lucas_number(k: u64) -> Option<u64> {
    if k <= 1 {
        return Some(2 - k);
    }
    let (a, b) = calc_fib(k - 1)?;
    a.checked_add(b)?.checked_add(a)
}

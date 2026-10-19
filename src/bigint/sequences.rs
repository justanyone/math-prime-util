//! Arbitrary precision sequences and products from MPU 0.75.

use super::{arith::gcd, primality::next_prime};
use num_bigint::{BigInt, BigUint};

fn fib_pair(k: &BigUint) -> (BigUint, BigUint) {
    let (mut a, mut b) = (BigUint::from(0_u8), BigUint::from(1_u8));
    for bit in (0..k.bits()).rev() {
        let c = &a * ((&b << 1) - &a);
        let d = &a * &a + &b * &b;
        if k.bit(bit) {
            a = d;
            b = c + &a;
        } else {
            a = c;
            b = d;
        }
    }
    (a, b)
}

/// Return the Fibonacci number at `k`.
/// Port of `reference/mpu/lucas_seq.c::fibonacci_number` and `_calc_fib`.
/// ```
/// use math_prime_util::bigint::sequences::fibonacci;
/// assert_eq!(fibonacci(&10_u8.into()), 55_u8.into());
/// ```
#[must_use]
pub fn fibonacci(k: &BigUint) -> BigUint {
    fib_pair(k).0
}

/// Return the Lucas number at `k`.
/// Port of `reference/mpu/lucas_seq.c::lucas_number` and `_calc_fib`.
/// ```
/// use math_prime_util::bigint::sequences::lucas_number;
/// assert_eq!(lucas_number(&10_u8.into()), 123_u8.into());
/// ```
#[must_use]
pub fn lucas_number(k: &BigUint) -> BigUint {
    let (a, b) = fib_pair(k);
    (b << 1) - a
}

/// Return `(U_k, V_k)` for signed coefficients `p` and `q`.
/// Port of `reference/mpu/lucas_seq.c::lucasuv`, without overflow guards.
/// ```
/// use math_prime_util::bigint::sequences::lucasuv;
/// assert_eq!(lucasuv(&1.into(), &(-1).into(), &10_u8.into()), (55.into(), 123.into()));
/// ```
#[must_use]
pub fn lucasuv(p: &BigInt, q: &BigInt, k: &BigUint) -> (BigInt, BigInt) {
    if k.bits() == 0 {
        return (0.into(), 2.into());
    }
    let s = k.trailing_zeros().unwrap_or(0);
    let (mut uh, mut vl, mut vh, mut ql, mut qh) = (
        BigInt::from(1),
        BigInt::from(2),
        p.clone(),
        BigInt::from(1),
        BigInt::from(1),
    );
    for j in ((s + 1)..k.bits()).rev() {
        ql *= &qh;
        if k.bit(j) {
            qh = &ql * q;
            uh *= &vh;
            vl = &vh * vl - p * &ql;
            vh = &vh * &vh - (&qh << 1);
        } else {
            qh = ql.clone();
            uh = uh * &vl - &ql;
            vh = vh * &vl - p * &ql;
            vl = &vl * &vl - (&ql << 1);
        }
    }
    ql *= &qh;
    qh = &ql * q;
    uh = uh * &vl - &ql;
    vl = vh * vl - p * &ql;
    ql *= qh;
    for _ in 0..s {
        uh *= &vl;
        vl = &vl * &vl - (&ql << 1);
        ql = &ql * &ql;
    }
    (uh, vl)
}

/// Return `U_k` for signed coefficients `p` and `q`.
/// Port of `reference/mpu/lucas_seq.c::lucasuv`.
/// ```
/// use math_prime_util::bigint::sequences::lucasu;
/// assert_eq!(lucasu(&1.into(), &(-1).into(), &10_u8.into()), 55.into());
/// ```
#[must_use]
pub fn lucasu(p: &BigInt, q: &BigInt, k: &BigUint) -> BigInt {
    lucasuv(p, q, k).0
}

/// Return `V_k` for signed coefficients `p` and `q`.
/// Port of `reference/mpu/lucas_seq.c::lucasuv`.
/// ```
/// use math_prime_util::bigint::sequences::lucasv;
/// assert_eq!(lucasv(&1.into(), &(-1).into(), &10_u8.into()), 123.into());
/// ```
#[must_use]
pub fn lucasv(p: &BigInt, q: &BigInt, k: &BigUint) -> BigInt {
    lucasuv(p, q, k).1
}

/// Return the product of primes at most `n`.
/// Port of `reference/mpu/util.c::primorial`, with the prime loop from `PP.pm::primorial`.
/// The output size grows with `n`.
/// ```
/// use math_prime_util::bigint::sequences::primorial;
/// assert_eq!(primorial(&10_u8.into()), 210_u16.into());
/// ```
#[must_use]
pub fn primorial(n: &BigUint) -> BigUint {
    let (mut p, mut product) = (BigUint::from(2_u8), BigUint::from(1_u8));
    while &p <= n {
        product *= &p;
        p = next_prime(&p);
    }
    product
}

/// Return the product of the first `n` primes.
/// Port of `reference/mpu/util.c::pn_primorial` and `PP.pm::pn_primorial`.
/// The output size grows with `n`.
/// ```
/// use math_prime_util::bigint::sequences::pn_primorial;
/// assert_eq!(pn_primorial(&4_u8.into()), 210_u16.into());
/// ```
#[must_use]
pub fn pn_primorial(n: &BigUint) -> BigUint {
    let (mut p, mut count, mut product) = (
        BigUint::from(1_u8),
        BigUint::from(0_u8),
        BigUint::from(1_u8),
    );
    while &count < n {
        p = next_prime(&p);
        product *= &p;
        count += 1_u8;
    }
    product
}

/// Return the least common multiple of `1..=n`.
/// Port of `reference/mpu/util.c::consecutive_integer_lcm`.
/// The output size grows with `n`.
/// ```
/// use math_prime_util::bigint::sequences::consecutive_integer_lcm;
/// assert_eq!(consecutive_integer_lcm(&10_u8.into()), 2520_u16.into());
/// ```
#[must_use]
pub fn consecutive_integer_lcm(n: &BigUint) -> BigUint {
    let (mut p, mut product) = (BigUint::from(2_u8), BigUint::from(1_u8));
    while &p <= n {
        let mut power = p.clone();
        let limit = n / &p;
        while power <= limit {
            power *= &p;
        }
        product *= power;
        p = next_prime(&p);
    }
    product
}

/// Return the sum of the input values. An empty input returns zero.
/// Port of `reference/mpu/XS.xs::vecsum` and `PP.pm::vecsum`.
/// ```
/// use math_prime_util::bigint::sequences::vecsum;
/// assert_eq!(vecsum(&[2_u8.into(), 3_u8.into()]), 5_u8.into());
/// ```
#[must_use]
pub fn vecsum(values: &[BigUint]) -> BigUint {
    values.iter().sum()
}

/// Return the product of the input values. An empty input returns one.
/// Port of `reference/mpu/XS.xs::vecprod` and `PP.pm::vecprod`.
/// ```
/// use math_prime_util::bigint::sequences::vecprod;
/// assert_eq!(vecprod(&[2_u8.into(), 3_u8.into()]), 6_u8.into());
/// ```
#[must_use]
pub fn vecprod(values: &[BigUint]) -> BigUint {
    values.iter().product()
}

/// Return the inclusive running sums of the input values.
/// Port of `reference/mpu/XS.xs::vecprefixsum` and `PP.pm::vecprefixsum`.
/// ```
/// use math_prime_util::bigint::sequences::vecprefixsum;
/// assert_eq!(vecprefixsum(&[2_u8.into(), 3_u8.into()]), vec![2_u8.into(), 5_u8.into()]);
/// ```
#[must_use]
pub fn vecprefixsum(values: &[BigUint]) -> Vec<BigUint> {
    let mut sum = BigUint::from(0_u8);
    values
        .iter()
        .map(|v| {
            sum += v;
            sum.clone()
        })
        .collect()
}

/// Sum `floor((a*i+b)/m)` over `0..n`.
/// Port of `reference/mpu/util.c::floor_sum`.
/// # Panics
/// Panics if `m` is zero.
/// ```
/// use math_prime_util::bigint::sequences::floor_sum;
/// assert_eq!(floor_sum(&4_u8.into(), &10_u8.into(), &6_u8.into(), &3_u8.into()), 3_u8.into());
/// ```
#[must_use]
pub fn floor_sum(n: &BigUint, m: &BigUint, a: &BigUint, b: &BigUint) -> BigUint {
    assert!(m.bits() != 0, "floor_sum: divide by zero");
    let (mut n, mut m, mut a, mut b) = (n.clone(), m.clone(), a.clone(), b.clone());
    let mut sum = BigUint::from(0_u8);
    while n.bits() != 0 {
        if a >= m {
            sum += &n * (&n - 1_u8) / 2_u8 * (&a / &m);
            a %= &m;
        }
        if b >= m {
            sum += &n * (&b / &m);
            b %= &m;
        }
        let ymax = &a * &n + &b;
        if ymax < m {
            break;
        }
        n = &ymax / &m;
        b = ymax % &m;
        std::mem::swap(&mut m, &mut a);
    }
    sum
}

/// Return `1^k + ... + n^k`.
/// Port of `reference/mpu/util.c::powersum`. Use its factorial, Stirling,
/// and binomial expansion without native overflow limits.
/// The work and storage grow with `k`.
/// ```
/// use math_prime_util::bigint::sequences::powersum;
/// assert_eq!(powersum(&10_u8.into(), &2_u8.into()), 385_u16.into());
/// ```
#[must_use]
pub fn powersum(n: &BigUint, k: &BigUint) -> BigUint {
    if n <= &BigUint::from(1_u8) || k.bits() == 0 {
        return n.clone();
    }
    if k == &BigUint::from(1_u8) {
        return n * (n + 1_u8) / 2_u8;
    }
    let mut row = vec![BigUint::from(1_u8)];
    let mut degree = BigUint::from(0_u8);
    while &degree < k {
        row.push(BigUint::from(0_u8));
        for j in (1..row.len()).rev() {
            row[j] = &row[j] * j + &row[j - 1];
        }
        row[0] = BigUint::from(0_u8);
        degree += 1_u8;
    }
    let (mut sum, mut factorial, mut choose) = (BigUint::from(0_u8), BigUint::from(1_u8), n + 1_u8);
    for (j, stirling) in row.iter().enumerate().skip(1) {
        let bj = BigUint::from(j);
        if &bj > n {
            break;
        }
        factorial *= j;
        choose = choose * (n + 1_u8 - &bj) / (j + 1);
        sum += &factorial * stirling * &choose;
    }
    sum
}

/// Return the largest integer outside the nonnegative span of `values`.
/// Return `Some(-1)` if a value is one, or `None` for fewer than two values.
/// Port of `reference/mpu/util.c::frobenius_number`, using Round Robin.
/// # Panics
/// Panics for zero values, a noncoprime set, or an unallocatable residue table.
/// ```
/// use math_prime_util::bigint::sequences::frobenius_number;
/// assert_eq!(frobenius_number(&[6_u8.into(), 9_u8.into(), 20_u8.into()]), Some(43.into()));
/// ```
pub fn frobenius_number(values: &[BigUint]) -> Option<BigInt> {
    assert!(
        values.iter().all(|v| v.bits() != 0),
        "Frobenius values must be positive"
    );
    let mut basis = values.to_vec();
    basis.sort_unstable();
    if basis.first() == Some(&BigUint::from(1_u8)) {
        return Some((-1).into());
    }
    if basis.len() < 2 {
        return None;
    }
    let common = basis.iter().fold(BigUint::from(0_u8), |g, v| gcd(&g, v));
    assert_eq!(
        common,
        BigUint::from(1_u8),
        "Frobenius number set must be coprime"
    );
    if basis.len() == 2 {
        return Some(BigInt::from(&basis[0] * &basis[1] - &basis[0] - &basis[1]));
    }
    let modulus = usize::try_from(&basis[0]).expect("Frobenius residue table is too large");
    let mut distances: Vec<Option<BigUint>> = Vec::new();
    distances
        .try_reserve_exact(modulus)
        .expect("Frobenius residue table cannot be allocated");
    distances.resize(modulus, None);
    distances[0] = Some(0_u8.into());
    for coin in &basis[1..] {
        let residue = usize::try_from(coin % &basis[0]).ok()?;
        if distances[residue].as_ref().is_some_and(|v| v <= coin) {
            continue;
        }
        let divisor = usize::try_from(gcd(&basis[0], coin)).ok()?;
        for r in 0..divisor {
            let start = (r..modulus)
                .step_by(divisor)
                .filter_map(|q| distances[q].as_ref())
                .min()
                .cloned();
            if let Some(mut distance) = start {
                for _ in 1..modulus / divisor {
                    distance += coin;
                    let p = usize::try_from(&distance % &basis[0]).ok()?;
                    if let Some(old) = &distances[p] {
                        if old < &distance {
                            distance = old.clone();
                        }
                    }
                    distances[p] = Some(distance.clone());
                }
            }
        }
    }
    Some(BigInt::from(
        distances
            .into_iter()
            .collect::<Option<Vec<_>>>()?
            .into_iter()
            .max()?
            - &basis[0],
    ))
}

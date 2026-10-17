//! Powerfree integers, from MPU 0.75 `powerfree.c`.
#![allow(clippy::cast_possible_truncation)] // Sieve indices are bounded by native roots.
use super::powerful::factor_exponents;
use crate::rootint;

pub(super) fn mobius(limit: u64) -> Vec<i8> {
    let size = usize::try_from(limit).expect("sieve size fits usize");
    let mut mu = vec![1_i8; size + 1];
    let mut composite = vec![false; size + 1];
    mu[0] = 0;
    for p in 2..=size {
        if !composite[p] {
            for j in (p..=size).step_by(p) {
                mu[j] = -mu[j];
                composite[j] = true;
            }
            if p <= size / p {
                for j in (p * p..=size).step_by(p * p) {
                    mu[j] = 0;
                }
            }
        }
    }
    mu
}

pub(super) fn squarefree_count(n: u64) -> u64 {
    if n < 4 {
        return n;
    }
    let big_i = rootint(n, 5);
    let d = rootint(n / big_i, 2);
    let mu = mobius(d);
    let mut mertens = vec![0_i32; mu.len()];
    let mut sum = i128::from(n);
    for j in 1..mu.len() {
        mertens[j] = mertens[j - 1] + i32::from(mu[j]);
        if j >= 2 {
            let i = j as u64;
            sum += i128::from(mu[j]) * i128::from(n / (i * i));
        }
    }
    let mut mx = vec![0_i64; big_i as usize + 1];
    let mut mx_sum = 0_i128;
    for i in (1..big_i).rev() {
        let xi = rootint(n / i, 2);
        let l = rootint(xi, 2);
        let mut value = 1_i64;
        for j in 1..=xi / (l + 1) {
            value -= i64::from(mertens[j as usize])
                * i64::try_from(xi / j - xi / (j + 1)).expect("root fits i64");
        }
        for j in 2..=l {
            value -= if xi / j <= d {
                i64::from(mertens[(xi / j) as usize])
            } else {
                mx[(j * j * i) as usize]
            };
        }
        mx[i as usize] = value;
        mx_sum += i128::from(value);
    }
    u64::try_from(sum + mx_sum - i128::from(big_i - 1) * i128::from(mertens[d as usize]))
        .expect("count fits u64")
}

fn triangular(n: u64) -> u128 {
    u128::from(n) * (u128::from(n) + 1) / 2
}

/// Sum positive `k`-powerfree integers up to `n` with wide arithmetic.
/// Port of `powerfree.c` / `powerfree_sum`.
/// ```
/// assert_eq!(math_prime_util::powerfree_sum(10, 2), 34);
/// ```
pub fn powerfree_sum(n: u64, k: u32) -> u128 {
    if k < 2 {
        return u128::from(n >= 1);
    }
    let mu = mobius(rootint(n, k));
    let mut sum = triangular(n);
    let mut subtract = 0;
    for (i, &m) in mu.iter().enumerate().skip(2) {
        if m != 0 {
            let ik = (i as u64).pow(k);
            let term = u128::from(ik) * triangular(n / ik);
            if m > 0 {
                sum += term;
            } else {
                subtract += term;
            }
        }
    }
    sum - subtract
}

/// Remove all complete `k`th powers from `n`.
/// Port of `powerfree.c` / `powerfree_part`.
/// ```
/// assert_eq!(math_prime_util::powerfree_part(48, 2), 3);
/// ```
pub fn powerfree_part(n: u64, k: u32) -> u64 {
    if k < 2 || n <= 1 {
        return u64::from(n == 1);
    }
    if k >= 64 {
        return n;
    }
    factor_exponents(n)
        .iter()
        .map(|&(p, e)| p.pow(e % k))
        .product()
}

/// Sum the powerfree parts of positive integers up to `n`.
/// Port of `powerfree.c` / `powerfree_part_sum` and `fprod`.
/// ```
/// assert_eq!(math_prime_util::powerfree_part_sum(10, 2), 38);
/// ```
pub fn powerfree_part_sum(n: u64, k: u32) -> u128 {
    if k < 2 || n <= 1 {
        return u128::from(n >= 1);
    }
    let mut sum = triangular(n);
    let mut subtract = 0;
    for j in 2..=rootint(n, k) {
        let factors = factor_exponents(j);
        let product: u128 = factors
            .iter()
            .map(|&(p, _)| u128::from(p.pow(k)) - 1)
            .product();
        let term = product * triangular(n / j.pow(k));
        if factors.len() % 2 == 0 {
            sum += term;
        } else {
            subtract += term;
        }
    }
    sum - subtract
}

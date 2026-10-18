//! Powerfree integers, from MPU 0.75 `powerfree.c`.
#![allow(clippy::cast_possible_truncation)] // Sieve indices are bounded by native roots.
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

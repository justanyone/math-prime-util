use super::euler_phi_range;
use crate::{rootint, sqrtint};
use std::collections::HashMap;

// MPU totients.c::_sumtotient_direct. The two caches index small values
// and quotients separately, so both loops stop at a square root.
fn sumtotient_direct(n: u64) -> u64 {
    if n <= 2 {
        return n;
    }
    let r = usize::try_from(sqrtint(n)).unwrap();
    let flag = n < (r as u64) * (r as u64 + 1);
    let mut quotients = euler_phi_range(0, r as u64);
    let mut small = vec![0_u64; r + 1];
    let mut sum = 1;
    for i in 2..=r {
        sum += quotients[i];
        small[i] = sum;
    }
    if flag {
        quotients[r] = small[r];
    }
    for i in (1..=r - usize::from(flag)).rev() {
        let m = n / i as u64;
        let s = sqrtint(m);
        let mut sum = (m + 1) / 2 * (m | 1) - (m - m / 2);
        let mut k = 2;
        while k * i as u64 <= r as u64 {
            sum -= quotients[usize::try_from(k).unwrap() * i];
            sum -= (m / k - m / (k + 1)) * small[usize::try_from(k).unwrap()];
            k += 1;
        }
        while k <= s {
            sum -= small[usize::try_from(m / k).unwrap()];
            sum -= (m / k - m / (k + 1)) * small[usize::try_from(k).unwrap()];
            k += 1;
        }
        if m < s * (s + 1) {
            sum += small[usize::try_from(s).unwrap()];
        }
        quotients[i] = sum;
    }
    quotients[1]
}

/// Sum Euler's totient through `n`, or return `None` on native overflow.
/// Port of `reference/mpu/totients.c::sumtotient`, `_sumtotient_direct`, and `_sumt`.
/// ```
/// assert_eq!(math_prime_util::sumtotient(10), Some(32));
/// ```
pub fn sumtotient(n: u64) -> Option<u64> {
    fn sumt(n: u64, prefix: &[u64], cache: &mut HashMap<u64, u64>) -> u64 {
        if let Ok(i) = usize::try_from(n) {
            if let Some(&v) = prefix.get(i) {
                return v;
            }
        }
        if let Some(&v) = cache.get(&n) {
            return v;
        }
        let s = sqrtint(n);
        let lim = n / (s + 1);
        let mut v = u128::from(n) * u128::from(n + 1) / 2 - u128::from(n - n / 2);
        for k in 2..=lim {
            v -= u128::from(sumt(n / k, prefix, cache));
            v -= u128::from(n / k - n / (k + 1)) * u128::from(sumt(k, prefix, cache));
        }
        if s > lim {
            v -= u128::from(n / s - n / (s + 1)) * u128::from(sumt(s, prefix, cache));
        }
        let v = u64::try_from(v).unwrap();
        cache.insert(n, v);
        v
    }
    if n > 7_790_208_950 {
        return None;
    }
    if n < 4000 {
        return Some(sumtotient_direct(n));
    }
    let mut prefix = euler_phi_range(0, rootint(n, 3).pow(2) - 1);
    for i in 2..prefix.len() {
        prefix[i] += prefix[i - 1];
    }
    Some(sumt(n, &prefix, &mut HashMap::new()))
}

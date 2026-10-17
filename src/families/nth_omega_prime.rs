use crate::{is_omega_prime, nth_prime, pn_primorial};
use super::{nth_almost_prime_upper::interpolate, omega_prime_count};

/// Return the nth number with `k` distinct prime factors.
///
/// Return `None` for index zero or a result outside the native range.
/// Port of MPU 0.75 `omega_primes.c`, `nth_omega_prime`.
/// Use inverse count interpolation, then scan back to a member.
///
/// ```
/// use math_prime_util::nth_omega_prime;
/// assert_eq!(nth_omega_prime(2, 1000), Some(2102));
/// assert_eq!(nth_omega_prime(0, 2), None);
/// ```
pub fn nth_omega_prime(k: u32, n: u64) -> Option<u64> {
    const MAX: [u64;16] = [1,425_656_284_140_516_112,u64::MAX,u64::MAX,u64::MAX,u64::MAX,u64::MAX,u64::MAX,u64::MAX,u64::MAX,u64::MAX,u64::MAX,5_512_134_903_353,87_133_769_732,446_745_559,299_178];
    if n==0 || k>15 || n>MAX[k as usize] { return None; }
    if k==0 { return (n==1).then_some(1); }
    let mut lo=pn_primorial(u64::from(k))?;
    if n==1 { return Some(lo); }
    let hi=if k==1 { let hi=nth_prime(n).unwrap_or(u64::MAX-58); lo=hi>>1; hi } else { 0 };
    let mut result=interpolate(lo,hi,n,600,|x| omega_prime_count(k,x))?;
    while !is_omega_prime(k,result) { result-=1; }
    Some(result)
}

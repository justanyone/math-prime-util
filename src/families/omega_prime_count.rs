use crate::{next_prime, pn_primorial, prime_count, rootint};

/// Count numbers at most `n` with `k` distinct prime factors.
///
/// Port of MPU 0.75 `omega_primes.c`, `omega_prime_count` and
/// `_omega_prime_count_rec2`. Sum over increasing primes and their powers.
///
/// ```
/// use math_prime_util::omega_prime_count;
/// assert_eq!(omega_prime_count(2, 206), 113);
/// assert_eq!(omega_prime_count(0, 100), 1);
/// ```
pub fn omega_prime_count(k: u32, n: u64) -> u64 {
    if k == 0 { return u64::from(n >= 1); }
    if k == 1 { return super::nth_prime_power_approx::count(n, 0); }
    if k > 15 || pn_primorial(u64::from(k)).is_none_or(|min| n < min) { return 0; }
    count_rec(k, n, 1, 2, rootint(n, k), 1)
}

// The prime iterator replaces C's optional cached prime array.
fn count_rec(k: u32, n: u64, m: u64, mut p: u64, s: u64, mut j: u64) -> u64 {
    let mut count=0;
    while p<=s {
        let Some(r)=next_prime(p) else { break; };
        let mut t=m*p;
        if k==2 {
            while t<=n && n/t>=r {
                let w=n/t;
                count+=prime_count(w)-j;
                for e in 2..=64 {
                    let r2=rootint(w,e);
                    if r2<r { break; }
                    count+=prime_count(r2)-j;
                }
                if t>n/p { break; }
                t*=p;
            }
        } else {
            while t<=n {
                let bound=rootint(n/t,k-1);
                if r>bound { break; }
                count+=count_rec(k-1,n,t,r,bound,j+1);
                if t>n/p { break; }
                t*=p;
            }
        }
        j+=1; p=r;
    }
    count
}

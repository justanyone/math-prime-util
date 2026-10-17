use super::sqrtmod::{crt, prime_power};
use crate::factor_exp;

pub(super) const MAX_ROOTS: u64 = 600_000_000;
pub(super) fn check_count(n: u64) {
    assert!(n <= MAX_ROOTS, "Maximum returned roots exceeded");
}
pub(super) fn combine(left: &[u64], n: u64, right: &[u64], m: u64) -> Vec<u64> {
    check_count(left.len() as u64 * right.len() as u64);
    left.iter().flat_map(|&r| right.iter().map(move |&s| crt(r,n,s,m))).collect()
}
// rootmod.c::_allsqrtmodpk.
pub(super) fn roots_pk(a: u64, p: u64, k: u32) -> Vec<u64> {
    let pk = p.pow(k);
    if a % p == 0 {
        if a % pk == 0 {
            let low = p.pow(k/2);
            let high = if k % 2 == 1 { low*p } else { low };
            check_count(low);
            return (0..low).map(|i| high*i).collect();
        }
        if (a/p) % p != 0 { return vec![]; }
        let roots = roots_pk(a/p/p,p,k-2);
        check_count(roots.len() as u64 * p);
        return roots.iter().flat_map(|&r| (0..p).map(move |j| r*p+j*(pk/p))).collect();
    }
    let Some(q) = prime_power(a,p,k) else { return vec![]; };
    if p == 2 && k == 1 { return vec![q]; }
    if p != 2 || k == 2 { return vec![q,pk-q]; }
    let q2 = super::mulmod_u(q,pk/p-1,pk);
    vec![q,pk-q,q2,pk-q2]
}

/// Return all square roots in ascending order. Return an empty list if none exist.
/// Port of MPU 0.75 `rootmod.c::allsqrtmod` and `_allsqrtmodpk`.
///
/// # Panics
/// Panics if the result exceeds MPU's limit of 600 million roots.
///
/// ```
/// use math_prime_util::allsqrtmod;
/// assert_eq!(allsqrtmod(1, 8), vec![1, 3, 5, 7]);
/// ```
pub fn allsqrtmod(a: u64, n: u64) -> Vec<u64> {
    if n == 0 { return vec![]; }
    let a = a % n;
    if n <= 2 { return vec![a]; }
    let (mut roots,mut modulus) = (vec![0],1);
    for (p,e) in factor_exp(n) {
        let m = p.pow(e);
        roots = combine(&roots,modulus,&roots_pk(a,p,e),m);
        if roots.is_empty() { break; }
        modulus *= m;
    }
    roots.sort_unstable();
    roots
}

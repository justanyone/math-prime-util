#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
use super::almost_prime_count_upper::{max_count,max_nth,FIRST_3};
use crate::{nth_prime,prime_count_upper};
// MPU inverse_interpolate.c: _linear_interpolate and _inverse_interpolate.
fn linear(lo: u64, hi: u64, rlo: u64, rhi: u64, n: u64) -> u64 {
    if n == rlo {
        return lo + 1;
    }
    if n == rhi {
        return hi - 1;
    }
    let span = hi - lo;
    let movement = ((n - rlo) as f64 * span as f64 / (rhi - rlo) as f64 + 0.5) as u64;
    lo + movement.clamp(1, span - 1)
}
pub(super) fn inverse(mut lo: u64, mut hi: u64, n: u64, f: impl Fn(u64) -> u64) -> u64 {
    if hi==0 {
        let mut rlo=f(lo);
        if rlo==n {return lo;}
        loop {
            let factor=(n as f64/rlo as f64-0.004).clamp(1.004,8.0);
            let mid=if u64::MAX as f64/lo as f64<=factor {u64::MAX} else {(factor*lo as f64+1.0) as u64};
            let rmid=f(mid);
            if rmid>=n {hi=mid;break;}
            lo=mid;rlo=rmid;
            if lo==u64::MAX {return 0;}
        }
    }
    let mut mid = lo + (hi - lo) / 2;
    let rmid = f(mid);
    let (mut rlo, mut rhi);
    if rmid >= n {
        hi = mid;
        rhi = rmid;
        rlo = f(lo);
        if rlo == n {
            return lo;
        }
    } else {
        lo = mid;
        rlo = rmid;
        rhi = f(hi);
    }
    if hi - lo <= 1 {
        return if rlo == n || rhi > n { lo } else { hi };
    }
    mid = linear(lo, hi, rlo, rhi, n);
    let mut iteration = 1;
    while hi - lo > 1 && rhi > n {
        let rmid = f(mid);
        if rmid >= n {
            hi = mid;
            rhi = rmid;
        } else {
            lo = mid;
            rlo = rmid;
        }
        if rhi == n || hi - lo <= 1 {
            break;
        }
        mid = linear(lo, hi, rlo, rhi, n);
        if iteration % 6 == 0 {
            let close = (0.003 * (hi - lo) as f64 + 1.0) as u64;
            if lo + close > mid {
                mid = lo + close;
            } else if hi - close < mid {
                mid = hi - close;
            }
        }
        mid = mid.clamp(lo + 1, hi - 1);
        iteration += 1;
    }
    if rlo == n {
        return lo;
    }
    if hi - lo <= 1 {
        return if rlo == n || rhi > n { lo } else { hi };
    }
    while hi - lo > 8 {
        let mut x1 = lo + (hi - lo) / 2;
        let mut rx1 = f(x1);
        let d0 = (n - rlo) as f64;
        let d1 = rx1.abs_diff(n) as f64;
        let d2 = (rhi - n + 1) as f64;
        let sign = if rx1 >= n { 1.0 } else { -1.0 };
        let pos = sign * (x1 - lo) as f64 * d1 / (d1 * d1 + d0 * d2).sqrt();
        let movement = (pos + if pos >= 0.0 { 0.5 } else { -0.5 }) as i128;
        let candidate = i128::from(x1) - movement;
        if candidate <= i128::from(lo) || candidate >= i128::from(hi) || candidate == i128::from(x1)
        {
            if rx1 >= n {
                hi = x1;
                rhi = rx1;
            } else {
                lo = x1;
                rlo = rx1;
            }
        } else {
            let mut x3 = candidate as u64;
            let mut rx3 = f(x3);
            if rx1 > rx3 {
                std::mem::swap(&mut x1, &mut x3);
                std::mem::swap(&mut rx1, &mut rx3);
            }
            if rx1 >= n {
                hi = x1;
                rhi = rx1;
            } else if rx3 >= n {
                lo = x1;
                rlo = rx1;
                hi = x3;
                rhi = rx3;
            } else {
                lo = x3;
                rlo = rx3;
            }
        }
    }
    while hi - lo > 1 {
        mid = lo + (hi - lo) / 2;
        if f(mid) < n {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    hi
}


// MPU almost_primes.c: reduce_nth_factor and _fast_small_nth_almost_prime.
pub(super) fn reduction(k:u32,n:u64)->u32 {
 if k<=1 || k>=64 {return 0;}
 let mut r=0;
 if k>49 {if n>=FIRST_3[49] {return 0;} r=k-49+1;}
 while n<FIRST_3[(k-r) as usize] {r+=1;}
 r
}
pub(super) fn small(k:u32,n:u64)->u64 {
 if k==2 {[0,4,6,9,10,14,15,21][n as usize]} else {[0,8,12,18,20,27,28,30][n as usize]<<(k-3)}
}
// MPU prime_counts.c: nth_prime_lower and simple_nth_limits.
pub(super) fn prime_lower(n:u64)->u64 {
 if n<306 {return nth_prime(n).unwrap_or(0);}
 if n>=max_count(1) {return if n==max_count(1) {max_nth(1)} else {0};}
 let x=n as f64;let l=x.ln();let ll=l.ln();
 if n<2000000 {
 let a=if n<228 {0.6483} else if n<948 {0.8032} else if n<2195 {0.8800} else if n<39017 {0.9019} else {0.9484};
 let mut lo=(x*(l+ll-1.0+(ll-2.10)/l)) as u64;let mut hi=(x*(l+ll-a)) as u64;
 while lo<hi {let mid=lo+(hi-lo)/2;if prime_count_upper(mid)<n {lo=mid+1;} else {hi=mid;}} return lo;
 }
 let b=if n<56000000 {11.200} else {11.50800000002};
 (x*(l+ll-1.0+(ll-2.0)/l-(ll*ll-6.0*ll+b)/(2.0*l*l))).ceil().min(max_nth(1) as f64) as u64
}
/// Return a lower bound for the `n`th integer with `k` prime factors.
///
/// Port of MPU 0.75 `almost_primes.c`, `nth_almost_prime_lower`.
/// Return `None` for index zero or a value outside the native range.
///
/// ```
/// assert_eq!(math_prime_util::nth_almost_prime_lower(3, 4), Some(20));
/// ```
#[must_use]
pub fn nth_almost_prime_lower(k:u32,n:u64)->Option<u64> {
 if n==0 {return None;} if k==0 {return (n==1).then_some(1);} if k==1 {return Some(prime_lower(n));}
 let max=max_count(k);if n>=max {return (n==max && max>0).then(||max_nth(k));}
 if n<8 {return Some(small(k,n));}
 let r=reduction(k,n);if r>0 {return nth_almost_prime_lower(k-r,n).map(|v|v<<r);}
 Some(inverse(3_u64<<k,0,n,|x|super::almost_prime_count_upper(k,x)))
}

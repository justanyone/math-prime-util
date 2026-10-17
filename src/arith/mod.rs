//! Totients and multiplicative arithmetic from MPU 0.75.

use crate::{gcd, is_prime, rootint, sqrtint};
use std::collections::HashMap;

fn factors(mut n: u64) -> Vec<(u64, u32)> {
    fn split(n: u64, out: &mut Vec<u64>) {
        if n <= 1 { return; }
        if is_prime(n) != 0 { out.push(n); return; }
        let mut c = 1_u64;
        loop {
            let step = |x: u64| u64::try_from((u128::from(x) * u128::from(x) + u128::from(c)) % u128::from(n)).unwrap();
            let (mut x, mut y, mut d) = (2, 2, 1);
            for _ in 0..200_000 {
                x = step(x); y = step(step(y)); d = gcd(x.abs_diff(y), n);
                if d != 1 { break; }
            }
            if d > 1 && d < n { split(d, out); split(n / d, out); return; }
            c += 1;
        }
    }
    if n < 2 { return Vec::new(); }
    let mut flat = Vec::new();
    for p in [2,3,5,7,11,13,17,19,23,29,31,37] {
        while n % p == 0 { flat.push(p); n /= p; }
    }
    split(n, &mut flat);
    flat.sort_unstable();
    let mut result: Vec<(u64,u32)> = Vec::new();
    for p in flat {
        if let Some((q,e)) = result.last_mut() { if *q == p { *e += 1; continue; } }
        result.push((p,1));
    }
    result
}

/// Return Euler's totient. Port of `totients.c::totient` and `_totient_fac`.
/// ```
/// assert_eq!(math_prime_util::euler_phi(36), 12);
/// ```
pub fn euler_phi(n: u64) -> u64 {
    factors(n).into_iter().fold(n, |v,(p,_)| v / p * (p-1))
}

fn range_len(lo: u64, hi: u64) -> usize {
    usize::try_from(hi.checked_sub(lo).and_then(|v| v.checked_add(1)).expect("invalid or oversized range")).expect("range fits usize")
}

/// Return totients on an inclusive interval. Port of `totients.c::range_totient`.
/// # Panics
/// Panics if the interval is reversed or too large to allocate.
/// ```
/// assert_eq!(math_prime_util::euler_phi_range(0, 4), [0,1,1,2,2]);
/// ```
pub fn euler_phi_range(lo: u64, hi: u64) -> Vec<u64> {
    let len = range_len(lo,hi);
    if lo == 0 {
        let mut t = vec![0;len];
        if hi == 0 { return t; }
        t[1] = 1;
        let mut primes = Vec::new();
        for i in (1..=hi/2).step_by(2) {
            let ix = usize::try_from(i).unwrap();
            if t[ix] == 0 { t[ix] = i-1; if i <= sqrtint(hi) { primes.push(i); } }
            let v = t[ix];
            for &p in &primes {
                if p > hi/i { break; }
                let j = usize::try_from(i*p).unwrap();
                if i % p == 0 { t[j] = v*p; break; }
                t[j] = v*(p-1);
            }
            t[ix*2] = v;
            if i < hi/2 { t[ix*2+2] = t[ix+1]*2; }
        }
        for i in (1..len).step_by(2) { if t[i] == 0 { t[i] = u64::try_from(i).unwrap()-1; } }
        t
    } else {
        // Ranged factoring: sieve small factors, then factor each remaining cofactor.
        let mut rem: Vec<u64> = (lo..=hi).collect();
        let mut t = rem.clone();
        for p in crate::sieve::primes(2, sqrtint(hi).min(1_000_000)) {
            let offset = (p-lo%p)%p;
            if offset > hi-lo { continue; }
            for i in (usize::try_from(offset).unwrap()..len).step_by(usize::try_from(p).unwrap()) {
                t[i] = t[i]/p*(p-1);
                while rem[i]%p == 0 { rem[i] /= p; }
            }
        }
        for (r,v) in rem.into_iter().zip(&mut t) { for (p,_) in factors(r) { *v = *v/p*(p-1); } }
        t
    }
}

/// Return the Jordan totient, or `None` on overflow.
/// Port of `totients.c::jordan_totient`; checked powers also cover the Perl fallback.
/// ```
/// assert_eq!(math_prime_util::jordan_totient(2, 12), Some(96));
/// ```
pub fn jordan_totient(k: u64, n: u64) -> Option<u64> {
    if k == 0 || n <= 1 { return Some(u64::from(n == 1)); }
    if n == 2 { return 2_u64.checked_pow(u32::try_from(k).ok()?).map(|v|v-1); }
    let k = u32::try_from(k).ok()?;
    let mut v = 1_u64;
    for (p,e) in factors(n) {
        let pk = p.checked_pow(k)?;
        v = v.checked_mul(pk-1)?;
        for _ in 1..e { v = v.checked_mul(pk)?; }
    }
    Some(v)
}

/// Return Carmichael's lambda. Port of `totients.c::carmichael_lambda`.
/// ```
/// assert_eq!(math_prime_util::carmichael_lambda(24), 2);
/// ```
pub fn carmichael_lambda(n: u64) -> u64 {
    if n < 8 { return [0,1,1,2,2,4,2,6][usize::try_from(n).unwrap()]; }
    factors(n).into_iter().fold(1, |v,(p,e)| {
        let mut term = p.pow(e-1)*(p-1);
        if p == 2 && e > 2 { term /= 2; }
        v / gcd(v,term) * term
    })
}

/// Return the Moebius function. Port of `moebius.c::moebius`.
/// ```
/// assert_eq!(math_prime_util::moebius(30), -1);
/// ```
pub fn moebius(n: u64) -> i8 {
    const SMALL: [u32;16] = [2703565065,23406865,620863913,1630114197,157354249,2844895525,2166423889,363177345,2835441929,2709852521,1095049497,92897577,1772687649,162113833,160497957,689538385];
    if n < 256 { return i8::try_from((SMALL[usize::try_from(n/16).unwrap()] >> (2*(n%16))) & 3).unwrap()-1; }
    for p in [2,3,5,7,11,13,17,19,23,29,31,37] { if n % (p*p) == 0 { return 0; } }
    let f = factors(n);
    if f.iter().any(|&(_,e)|e>1) { 0 } else if f.len()%2 == 0 { 1 } else { -1 }
}

/// Return Moebius values on an inclusive interval.
/// Port of `moebius.c::range_moebius`, `_sieve_moebius`, and `_moebius_by_hand`.
/// # Panics
/// Panics if the interval is reversed or too large to allocate.
/// ```
/// assert_eq!(math_prime_util::moebius_range(1, 4), [1,-1,-1,0]);
/// ```
pub fn moebius_range(lo: u64, hi: u64) -> Vec<i8> {
    let len = range_len(lo,hi);
    let width = hi-lo;
    if hi < 100 || width < 10 || (hi > 1<<25 && width < rootint(hi,3)/4) {
        return (lo..=hi).map(moebius).collect();
    }
    let mut mu = vec![0_u8;len];
    let (mut logp, mut nextlog) = (1_u8,3_u64);
    let root = sqrtint(hi);
    for p in crate::sieve::primes(2, root + u64::from(root*root != hi)) {
        if p > nextlog { logp += 2; nextlog = (nextlog-1)*4+1; }
        let start = ((p-lo%p)%p).max(if lo == 0 {p} else {0});
        for i in (usize::try_from(start).unwrap()..len).step_by(usize::try_from(p).unwrap()) { mu[i] = mu[i].wrapping_add(logp); }
        let p2 = p*p;
        let start = ((p2-lo%p2)%p2).max(if lo == 0 {p2} else {0});
        for i in (usize::try_from(start).unwrap()..len).step_by(usize::try_from(p2).unwrap()) { mu[i] = 128; }
    }
    mu.into_iter().enumerate().map(|(i,a)| {
        let n = lo+u64::try_from(i).unwrap();
        if n == 0 || a & 128 != 0 { 0 } else {
            let sign = if a & 1 == 0 {1} else {-1};
            if u32::from(a) >= n.ilog2() {sign} else {-sign}
        }
    }).collect()
}

struct Mertens { prefix: Vec<i64>, cache: HashMap<u64,i64> }
impl Mertens {
    fn new(n: u64) -> Self {
        let limit = (rootint(n,3).pow(2)).max(sqrtint(n)).max(16);
        let mut prefix = vec![0];
        let mut sum = 0;
        for m in moebius_range(1,limit) { sum += i64::from(m); prefix.push(sum); }
        Self {prefix, cache: HashMap::new()}
    }
    fn get(&mut self,n:u64) -> i64 {
        if let Ok(i) = usize::try_from(n) { if let Some(&v) = self.prefix.get(i) { return v; } }
        if let Some(&v) = self.cache.get(&n) {return v;}
        let s = sqrtint(n);
        let ns = n/(s+1);
        let mut sum = 1-i128::from(n/2)-i128::from(n&1);
        for k in 2..=ns {
            sum -= i128::from(self.get(n/k)) + i128::from(self.prefix[usize::try_from(k).unwrap()])*i128::from(n/k-n/(k+1));
        }
        if s > ns { sum -= i128::from(self.prefix[usize::try_from(s).unwrap()])*i128::from(n/s-n/(s+1)); }
        let sum = i64::try_from(sum).unwrap();
        self.cache.insert(n,sum); sum
    }
}

/// Return the sum of Moebius values through `n`.
/// Port of `moebius.c::mertens` and `_rmertens`.
/// ```
/// assert_eq!(math_prime_util::mertens(10), -1);
/// ```
pub fn mertens(n:u64) -> i64 { Mertens::new(n).get(n) }

/// Return Liouville's function. Port of `moebius.c::liouville`.
/// ```
/// assert_eq!(math_prime_util::liouville(12), -1);
/// ```
pub fn liouville(n:u64) -> i8 {
    if n == 0 {return -1;}
    if factors(n).iter().map(|&(_,e)|e).sum::<u32>()%2 == 0 {1} else {-1}
}

/// Sum Liouville's function through `n`. Port of `moebius.c::sumliouville`.
/// ```
/// assert_eq!(math_prime_util::sumliouville(10), 0);
/// ```
pub fn sumliouville(n:u64) -> i64 {
    let mut ctx = Mertens::new(n);
    let mut sum = 0;
    for k in 1..=sqrtint(n) {sum += ctx.get(n/(k*k));}
    sum
}

/// Return the prime base of a prime power, or one.
/// Port of `util.c::exp_mangoldt`.
/// ```
/// assert_eq!(math_prime_util::exp_mangoldt(27), 3);
/// ```
pub fn exp_mangoldt(n:u64) -> u64 {
    let f = factors(n); if f.len() == 1 {f[0].0} else {1}
}

/// Sum Euler's totient through `n`, or return `None` on native overflow.
/// Port of `totients.c::sumtotient` and `_sumt`.
/// ```
/// assert_eq!(math_prime_util::sumtotient(10), Some(32));
/// ```
pub fn sumtotient(n:u64) -> Option<u64> {
    fn sumt(n:u64, prefix:&[u64],cache:&mut HashMap<u64,u64>) -> u64 {
        if let Ok(i) = usize::try_from(n) {if let Some(&v) = prefix.get(i) {return v;}}
        if let Some(&v) = cache.get(&n) {return v;}
        let s = sqrtint(n); let lim = n/(s+1);
        let mut v = u128::from(n)*u128::from(n+1)/2-u128::from(n-n/2);
        for k in 2..=lim {
            v -= u128::from(sumt(n/k,prefix,cache));
            v -= u128::from(n/k-n/(k+1))*u128::from(sumt(k,prefix,cache));
        }
        if s > lim {v -= u128::from(n/s-n/(s+1))*u128::from(sumt(s,prefix,cache));}
        let v = u64::try_from(v).unwrap(); cache.insert(n,v); v
    }
    if n > 7_790_208_950 {return None;}
    let mut prefix = euler_phi_range(0,rootint(n,3).pow(2).max(2));
    for i in 2..prefix.len() {prefix[i] += prefix[i-1];}
    Some(sumt(n,&prefix,&mut HashMap::new()))
}

/// Return Dedekind's psi, or `None` on overflow.
/// Port of `totients.c::dedekind_psi`.
/// ```
/// assert_eq!(math_prime_util::dedekind_psi(12), Some(24));
/// ```
pub fn dedekind_psi(n:u64) -> Option<u64> {
    if n == 0 {return Some(0);}
    let mut v = 1_u64;
    for (p,e) in factors(n) {v = v.checked_mul(p.checked_add(1)?)?.checked_mul(p.pow(e-1))?;}
    Some(v)
}

fn divisors(n:u64) -> Vec<u64> {
    let mut d = vec![1];
    for (p,e) in factors(n) {
        let len = d.len(); let mut power = 1;
        for _ in 0..e {power *= p; for i in 0..len {d.push(d[i]*power);}}
    }
    d.sort_unstable(); d
}

/// List all native integers with totient `n`.
/// Port of `totients.c::inverse_totient_list`; returns an empty list if none exist.
/// ```
/// assert_eq!(math_prime_util::inverse_totient(4), [5,8,10,12]);
/// ```
pub fn inverse_totient(n:u64) -> Vec<u64> {
    if n == 1 {return vec![1,2];}
    if n == 0 || n%2 == 1 {return Vec::new();}
    let divs = divisors(n);
    let mut sets: HashMap<u64,Vec<u64>> = HashMap::from([(1,vec![1])]);
    for &d in divs.iter().rev() {
        let Some(p) = d.checked_add(1) else {continue;};
        if is_prime(p) == 0 {continue;}
        let (mut dp,mut pp) = (d,p);
        let mut additions: HashMap<u64,Vec<u64>> = HashMap::new();
        loop {
            let nd = n/dp;
            for &d2 in &divs {
                if d2 > nd {break;}
                if nd%d2 != 0 || (d == 1 && d2*dp != n) {continue;}
                if let Some(vals) = sets.get(&d2) {
                    for &v in vals {if let Some(v) = v.checked_mul(pp) {additions.entry(d2*dp).or_default().push(v);}}
                }
            }
            if nd%p != 0 {break;}
            dp *= p;
            let Some(next) = pp.checked_mul(p) else {break;}; pp = next;
        }
        for (k,v) in additions {sets.entry(k).or_default().extend(v);}
    }
    let mut result = sets.remove(&n).unwrap_or_default(); result.sort_unstable(); result.dedup(); result
}

fn totpred(n:u64,maxd:u64) -> bool {
    if n&1 == 1 {return false;}
    if n.is_power_of_two() {return true;}
    let n = n/2;
    if n < maxd && n.checked_mul(2).and_then(|v|v.checked_add(1)).is_some_and(|p|is_prime(p)!=0) {return true;}
    for d in divisors(n) {
        if d >= maxd {break;}
        let Some(p) = d.checked_mul(2).and_then(|v|v.checked_a
KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 1350 bytes)

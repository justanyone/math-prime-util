#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
use crate::{logarithmic_integral, nth_prime, prime_count, primes};

pub(super) const SMALL: [u64; 23] = [
    2, 11, 17, 29, 41, 47, 59, 67, 71, 97, 101, 107, 127, 149, 151, 167, 179, 181,
    227, 229, 233, 239, 241,
];

/// Return the Ramanujan primes in the inclusive interval.
///
/// Port of MPU 0.75 `ramanujan_primes.c`, `ramanujan_primes` and
/// `n_range_ramanujan_primes`. This uses Noe's sieve.
///
/// ```
/// assert_eq!(math_prime_util::ramanujan_primes(10, 30), vec![11, 17, 29]);
/// ```
pub fn ramanujan_primes(low: u64, high: u64) -> Vec<u64> {
    if high < 2 || low > high {
        return Vec::new();
    }
    if high <= 241 {
        return SMALL.into_iter().filter(|&p| p >= low && p <= high).collect();
    }
    let first = super::ramanujan_prime_count_lower(low).max(1);
    // R_n lies between p_(2n) and p_(3n).
    let last = prime_count(high) / 2 + 1;
    range(first, last).into_iter().filter(|&p| p >= low && p <= high).collect()
}

// MPU 0.75 ramanujan_primes.c::n_range_ramanujan_primes.
pub(super) fn range(first: u64, last: u64) -> Vec<u64> {
    if first > last {
        return Vec::new();
    }
    if last <= 23 {
        return SMALL[(first - 1) as usize..last as usize].to_vec();
    }
    let mut result = vec![0; usize::try_from(last - first + 1).expect("Ramanujan sieve size")];
    for (i, &p) in SMALL[..2].iter().enumerate() {
        let n = i as u64 + 1;
        if n >= first && n <= last { result[(n - first) as usize] = p; }
    }
    let mut start = if first == 1 { 7 } else { nth_bound(first, false).saturating_sub(1).max(15) };
    if start % 2 == 0 { start -= 1; }
    let end = nth_bound(last, true).saturating_add(1);
    let ps = primes(start, end);
    let halves = primes(start.div_ceil(2), end.div_ceil(2));
    let mut pi = 0;
    let mut qi = 0;
    let mut s = if first == 1 { 1 } else { 1 + prime_count(start - 2) - prime_count((start - 1) / 2) };
    // The monolithic C version indexes L[s], the range version indexes L[s-first].
    if first == 1 { s = 1; }
    for k in (start..=end).step_by(2) {
        if ps.get(pi) == Some(&k) { s += 1; pi += 1; }
        if s >= first && s <= last { result[(s - first) as usize] = k + 1; }
        if k & 3 == 1 && halves.get(qi) == Some(&k.div_ceil(2)) { s -= 1; qi += 1; }
        if s >= first && s <= last { result[(s - first) as usize] = k + 2; }
    }
    result
}

fn axler(n: u64, c: f64, d: f64) -> u64 {
    let x = n as f64;
    let a = 2_f64.ln();
    let l = x.ln();
    let ll = l.ln();
    let c1 = 2.0*a*a+a+c;
    let c2 = a*a*a+2.0*a*a+d;
    let u = (a*l*ll*ll-c1*l*ll+c2*l-a*a*ll+a*a*a+a*a)
        / (l*l*l*l+l*l*l*ll-l*l*l*a-l*l*a);
    (2.0*x*(1.0+a/l-(a*ll-a*a-a)/(l*l)+u)) as u64
}

pub(super) fn nth_bound(n: u64, upper: bool) -> u64 {
    if n <= 23 { return if n == 0 { 0 } else { SMALL[(n-1) as usize] }; }
    let (c,d) = if upper {
        if n < 12581 {
            let (c,d) = select(n, &[(168,-4.7691,-6.2682),(2290,-0.9315,-0.5635),(5225,-0.5318,-0.0710),(12581,0.1212,0.7973)]);
            return prime_nth_bound(axler(n,c,d), true);
        }
        let (_,mut d) = select(n,&[(18175,0.0,0.3548),(82883,0.0,-0.2450),(316314,0.0,-0.6384),(1000001,0.0,-0.9353),(4000001,0.0,-1.1271),(16000001,0.0,-1.4152),(64000001,0.0,-1.6671),(128000001,0.0,-1.8855),(256000001,0.0,-1.9325),(384000 <<cut 66 chars by kit>>
        for (t,v) in [(3999654659,-2.235),(84086679236,-2.435),(514808375201,-2.535),(3594243587299,-2.635),(28330126673435,-2.735),(117462814829787,-2.8)] { if n > t { d=v; } }
        (0.0,d)
    } else {
        if n < 34816 {
            let (c,d) = select(n,&[(189,4.2720,0.340),(1245,-0.2179,-6.179),(2984,0.1446,-4.8693),(14303,-0.3570,-5.1154),(34816,-1.5770,-7.5332)]);
            return prime_nth_bound(axler(n,c,d),false);
        }
        let (_,mut d)=select(n,&[(76400,0.0,0.0126),(280816,0.0,0.5132),(915887,0.0,0.9967),(4000001,0.0,1.5004),(16000001,0.0,1.7184),(64000001,0.0,1.9860),(128000001,0.0,2.1352),(256000001,0.0,2.1658),(384000001,0.0,2.1999),(512000001,0.0 <<cut 56 chars by kit>>
        for (t,v) in [(14888378285,2.29),(467037926604,2.31),(2778491401197,2.315),(10656144781918,2.317),(63698770351741,2.319)] { if n>t { d=v; } }
        (1.472,d)
    };
    prime_nth_bound(axler(n,c,d),upper)
}

fn select(n:u64, table:&[(u64,f64,f64)]) -> (f64,f64) {
    let &(_,c,d)=table.iter().find(|&&(t,_,_)| n<t).unwrap_or(&table[table.len()-1]);
    (c,d)
}

// MPU 0.75 prime_counts.c::nth_prime_upper / nth_prime_lower.
fn prime_nth_bound(n:u64, upper:bool) -> u64 {
    if n<306 { return nth_prime(n).unwrap_or(0); }
    const MAX_INDEX:u64=425656284035217743;
    const MAX_PRIME:u64=18446744073709551557;
    if n>=MAX_INDEX { return if n==MAX_INDEX { MAX_PRIME } else { 0 }; }
    let x=n as f64;
    let l=x.ln(); let ll=l.ln();
    if n < if upper {15360} else {2000000} {
        let a=if n<948 {0.8032} else if n<2195 {0.8800} else if n<39017 {0.9019} else {0.9484};
        let mut lo=(x*(l+ll-1.0+(ll-2.10)/l)) as u64;
        let mut hi=(x*(l+ll-a)) as u64;
        while lo<hi {
            let mid=lo+(hi-lo)/2;
            if count_bound(mid,!upper)<n {lo=mid+1;} else {hi=mid;}
        }
        return lo;
    }
    let (c,d)=if upper {
        select(n,&[(25070,4.80,-51.5928),(31490,11.89,-197.8951),(39016,1.49,15.1821),(85820,2.76,-11.5918),(138630,2.07,5.2103),(301130,2.01,7.2842),(382440,5.92,-91.3415),(467650,2.92,-15.4768),(575750,0.93,37.1597),(688382,-0.79,83.5215) <<cut 160 chars by kit>>
    } else { (2.0,if n<56000000 {11.200} else {11.50800000002}) };
    let v=x*(l+ll-1.0+(ll-c)/l-(ll*ll-6.0*ll+d)/(2.0*l*l));
    if upper { (v.floor() as u64).min(MAX_PRIME) } else { (v.ceil() as u64).min(MAX_PRIME) }
}

// MPU 0.75 prime_counts.c::prime_count_lower / prime_count_upper.
pub(super) fn count_bound(n:u64, upper:bool)->u64 {
    if n<33000 {return prime_count(n);}
    let x=n as f64; let l=x.ln(); let l2=l*l;
    let v=if upper {
        if n<=821800000 {
            let (a,_)=select(n,&[(59000,2.48_f32 as f64,0.0),(355991,2.54_f32 as f64,0.0),(3550000,2.51_f32 as f64,0.0),(3560000,2.49_f32 as f64,0.0),(5000000,2.48_f32 as f64,0.0),(8000000,2.47_f32 as f64,0.0),(13000000,2.46_f32 as f64,0.0) <<cut 456 chars by kit>>
            x/l*(1.0+1.0/l+a/l2)
        } else {
            let a=if n<1100000000 {0.032} else if n<10010000000 {0.027} else if n<101260000000 {0.021} else {0.0};
            logarithmic_integral(x)-a*l*x.sqrt()/(8.0*std::f64::consts::PI)
        }
    } else if n<=300070 {
        let (a,_)=select(n,&[(69720,905.0,0.0),(70120,961.0,0.0),(88800,918.2,0.0),(176000,887.7,0.0),(299270,839.46,0.0),(300071,846.66,0.0)]);
        x/(l-1.0-1.0/l-2.975666/l2-13.024334/(l*l2)+a/(l2*l2))
    } else {
        let (a,_)=select(n,&[(303000,5.0,0.0),(1100000,-7.0,0.0),(4500000,-37.0,0.0),(10200000,-70.0,0.0),(36900000,-53.0,0.0),(38100000,-29.0,0.0),(4000000000,-84.0,0.0)]);
        logarithmic_integral(x)-x.sqrt()/l*if n<4000000000 {1.94+2.50/l+a/l2} else {2.0}
    };
    if upper { v.floor() as u64 } else { v.ceil() as u64 }
}

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 640 bytes)

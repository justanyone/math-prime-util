use super::nth_ramanujan_prime_lower::{SMALL, axler, prime_bound};

// MPU ramanujan_primes.c, nth_ramanujan_prime_upper.
pub(super) fn upper(n: u64) -> u64 {
    if n <= 23 {return if n == 0 {0} else {SMALL[(n-1) as usize]};}
    let (c,d) = if n < 12_581 {
        match n {0..168 => (-4.7691,-6.2682), 168..2290 => (-0.9315,-0.5635),
            2290..5225 => (-0.5318,-0.0710), _ => (0.1212,0.7973)}
    } else {
        (0.0, match n {
            0..18175 => 0.3548, 18175..82883 => -0.2450,
            82883..316314 => -0.6384, 316314..1000001 => -0.9353,
            1000001..4000001 => -1.1271, 4000001..16000001 => -1.4152,
            16000001..64000001 => -1.6671, 64000001..128000001 => -1.8855,
            128000001..256000001 => -1.9325, 256000001..384000001 => -2.0190,
            384000001..512000001 => -2.0310, 512000001..=3999654659 => -2.0884,
            3999654660..=84086679236 => -2.235, 84086679237..=514808375201 => -2.435,
            514808375202..=3594243587299 => -2.535, 3594243587300..=28330126673435 => -2.635,
            28330126673436..=117462814829787 => -2.735, _ => -2.8,
        })
    };
    prime_bound(axler(n,c,d),true)
}

/// Return the nth Ramanujan prime. Return `None` for index zero.
///
/// Port of MPU 0.75 `ramanujan_primes.c`, `nth_ramanujan_prime` and
/// `n_range_ramanujan_primes`. Use Noe's sieve between the index bounds.
///
/// ```
/// assert_eq!(math_prime_util::nth_ramanujan_prime(100), Some(1439));
/// assert_eq!(math_prime_util::nth_ramanujan_prime(0), None);
/// ```
///
/// # Panics
/// Panics if the sieve range cannot fit in memory.
pub fn nth_ramanujan_prime(n: u64) -> Option<u64> {
    if n == 0 { return None; }
    if n <= 23 {return Some(SMALL[(n-1) as usize]);}
    let mut low = super::nth_ramanujan_prime_lower(n)? - 1;
    let high = upper(n).checked_add(1)?;
    low = low.max(15);
    if low % 2 == 0 { low -= 1; }
    let full = crate::primes(low, high);
    let half = crate::primes((low+1)/2, high.div_ceil(2));
    let mut i = 0;
    let mut j = 0;
    let mut s = 1+crate::prime_count(low-2)-crate::prime_count((low-1)/2);
    let mut result = 0;
    for k in (low..=high).step_by(2) {
        if full.get(i) == Some(&k) {s += 1; i += 1;}
        if s == n {result = k+1;}
        if k & 3 == 1 {
            let h = (k+1)/2;
            while half.get(j).is_some_and(|&p| p < h) {j += 1;}
            if half.get(j) == Some(&h) {s -= 1;}
        }
        if s == n {result = k+2;}
    }
    Some(result)
}

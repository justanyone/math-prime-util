//! Prime families in short intervals of arbitrary precision.
//!
//! Factor-based lists return the unresolved cofactor if factoring fails.
//!
//! These MPU names stay native (`u64` only) because MPU's own pure-Perl path
//! is impractical for inputs past 2^64: `is_lucky`, `lucky_numbers`,
//! `is_ramanujan_prime`, `ramanujan_primes` (they sieve or scan every value up
//! to the argument), and `powerful_numbers` (it generates every powerful
//! number up to the high end). [`goldbach_pairs`] and [`goldbach_pair_count`]
//! have a form here, as MPU's pure-Perl path answers odd `n` of any size at
//! once; for even `n` past 2^64 it tests every prime from `n/2` to `n - 3` and
//! does not finish, and so does this form.
use num_bigint::BigUint;
use std::io::{self, Write};

use super::{arith, factor, primality};

fn scan(
    low: &BigUint,
    high: &BigUint,
    mut keep: impl FnMut(&BigUint) -> Result<bool, BigUint>,
) -> Result<Vec<BigUint>, BigUint> {
    let mut out = Vec::new();
    let mut n = low.clone();
    while n <= *high {
        if keep(&n)? {
            out.push(n.clone());
        }
        n += 1_u8;
    }
    Ok(out)
}

/// Return primes in the inclusive interval. Use MPU `sieve.c::sieve_segment`
/// and the short bigint trial path in `PP.pm::primes`.
/// ```
/// use math_prime_util::bigint::families::primes;
/// assert_eq!(primes(&2_u8.into(), &5_u8.into()).len(), 3);
/// ```
#[must_use]
pub fn primes(low: &BigUint, high: &BigUint) -> Vec<BigUint> {
    let mut out = Vec::new();
    if high < &BigUint::from(2_u8) || low > high {
        return out;
    }
    let mut p = if low <= &BigUint::from(2_u8) {
        2_u8.into()
    } else {
        primality::next_prime(&(low - 1_u8))
    };
    while p <= *high {
        out.push(p.clone());
        p = primality::next_prime(&p);
    }
    out
}

/// Write one prime per line. Port of MPU `util.c::print_primes`.
/// # Errors
/// Return the writer error if output fails.
/// ```
/// use math_prime_util::bigint::families::print_primes;
/// let mut out = Vec::new();
/// print_primes(&2_u8.into(), &3_u8.into(), &mut out).unwrap();
/// assert_eq!(out, b"2\n3\n");
/// ```
pub fn print_primes(low: &BigUint, high: &BigUint, mut writer: impl Write) -> io::Result<()> {
    for p in primes(low, high) {
        writeln!(writer, "{p}")?;
    }
    Ok(())
}

/// Return lower members of twin pairs. The upper member may exceed `high`.
/// Port of MPU `twin_primes.c::twin_primes`.
/// ```
/// use math_prime_util::bigint::families::twin_primes;
/// assert_eq!(twin_primes(&3_u8.into(), &5_u8.into()).len(), 2);
/// ```
#[must_use]
pub fn twin_primes(low: &BigUint, high: &BigUint) -> Vec<BigUint> {
    primes(low, high)
        .into_iter()
        .filter(|p| primality::is_prob_prime(&(p + 2_u8)) != 0)
        .collect()
}

/// Return prime cluster starts. Port of MPU `sieve.c::sieve_prime_cluster`.
/// # Panics
/// Panic if offsets are odd or not strictly increasing.
/// ```
/// use math_prime_util::bigint::families::sieve_prime_cluster;
/// assert_eq!(sieve_prime_cluster(&3_u8.into(), &5_u8.into(), &[2]).len(), 2);
/// ```
#[must_use]
pub fn sieve_prime_cluster(low: &BigUint, high: &BigUint, offsets: &[u64]) -> Vec<BigUint> {
    assert!(
        offsets.iter().all(|x| x % 2 == 0) && offsets.windows(2).all(|w| w[0] < w[1]),
        "offsets must be even and increasing"
    );
    primes(low, high)
        .into_iter()
        .filter(|p| {
            offsets
                .iter()
                .all(|&d| primality::is_prob_prime(&(p + d)) != 0)
        })
        .collect()
}

/// Return offsets with no prime divisor at most `depth`, except the prime itself.
/// The interval is inclusive. Port of MPU `sieve.c::sieve_range`.
/// ```
/// use math_prime_util::bigint::families::sieve_range;
/// assert_eq!(sieve_range(&10_u8.into(), &14_u8.into(), &3_u8.into()).len(), 2);
/// ```
#[must_use]
pub fn sieve_range(low: &BigUint, high: &BigUint, depth: &BigUint) -> Vec<BigUint> {
    let divisors = primes(&2_u8.into(), depth);
    let mut out = Vec::new();
    let mut n = low.clone().max(2_u8.into());
    while n <= *high {
        if divisors
            .iter()
            .all(|p| n == *p || &n % p != BigUint::from(0_u8))
        {
            out.push(&n - low);
        }
        n += 1_u8;
    }
    out
}

/// Return numbers with `k` prime factors counted with repetition.
/// Port of MPU `almost_primes.c::generate_almost_primes` and its trial path.
/// # Errors
/// Return an unresolved cofactor if factoring fails.
/// ```
/// use math_prime_util::bigint::families::almost_primes;
/// assert_eq!(almost_primes(2, &4_u8.into(), &6_u8.into()).unwrap().len(), 2);
/// ```
pub fn almost_primes(k: u64, low: &BigUint, high: &BigUint) -> Result<Vec<BigUint>, BigUint> {
    scan(low, high, |n| {
        if *n == BigUint::from(0_u8) {
            return Ok(false);
        }
        Ok(factor::factor(n)?.len() as u64 == k)
    })
}

/// Return semiprimes. Port of MPU `semi_primes.c::semi_primes`.
/// # Errors
/// Return an unresolved cofactor if factoring fails.
/// ```
/// use math_prime_util::bigint::families::semi_primes;
/// assert_eq!(semi_primes(&4_u8.into(), &6_u8.into()).unwrap().len(), 2);
/// ```
pub fn semi_primes(low: &BigUint, high: &BigUint) -> Result<Vec<BigUint>, BigUint> {
    almost_primes(2, low, high)
}

/// Return numbers with `k` distinct prime factors.
/// Port of MPU `omega_primes.c::omega_primes`, trial path.
/// # Errors
/// Return an unresolved cofactor if factoring fails.
/// ```
/// use math_prime_util::bigint::families::omega_primes;
/// assert_eq!(omega_primes(2, &6_u8.into(), &6_u8.into()).unwrap().len(), 1);
/// ```
pub fn omega_primes(k: u32, low: &BigUint, high: &BigUint) -> Result<Vec<BigUint>, BigUint> {
    scan(low, high, |n| {
        if *n == BigUint::from(0_u8) {
            return Ok(false);
        }
        Ok(factor::factor_exp(n)?.len() as u64 == u64::from(k))
    })
}

fn prime_power(n: &BigUint) -> bool {
    if primality::is_prob_prime(n) != 0 {
        return true;
    }
    let e = arith::is_power_exp(n);
    e > 1 && primality::is_prob_prime(&arith::rootint(n, e)) != 0
}

/// Return prime powers. Port of MPU `prime_powers.c::next_prime_power` and
/// `PP.pm::prime_powers`, using trial tests for short bigint intervals.
/// ```
/// use math_prime_util::bigint::families::prime_powers;
/// assert_eq!(prime_powers(&4_u8.into(), &9_u8.into()).len(), 5);
/// ```
#[must_use]
pub fn prime_powers(low: &BigUint, high: &BigUint) -> Vec<BigUint> {
    let mut out = Vec::new();
    let mut n = low.clone();
    while n <= *high {
        if prime_power(&n) {
            out.push(n.clone());
        }
        n += 1_u8;
    }
    out
}

/// Return the next prime power. Port of MPU `prime_powers.c::next_prime_power`
/// and the trial loop in `PP.pm::next_prime_power`.
/// ```
/// use math_prime_util::bigint::families::next_prime_power;
/// assert_eq!(next_prime_power(&5_u8.into()), 7_u8.into());
/// ```
#[must_use]
pub fn next_prime_power(n: &BigUint) -> BigUint {
    let mut n = n + 1_u8;
    while !prime_power(&n) {
        n += 1_u8;
    }
    n
}

/// Return the previous prime power, if one exists.
/// Port of MPU `prime_powers.c::prev_prime_power` and its PP trial loop.
/// ```
/// use math_prime_util::bigint::families::prev_prime_power;
/// assert_eq!(prev_prime_power(&5_u8.into()), Some(4_u8.into()));
/// ```
#[must_use]
pub fn prev_prime_power(n: &BigUint) -> Option<BigUint> {
    if n <= &2_u8.into() {
        return None;
    }
    let mut n = n - 1_u8;
    while !prime_power(&n) {
        n -= 1_u8;
    }
    Some(n)
}

/// Return the next perfect power. Port of MPU
/// `perfect_powers.c::_next_perfect_power`.
/// ```
/// use math_prime_util::bigint::families::next_perfect_power;
/// assert_eq!(next_perfect_power(&9_u8.into()), 16_u8.into());
/// ```
#[must_use]
pub fn next_perfect_power(n: &BigUint) -> BigUint {
    if n == &0_u8.into() {
        return 1_u8.into();
    }
    if n == &1_u8.into() {
        return 4_u8.into();
    }
    let mut best = (arith::rootint(n, 2) + 1_u8).pow(2);
    for k in 3..=u32::try_from(n.bits()).unwrap_or(u32::MAX) {
        best = best.min((arith::rootint(n, k) + 1_u8).pow(k));
    }
    best
}

/// Return the previous unsigned perfect power, if one exists.
/// Port of MPU `perfect_powers.c::_prev_perfect_power`.
/// ```
/// use math_prime_util::bigint::families::prev_perfect_power;
/// assert_eq!(prev_perfect_power(&9_u8.into()), Some(8_u8.into()));
/// ```
#[must_use]
pub fn prev_perfect_power(n: &BigUint) -> Option<BigUint> {
    if n == &0_u8.into() {
        return None;
    }
    if n <= &4_u8.into() {
        return Some(BigUint::from(u8::from(n > &1_u8.into())));
    }
    let m = n - 1_u8;
    let mut best = BigUint::from(1_u8);
    for k in 2..=u32::try_from(m.bits()).unwrap_or(u32::MAX) {
        best = best.max(arith::rootint(&m, k).pow(k));
    }
    Some(best)
}

/// Return the least Goldbach pair. Port of MPU
/// `goldbach.c::minimal_goldbach_pair`, by ascending prime trials.
/// ```
/// use math_prime_util::bigint::families::minimal_goldbach_pair;
/// assert_eq!(minimal_goldbach_pair(&10_u8.into()), Some(3_u8.into()));
/// ```
#[must_use]
pub fn minimal_goldbach_pair(n: &BigUint) -> Option<BigUint> {
    if n < &4_u8.into() {
        return None;
    }
    if n % 2_u8 != BigUint::from(0_u8) {
        let q = n - 2_u8;
        return (primality::is_prob_prime(&q) != 0).then(|| 2_u8.into());
    }
    let mut p = BigUint::from(2_u8);
    while p <= (n >> 1_usize) {
        let q = n - &p;
        if primality::is_prob_prime(&q) != 0 {
            return Some(p);
        }
        p = primality::next_prime(&p);
    }
    None
}

/// Visit the smaller member of each Goldbach pair of `n`, in ascending order.
/// Port of `PP.pm::goldbach_pairs`: an odd `n` (or 4) has the one pair
/// `2 + (n - 2)` when `n - 2` is prime; an even `n` tests `n - p` for every
/// prime `p` from 3 to `n/2`. PP walks the larger members from `n/2` up, which
/// visits the same pairs.
fn visit_goldbach(n: &BigUint, mut visit: impl FnMut(&BigUint)) {
    if n < &4_u8.into() {
        return;
    }
    if n.bit(0) || n == &4_u8.into() {
        if primality::is_prob_prime(&(n - 2_u8)) != 0 {
            visit(&2_u8.into());
        }
        return;
    }
    let half = n >> 1_usize;
    let mut p = BigUint::from(3_u8);
    while p <= half {
        if primality::is_prob_prime(&(n - &p)) != 0 {
            visit(&p);
        }
        p = primality::next_prime(&p);
    }
}

/// Count the unordered prime pairs whose sum is `n`.
///
/// Up to `u64::MAX` the native [`goldbach_pair_count`](crate::goldbach_pair_count)
/// answers. Above it this is `PP.pm::goldbach_pair_count`: an odd `n` counts
/// one pair when `n - 2` is prime, at once; an even `n` tests every prime from
/// `n/2` to `n - 3`, which does not finish in practical time past 2^64 (MPU's
/// pure-Perl path does the same).
/// ```
/// use math_prime_util::bigint::families::goldbach_pair_count;
/// use num_bigint::BigUint;
/// assert_eq!(goldbach_pair_count(&22_u8.into()), 3_u8.into());
/// // 2^64 + 13 is prime, so 2^64 + 15 = 2 + (2^64 + 13).
/// let n = (BigUint::from(1_u8) << 64_usize) + 15_u8;
/// assert_eq!(goldbach_pair_count(&n), 1_u8.into());
/// ```
#[must_use]
pub fn goldbach_pair_count(n: &BigUint) -> BigUint {
    if let Ok(small) = u64::try_from(n) {
        return crate::goldbach_pair_count(small).into();
    }
    let mut count = BigUint::from(0_u8);
    visit_goldbach(n, |_| count += 1_u8);
    count
}

/// Return the smaller primes of all Goldbach pairs of `n`, in ascending order.
/// Return an empty list if no pair exists.
///
/// Up to `u64::MAX` the native [`goldbach_pairs`](crate::goldbach_pairs)
/// answers. Above it this is `PP.pm::goldbach_pairs`: an odd `n` gives `[2]`
/// when `n - 2` is prime and `[]` otherwise, at once; an even `n` tests every
/// prime from `n/2` to `n - 3`, which does not finish in practical time past
/// 2^64 (MPU's pure-Perl path does the same).
/// ```
/// use math_prime_util::bigint::families::goldbach_pairs;
/// use num_bigint::BigUint;
/// assert_eq!(goldbach_pairs(&22_u8.into()), [3_u8, 5, 11].map(BigUint::from));
/// let n = (BigUint::from(1_u8) << 64_usize) + 15_u8;
/// assert_eq!(goldbach_pairs(&n), [BigUint::from(2_u8)]);
/// assert!(goldbach_pairs(&(n + 2_u8)).is_empty());
/// ```
#[must_use]
pub fn goldbach_pairs(n: &BigUint) -> Vec<BigUint> {
    if let Ok(small) = u64::try_from(n) {
        return crate::goldbach_pairs(small)
            .into_iter()
            .map(BigUint::from)
            .collect();
    }
    let mut pairs = Vec::new();
    visit_goldbach(n, |p| pairs.push(p.clone()));
    pairs
}

/// Return the next Chen prime. Port of MPU `almost_primes.c::next_chen_prime`.
/// # Errors
/// Return an unresolved cofactor if factoring fails.
/// ```
/// use math_prime_util::bigint::families::next_chen_prime;
/// assert_eq!(next_chen_prime(&7_u8.into()).unwrap(), 11_u8.into());
/// ```
pub fn next_chen_prime(n: &BigUint) -> Result<BigUint, BigUint> {
    let mut p = primality::next_prime(n);
    loop {
        let q = &p + 2_u8;
        if primality::is_prob_prime(&q) != 0 || factor::factor(&q)?.len() == 2 {
            return Ok(p);
        }
        p = primality::next_prime(&p);
    }
}

#[cfg(test)]
mod tests {
    use super::visit_goldbach;
    use num_bigint::BigUint;

    /// The trial path that inputs past `u64::MAX` take, against the native sieve.
    #[test]
    fn goldbach_trial_path_matches_native() {
        for n in 0_u64..3000 {
            let mut pairs = Vec::new();
            visit_goldbach(&n.into(), |p| pairs.push(p.clone()));
            let native: Vec<BigUint> = crate::goldbach_pairs(n)
                .into_iter()
                .map(BigUint::from)
                .collect();
            assert_eq!(pairs, native, "goldbach_pairs({n})");
        }
    }
}

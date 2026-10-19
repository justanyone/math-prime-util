//! Random integers and probable primes from MPU's thread-local seeded stream.

use num_bigint::BigUint;

use super::primality::{is_prob_prime, is_strong_pseudoprime, next_prime, prev_prime};

/// Draws at most `bits` random bits.
/// Port of `reference/mpu/csprng.c::urandomb` and `PP.pm::urandomb`.
/// # Panics
/// Panics if the byte count does not fit in memory.
/// ```
/// use math_prime_util::bigint::random::urandomb;
/// assert!(urandomb(100).bits() <= 100);
/// ```
#[must_use]
pub fn urandomb(bits: u32) -> BigUint {
    if bits <= 64 {
        return crate::urandomb(bits).into();
    }
    let bytes =
        crate::random_bytes(usize::try_from(bits.div_ceil(8)).expect("byte count fits usize"));
    BigUint::from_bytes_be(&bytes) >> ((8 - bits % 8) % 8)
}

/// Draws uniformly below a positive modulus.
/// Port of `reference/mpu/csprng.c::urandomm64` and `PP.pm::urandomm`.
/// # Panics
/// Panics if the modulus is zero.
/// ```
/// use math_prime_util::bigint::random::urandomm;
/// assert!(urandomm(&100_u8.into()) < 100_u8.into());
/// ```
#[must_use]
pub fn urandomm(modulus: &BigUint) -> BigUint {
    assert!(*modulus != BigUint::from(0_u8), "modulus must be positive");
    if let Ok(n) = u64::try_from(modulus) {
        return crate::urandomm(n).into();
    }
    // PP estimates log2(n) from its decimal length, then adds a full byte.
    let digits = modulus.to_str_radix(10).len();
    let bytes = (digits * 3322 / 1000 + 15) >> 3;
    let bits = u32::try_from(bytes * 8).expect("bit count fits u32");
    let max = (BigUint::from(1_u8) << bits) - 1_u8;
    let overflow = &max - &max % modulus;
    loop {
        let r = urandomb(bits);
        if r < overflow {
            return r % modulus;
        }
    }
}

/// Draws uniformly from an inclusive range. An empty range returns `None`.
/// Port of `reference/mpu/XS.xs::urandomr` and `PP.pm::urandomr`.
/// ```
/// use math_prime_util::bigint::random::urandomr;
/// assert_eq!(urandomr(&7_u8.into(), &7_u8.into()), Some(7_u8.into()));
/// ```
#[must_use]
pub fn urandomr(low: &BigUint, high: &BigUint) -> Option<BigUint> {
    if let (Some(l), Some(h)) = (u64::try_from(low).ok(), u64::try_from(high).ok()) {
        return crate::urandomr(l, h).map(BigUint::from);
    }
    (low <= high).then(|| low + urandomm(&(high - low + 1_u8)))
}

/// Draws a probable prime from an inclusive range.
/// Port of `reference/mpu/random_prime.c::random_prime` and
/// `RandomPrimes.pm::random_prime`. Returns `None` if the range has no prime.
/// ```
/// use math_prime_util::bigint::random::random_prime;
/// assert_eq!(random_prime(&14_u8.into(), &17_u8.into()), Some(17_u8.into()));
/// ```
#[must_use]
pub fn random_prime(low: &BigUint, high: &BigUint) -> Option<BigUint> {
    if let (Some(l), Some(h)) = (u64::try_from(low).ok(), u64::try_from(high).ok()) {
        return crate::random_prime(l, h).map(BigUint::from);
    }
    if low > high || high < &2_u8.into() {
        return None;
    }
    let mut lo = low.max(&2_u8.into()).clone();
    let mut hi = high.clone();
    if high - low <= 1_000_000_000_u64.into() {
        lo = next_prime(&(&lo - 1_u8));
        hi = prev_prime(&(&hi + 1_u8))?;
        if lo > hi {
            return None;
        }
    }
    if lo == BigUint::from(2_u8) {
        lo -= 1_u8;
    } else if !lo.bit(0) {
        lo += 1_u8;
    }
    if !hi.bit(0) {
        hi -= 1_u8;
    }
    let mut range = ((&hi - &lo) >> 1_usize) + 1_u8;
    // RandomPrimes.pm partitions intervals wider than a native parameter.
    if range > BigUint::from(u64::MAX) {
        let nbins = ceil_div(&range, &(BigUint::from(1_u8) << 32_usize));
        let binsize = ceil_div(&range, &nbins);
        let mut nparts = &range / &binsize;
        if &nparts * &binsize == range {
            nparts -= 1_u8;
        }
        let part = urandomm(&(&nparts + 1_u8));
        lo += (&binsize * &part) << 1_usize;
        range = if part < nparts {
            binsize
        } else {
            range - nparts * binsize
        };
    }
    loop {
        let n = &lo + (urandomm(&range) << 1_usize);
        if n == BigUint::from(1_u8) {
            return Some(2_u8.into());
        }
        if is_prob_prime(&n) != 0 {
            return Some(n);
        }
    }
}

/// Draws a probable prime with exactly `bits` bits.
/// Port of `reference/mpu/random_prime.c::random_nbit_prime` and
/// `RandomPrimes.pm::random_nbit_prime` (Fouque and Tibouchi algorithm 1).
/// # Panics
/// Panics if `bits < 2`.
/// ```
/// use math_prime_util::bigint::random::random_nbit_prime;
/// assert_eq!(random_nbit_prime(512).expect("prime").bits(), 512);
/// ```
#[must_use]
pub fn random_nbit_prime(bits: u32) -> Option<BigUint> {
    assert!(bits >= 2, "bits must be >= 2");
    if bits <= 64 {
        return crate::random_nbit_prime(bits).map(BigUint::from);
    }
    let l = if bits > 79 { 63 } else { 31 };
    loop {
        let b = (urandomb(bits - l - 2) << 1_usize) + 1_u8;
        for _ in 0..1_000_000 {
            let a = (BigUint::from(1_u8) << l) + urandomb(l);
            let p = (a << (bits - l - 1)) + &b;
            if is_prob_prime(&p) != 0 {
                return Some(p);
            }
        }
    }
}

/// Draws a probable prime with exactly `digits` decimal digits.
/// Port of `reference/mpu/random_prime.c::random_ndigit_prime` and
/// `RandomPrimes.pm::random_ndigit_prime`.
/// # Panics
/// Panics if `digits == 0`.
/// ```
/// use math_prime_util::bigint::random::random_ndigit_prime;
/// assert_eq!(random_ndigit_prime(21).expect("prime").to_string().len(), 21);
/// ```
#[must_use]
pub fn random_ndigit_prime(digits: u32) -> Option<BigUint> {
    assert!(digits > 0, "digits must be positive");
    if digits < 20 {
        return crate::random_ndigit_prime(digits).map(BigUint::from);
    }
    let lo = BigUint::from(10_u8).pow(digits - 1);
    random_prime(&(&lo + 1_u8), &(lo * 10_u8 - 1_u8))
}

/// Draws a safe probable prime `p`, where `(p-1)/2` is also prime.
/// Port of `reference/mpu/random_prime.c::random_safe_prime` and
/// `RandomPrimes.pm::_random_safe_prime_large`.
/// # Panics
/// Panics if `bits < 3`.
/// ```
/// use math_prime_util::bigint::random::random_safe_prime;
/// assert_eq!(random_safe_prime(65).expect("prime").bits(), 65);
/// ```
#[must_use]
pub fn random_safe_prime(bits: u32) -> Option<BigUint> {
    assert!(bits >= 3, "bits must be >= 3");
    if bits <= 64 {
        return crate::random_safe_prime(bits).map(BigUint::from);
    }
    let base = (BigUint::from(1_u8) << (bits - 1)) + 3_u8 + (urandomb(bits - 35) << 2_usize);
    loop {
        let mut p = &base + (urandomb(32) << (bits - 33));
        let mut q = &p >> 1_usize;
        let rem = (&q + 1_u8) % 6_u8;
        if rem != BigUint::from(0_u8) {
            q -= rem;
            if q.bits() != u64::from(bits - 1) {
                q += 12_u8;
            }
            p = (&q << 1_usize) + 1_u8;
        }
        if is_prob_prime(&q) != 0 && is_prob_prime(&p) != 0 {
            return Some(p);
        }
    }
}

/// Draws a Gordon strong probable prime.
/// Port of `reference/mpu/lib/Math/Prime/Util/RandomPrimes.pm::random_strong_prime`.
/// # Panics
/// Panics if `bits < 128`.
/// ```
/// use math_prime_util::bigint::random::random_strong_prime;
/// assert_eq!(random_strong_prime(128).expect("prime").bits(), 128);
/// ```
#[must_use]
pub fn random_strong_prime(bits: u32) -> Option<BigUint> {
    assert!(bits >= 128, "bits must be >= 128");
    let l = bits.div_ceil(2) - 2;
    loop {
        let qp = random_nbit_prime(bits / 2 - 20)?;
        let qpp = random_nbit_prime(l - 20)?;
        let qp2 = &qp << 1_usize;
        let qpp2 = qpp << 1_usize;
        let il = ceil_div(&((BigUint::from(1_u8) << (l - 1)) - 1_u8), &qpp2);
        let iu = ((BigUint::from(2_u8) << l) - 2_u8) / &qpp2;
        let mut i = &il + urandomm(&(&iu - &il + 1_u8));
        while i <= iu {
            let q = &i * &qpp2 + 1_u8;
            i += 1_u8;
            if is_prob_prime(&q) == 0 {
                continue;
            }
            let step = &q * &qp2;
            let pp = &qp2 * qp.modpow(&(&q - 2_u8), &q) - 1_u8;
            let jl = ceil_div(&((BigUint::from(1_u8) << (bits - 1)) - &pp), &step);
            let ju = ((BigUint::from(1_u8) << bits) - &pp - 1_u8) / &step;
            let mut j = &jl + urandomm(&(&ju - &jl + 1_u8));
            while j <= ju {
                let p = &j * &step + &pp;
                if is_prob_prime(&p) != 0 {
                    return Some(p);
                }
                j += 1_u8;
            }
        }
    }
}

/// Draws a product of two primes with exactly `bits` bits.
/// Port of `reference/mpu/random_prime.c::random_semiprime` and `PP.pm::random_semiprime`.
/// # Panics
/// Panics if `bits < 4`.
/// ```
/// use math_prime_util::bigint::random::random_semiprime;
/// assert_eq!(random_semiprime(65).expect("semiprime").bits(), 65);
/// ```
#[must_use]
pub fn random_semiprime(bits: u32) -> Option<BigUint> {
    assert!(bits >= 4, "bits must be >= 4");
    if bits <= 64 {
        return crate::random_semiprime(bits).map(BigUint::from);
    }
    loop {
        let n = random_nbit_prime(bits / 2)? * random_nbit_prime(bits - bits / 2)?;
        if n.bits() == u64::from(bits) {
            return Some(n);
        }
    }
}

/// Draws a uniform positive integer at most `n` and its sorted prime factors.
/// Port of `reference/mpu/util.c::random_factored_integer` and `PP.pm::random_factored_integer`.
/// # Panics
/// Panics if `n` is zero.
/// ```
/// use math_prime_util::bigint::random::random_factored_integer;
/// let (n, f) = random_factored_integer(&(num_bigint::BigUint::from(1_u8) << 65));
/// assert_eq!(f.into_iter().product::<num_bigint::BigUint>(), n);
/// ```
#[must_use]
pub fn random_factored_integer(n: &BigUint) -> (BigUint, Vec<BigUint>) {
    assert!(*n != BigUint::from(0_u8), "n must be positive");
    if let Ok(n) = u64::try_from(n) {
        let (r, f) = crate::random_factored_integer(n);
        return (r.into(), f.into_iter().map(BigUint::from).collect());
    }
    loop {
        let mut s = n.clone();
        let mut factors = Vec::new();
        while s > BigUint::from(1_u8) {
            s = urandomm(&s) + 1_u8;
            if s > BigUint::from(1_u8) && is_prob_prime(&s) != 0 {
                factors.push(s.clone());
            }
        }
        let r: BigUint = factors.iter().product();
        if &r <= n && urandomm(n) < r {
            factors.sort_unstable();
            return (r, factors);
        }
    }
}

/// Tests random Miller-Rabin bases. Returns 1 on success and 0 on failure.
/// Port of `reference/mpu/random_prime.c::is_mr_random` and `PP.pm::miller_rabin_random`.
/// # Panics
/// Panics if `bases` is zero.
/// ```
/// use math_prime_util::bigint::random::miller_rabin_random;
/// assert_eq!(miller_rabin_random(&31_u8.into(), 5), Some(1));
/// ```
#[must_use]
pub fn miller_rabin_random(n: &BigUint, bases: u64) -> Option<u8> {
    assert!(bases > 0, "bases must be positive");
    if let Ok(n) = u64::try_from(n) {
        return crate::miller_rabin_random(u128::from(n), bases);
    }
    for _ in 0..bases {
        let base = urandomm(&(n - 2_u8)) + 2_u8;
        if !is_strong_pseudoprime(n, &[base]) {
            return Some(0);
        }
    }
    Some(1)
}

/// Draws a semiprime with no balance rule for its two prime factors.
/// Port of `reference/mpu/random_prime.c::random_unrestricted_semiprime`
/// and `PP.pm::random_unrestricted_semiprime`.
/// # Panics
/// Panics if `bits < 3`.
/// ```
/// use math_prime_util::bigint::random::random_unrestricted_semiprime;
/// assert_eq!(random_unrestricted_semiprime(65).expect("semiprime").bits(), 65);
/// ```
#[must_use]
#[allow(clippy::cast_precision_loss)] // PP uses double precision for its weights.
pub fn random_unrestricted_semiprime(bits: u32) -> Option<BigUint> {
    const WEIGHTS: &[(u32, f64)] = &[
        (2, 1.912_183_974_522_43),
        (3, 1.339_548_265_550_21),
        (5, 0.854_756_717_114_822),
        (7, 0.635_492_301_836_862),
        (11, 0.426_616_792_046_787),
        (13, 0.368_193_843_118_344),
        (17, 0.290_512_701_603_111),
        (19, 0.263_359_264_658_156),
        (23, 0.222_406_328_935_102),
        (29, 0.181_229_250_520_242),
        (31, 0.170_874_199_059_434),
        (37, 0.146_112_155_735_473),
        (41, 0.133_427_839_963_585),
        (43, 0.127_929_010_905_662),
        (47, 0.118_254_609_086_782),
        (53, 0.106_316_418_106_489),
        (59, 0.096_698_967_543_864_3),
        (61, 0.093_883_365_800_854_7),
        (67, 0.086_415_182_315_167_1),
        (71, 0.082_082_295_318_829_7),
        (73, 0.080_096_441_634_074_6),
        (79, 0.074_706_091_483_334_4),
        (83, 0.071_497_370_665_485_1),
        (89, 0.067_211_546_843_628_4),
        (97, 0.062_281_889_248_619_1),
        (101, 0.060_085_589_154_993_9),
        (103, 0.059_061_357_001_540_7),
        (107, 0.057_092_113_562_697_6),
        (109, 0.056_169_166_764_148_5),
        (113, 0.054_433_014_108_187_4),
        (127, 0.049_062_020_431_570_1),
    ];

    assert!(bits >= 3, "bits must be >= 3");
    if bits <= 64 {
        return crate::random_unrestricted_semiprime(bits).map(BigUint::from);
    }
    let min = BigUint::from(1_u8) << (bits - 1);
    let max = (&min << 1_usize) - 1_u8;
    let mut r = crate::drand(1.0);
    let mut selected = None;
    for &(p, weight) in WEIGHTS {
        let probability = weight / f64::from(bits) + 0.195_56 / f64::from(p);
        if r <= probability {
            selected = Some(BigUint::from(p));
            break;
        }
        r -= probability;
    }
    let p = if let Some(p) = selected {
        p
    } else {
        let mertens = 0.261_497_212_847_642_8;
        let weight = mertens + (f64::from(bits) * std::f64::consts::LN_2 / 2.0).ln();
        let lower = 131_f64.ln().ln();
        let r = loop {
            let r = crate::drand(weight) - mertens;
            if r >= lower {
                break r;
            }
        };
        let a = if r <= 3.54 {
            float_integer((r.exp().exp() + 0.5).floor())?
        } else {
            let re = r.exp();
            let exponent = (re / 34.5).floor() + 1.0;
            let exponent = format!("{exponent:.0}").parse::<u32>().ok()?;
            float_integer(((re / f64::from(exponent)).exp() + 0.5).floor())?.pow(exponent)
        };
        if a < 2_u8.into() {
            2_u8.into()
        } else {
            prev_prime(&(a + 1_u8))?
        }
    };
    let q = random_prime(&ceil_div(&min, &p), &(&max / &p))?;
    Some(p * q)
}

fn ceil_div(n: &BigUint, d: &BigUint) -> BigUint {
    (n + d - 1_u8) / d
}

fn float_integer(n: f64) -> Option<BigUint> {
    format!("{n:.0}").parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_helpers() {
        assert_eq!(ceil_div(&7_u8.into(), &3_u8.into()), 3_u8.into());
        assert_eq!(float_integer(1234.0), Some(1234_u32.into()));
        assert_eq!(float_integer(f64::INFINITY), None);
        for seed in 0..24 {
            crate::srand(seed);
            let n = random_unrestricted_semiprime(256).expect("semiprime");
            assert_eq!(n.bits(), 256);
        }
    }
}

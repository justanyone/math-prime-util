//! Arbitrary precision implementations of the native MPU algorithms.
use super::Int;
use num_bigint::BigUint;

fn zero() -> BigUint {
    BigUint::from(0_u8)
}

fn one() -> BigUint {
    BigUint::from(1_u8)
}

/// Euclidean Jacobi reduction, from MPU `util.c::jacobi_iu`.
fn jacobi(mut a: BigUint, mut n: BigUint) -> i8 {
    let mut sign = 1;
    a %= &n;
    while a != zero() {
        while !a.bit(0) {
            a >>= 1;
            let residue = &n % 8_u8;
            if residue == BigUint::from(3_u8) || residue == BigUint::from(5_u8) {
                sign = -sign;
            }
        }
        std::mem::swap(&mut a, &mut n);
        if &a % 4_u8 == BigUint::from(3_u8) && &n % 4_u8 == BigUint::from(3_u8) {
            sign = -sign;
        }
        a %= &n;
    }
    if n == one() { sign } else { 0 }
}

fn half(mut value: BigUint, n: &BigUint) -> BigUint {
    if value.bit(0) {
        value += n;
    }
    value >> 1
}

/// Base-two strong test, MPU `primality.c::is_strong_pseudoprime`.
fn strong_base_two(n: &BigUint) -> bool {
    let nm1 = n - 1_u8;
    let mut d = nm1.clone();
    let mut s = 0;
    while !d.bit(0) {
        d >>= 1;
        s += 1;
    }
    let mut x = BigUint::from(2_u8).modpow(&d, n);
    if x == one() || x == nm1 {
        return true;
    }
    for _ in 1..s {
        x = (&x * &x) % n;
        if x == nm1 {
            return true;
        }
        if x == one() {
            return false;
        }
    }
    false
}

/// Extra-strong Lucas test, MPU `primality.c::is_lucas_pseudoprime`
/// (strength 3), including `select_extra_strong_parameters`.
fn extra_strong_lucas(n: &BigUint) -> bool {
    let root = n.sqrt();
    if &root * &root == *n {
        return false;
    }
    let mut p = BigUint::from(3_u8);
    let discriminant = loop {
        let d = &p * &p - 4_u8;
        match jacobi(d.clone(), n.clone()) {
            -1 => break d,
            0 if d.gcd(n) != *n => return false,
            _ => p += 1_u8,
        }
    };
    let mut d = n + 1_u8;
    let mut s = 0;
    while !d.bit(0) {
        d >>= 1;
        s += 1;
    }
    let (mut u, mut v) = (one(), p.clone());
    for bit in (0..d.bits() - 1).rev() {
        u = (&u * &v) % n;
        v = (&v * &v + n - 2_u8) % n;
        if d.bit(bit) {
            let next_u = half((&p * &u + &v) % n, n);
            v = half((&discriminant * &u + &p * &v) % n, n);
            u = next_u;
        }
    }
    let two = BigUint::from(2_u8);
    if u == zero() && (v == two || v == n - 2_u8) {
        return true;
    }
    for _ in 1..s {
        if v == zero() {
            return true;
        }
        v = (&v * &v + n - 2_u8) % n;
        if v == two {
            return false;
        }
    }
    false
}

fn difference(a: &BigUint, b: &BigUint) -> BigUint {
    if a >= b { a - b } else { b - a }
}

/// Pollard-Brent, MPU `factor.c::pbrent_factor` (non-Montgomery path).
/// A finite work budget preserves the trait's unsplit-cofactor error contract.
fn brent(n: &BigUint, mut rounds: usize) -> Option<BigUint> {
    let (mut x, mut anchor) = (one(), one());
    let (mut r, mut failures, mut a) = (1_usize, 6_u8, 1_u32);
    while rounds > 0 {
        let mut left = r.min(rounds);
        let mut saved = x.clone();
        let mut f = one();
        while left > 0 {
            let batch = left.min(160);
            left -= batch;
            rounds -= batch;
            saved.clone_from(&x);
            let mut product = one();
            for _ in 0..batch {
                x = (&x * &x + a) % n;
                product = (product * difference(&x, &anchor)) % n;
            }
            f = product.gcd(n);
            if f != one() {
                break;
            }
        }
        if f == one() {
            r *= 2;
            anchor.clone_from(&x);
            continue;
        }
        if f == *n {
            x = saved;
            loop {
                x = (&x * &x + a) % n;
                f = difference(&x, &anchor).gcd(n);
                if f != one() || r == 0 {
                    break;
                }
                r -= 1;
            }
        }
        if f == zero() || f == *n {
            if failures == 0 {
                break;
            }
            failures -= 1;
            anchor = (anchor + 11_u8) % n;
            x.clone_from(&anchor);
            a += 1;
            continue;
        }
        return Some(f);
    }
    None
}

impl Int for BigUint {
    /// MPU `primality.c::BPSW` and `PP.pm::_is_prime7`: base-two MR and
    /// extra-strong Lucas above the native range.
    ///
    /// ```
    /// use math_prime_util::Int;
    /// use num_bigint::BigUint;
    /// assert_eq!(((BigUint::from(1_u8) << 521_usize) - 1_u8).is_prime(), 1);
    /// ```
    fn is_prime(&self) -> u8 {
        if let Ok(n) = u128::try_from(self) {
            return n.is_prime();
        }
        for p in [
            2_u8, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53,
        ] {
            if self % p == zero() {
                return 0;
            }
        }
        u8::from(strong_base_two(self) && extra_strong_lucas(self))
    }

    /// MPU `factor.c::factor`: trial division, prime and square checks, then Brent.
    ///
    /// ```
    /// use math_prime_util::Int;
    /// use num_bigint::BigUint;
    /// assert_eq!(BigUint::from(12_u8).factor().unwrap(),
    ///            vec![BigUint::from(2_u8), BigUint::from(2_u8), BigUint::from(3_u8)]);
    /// ```
    fn factor(&self) -> Result<Vec<Self>, Self> {
        let mut pending = vec![self.clone()];
        let mut factors = Vec::new();
        while let Some(mut n) = pending.pop() {
            if let Ok(native) = u128::try_from(&n) {
                factors.extend(
                    native
                        .factor()
                        .map_err(Self::from)?
                        .into_iter()
                        .map(Self::from),
                );
                continue;
            }
            for p in [
                2_u8, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53,
            ] {
                while &n % p == zero() {
                    factors.push(Self::from(p));
                    n /= p;
                }
            }
            if n == one() {
                continue;
            }
            if n.is_prime() != 0 {
                factors.push(n);
                continue;
            }
            let root = n.sqrt();
            let f = if &root * &root == n {
                root
            } else {
                brent(&n, 20_000).ok_or_else(|| n.clone())?
            };
            pending.push(&n / &f);
            pending.push(f);
        }
        factors.sort_unstable();
        Ok(factors)
    }

    /// MPU `mulmod.h::powmod`, using arbitrary precision modular exponentiation.
    ///
    /// ```
    /// use math_prime_util::Int;
    /// use num_bigint::BigUint;
    /// let n = BigUint::from(7_u8);
    /// assert_eq!(BigUint::from(3_u8).powmod(&BigUint::from(5_u8), &n), Some(BigUint::from(5_u8)));
    /// ```
    fn powmod(&self, exponent: &Self, modulus: &Self) -> Option<Self> {
        (modulus != &zero()).then(|| self.modpow(exponent, modulus))
    }

    /// Euclidean reduction, MPU `util.h::gcd_ui`.
    ///
    /// ```
    /// use math_prime_util::Int;
    /// use num_bigint::BigUint;
    /// assert_eq!(BigUint::from(12_u8).gcd(&BigUint::from(18_u8)), BigUint::from(6_u8));
    /// ```
    fn gcd(&self, other: &Self) -> Self {
        let (mut a, mut b) = (self.clone(), other.clone());
        while b != zero() {
            let remainder = &a % &b;
            a = b;
            b = remainder;
        }
        a
    }

    /// Candidate search, MPU `util.c::next_prime` and `PP.pm::next_prime`.
    ///
    /// ```
    /// use math_prime_util::Int;
    /// use num_bigint::BigUint;
    /// assert_eq!(BigUint::from(14_u8).next_prime(), Some(BigUint::from(17_u8)));
    /// ```
    fn next_prime(&self) -> Option<Self> {
        const ADVANCE: [u8; 30] = [
            1, 6, 5, 4, 3, 2, 1, 4, 3, 2, 1, 2, 1, 4, 3, 2, 1, 2, 1, 4, 3, 2, 1, 6, 5, 4, 3, 2, 1,
            2,
        ];
        if let Ok(n) = u64::try_from(self) {
            if let Some(p) = n.next_prime() {
                return Some(Self::from(p));
            }
        }
        let mut n = self.clone();
        loop {
            // The remainder is at most 29 on every supported architecture.
            let residue = usize::try_from(&n % 30_u8).unwrap_or_default();
            n += ADVANCE[residue];
            if &n % 7_u8 != zero() && n.is_prime() != 0 {
                return Some(n);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn mersenne(bits: usize) -> BigUint {
        (one() << bits) - 1_u8
    }

    #[test]
    fn arbitrary_precision_primality() {
        for bits in [127, 521, 607] {
            let p = mersenne(bits);
            assert_ne!(p.is_prime(), 0);
            assert!(strong_base_two(&p));
            assert!(extra_strong_lucas(&p));
            assert_eq!((&p * 3_u8).is_prime(), 0);
            assert_eq!((&p * &p).is_prime(), 0);
            assert!(!extra_strong_lucas(&(&p * &p)));
        }
        // Strong base-two pseudoprimes must still fail the Lucas test.
        for n in [2047_u32, 3277, 4033, 4681, 8321, 15841, 29341, 42799, 49141] {
            let n = BigUint::from(n);
            assert!(strong_base_two(&n));
            assert!(!extra_strong_lucas(&n));
        }
        assert!(!strong_base_two(&BigUint::from(341_u32)));
        assert!(!strong_base_two(&BigUint::from(9_u8)));
        assert_eq!(jacobi(BigUint::from(5_u8), BigUint::from(11_u8)), 1);
        assert_eq!(jacobi(BigUint::from(6_u8), BigUint::from(9_u8)), 0);
        assert_eq!(jacobi(BigUint::from(2_u8), BigUint::from(11_u8)), -1);
    }

    #[test]
    fn arbitrary_precision_arithmetic_and_factors() {
        let p = mersenne(521);
        assert_eq!(p.factor(), Ok(vec![p.clone()]));
        assert_eq!(
            (&p * 6_u8).factor(),
            Ok(vec![BigUint::from(2_u8), BigUint::from(3_u8), p.clone()])
        );
        let q = mersenne(127);
        assert_eq!((&q * &q).factor(), Ok(vec![q.clone(), q.clone()]));
        assert_eq!((&q * 101_u8).factor(), Ok(vec![BigUint::from(101_u8), q]));
        assert_eq!(
            (one() << 140_usize).factor(),
            Ok(vec![BigUint::from(2_u8); 140])
        );
        assert_eq!(zero().factor(), Ok(vec![zero()]));
        assert_eq!(one().factor(), Ok(vec![]));
        assert_eq!(p.powmod(&zero(), &one()), Some(zero()));
        assert_eq!(p.powmod(&one(), &zero()), None);
        assert_eq!(p.powmod(&(&p - 1_u8), &p), Some(zero()));
        assert_eq!(BigUint::from(2_u8).powmod(&(&p - 1_u8), &p), Some(one()));
        assert_eq!((&p * 15_u8).gcd(&(&p * 21_u8)), &p * 3_u8);
        assert_eq!(zero().gcd(&zero()), zero());
        assert_eq!(zero().gcd(&p), p.clone());
        assert_eq!(p.gcd(&zero()), p);
        assert_eq!(brent(&BigUint::from(101_u8), 0), None);
        assert_eq!(brent(&BigUint::from(101_u8), 100), None);
        // Neither factor is within the bounded Brent search's reach.
        let hard_composite = mersenne(521) * mersenne(607);
        assert_eq!(hard_composite.factor(), Err(hard_composite));
        for n in [9_u32, 15, 25, 49, 77, 101 * 103] {
            let n = BigUint::from(n);
            let f = brent(&n, 10_000).unwrap();
            assert!(f > one() && f < n);
            assert_eq!(&n % f, zero());
        }
    }

    #[test]
    fn next_prime_crosses_native_ranges() {
        for n in [0_u8, 1, 2, 3, 4, 5, 6, 7, 14] {
            assert_eq!(
                BigUint::from(n).next_prime(),
                u64::from(n).next_prime().map(BigUint::from)
            );
        }
        let boundary = BigUint::from(u128::MAX);
        // First prime after 2^128 is 2^128 + 51.
        assert_eq!(boundary.next_prime(), Some((one() << 128_usize) + 51_u8));
        assert_eq!(
            (boundary + 1_u8).next_prime(),
            Some((one() << 128_usize) + 51_u8)
        );
        assert_eq!(
            BigUint::from(u64::MAX).next_prime(),
            u128::from(u64::MAX).next_prime().map(BigUint::from)
        );
    }

    proptest! {
        #[test]
        fn native_agreement(a: u64, b: u64, exponent in 0_u64..128, n in 0_u64..100_000) {
            let (big_a, big_b) = (BigUint::from(a), BigUint::from(b));
            prop_assert_eq!(big_a.is_prime(), a.is_prime());
            prop_assert_eq!(big_a.gcd(&big_b), BigUint::from(a.gcd(&b)));
            prop_assert_eq!(big_a.powmod(&BigUint::from(exponent), &big_b),
                a.powmod(&exponent, &b).map(BigUint::from));
            let big_n = BigUint::from(n);
            prop_assert_eq!(big_n.factor(), n.factor().map(|v| v.into_iter().map(BigUint::from).collect()).map_err(BigUint::from));
            prop_assert_eq!(big_n.next_prime(), n.next_prime().map(BigUint::from));
        }

        #[test]
        fn wide_modular_powers(a: u128, b in 1_u128..=u128::MAX, e in 0_u128..128) {
            prop_assert_eq!(BigUint::from(a).powmod(&BigUint::from(e), &BigUint::from(b)),
                a.powmod(&e, &b).map(BigUint::from));
        }

        #[test]
        fn unbounded_gcd_and_power_laws(a in 1_u64..=u64::MAX, b in 1_u64..=u64::MAX, e in 0_u8..32) {
            let scale = one() << 192_usize;
            let big_a = &scale * a;
            let big_b = &scale * b;
            prop_assert_eq!(big_a.gcd(&big_b), &scale * a.gcd(&b));
            let modulus = &scale + 123_u8;
            let x = big_a.powmod(&BigUint::from(e), &modulus).unwrap();
            let y = big_a.powmod(&BigUint::from(e + 1), &modulus).unwrap();
            prop_assert_eq!(y, (x * big_a) % modulus);
        }
    }
}

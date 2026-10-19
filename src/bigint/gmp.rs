//! Arbitrary precision implementations of the native MPU algorithms on GMP.
//!
//! The functions mirror `biguint.rs`: the same trial division, BPSW and
//! Pollard-Brent paths, so both backends return the same answers. GMP
//! supplies only the arithmetic (`pow_mod`, `gcd`, `sqrt`, `jacobi`).
//!
//! `rug::Integer` is signed and the [`Int`] trait is unsigned. Values below
//! zero are outside the trait's contract; each method says what it does with them.
//!
//! `rug::Integer` has its own `gcd`, `next_prime`, `is_even` and `to_u64`,
//! which win method resolution on a concrete `Integer`. `is_even` and `to_u64`
//! agree with the trait; GMP's `gcd` and `next_prime` take `self` by value and
//! `next_prime` runs GMP's own test. Write `Int::gcd(&a, &b)` and
//! `Int::next_prime(&n)` for the trait's forms; generic code bound on [`Int`]
//! always gets the trait.
use super::Int;
use rug::Integer;
use rug::ops::RemRounding;

/// The trial division primes used before the probable prime test.
const SMALL_PRIMES: [u32; 16] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53];

/// Returns `a * b mod n`.
fn mul_mod(a: &Integer, b: &Integer, n: &Integer) -> Integer {
    Integer::from(a * b) % n
}

/// Returns `v^2 - 2 mod n`, the Lucas V doubling step.
fn square_minus_two(v: &Integer, n: &Integer) -> Integer {
    let mut result = Integer::from(v.square_ref());
    result += n;
    result -= 2_u32;
    result % n
}

/// Returns `value / 2 mod n` for odd `n`.
fn half(mut value: Integer, n: &Integer) -> Integer {
    if value.is_odd() {
        value += n;
    }
    value >> 1_u32
}

/// Base-two strong test, MPU `primality.c::is_strong_pseudoprime`.
fn strong_base_two(n: &Integer) -> bool {
    let nm1 = Integer::from(n - 1_u32);
    let mut d = nm1.clone();
    let mut s = 0;
    while d.is_even() {
        d >>= 1_u32;
        s += 1;
    }
    let base = Integer::from(2_u32);
    let mut x = Integer::from(
        base.pow_mod_ref(&d, n)
            .expect("a non-negative exponent needs no inverse"),
    );
    if x == 1 || x == nm1 {
        return true;
    }
    for _ in 1..s {
        x = mul_mod(&x, &x, n);
        if x == nm1 {
            return true;
        }
        if x == 1 {
            return false;
        }
    }
    false
}

/// Extra-strong Lucas test, MPU `primality.c::is_lucas_pseudoprime`
/// (strength 3), including `select_extra_strong_parameters`.
fn extra_strong_lucas(n: &Integer) -> bool {
    if n.is_perfect_square() {
        return false;
    }
    let mut p = Integer::from(3_u32);
    let discriminant = loop {
        let d = Integer::from(p.square_ref()) - 4_u32;
        match d.jacobi(n) {
            -1 => break d,
            0 if Integer::from(d.gcd_ref(n)) != *n => return false,
            _ => p += 1_u32,
        }
    };
    let mut d = Integer::from(n + 1_u32);
    let mut s = 0;
    while d.is_even() {
        d >>= 1_u32;
        s += 1;
    }
    let (mut u, mut v) = (Integer::from(1_u32), p.clone());
    for bit in (0..d.significant_bits() - 1).rev() {
        u = mul_mod(&u, &v, n);
        v = square_minus_two(&v, n);
        if d.get_bit(bit) {
            let next_u = half((Integer::from(&p * &u) + &v) % n, n);
            v = half(
                (Integer::from(&discriminant * &u) + Integer::from(&p * &v)) % n,
                n,
            );
            u = next_u;
        }
    }
    let nm2 = Integer::from(n - 2_u32);
    if u == 0 && (v == 2 || v == nm2) {
        return true;
    }
    for _ in 1..s {
        if v == 0 {
            return true;
        }
        v = square_minus_two(&v, n);
        if v == 2 {
            return false;
        }
    }
    false
}

/// Returns `|a - b|`.
#[cfg(any(not(feature = "bigint"), test))]
fn difference(a: &Integer, b: &Integer) -> Integer {
    Integer::from(a - b).abs()
}

/// Returns `x^2 + a mod n`, the Pollard-Brent iteration.
#[cfg(any(not(feature = "bigint"), test))]
fn step(x: &Integer, a: u32, n: &Integer) -> Integer {
    let mut result = Integer::from(x.square_ref());
    result += a;
    result % n
}

/// Pollard-Brent, MPU `factor.c::pbrent_factor` (non-Montgomery path).
/// A finite work budget preserves the trait's unsplit-cofactor error contract.
#[cfg(any(not(feature = "bigint"), test))]
fn brent(n: &Integer, mut rounds: usize) -> Option<Integer> {
    let (mut x, mut anchor) = (Integer::from(1_u32), Integer::from(1_u32));
    let (mut r, mut failures, mut a) = (1_usize, 6_u8, 1_u32);
    while rounds > 0 {
        let mut left = r.min(rounds);
        let mut saved = x.clone();
        let mut f = Integer::from(1_u32);
        while left > 0 {
            let batch = left.min(160);
            left -= batch;
            rounds -= batch;
            saved.clone_from(&x);
            let mut product = Integer::from(1_u32);
            for _ in 0..batch {
                x = step(&x, a, n);
                product = mul_mod(&product, &difference(&x, &anchor), n);
            }
            f = Integer::from(product.gcd_ref(n));
            if f != 1 {
                break;
            }
        }
        if f == 1 {
            r *= 2;
            anchor.clone_from(&x);
            continue;
        }
        if f == *n {
            x = saved;
            loop {
                x = step(&x, a, n);
                f = difference(&x, &anchor).gcd(n);
                if f != 1 || r == 0 {
                    break;
                }
                r -= 1;
            }
        }
        if f == 0 || f == *n {
            if failures == 0 {
                break;
            }
            failures -= 1;
            anchor += 11_u32;
            anchor %= n;
            x.clone_from(&anchor);
            a += 1;
            continue;
        }
        return Some(f);
    }
    None
}

/// MPU `factor.c::factor` with a bounded Brent: the `gmp`-only path of
/// [`Int::factor`].
#[cfg(not(feature = "bigint"))]
fn brent_factor(value: &Integer) -> Result<Vec<Integer>, Integer> {
    let mut pending = vec![value.clone()];
    let mut factors = Vec::new();
    while let Some(mut n) = pending.pop() {
        if let Some(native) = n.to_u128() {
            factors.extend(
                native
                    .factor()
                    .map_err(Integer::from)?
                    .into_iter()
                    .map(Integer::from),
            );
            continue;
        }
        for p in SMALL_PRIMES {
            while n.is_divisible_u(p) {
                factors.push(Integer::from(p));
                n /= p;
            }
        }
        if n == 1 {
            continue;
        }
        if n.is_prime() != 0 {
            factors.push(n);
            continue;
        }
        let f = if n.is_perfect_square() {
            Integer::from(n.sqrt_ref())
        } else {
            brent(&n, 20_000).ok_or_else(|| n.clone())?
        };
        pending.push(Integer::from(&n / &f));
        pending.push(f);
    }
    factors.sort_unstable();
    Ok(factors)
}

impl Int for Integer {
    fn add_int(&self, other: &Self) -> Option<Self> {
        Some(Self::from(self + other))
    }

    fn sub_int(&self, other: &Self) -> Option<Self> {
        let difference = Self::from(self - other);
        (!difference.is_negative()).then_some(difference)
    }

    fn mul_int(&self, other: &Self) -> Option<Self> {
        Some(Self::from(self * other))
    }

    fn divrem(&self, divisor: &Self) -> Option<(Self, Self)> {
        (*divisor != 0).then(|| <(Self, Self)>::from(self.div_rem_ref(divisor)))
    }

    /// The result is never negative, whatever the signs of the operands.
    fn mulmod(&self, other: &Self, modulus: &Self) -> Option<Self> {
        (*modulus != 0).then(|| Self::from(self * other).rem_euc(modulus))
    }

    fn shl_bits(&self, bits: u32) -> Option<Self> {
        Some(Self::from(self << bits))
    }

    fn shr_bits(&self, bits: u32) -> Self {
        Self::from(self >> bits)
    }

    fn bit_length(&self) -> u64 {
        u64::from(self.significant_bits())
    }

    fn is_even(&self) -> bool {
        Self::is_even(self)
    }

    fn from_u64(value: u64) -> Self {
        Self::from(value)
    }

    fn to_u64(&self) -> Option<u64> {
        Self::to_u64(self)
    }

    fn from_decimal(text: &str) -> Option<Self> {
        if text.is_empty() || !text.bytes().all(|b| b.is_ascii_digit()) {
            return None;
        }
        text.parse().ok()
    }

    fn to_decimal(&self) -> String {
        self.to_string()
    }

    /// MPU `primality.c::BPSW` and `PP.pm::_is_prime7`: base-two MR and
    /// extra-strong Lucas above the native range. A negative value returns 0.
    ///
    /// ```
    /// use math_prime_util::Int;
    /// use rug::Integer;
    /// let p = (Integer::from(1_u32) << 521_u32) - 1_u32;
    /// assert_eq!(p.is_prime(), 1);
    /// ```
    fn is_prime(&self) -> u8 {
        if self.is_negative() {
            return 0;
        }
        if let Some(n) = self.to_u128() {
            return n.is_prime();
        }
        if SMALL_PRIMES.iter().any(|&p| self.is_divisible_u(p)) {
            return 0;
        }
        u8::from(strong_base_two(self) && extra_strong_lucas(self))
    }

    /// With feature `bigint`, MPU `PP.pm::factor` through
    /// [`factor::factor`](super::factor::factor) on the digits (the recipe of
    /// perfect powers, p-1, ECM, HOLF and Pollard-Brent, no time limit);
    /// without it, MPU `factor.c::factor`: trial division, prime and square
    /// checks, then a bounded Brent.
    ///
    /// # Panics
    /// Panics for a negative value, which has no factorization in this
    /// unsigned trait, as MPU's `factor` dies on one.
    ///
    /// ```
    /// use math_prime_util::Int;
    /// use rug::Integer;
    /// assert_eq!(Integer::from(12_u32).factor().unwrap(),
    ///            vec![Integer::from(2_u32), Integer::from(2_u32), Integer::from(3_u32)]);
    /// ```
    fn factor(&self) -> Result<Vec<Self>, Self> {
        assert!(!self.is_negative(), "factor needs a non-negative integer");
        #[cfg(feature = "bigint")]
        {
            // Decimal text carries the value both ways; a non-negative
            // integer's digits always parse.
            let back = |b: &num_bigint::BigUint| {
                Self::parse(b.to_string())
                    .map(Self::from)
                    .unwrap_or_default()
            };
            let n = num_bigint::BigUint::parse_bytes(self.to_string().as_bytes(), 10)
                .unwrap_or_default();
            super::factor::factor(&n)
                .map(|factors| factors.iter().map(back).collect())
                .map_err(|cofactor| back(&cofactor))
        }
        #[cfg(not(feature = "bigint"))]
        {
            brent_factor(self)
        }
    }

    /// MPU `mulmod.h::powmod`, using GMP's modular exponentiation. A negative
    /// base is reduced first; a negative exponent uses the modular inverse and
    /// returns `None` when there is none.
    ///
    /// ```
    /// use math_prime_util::Int;
    /// use rug::Integer;
    /// let n = Integer::from(7_u32);
    /// assert_eq!(Integer::from(3_u32).powmod(&Integer::from(5_u32), &n), Some(Integer::from(5_u32)));
    /// ```
    fn powmod(&self, exponent: &Self, modulus: &Self) -> Option<Self> {
        if *modulus == 0 {
            return None;
        }
        self.pow_mod_ref(exponent, modulus).map(Self::from)
    }

    /// GMP's gcd, the result MPU `util.h::gcd_ui` computes. The result is never negative.
    ///
    /// ```
    /// use math_prime_util::Int;
    /// use rug::Integer;
    /// assert_eq!(Int::gcd(&Integer::from(12_u32), &Integer::from(18_u32)), Integer::from(6_u32));
    /// ```
    fn gcd(&self, other: &Self) -> Self {
        Self::from(self.gcd_ref(other))
    }

    /// Candidate search, MPU `util.c::next_prime` and `PP.pm::next_prime`.
    /// Every value below 2, negative ones included, returns 2.
    ///
    /// ```
    /// use math_prime_util::Int;
    /// use rug::Integer;
    /// assert_eq!(Int::next_prime(&Integer::from(14_u32)), Some(Integer::from(17_u32)));
    /// ```
    fn next_prime(&self) -> Option<Self> {
        const ADVANCE: [u32; 30] = [
            1, 6, 5, 4, 3, 2, 1, 4, 3, 2, 1, 2, 1, 4, 3, 2, 1, 2, 1, 4, 3, 2, 1, 6, 5, 4, 3, 2, 1,
            2,
        ];
        if self.is_negative() {
            return Some(Self::from(2_u32));
        }
        if let Some(n) = self.to_u64() {
            if let Some(p) = n.next_prime() {
                return Some(Self::from(p));
            }
        }
        let mut n = self.clone();
        loop {
            // The remainder is at most 29 on every supported architecture.
            let residue = usize::try_from(n.mod_u(30)).unwrap_or_default();
            n += ADVANCE[residue];
            if !n.is_divisible_u(7) && n.is_prime() != 0 {
                return Some(n);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use rug::integer::IsPrime;

    fn int<T>(value: T) -> Integer
    where
        Integer: From<T>,
    {
        Integer::from(value)
    }

    fn mersenne(bits: u32) -> Integer {
        (int(1_u32) << bits) - 1_u32
    }

    #[test]
    fn arbitrary_precision_primality() {
        for bits in [127, 521, 607] {
            let p = mersenne(bits);
            assert_ne!(p.is_prime(), 0);
            assert!(strong_base_two(&p));
            assert!(extra_strong_lucas(&p));
            assert_eq!(int(&p * 3_u32).is_prime(), 0);
            assert_eq!(int(&p * &p).is_prime(), 0);
            assert!(!extra_strong_lucas(&int(&p * &p)));
        }
        // Strong base-two pseudoprimes must still fail the Lucas test.
        for n in [2047_u32, 3277, 4033, 4681, 8321, 15841, 29341, 42799, 49141] {
            let n = int(n);
            assert!(strong_base_two(&n));
            assert!(!extra_strong_lucas(&n));
        }
        assert!(!strong_base_two(&int(341_u32)));
        assert!(!strong_base_two(&int(9_u32)));
        // Above the native range the answer is "probable", never "proven".
        assert_eq!(mersenne(521).is_prime(), 1);
        assert_eq!(int(u128::MAX).is_prime(), 0);
        assert_eq!(int(97_u32).is_prime(), 2);
    }

    #[test]
    fn agrees_with_gmp_probable_prime_test() {
        // GMP's own test (Miller-Rabin after a BPSW-like stage) and the ported BPSW agree.
        let mut n = int(1_u32) << 130_u32;
        for _ in 0..200 {
            n += 1_u32;
            let gmp = n.is_probably_prime(25) != IsPrime::No;
            assert_eq!(n.is_prime() != 0, gmp, "{n}");
        }
        let mut candidate = Integer::from(Integer::u_pow_u(10, 100));
        for _ in 0..20 {
            candidate = Int::next_prime(&candidate).unwrap();
            assert_ne!(candidate.is_probably_prime(25), IsPrime::No, "{candidate}");
            assert_eq!(candidate.is_prime(), 1);
        }
    }

    #[test]
    fn arbitrary_precision_arithmetic_and_factors() {
        let p = mersenne(521);
        assert_eq!(p.factor(), Ok(vec![p.clone()]));
        assert_eq!(
            int(&p * 6_u32).factor(),
            Ok(vec![int(2_u32), int(3_u32), p.clone()])
        );
        let q = mersenne(127);
        assert_eq!(int(&q * &q).factor(), Ok(vec![q.clone(), q.clone()]));
        assert_eq!(int(&q * 101_u32).factor(), Ok(vec![int(101_u32), q]));
        assert_eq!((int(1_u32) << 140_u32).factor(), Ok(vec![int(2_u32); 140]));
        assert_eq!(int(0_u32).factor(), Ok(vec![int(0_u32)]));
        assert_eq!(int(1_u32).factor(), Ok(vec![]));
        assert_eq!(p.powmod(&int(0_u32), &int(1_u32)), Some(int(0_u32)));
        assert_eq!(p.powmod(&int(1_u32), &int(0_u32)), None);
        assert_eq!(p.powmod(&int(&p - 1_u32), &p), Some(int(0_u32)));
        assert_eq!(int(2_u32).powmod(&int(&p - 1_u32), &p), Some(int(1_u32)));
        assert_eq!(
            Int::gcd(&int(&p * 15_u32), &int(&p * 21_u32)),
            int(&p * 3_u32)
        );
        assert_eq!(Int::gcd(&int(0_u32), &int(0_u32)), int(0_u32));
        assert_eq!(Int::gcd(&int(0_u32), &p), p);
        assert_eq!(Int::gcd(&p, &int(0_u32)), p);
        assert_eq!(brent(&int(101_u32), 0), None);
        assert_eq!(brent(&int(101_u32), 100), None);
        // Neither factor is within the bounded Brent search's reach; with
        // feature bigint the recipe runs instead and splits by p-1.
        let hard_composite = mersenne(521) * mersenne(607);
        #[cfg(not(feature = "bigint"))]
        assert_eq!(hard_composite.factor(), Err(hard_composite.clone()));
        #[cfg(feature = "bigint")]
        assert_eq!(hard_composite.factor(), Ok(vec![mersenne(521), mersenne(607)]));
        for n in [9_u32, 15, 25, 49, 77, 101 * 103] {
            let n = int(n);
            let f = brent(&n, 10_000).unwrap();
            assert!(f > 1 && f < n);
            assert!(n.is_divisible(&f));
        }
    }

    #[test]
    fn arithmetic_surface() {
        let big = int(1_u32) << 200_u32;
        assert_eq!(big.bit_length(), 201);
        assert!(Int::is_even(&big) && !Int::is_even(&int(&big + 1_u32)));
        assert_eq!(big.shr_bits(190), int(1024_u32));
        assert_eq!(int(1_u32).shl_bits(70), Some(int(1_u32) << 70_u32));
        assert_eq!(Int::to_u64(&big), None);
        assert_eq!(Int::to_u64(&<Integer as Int>::from_u64(7)), Some(7));
        assert_eq!(big.sub_int(&int(&big + 1_u32)), None);
        assert_eq!(big.sub_int(&big), Some(int(0_u32)));
        assert_eq!(big.add_int(&int(1_u32)), Some(int(&big + 1_u32)));
        assert_eq!(big.mul_int(&big), Some(int(1_u32) << 400_u32));
        assert_eq!(big.divrem(&int(0_u32)), None);
        assert_eq!(big.mulmod(&big, &int(0_u32)), None);
        let (ten, three) = (int(10_u32), int(3_u32));
        assert_eq!(ten.divrem(&three), Some((three.clone(), int(1_u32))));
        assert_eq!(int(7_u32).mulmod(&int(8_u32), &ten), Some(int(6_u32)));
        assert_eq!(int(-7_i32).mulmod(&int(8_u32), &ten), Some(int(4_u32)));
        let text = big.to_decimal();
        assert_eq!(<Integer as Int>::from_decimal(&text), Some(big));
        assert_eq!(<Integer as Int>::from_decimal(""), None);
        assert_eq!(<Integer as Int>::from_decimal("-1"), None);
        assert_eq!(<Integer as Int>::from_decimal("+1"), None);
    }

    #[test]
    fn negative_values() {
        assert_eq!(int(-7_i32).is_prime(), 0);
        assert_eq!(int(-1_i32).is_prime(), 0);
        assert_eq!(Int::next_prime(&int(-5_i32)), Some(int(2_u32)));
        assert_eq!(Int::gcd(&int(-12_i32), &int(18_u32)), int(6_u32));
        assert_eq!(Int::gcd(&int(12_u32), &int(-18_i32)), int(6_u32));
        // -8 mod 7 = 6; 2^-1 mod 7 = 4; 2 has no inverse mod 8.
        assert_eq!(
            int(-2_i32).powmod(&int(3_u32), &int(7_u32)),
            Some(int(6_u32))
        );
        assert_eq!(
            int(2_u32).powmod(&int(-1_i32), &int(7_u32)),
            Some(int(4_u32))
        );
        assert_eq!(int(2_u32).powmod(&int(-1_i32), &int(8_u32)), None);
    }

    #[test]
    #[should_panic(expected = "non-negative")]
    fn factor_rejects_negative_values() {
        let _ = int(-6_i32).factor();
    }

    #[test]
    fn next_prime_crosses_native_ranges() {
        for n in [0_u8, 1, 2, 3, 4, 5, 6, 7, 14] {
            assert_eq!(
                Int::next_prime(&int(n)),
                u64::from(n).next_prime().map(Integer::from)
            );
        }
        let boundary = int(u128::MAX);
        // First prime after 2^128 is 2^128 + 51.
        let expected = (int(1_u32) << 128_u32) + 51_u32;
        assert_eq!(Int::next_prime(&boundary), Some(expected.clone()));
        assert_eq!(Int::next_prime(&(boundary + 1_u32)), Some(expected));
        assert_eq!(
            Int::next_prime(&int(u64::MAX)),
            u128::from(u64::MAX).next_prime().map(Integer::from)
        );
    }

    #[cfg(feature = "bigint")]
    #[test]
    fn agrees_with_num_bigint_backend() {
        use num_bigint::BigUint;
        fn text<T: ToString>(factors: &[T]) -> Vec<String> {
            factors.iter().map(ToString::to_string).collect()
        }
        let big = |n: &Integer| n.to_string().parse::<BigUint>().unwrap();
        let mut n = Integer::from(Integer::u_pow_u(10, 40));
        for _ in 0..40 {
            n += 7_u32;
            let b = big(&n);
            assert_eq!(n.is_prime(), b.is_prime(), "{n}");
            assert_eq!(
                n.factor().map(|f| text(&f)).map_err(|e| e.to_string()),
                b.factor().map(|f| text(&f)).map_err(|e| e.to_string()),
                "{n}"
            );
            assert_eq!(
                Int::next_prime(&n).unwrap().to_string(),
                b.next_prime().unwrap().to_string()
            );
        }
    }

    proptest! {
        #[test]
        fn native_agreement(a: u64, b: u64, exponent in 0_u64..128, n in 0_u64..100_000) {
            let (big_a, big_b) = (int(a), int(b));
            prop_assert_eq!(big_a.is_prime(), a.is_prime());
            prop_assert_eq!(Int::gcd(&big_a, &big_b), int(a.gcd(&b)));
            prop_assert_eq!(big_a.powmod(&int(exponent), &big_b),
                a.powmod(&exponent, &b).map(Integer::from));
            let big_n = int(n);
            prop_assert_eq!(big_n.factor(), n.factor().map(|v| v.into_iter().map(Integer::from).collect()).map_err(Integer::from));
            prop_assert_eq!(Int::next_prime(&big_n), n.next_prime().map(Integer::from));
        }

        #[test]
        fn arithmetic_agreement(a: u64, b: u64, m in 1_u64..=u64::MAX, bits in 0_u32..70) {
            let (x, y, z) = (int(a), int(b), int(m));
            prop_assert_eq!(a.mulmod(&b, &m).map(Integer::from), x.mulmod(&y, &z));
            prop_assert_eq!(a.bit_length(), x.bit_length());
            prop_assert_eq!(a.is_even(), Int::is_even(&x));
            prop_assert_eq!(a.shr_bits(bits).to_decimal(), x.shr_bits(bits).to_decimal());
            prop_assert_eq!(a.divrem(&m).map(|(q, r)| (int(q), int(r))), x.divrem(&z));
            prop_assert_eq!(a.sub_int(&b).map(Integer::from), x.sub_int(&y));
            prop_assert_eq!(u128::from(a).add_int(&u128::from(b)).map(|v| v.to_decimal()),
                x.add_int(&y).map(|v| v.to_decimal()));
            prop_assert_eq!(u128::from(a).mul_int(&u128::from(b)).map(|v| v.to_decimal()),
                x.mul_int(&y).map(|v| v.to_decimal()));
            prop_assert_eq!(x.shl_bits(bits).map(|v| v.to_decimal()),
                u128::from(a).shl_bits(bits).map(|v| v.to_decimal()).or_else(|| Some(int(&x << bits).to_decimal())));
        }

        #[test]
        fn wide_modular_powers(a: u128, b in 1_u128..=u128::MAX, e in 0_u128..128) {
            prop_assert_eq!(int(a).powmod(&int(e), &int(b)),
                a.powmod(&e, &b).map(Integer::from));
        }

        #[test]
        fn unbounded_gcd_and_power_laws(a in 1_u64..=u64::MAX, b in 1_u64..=u64::MAX, e in 0_u8..32) {
            let scale = int(1_u32) << 192_u32;
            let big_a = int(&scale * a);
            let big_b = int(&scale * b);
            prop_assert_eq!(Int::gcd(&big_a, &big_b), int(&scale * a.gcd(&b)));
            let modulus = int(&scale + 123_u32);
            let x = big_a.powmod(&int(e), &modulus).unwrap();
            let y = big_a.powmod(&int(e + 1), &modulus).unwrap();
            prop_assert_eq!(y, (x * big_a) % modulus);
        }
    }
}

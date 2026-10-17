use crate::is_prime;

/// Return `2`, `1`, or `0` when `a + bi` is a proven prime, a probable prime, or composite.
///
/// `a` and `b` are absolute values. The number is a Gaussian prime when one
/// argument is `0` and the other is a prime congruent to `3` modulo `4`, or
/// when both are nonzero and `a` squared plus `b` squared is prime.
/// A norm that fits in `u64` uses [`is_prime`] and returns `0` or `2`.
/// A larger norm returns `1` when a Miller-Rabin test accepts it, else `0`.
///
/// Port of `is_gaussian_prime` in MPU 0.75 `XS.xs`.
/// `lib/Math/Prime/Util/PP.pm` uses the same test when the norm does not fit in one word.
///
/// ```
/// use math_prime_util::is_gaussian_prime;
/// assert_eq!(is_gaussian_prime(3, 0), 2);
/// assert_eq!(is_gaussian_prime(1, 1), 2);
/// assert_eq!(is_gaussian_prime(5, 0), 0);
/// ```
pub fn is_gaussian_prime(a: u64, b: u64) -> u8 {
    if a == 0 {
        return prime_3_mod_4(b);
    }
    if b == 0 {
        return prime_3_mod_4(a);
    }
    // 1+i has norm 2, the only even Gaussian prime.
    if a == 1 && b == 1 {
        return 2;
    }
    // Same parity makes an even norm larger than 2.
    if a % 2 == b % 2 {
        return 0;
    }
    prime_status(norm(a, b))
}

fn prime_3_mod_4(n: u64) -> u8 {
    if n % 4 == 3 { is_prime(n) } else { 0 }
}

/// 0, 1, or 2 bits above 2^128. The sum of two `u64` squares is below 2^129.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Wide {
    hi: u8,
    lo: u128,
}

fn norm(a: u64, b: u64) -> Wide {
    let left = u128::from(a) * u128::from(a);
    let right = u128::from(b) * u128::from(b);
    match left.checked_add(right) {
        Some(lo) => Wide { hi: 0, lo },
        None => Wide {
            hi: 1,
            lo: left.wrapping_add(right),
        },
    }
}

fn prime_status(n: Wide) -> u8 {
    if n.hi == 0
        && let Ok(small) = u64::try_from(n.lo)
    {
        return is_prime(small);
    }
    u8::from(passes_sprp(n))
}

const ONE: Wide = Wide { hi: 0, lo: 1 };

fn passes_sprp(n: Wide) -> bool {
    let (d, s) = split_even(decrement(n));
    let nm1 = decrement(n);
    [2, 3, 5, 7, 11, 13, 23]
        .into_iter()
        .all(|base| one_base(n, nm1, d, s, base))
}

fn one_base(n: Wide, nm1: Wide, d: Wide, s: u32, base: u128) -> bool {
    let mut x = modpow(Wide { hi: 0, lo: base }, d, n);
    if x == ONE || x == nm1 {
        return true;
    }
    for _ in 1..s {
        x = modmul(x, x, n);
        if x == nm1 {
            return true;
        }
    }
    false
}

fn split_even(mut d: Wide) -> (Wide, u32) {
    let mut s = 0u32;
    while d.lo & 1 == 0 {
        d = d.shr1();
        s += 1;
    }
    (d, s)
}

fn decrement(n: Wide) -> Wide {
    if n.lo == 0 {
        Wide {
            hi: n.hi - 1,
            lo: u128::MAX,
        }
    } else {
        Wide {
            hi: n.hi,
            lo: n.lo - 1,
        }
    }
}

impl Wide {
    fn is_zero(self) -> bool {
        self.hi == 0 && self.lo == 0
    }

    fn shr1(self) -> Self {
        Self {
            hi: self.hi >> 1,
            lo: (self.lo >> 1) | (u128::from(self.hi & 1) << 127),
        }
    }

    fn add(self, other: Self) -> Self {
        let (lo, carry) = self.lo.overflowing_add(other.lo);
        Self {
            hi: self.hi + other.hi + u8::from(carry),
            lo,
        }
    }

    fn sub(self, other: Self) -> Self {
        let (lo, borrow) = self.lo.overflowing_sub(other.lo);
        Self {
            hi: self.hi - other.hi - u8::from(borrow),
            lo,
        }
    }

    fn cmp_ge(self, other: Self) -> bool {
        (self.hi, self.lo) >= (other.hi, other.lo)
    }
}

fn addmod(a: Wide, b: Wide, m: Wide) -> Wide {
    let sum = a.add(b);
    if sum.cmp_ge(m) { sum.sub(m) } else { sum }
}

fn modmul(mut a: Wide, mut b: Wide, m: Wide) -> Wide {
    let mut result = Wide { hi: 0, lo: 0 };
    while !b.is_zero() {
        if b.lo & 1 == 1 {
            result = addmod(result, a, m);
        }
        a = addmod(a, a, m);
        b = b.shr1();
    }
    result
}

fn modpow(mut base: Wide, mut exp: Wide, m: Wide) -> Wide {
    let mut result = ONE;
    while !exp.is_zero() {
        if exp.lo & 1 == 1 {
            result = modmul(result, base, m);
        }
        base = modmul(base, base, m);
        exp = exp.shr1();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::is_gaussian_prime;

    #[test]
    fn small_axis_and_norm_cases() {
        assert_eq!(is_gaussian_prime(0, 0), 0);
        assert_eq!(is_gaussian_prime(0, 2), 0);
        assert_eq!(is_gaussian_prime(0, 3), 2);
        assert_eq!(is_gaussian_prime(0, 9), 0);
        assert_eq!(is_gaussian_prime(15, 0), 0);
        assert_eq!(is_gaussian_prime(2, 1), 2);
        assert_eq!(is_gaussian_prime(8, 1), 0);
        assert_eq!(is_gaussian_prime(2, 2), 0);
        assert_eq!(is_gaussian_prime(3, 5), 0);
        assert_eq!(is_gaussian_prime(1, 1), 2);
    }

    #[test]
    fn large_norm_is_probable_or_composite() {
        // 2^2 + 4294967307^2 is prime and larger than u64::MAX, so the result is 1.
        assert_eq!(is_gaussian_prime(2, 4_294_967_307), 1);
        // (2^64-1)^2 + 2^2 is composite and still fits in u128.
        assert_eq!(is_gaussian_prime(u64::MAX, 2), 0);
        // The sum of squares overflows u128 and is composite.
        assert_eq!(is_gaussian_prime(u64::MAX, u64::MAX - 1), 0);
        assert_eq!(is_gaussian_prime(u64::MAX, u64::MAX), 0);
    }

    #[test]
    fn repeat_and_threads_agree() {
        let samples = [(0, 3), (1, 1), (5, 0), (2, 4_294_967_307), (u64::MAX, 2)];
        let once: Vec<_> = samples
            .iter()
            .map(|&(a, b)| is_gaussian_prime(a, b))
            .collect();
        let twice: Vec<_> = samples
            .iter()
            .map(|&(a, b)| is_gaussian_prime(a, b))
            .collect();
        assert_eq!(once, twice);
        let threads: Vec<_> = (0..4)
            .map(|_| {
                std::thread::spawn(move || {
                    samples
                        .iter()
                        .map(|&(a, b)| is_gaussian_prime(a, b))
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        for thread in threads {
            assert_eq!(thread.join().unwrap(), once);
        }
    }
}

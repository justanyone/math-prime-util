//! Montgomery arithmetic on odd `u64` moduli, ported from MPU's `montmath.h`.
//!
//! The port uses the portable `u128` formulation (the `montmul` MPU keeps
//! for `AArch64`) instead of the x86-64 inline assembly, so it runs on every
//! target and needs no `unsafe`.

/// Montgomery context for an odd modulus `n`, with `R = 2^64`.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Mont {
    /// The odd modulus.
    pub n: u64,
    /// `-n^{-1} mod 2^64`.
    npi: u64,
    /// `R mod n`: the Montgomery form of 1.
    pub one: u64,
}

impl Mont {
    /// Builds the context. `n` must be odd (and so nonzero).
    #[inline]
    pub(crate) fn new(n: u64) -> Self {
        debug_assert!(n & 1 == 1, "Montgomery modulus must be odd");
        // MPU mont_inverse: Newton iteration from (3n)^2, four rounds give 64 bits.
        let mut inv = (3u64.wrapping_mul(n)) ^ 2;
        for _ in 0..4 {
            inv = inv.wrapping_mul(2u64.wrapping_sub(n.wrapping_mul(inv)));
        }
        let npi = 0u64.wrapping_sub(inv);
        // mont_get1(n) = 2^64 mod n.
        #[allow(clippy::cast_possible_truncation)]
        let one = ((1u128 << 64) % u128::from(n)) as u64;
        Self { n, npi, one }
    }

    /// Montgomery product `a * b / R mod n` for `a, b < n`.
    ///
    /// `n < 2^63` uses the `_mulredc63` reduction from `montmath.h` (one
    /// conditional subtract). Larger odd moduli use `_mulredc64`, which
    /// keeps the overflow bit out of the high half.
    #[inline]
    pub(crate) fn mul(self, a: u64, b: u64) -> u64 {
        if self.n < 1 << 63 {
            self.mul63(a, b)
        } else {
            self.mul64(a, b)
        }
    }

    /// `_mulredc63`: `n < 2^63` and `a, b < n`, so the REDC result is `< 2n`.
    #[inline]
    fn mul63(self, a: u64, b: u64) -> u64 {
        let x = u128::from(a) * u128::from(b);
        #[allow(clippy::cast_possible_truncation)]
        let m = (x as u64).wrapping_mul(self.npi);
        let y = u128::from(m) * u128::from(self.n);
        #[allow(clippy::cast_possible_truncation)]
        let (xh, xl, yh, yl) = ((x >> 64) as u64, x as u64, (y >> 64) as u64, y as u64);
        let carry = u64::from(xl.overflowing_add(yl).1);
        let z = xh.wrapping_add(yh).wrapping_add(carry);
        if z >= self.n { z - self.n } else { z }
    }

    /// `_mulredc64`, including moduli with the high bit set.
    #[inline]
    fn mul64(self, a: u64, b: u64) -> u64 {
        let x = u128::from(a) * u128::from(b);
        #[allow(clippy::cast_possible_truncation)]
        let m = (x as u64).wrapping_mul(self.npi);
        let y = u128::from(m) * u128::from(self.n);
        // z = (x + y) / R; the low halves cancel to zero mod R by construction,
        // so only the carry out of the low half matters.
        #[allow(clippy::cast_possible_truncation)]
        let (xh, xl, yh, yl) = ((x >> 64) as u64, x as u64, (y >> 64) as u64, y as u64);
        let (_, carry) = xl.overflowing_add(yl);
        let (z, c1) = xh.overflowing_add(yh);
        let (z, c2) = z.overflowing_add(u64::from(carry));
        if c1 || c2 || z >= self.n {
            z.wrapping_sub(self.n)
        } else {
            z
        }
    }

    /// `a^2 / R mod n`.
    #[inline]
    pub(crate) fn sqr(self, a: u64) -> u64 {
        self.mul(a, a)
    }

    /// Converts `a < n` into Montgomery form (`a * R mod n`).
    #[inline]
    pub(crate) fn to_mont(self, a: u64) -> u64 {
        crate::modular::mulmod_u(a, self.one, self.n)
    }

    /// Converts out of Montgomery form.
    #[inline]
    pub(crate) fn recover(self, a: u64) -> u64 {
        self.mul(a, 1)
    }

    /// `a^k` in Montgomery form, `a` already in Montgomery form.
    #[inline]
    pub(crate) fn pow(self, mut a: u64, mut k: u64) -> u64 {
        let mut t = self.one;
        while k != 0 {
            if k & 1 != 0 {
                t = self.mul(t, a);
            }
            k >>= 1;
            if k != 0 {
                a = self.sqr(a);
            }
        }
        t
    }
}

#[cfg(test)]
mod tests {
    use super::Mont;
    use crate::modular::{mulmod_u, powmod_u};

    #[test]
    fn agrees_with_mulmod() {
        let mods = [
            3u64,
            5,
            7,
            65_537,
            (1 << 61) - 1,
            u64::MAX - 58,
            u64::MAX,
            1 << 63 | 1,
        ];
        let vals = [
            0u64,
            1,
            2,
            3,
            12_345,
            u64::from(u32::MAX),
            1 << 40,
            u64::MAX - 1,
            u64::MAX,
        ];
        for &n in &mods {
            let m = Mont::new(n);
            assert_eq!(m.recover(m.one), 1 % n);
            for &a in &vals {
                for &b in &vals {
                    let (a, b) = (a % n, b % n);
                    let got = m.recover(m.mul(m.to_mont(a), m.to_mont(b)));
                    assert_eq!(got, mulmod_u(a, b, n), "{a} * {b} mod {n}");
                    let got = m.recover(m.pow(m.to_mont(a), b));
                    assert_eq!(got, powmod_u(a, b, n), "{a} ^ {b} mod {n}");
                }
            }
        }
    }
}

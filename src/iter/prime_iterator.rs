/// Buffered prime cursor from `reference/mpu/lib/Math/Prime/Util.pm`, `prime_iterator`.
///
/// Each refill asks `primes` for about `10_000` primes, matching the Perl closure.
#[derive(Debug, Clone)]
pub struct PrimeIterator {
    cursor: Option<u64>,
    buffer: std::vec::IntoIter<u64>,
}

impl Iterator for PrimeIterator {
    type Item = u64;

    // The segment width follows the reference: `int(10000*log($p)+1)`.
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    fn next(&mut self) -> Option<u64> {
        if let Some(prime) = self.buffer.next() {
            self.cursor = Some(prime);
            return Some(prime);
        }
        let cursor = self.cursor.take()?;
        let lo = cursor.checked_add(1)?;
        let segment = if cursor <= 10_000 {
            10_000
        } else {
            (10_000.0 * (cursor as f64).ln() + 1.0) as u64
        };
        let hi = cursor.saturating_add(segment);
        self.buffer = crate::primes(lo, hi).into_iter();
        let prime = self.buffer.next()?;
        self.cursor = Some(prime);
        Some(prime)
    }
}

/// Create an iterator at the first prime at least `start`.
///
/// See `reference/mpu/lib/Math/Prime/Util.pm`, `prime_iterator`.
/// The native stream stops at `u64::MAX`. MPU continues with big integers.
/// A start of `0` is the same as a start of `2`.
///
/// ```
/// assert_eq!(math_prime_util::prime_iterator(10).next(), Some(11));
/// ```
#[must_use]
pub fn prime_iterator(start: u64) -> PrimeIterator {
    PrimeIterator {
        // `$p = ($start > 0) ? $start - 1 : 0`, then `next_prime($p)`.
        cursor: Some(start.saturating_sub(1)),
        buffer: Vec::new().into_iter(),
    }
}

/// The prime stream on arbitrary precision values: the native segments of
/// [`PrimeIterator`] up to the largest 64-bit prime, then
/// [`next_prime`](crate::bigint::primality::next_prime) on each value, as
/// `Util.pm` `prime_iterator` does once past `MPU_MAXPRIME`. The stream
/// never ends.
///
/// Built by [`prime_iterator`](crate::bigint::counts::prime_iterator) in
/// `bigint::counts`.
#[cfg(feature = "bigint")]
#[cfg_attr(docsrs, doc(cfg(feature = "bigint")))]
#[derive(Debug, Clone)]
pub struct BigPrimeIterator {
    native: Option<PrimeIterator>,
    /// The last prime returned, or the value before the first candidate.
    last: num_bigint::BigUint,
}

#[cfg(feature = "bigint")]
impl BigPrimeIterator {
    /// Create the stream at the first prime at least `start`.
    ///
    /// ```
    /// use math_prime_util::BigPrimeIterator;
    /// use num_bigint::BigUint;
    /// let mut primes = BigPrimeIterator::new(&BigUint::from(u64::MAX));
    /// assert_eq!(primes.next(), Some((BigUint::from(1_u8) << 64_usize) + 13_u8));
    /// ```
    #[must_use]
    pub fn new(start: &num_bigint::BigUint) -> Self {
        use crate::Int;
        match start.to_u64() {
            Some(small) => Self {
                native: Some(prime_iterator(small)),
                last: num_bigint::BigUint::from(small.saturating_sub(1)),
            },
            None => Self {
                native: None,
                last: start - 1_u8,
            },
        }
    }
}

#[cfg(feature = "bigint")]
impl Iterator for BigPrimeIterator {
    type Item = num_bigint::BigUint;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(native) = &mut self.native {
            if let Some(prime) = native.next() {
                self.last = num_bigint::BigUint::from(prime);
                return Some(self.last.clone());
            }
            self.native = None;
        }
        self.last = crate::bigint::primality::next_prime(&self.last);
        Some(self.last.clone())
    }
}

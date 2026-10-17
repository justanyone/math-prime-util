/// Buffered prime cursor from `reference/mpu/lib/Math/Prime/Util.pm`, `prime_iterator`.
///
/// Each refill asks `primes` for about 10_000 primes, matching the Perl closure.
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

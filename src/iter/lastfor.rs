use std::ops::ControlFlow;

/// Stop the current closure loop.
///
/// See `reference/mpu/XS.xs`, `lastfor`. MPU sets `forexit` and the `for*`
/// loop stops after the callback returns. Outside a loop, MPU croaks. This
/// function is the `Break` value the callback returns. It does not panic.
///
/// ```
/// use math_prime_util::{forprimes, lastfor};
/// assert!(forprimes(2..=10, |_| lastfor()).is_break());
/// ```
#[must_use]
pub fn lastfor() -> ControlFlow<()> {
    ControlFlow::Break(())
}

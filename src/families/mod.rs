//! Lucky numbers from MPU 0.75.

mod lucky;
pub use lucky::*;

mod ramanujan_primes;
pub use ramanujan_primes::ramanujan_primes;

mod ramanujan_prime_count;
pub use ramanujan_prime_count::ramanujan_prime_count;

mod ramanujan_prime_count_approx;
pub use ramanujan_prime_count_approx::ramanujan_prime_count_approx;

mod ramanujan_prime_count_lower;
pub use ramanujan_prime_count_lower::ramanujan_prime_count_lower;

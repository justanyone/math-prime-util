//! Lucky numbers from MPU 0.75.

mod lucky;
pub use lucky::*;

mod perfect_powers;
pub use perfect_powers::*;

mod nth_prime_power;
pub use nth_prime_power::nth_prime_power;

mod nth_prime_power_approx;
pub use nth_prime_power_approx::nth_prime_power_approx;

mod nth_prime_power_lower;
pub use nth_prime_power_lower::nth_prime_power_lower;

mod nth_prime_power_upper;
pub use nth_prime_power_upper::nth_prime_power_upper;

mod twin_primes;
pub use twin_primes::twin_primes;
mod twin_prime_count;
pub use twin_prime_count::twin_prime_count;
mod twin_prime_count_approx;
pub use twin_prime_count_approx::twin_prime_count_approx;
mod nth_twin_prime;
pub use nth_twin_prime::nth_twin_prime;

mod nth_twin_prime_approx;
pub use nth_twin_prime_approx::nth_twin_prime_approx;
mod semi_primes;
pub use semi_primes::semi_primes;
mod semiprime_count;
pub use semiprime_count::semiprime_count;
mod semiprime_count_approx;
pub use semiprime_count_approx::semiprime_count_approx;

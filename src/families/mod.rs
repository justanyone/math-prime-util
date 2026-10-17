//! Lucky numbers from MPU 0.75.

mod lucky;
pub use lucky::*;

mod nth_ramanujan_prime;
pub use nth_ramanujan_prime::nth_ramanujan_prime;
mod nth_ramanujan_prime_approx;
pub use nth_ramanujan_prime_approx::nth_ramanujan_prime_approx;
mod nth_ramanujan_prime_lower;
pub use nth_ramanujan_prime_lower::nth_ramanujan_prime_lower;
mod ramanujan_prime_count_upper;
pub use ramanujan_prime_count_upper::ramanujan_prime_count_upper;

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

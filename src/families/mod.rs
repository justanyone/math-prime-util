//! Lucky numbers from MPU 0.75.

mod lucky;
pub use lucky::*;

mod is_prime_power;
pub use is_prime_power::is_prime_power;
mod next_prime_power;
pub use next_prime_power::next_prime_power;
mod prev_prime_power;
pub use prev_prime_power::prev_prime_power;
mod prime_powers;
pub use prime_powers::prime_powers;

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

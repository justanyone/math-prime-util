//! factor: see PLAN.md section 5. Filled by the port rows; the crate root re-exports `pub use factor::*`.

mod is_square_free;
pub use is_square_free::is_square_free;

mod is_semiprime;
pub use is_semiprime::is_semiprime;

mod is_almost_prime;
pub use is_almost_prime::is_almost_prime;

mod is_omega_prime;
pub use is_omega_prime::is_omega_prime;

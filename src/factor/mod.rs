//! factor: see PLAN.md section 5. Filled by the port rows; the crate root re-exports `pub use factor::*`.

#[allow(clippy::module_inception)] // Keep each function in its own named file.
mod factor;
pub use factor::factor;

mod factor_exp;
pub use factor_exp::factor_exp;

mod divisors;
pub use divisors::divisors;

mod trial_factor;
pub use trial_factor::trial_factor;

mod cheb_factor;
mod fermat_factor;
mod holf_factor;
mod is_almost_prime;
mod is_carmichael;
mod is_cyclic;
mod is_omega_prime;
mod is_perfect_number;
mod is_quasi_carmichael;
mod is_semiprime;
mod is_square_free;
mod lehman_factor;
mod pbrent_factor;
mod pminus1_factor;
mod pplus1_factor;
mod prho_factor;
mod prime_bigomega;
mod prime_omega;
mod prime_signature;
mod squfof_factor;
pub use cheb_factor::cheb_factor;
pub use fermat_factor::fermat_factor;
pub use holf_factor::holf_factor;
pub use is_almost_prime::is_almost_prime;
pub use is_carmichael::is_carmichael;
pub use is_cyclic::is_cyclic;
pub use is_omega_prime::is_omega_prime;
pub use is_perfect_number::is_perfect_number;
pub use is_quasi_carmichael::is_quasi_carmichael;
pub use is_semiprime::is_semiprime;
pub use is_square_free::is_square_free;
pub use lehman_factor::lehman_factor;
pub use pbrent_factor::pbrent_factor;
pub use pminus1_factor::pminus1_factor;
pub use pplus1_factor::pplus1_factor;
pub use prho_factor::prho_factor;
pub use prime_bigomega::prime_bigomega;
pub use prime_omega::prime_omega;
pub use prime_signature::prime_signature;
pub use squfof_factor::squfof_factor;

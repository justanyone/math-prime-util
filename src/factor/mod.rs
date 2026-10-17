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

//! factor: see PLAN.md section 5. Filled by the port rows; the crate root re-exports `pub use factor::*`.
mod prho_factor;
pub use prho_factor::prho_factor;
mod pbrent_factor;
pub use pbrent_factor::pbrent_factor;
mod pminus1_factor;
pub use pminus1_factor::pminus1_factor;
mod pplus1_factor;
pub use pplus1_factor::pplus1_factor;

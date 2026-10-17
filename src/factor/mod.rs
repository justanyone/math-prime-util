//! factor: see PLAN.md section 5. Filled by the port rows; the crate root re-exports `pub use factor::*`.
mod fermat_factor;
pub use fermat_factor::fermat_factor;
mod holf_factor;
pub use holf_factor::holf_factor;
mod lehman_factor;
pub use lehman_factor::lehman_factor;
mod squfof_factor;
pub use squfof_factor::squfof_factor;

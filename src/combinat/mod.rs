//! combinat: see PLAN.md section 5. Filled by the port rows; the crate root re-exports `pub use combinat::*`.

mod factorial;
mod fubini;
mod multifactorial;
mod subfactorial;

pub use factorial::factorial;
pub use fubini::fubini;
pub use multifactorial::multifactorial;
pub use subfactorial::subfactorial;

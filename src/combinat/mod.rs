//! combinat: see PLAN.md section 5. Filled by the port rows; the crate root re-exports `pub use combinat::*`.

mod bernfrac;
mod harmfrac;
mod hclassno;
mod is_happy;
mod is_pillai;
mod numtoperm;
mod pn_primorial;
mod primorial;

pub use bernfrac::bernfrac;
pub use harmfrac::harmfrac;
pub use hclassno::hclassno;
pub use is_happy::is_happy;
pub use is_pillai::is_pillai;
pub use numtoperm::numtoperm;
pub use pn_primorial::pn_primorial;
pub use primorial::primorial;

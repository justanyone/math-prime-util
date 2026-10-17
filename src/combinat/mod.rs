//! combinat: see PLAN.md section 5. Filled by the port rows; the crate root re-exports `pub use combinat::*`.

mod hclassno;
mod is_happy;
mod is_pillai;
mod numtoperm;

pub use hclassno::hclassno;
pub use is_happy::is_happy;
pub use is_pillai::is_pillai;
pub use numtoperm::numtoperm;

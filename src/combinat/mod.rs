//! combinat: see PLAN.md section 5. Filled by the port rows; the crate root re-exports `pub use combinat::*`.

mod consecutive_integer_lcm;
pub use consecutive_integer_lcm::consecutive_integer_lcm;
mod frobenius_number;
pub use frobenius_number::frobenius_number;
mod is_congruent_number;
pub use is_congruent_number::is_congruent_number;
mod is_polygonal;
pub use is_polygonal::is_polygonal;

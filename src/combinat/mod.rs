//! combinat: see PLAN.md section 5. Filled by the port rows; the crate root re-exports `pub use combinat::*`.

mod falling_factorial;
mod integer_complexity;
mod powersum;
mod rising_factorial;

pub use falling_factorial::falling_factorial;
pub use integer_complexity::integer_complexity;
pub use powersum::powersum;
pub use rising_factorial::rising_factorial;

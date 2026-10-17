//! factor: see PLAN.md section 5. Filled by the port rows; the crate root re-exports `pub use factor::*`.

mod is_practical;
mod is_rough;
mod is_smooth;
mod smooth_count;

pub use is_practical::is_practical;
pub use is_rough::is_rough;
pub use is_smooth::is_smooth;
pub use smooth_count::smooth_count;

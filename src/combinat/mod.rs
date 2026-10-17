//! combinat: see PLAN.md section 5. Filled by the port rows; the crate root re-exports `pub use combinat::*`.
mod binomial;
pub use binomial::binomial;
mod catalan_number;
pub use catalan_number::catalan_number;
mod stirling;
pub use stirling::stirling;
mod bell_number;
pub use bell_number::bell_number;

//! Bit positions in nonnegative integers with a `u64` value.
//!
//! Bit zero is the least significant bit. Positions above 63 hold zero.
//! The names are proposed in MPU 0.75 `TODO` under "Maybe MPU" and
//! "Also add". That release has no C, XS, or Perl implementation of them.
//! These native bounds are explicit Rust API choices, pending an MPU oracle.

mod bitclear;
mod bitflip;
mod bitscan0;
mod bitscan1;
mod bitset;
mod bittest;

pub use bitclear::bitclear;
pub use bitflip::bitflip;
pub use bitscan0::bitscan0;
pub use bitscan1::bitscan1;
pub use bitset::bitset;
pub use bittest::bittest;

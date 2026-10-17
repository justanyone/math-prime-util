//! Combinatorics from MPU 0.75.
//!
//! The ports follow `util.c`, `intcomplexity.c`, and `congruent_numbers.c`.
//! A result that does not fit in the return type is `None` (MPU `undef` on
//! the native path, or a bigint the fixed-width API cannot hold).

#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap
)]

mod arith;
mod special;

pub use arith::*;
pub use special::*;

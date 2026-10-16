//! A Rust port of Perl's [Math::Prime::Util](https://metacpan.org/pod/Math::Prime::Util) (MPU).
//!
//! The crate keeps MPU's function names so that code and documentation
//! written for the Perl module carry over, while following Rust conventions
//! for the signatures:
//!
//! - Inputs and results are `u64` on the native path (`u128` where MPU has
//!   128-bit paths).
//! - Where MPU returns `undef`, the Rust function returns [`Option`].
//! - Where MPU dies on an invalid argument (for example `rootint(n, 0)`), the
//!   Rust function panics, exactly like `u64::ilog` in the standard library
//!   does. Each such function documents its panics.
//! - Where MPU dies on a runtime failure (a factoring method giving up, a
//!   certificate failing to verify), the Rust function returns a `Result`.
//!
//! Every public function is re-exported at the crate root, so
//! `math_prime_util::is_prime(n)` works like `use Math::Prime::Util ':all'`.
//! The modules group the functions the way MPU's own source does.
//!
//! See `PLAN.md` and `STATUS.md` in the repository for what is ported so far.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]

pub mod int;
pub mod modular;

pub use int::{crootint, gcd, gcdext, is_power, is_square, lcm, logint, powint, rootint, sqrtint};
pub use modular::{addmod, invmod, muladdmod, mulmod, mulsubmod, negmod, powmod, sqrmod, submod};

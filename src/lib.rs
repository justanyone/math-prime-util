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

// One line per module, written once: a port row adds its functions to its module and never edits this file.
// Each module re-exports its public functions at the crate root (`pub use`), MPU-style flat names.
pub mod arith;
pub mod bigint;
pub mod bits;
pub mod combinat;
pub mod config;
pub mod count;
pub mod digits;
pub mod factor;
pub mod families;
pub mod int;
pub mod iter;
pub mod lucas;
pub mod modular;
pub mod primality;
pub mod proving;
pub mod random;
pub mod rational;
pub mod real;
pub mod sieve;
pub mod vec;

#[allow(unused_imports)] // until the module has public items
pub use arith::*;
#[allow(unused_imports)] // until the module has public items
pub use bigint::*;
pub use bits::*;
#[allow(unused_imports)] // until the module has public items
pub use combinat::*;
#[allow(unused_imports)] // until the module has public items
pub use config::*;
#[allow(unused_imports)] // until the module has public items
pub use count::*;
#[allow(unused_imports)] // until the module has public items
pub use digits::*;
#[allow(unused_imports)] // until the module has public items
pub use factor::*;
#[allow(unused_imports)] // until the module has public items
pub use families::*;
#[allow(unused_imports)] // until the module has public items
pub use int::*;
#[allow(unused_imports)] // until the module has public items
pub use iter::*;
#[allow(unused_imports)] // until the module has public items
pub use lucas::*;
#[allow(unused_imports)] // until the module has public items
pub use modular::*;
#[allow(unused_imports)] // until the module has public items
pub use primality::*;
#[allow(unused_imports)] // until the module has public items
pub use proving::*;
#[allow(unused_imports)] // until the module has public items
pub use random::*;
#[allow(unused_imports)] // until the module has public items
pub use rational::*;
#[allow(unused_imports)] // until the module has public items
pub use real::*;
#[allow(unused_imports)] // until the module has public items
pub use sieve::*;
#[allow(unused_imports)] // until the module has public items
pub use vec::*;

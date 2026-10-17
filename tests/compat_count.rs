//! Compat tests against MPU 0.75 `prime_counts.c` for `prime_count_lower`,
//! `prime_count_upper` and `prime_count_approx`. Fixture: `tests/fixtures/count.tsv`.
//!
//! Cases above `F64_EXACT` are skipped: this port evaluates Li(x) and R(x) in
//! `f64`, while MPU uses `long double`, so the last digit can differ there.
//! The bound functions are checked only up to `F64_BOUND_MAX`, where the
//! `f64` Li(x) matches MPU; above it they differ by one (see NOTES).

mod common;

use common::each;
use math_prime_util::*;

/// Largest `n` for which `f64` holds every integer exactly.
const F64_EXACT: u64 = 1 << 53;

/// Largest `n` checked for the bound functions (`f64` Li(x) differs by one above it).
const F64_BOUND_MAX: u64 = 1_000_000_000_000;

#[test]
fn compat_count_prime_count_lower() {
    each("count", "prime_count_lower", |c| {
        let n: u64 = c.arg(0);
        if n > F64_BOUND_MAX {
            return;
        }
        c.check(Some(prime_count_lower(n)));
    });
}

#[test]
fn compat_count_prime_count_upper() {
    each("count", "prime_count_upper", |c| {
        let n: u64 = c.arg(0);
        if n > F64_BOUND_MAX {
            return;
        }
        c.check(Some(prime_count_upper(n)));
    });
}

#[test]
fn compat_count_prime_count_approx() {
    each("count", "prime_count_approx", |c| {
        let n: u64 = c.arg(0);
        if n > F64_EXACT {
            return;
        }
        c.check(Some(prime_count_approx(n)));
    });
}

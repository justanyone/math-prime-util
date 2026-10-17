//! Compat tests against MPU 0.75 `prime_counts.c` for `prime_count_lower`,
//! `prime_count_upper` and `prime_count_approx`. Fixture: `tests/fixtures/count.tsv`.

mod common;

use common::each;
use math_prime_util::*;

#[test]
fn compat_count_prime_count_lower() {
    each("count", "prime_count_lower", |c| {
        let n: u64 = c.arg(0);
        c.check(Some(prime_count_lower(n)));
    });
}

#[test]
fn compat_count_prime_count_upper() {
    each("count", "prime_count_upper", |c| {
        let n: u64 = c.arg(0);
        c.check(Some(prime_count_upper(n)));
    });
}

#[test]
fn compat_count_prime_count_approx() {
    each("count", "prime_count_approx", |c| {
        let n: u64 = c.arg(0);
        c.check(Some(prime_count_approx(n)));
    });
}

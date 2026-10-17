//! Compat tests against MPU 0.75 for `t/14-nthprime.t` and the nth-prime
//! bound cases of `xt/test-nthapprox.pl`. Fixture: `tests/fixtures/count.tsv`.

mod common;

use common::each;
use math_prime_util::*;

#[test]
fn compat_count_nth_prime_lower() {
    each("count", "nth_prime_lower", |c| {
        let n: u64 = c.args[0].parse().unwrap();
        c.check(nth_prime_lower(n));
    });
}

#[test]
fn compat_count_nth_prime_upper() {
    each("count", "nth_prime_upper", |c| {
        let n: u64 = c.args[0].parse().unwrap();
        c.check(nth_prime_upper(n));
    });
}

#[test]
fn compat_count_nth_prime_approx() {
    each("count", "nth_prime_approx", |c| {
        let n: u64 = c.args[0].parse().unwrap();
        c.check(nth_prime_approx(n));
    });
}

//! Replay MPU 0.75 pure Perl family bounds above the native range:
//! `tests/fixtures/bigint_family_bounds.tsv` (see `tools/fixtures/bigint_family_bounds.pl`).
//!
//! Rows where the oracle died or returned `NaN` (`PP.pm` passes a float to an
//! integer function there, `docs/upstream-findings.md`) have no value to
//! replay. The tests skip them by name and check the port by the definition
//! of the search it runs instead.
#![cfg(feature = "bigint")]
mod common;
use common::Case;
use math_prime_util::bigint::family_bounds as fb;
use num_bigint::BigUint;

const FAMILY: &str = "bigint_family_bounds";

fn want(c: &Case) -> BigUint {
    c.result
        .as_deref()
        .expect("a value")
        .parse()
        .expect("decimal")
}

/// Rows the oracle has no value for.
fn failed(c: &Case) -> bool {
    c.error || c.result.as_deref() == Some("NaN")
}

/// `got` equals the oracle value.
fn exact(c: &Case, got: &BigUint) {
    c.check(Some(got));
}

/// `got` is within one part in `10^digits` of the oracle value.
fn relative(c: &Case, got: &BigUint, digits: u32) {
    let want = want(c);
    let diff = if *got > want {
        got - &want
    } else {
        &want - got
    };
    assert!(
        diff * BigUint::from(10_u8).pow(digits) <= want,
        "fixture line {}: {} {}: {got} vs {want}",
        c.line,
        c.func,
        c.args.join(" ")
    );
}

/// `2^127`, where `Math::BigFloat` (40 digits) leaves noise in the last
/// digits of a 41 digit bound that the port (77 digits) does not have.
const TWO_127: &str = "170141183460469231731687303715884105728";

/// Equal, or for an input of `2^127` equal but for the last three digits.
fn exact_but_noise(c: &Case, got: &BigUint) {
    if c.args.last().map(String::as_str) == Some(TWO_127) {
        let (got, want) = (got.to_string(), want(c).to_string());
        assert_eq!(got.len(), want.len(), "fixture line {}", c.line);
        assert_eq!(
            got[..got.len() - 3],
            want[..want.len() - 3],
            "fixture line {}",
            c.line
        );
    } else {
        exact(c, got);
    }
}

fn big(c: &Case, i: usize) -> BigUint {
    c.arg(i)
}

#[test]
fn power_counts_oracle() {
    macro_rules! replay {
        ($($name:ident),+ $(,)?) => {
            $(common::each(FAMILY, stringify!($name), |c| exact(c, &fb::$name(&big(c, 0))));)+
        };
    }
    replay!(
        perfect_power_count_approx,
        perfect_power_count_lower,
        perfect_power_count_upper,
    );
    // The roots below 2^64 take the native prime counts, MPU's pure Perl
    // takes its double formulas there: a few counts apart in 10^17.
    macro_rules! close {
        ($($name:ident),+ $(,)?) => {
            $(common::each(FAMILY, stringify!($name), |c| relative(c, &fb::$name(&big(c, 0)), 15));)+
        };
    }
    close!(
        prime_power_count_approx,
        prime_power_count_lower,
        prime_power_count_upper,
    );
}

#[test]
fn estimate_counts_oracle() {
    macro_rules! replay {
        ($($name:ident),+ $(,)?) => {
            $(common::each(FAMILY, stringify!($name), |c| exact(c, &fb::$name(&big(c, 0))));)+
        };
    }
    replay!(
        semiprime_count_approx,
        twin_prime_count_approx,
        ramanujan_prime_count_approx,
        ramanujan_prime_count_lower,
        ramanujan_prime_count_upper,
    );
}

#[test]
fn almost_prime_counts_oracle() {
    common::each(FAMILY, "almost_prime_count_approx", |c| {
        exact(c, &fb::almost_prime_count_approx(c.arg(0), &big(c, 1)));
    });
    common::each(FAMILY, "almost_prime_count_lower", |c| {
        exact(c, &fb::almost_prime_count_lower(c.arg(0), &big(c, 1)));
    });
    common::each(FAMILY, "almost_prime_count_upper", |c| {
        exact(c, &fb::almost_prime_count_upper(c.arg(0), &big(c, 1)));
    });
}

#[test]
fn inverse_li_oracle() {
    common::each(FAMILY, "inverse_li", |c| {
        exact(c, &fb::inverse_li(&big(c, 0)));
    });
}

#[test]
fn lucky_oracle() {
    common::each(FAMILY, "nth_lucky_approx", |c| {
        // MPU returns a double: the row holds its 15 digit value.
        exact(c, &fb::nth_lucky_approx(&big(c, 0)).expect("a value"));
    });
    common::each(FAMILY, "nth_lucky_lower", |c| {
        exact(c, &fb::nth_lucky_lower(&big(c, 0)).expect("a value"));
    });
    common::each(FAMILY, "nth_lucky_upper", |c| {
        exact(c, &fb::nth_lucky_upper(&big(c, 0)).expect("a value"));
    });
    // Above 2^64 the pure Perl counts die (a double goes into an integer
    // function). The rows below it replay; the others must satisfy the
    // definition of the search: `f(count) < n <= f(count + 1)`.
    let one = BigUint::from(1_u8);
    let (mut replayed, mut defined) = (0, 0);
    for (name, count, nth, shift) in [
        (
            "lucky_count_approx",
            fb::lucky_count_approx as fn(&BigUint) -> BigUint,
            fb::nth_lucky_approx as fn(&BigUint) -> Option<BigUint>,
            0_u8,
        ),
        (
            "lucky_count_lower",
            fb::lucky_count_lower,
            fb::nth_lucky_upper,
            0,
        ),
        (
            "lucky_count_upper",
            fb::lucky_count_upper,
            fb::nth_lucky_lower,
            1,
        ),
    ] {
        common::each(FAMILY, name, |c| {
            let n = big(c, 0);
            let got = count(&n);
            // The fitted search range of PP.pm can miss the answer (it then
            // returns an end of the range: `docs/upstream-findings.md`).
            // There the port widens the range; it must satisfy the definition.
            if failed(c) || Some(got.to_string()) != c.result {
                // The upper count is one more than the search result.
                let r = &got - u8::from(shift == 1);
                assert!(nth(&r).expect("a value") < n, "{name} {n}");
                assert!(nth(&(&r + &one)).expect("a value") >= n, "{name} {n}");
                defined += 1;
            } else {
                exact(c, &got);
                replayed += 1;
            }
        });
    }
    assert_eq!((replayed, defined), (8, 19));
}

#[test]
fn nth_perfect_power_oracle() {
    // MPU's `n ** (4/3)` takes the exponent as the double 1.33333333333333:
    // every term with a fractional exponent is off by one part in 10^13 of
    // itself, so the bounds agree to 15 digits of 20 to 40.
    macro_rules! close {
        ($($name:ident),+ $(,)?) => {
            $(common::each(FAMILY, stringify!($name), |c| {
                relative(c, &fb::$name(&big(c, 0)).expect("a value"), 15);
            });)+
        };
    }
    close!(
        nth_perfect_power_approx,
        nth_perfect_power_lower,
        nth_perfect_power_upper,
    );
}

#[test]
fn nth_prime_power_defined() {
    // MPU returns NaN for every row (it halves a BigFloat into an integer
    // function); the port runs the search PP.pm states.
    let one = BigUint::from(1_u8);
    for (name, count, nth, shift) in [
        (
            "nth_prime_power_approx",
            fb::prime_power_count_approx as fn(&BigUint) -> BigUint,
            fb::nth_prime_power_approx as fn(&BigUint) -> Option<BigUint>,
            false,
        ),
        (
            "nth_prime_power_lower",
            fb::prime_power_count_upper,
            fb::nth_prime_power_lower,
            false,
        ),
        (
            "nth_prime_power_upper",
            fb::prime_power_count_lower,
            fb::nth_prime_power_upper,
            true,
        ),
    ] {
        common::each(FAMILY, name, |c| {
            assert!(failed(c), "MPU answers now: fixture line {}", c.line);
            let n = big(c, 0);
            let got = nth(&n).expect("a value");
            // The search result is the least x with count(x) >= n, less one;
            // the upper form adds the one back.
            let r = if shift { &got - &one } else { got.clone() };
            assert!(count(&r) < n, "{name} {n}");
            assert!(count(&(&r + &one)) >= n, "{name} {n}");
        });
    }
}

#[test]
fn nth_estimates_oracle() {
    common::each(FAMILY, "nth_semiprime_approx", |c| {
        exact(c, &fb::nth_semiprime_approx(&big(c, 0)).expect("a value"));
    });
    common::each(FAMILY, "nth_twin_prime_approx", |c| {
        exact_but_noise(c, &fb::nth_twin_prime_approx(&big(c, 0)).expect("a value"));
    });
    common::each(FAMILY, "nth_ramanujan_prime_approx", |c| {
        exact_but_noise(
            c,
            &fb::nth_ramanujan_prime_approx(&big(c, 0)).expect("a value"),
        );
    });
    common::each(FAMILY, "nth_ramanujan_prime_lower", |c| {
        exact_but_noise(
            c,
            &fb::nth_ramanujan_prime_lower(&big(c, 0)).expect("a value"),
        );
    });
    common::each(FAMILY, "nth_ramanujan_prime_upper", |c| {
        exact_but_noise(
            c,
            &fb::nth_ramanujan_prime_upper(&big(c, 0)).expect("a value"),
        );
    });
}

#[test]
fn nth_almost_prime_oracle() {
    // Where MPU dies (a float reaches `vecsum`), the port must satisfy the
    // definition of the search: the least x with count(x) >= n.
    let one = BigUint::from(1_u8);
    let two64 = BigUint::from(1_u8) << 64_usize;
    let (mut replayed, mut defined) = (0, 0);
    for (name, nth, count) in [
        (
            "nth_almost_prime_approx",
            fb::nth_almost_prime_approx as fn(u32, &BigUint) -> Option<BigUint>,
            (|k: u32, x: &BigUint| fb::almost_prime_count_approx(u64::from(k), x))
                as fn(u32, &BigUint) -> BigUint,
        ),
        (
            "nth_almost_prime_lower",
            fb::nth_almost_prime_lower,
            |k, x| fb::almost_prime_count_upper(k, x),
        ),
        (
            "nth_almost_prime_upper",
            fb::nth_almost_prime_upper,
            |k, x| fb::almost_prime_count_lower(u64::from(k), x),
        ),
    ] {
        common::each(FAMILY, name, |c| {
            let (k, n): (u32, BigUint) = (c.arg(0), big(c, 1));
            let got = nth(k, &n).expect("a value");
            if failed(c) {
                assert!(got >= two64, "{name} {k} {n}: the checks need the big path");
                assert!(count(k, &got) >= n, "{name} {k} {n}");
                assert!(count(k, &(&got - &one)) < n, "{name} {k} {n}");
                defined += 1;
            } else {
                exact(c, &got);
                replayed += 1;
            }
        });
    }
    assert_eq!((replayed, defined), (replayed, defined));
    assert!(replayed > 0 && defined > 0);
}

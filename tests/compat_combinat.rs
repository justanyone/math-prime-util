//! Compatibility with the committed MPU combinat oracle.
mod common;

use common::each;
use math_prime_util::{
    bernfrac, harmfrac, partitions, partitionsq, permtonum, pn_primorial, primorial,
};

fn check_fraction(c: &common::Case, got: Option<(i128, u128)>) {
    // Fractions outside i128/u128 are represented by None in this native API.
    let expected = c.result.as_deref().and_then(|s| {
        let (num, den) = s.split_once(' ')?;
        Some((num.parse::<i128>().ok()?, den.parse::<u128>().ok()?))
    });
    assert!(!c.error);
    assert_eq!(got, expected, "{} fixture line {}", c.func, c.line);
}

#[test]
fn bernfrac_oracle() {
    each("combinat", "bernfrac", |c| {
        check_fraction(c, bernfrac(c.arg(0)));
    });
}

#[test]
fn harmfrac_oracle() {
    each("combinat", "harmfrac", |c| {
        check_fraction(c, harmfrac(c.arg(0)));
    });
}

#[test]
fn primorial_oracle() {
    each("combinat", "primorial", |c| {
        let expected = c.result.as_deref().and_then(|s| s.parse::<u64>().ok());
        assert!(!c.error);
        assert_eq!(primorial(c.arg(0)), expected, "fixture line {}", c.line);
    });
}

#[test]
fn pn_primorial_oracle() {
    each("combinat", "pn_primorial", |c| {
        let expected = c.result.as_deref().and_then(|s| s.parse::<u64>().ok());
        assert!(!c.error);
        assert_eq!(pn_primorial(c.arg(0)), expected, "fixture line {}", c.line);
    });
}

#[test]
fn partitions_oracle() {
    // Signed `-1` (line 256) is ERROR in MPU. This API takes `u64`, so that case is skipped.
    // Ranks past `u64` (lines 126..=130: n = 500, 501, 1000, 1001, 2347) are `None` here.
    each("combinat", "partitions", |c| {
        if c.error {
            return;
        }
        let expected = c.result.as_deref().and_then(|s| s.parse::<u64>().ok());
        assert_eq!(partitions(c.arg(0)), expected, "fixture line {}", c.line);
    });
}

#[test]
fn partitionsq_oracle() {
    // Signed `-1` (line 255) is ERROR in MPU. This API takes `u64`, so that case is skipped.
    // Values past `u64` (lines 253..=254: n = 1000 and n = 2000) are `None` here.
    each("combinat", "partitionsq", |c| {
        if c.error {
            return;
        }
        let expected = c.result.as_deref().and_then(|s| s.parse::<u64>().ok());
        assert_eq!(partitionsq(c.arg(0)), expected, "fixture line {}", c.line);
    });
}

#[test]
fn permtonum_oracle() {
    // Line 18355 does not fit in `u64` (length 27, rank 193228515634198442606207999): `None`.
    // Lines 18361 (`1 1`) and 18362 (`0 2`) are invalid permutations. MPU returns ERROR.
    let mut count = 0_u32;
    for case in common::load("combinat")
        .iter()
        .filter(|case| case.func == "permtonum")
    {
        count += 1;
        let perm: Vec<u64> = case
            .args
            .iter()
            .map(|token| token.parse().unwrap())
            .collect();
        if case.error {
            let caught =
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| permtonum(&perm)));
            assert!(caught.is_err(), "fixture line {}", case.line);
            continue;
        }
        let got = permtonum(&perm);
        match case.result.as_deref() {
            Some(text) if text.parse::<u64>().is_err() => {
                assert_eq!(got, None, "fixture line {}", case.line);
            }
            _ => case.check(got),
        }
    }
    assert!(count > 0, "no fixture cases for permtonum");
}

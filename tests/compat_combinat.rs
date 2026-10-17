//! Compatibility with the committed MPU combinat oracle.
mod common;

use common::{Case, each};
use math_prime_util::{
<<<<<<< HEAD
    bell_number, bernfrac, binomial, catalan_number, factorial, falling_factorial, fubini,
    harmfrac, hclassno, integer_complexity, is_happy, is_pillai, multifactorial, numtoperm,
    pn_primorial, powersum, primorial, rising_factorial, stirling, subfactorial,
=======
    bell_number, bernfrac, binomial, catalan_number, consecutive_integer_lcm, factorial,
    falling_factorial, frobenius_number, fubini, harmfrac, integer_complexity, is_congruent_number,
    is_polygonal, multifactorial, partitions, partitionsq, permtonum, pn_primorial, powersum,
    primorial, rising_factorial, stirling, subfactorial,
>>>>>>> origin/dev-001
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

fn check_unsigned(c: &common::Case, got: Option<u64>) {
    // The oracle also contains bigint answers; the native API returns None.
    let expected = c.result.as_ref().and_then(|s| s.parse::<u64>().ok());
    assert!(!c.error);
    assert_eq!(got, expected, "fixture line {}: {c:?}", c.line);
}

#[test]
fn binomial_oracle() {
    each("combinat", "binomial", |c| {
        // Signed and bigint binomial arguments are outside this u64 API.
        if c.args.iter().any(|a| a.parse::<u64>().is_err()) {
            return;
        }
        check_unsigned(c, binomial(c.arg(0), c.arg(1)));
    });
}

#[test]
fn catalan_number_oracle() {
    each("combinat", "catalan_number", |c| {
        check_unsigned(c, catalan_number(c.arg(0)));
    });
}

#[test]
fn stirling_oracle() {
    each("combinat", "stirling", |c| {
        let kind = if c.args.len() == 3 { c.arg(2) } else { 1 };
        let got = std::panic::catch_unwind(|| stirling(c.arg(0), c.arg(1), kind));
        if c.error {
            assert!(got.is_err(), "fixture line {}", c.line);
        } else {
            let expected = c
                .result
                .as_ref()
                .and_then(|s| s.parse::<i128>().ok())
                .filter(|v| v.unsigned_abs() <= u128::from(u64::MAX));
            assert_eq!(got.unwrap(), expected, "fixture line {}: {c:?}", c.line);
        }
    });
}

#[test]
fn bell_number_oracle() {
    each("combinat", "bell_number", |c| {
        check_unsigned(c, bell_number(c.arg(0)));
    });
}

fn check_native(case: &Case, function: impl FnOnce(&[u64]) -> Option<u64>) {
    let Ok(args) = case
        .args
        .iter()
        .map(|arg| arg.parse())
        .collect::<Result<Vec<u64>, _>>()
    else {
        // Skip negative factorial/subfactorial/multifactorial arguments and
        // the multifactorial case whose n and k exceed u64.
        return;
    };
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| function(&args)));
    if case.error {
        assert!(result.is_err(), "expected panic: {case:?}");
    } else {
        // MPU's bigint answers beyond u64 map to the native API's None.
        let expected = case
            .result
            .as_ref()
            .and_then(|value| value.parse::<u64>().ok());
        assert_eq!(result.unwrap(), expected, "{case:?}");
    }
}

#[test]
fn fubini_oracle() {
    each("combinat", "fubini", |c| check_native(c, |a| fubini(a[0])));
}

#[test]
fn factorial_oracle() {
    each("combinat", "factorial", |c| {
        check_native(c, |a| factorial(a[0]));
    });
}

#[test]
fn multifactorial_oracle() {
    each("combinat", "multifactorial", |c| {
        check_native(c, |a| multifactorial(a[0], a[1]));
    });
}

#[test]
fn subfactorial_oracle() {
    each("combinat", "subfactorial", |c| {
        check_native(c, |a| subfactorial(a[0]));
    });
}

fn check_binary(name: &str, function: fn(u64, u64) -> Option<u64>) {
    each("combinat", name, |case| {
        // Skip negative n (signed factorial API), and bigint n or m inputs.
        let (Ok(n), Ok(m)) = (case.args[0].parse::<u64>(), case.args[1].parse::<u64>()) else {
            return;
        };
        assert!(!case.error, "unexpected oracle error: {case:?}");
        // MPU promotes overflowing results to bigint; this native API returns None.
        let expected = case
            .result
            .as_ref()
            .and_then(|value| value.parse::<u64>().ok());
        assert_eq!(function(n, m), expected, "{case:?}");
    });
}

#[test]
fn falling_factorial_oracle() {
    check_binary("falling_factorial", falling_factorial);
}

#[test]
fn rising_factorial_oracle() {
    check_binary("rising_factorial", rising_factorial);
}

#[test]
fn powersum_oracle() {
    check_binary("powersum", powersum);
}

#[test]
fn integer_complexity_oracle() {
    each("combinat", "integer_complexity", |case| {
        case.check(integer_complexity(case.arg(0)));
    });
}
<<<<<<< HEAD

#[test]
fn is_pillai_oracle() {
    each("combinat", "is_pillai", |c| {
        c.check(Some(is_pillai(c.arg(0))));
    });
}

#[test]
fn is_happy_oracle() {
    each("combinat", "is_happy", |c| {
        // Skip is_happy arguments above u64::MAX; the native API takes u64.
        let Ok(n) = c.args[0].parse::<u64>() else {
            return;
        };
        let base = c.args.get(1).map_or(10, |s| s.parse().unwrap());
        let exponent = c.args.get(2).map_or(2, |s| s.parse().unwrap());
        c.check(Some(is_happy(n, base, exponent)));
    });
}

#[test]
fn hclassno_oracle() {
    each("combinat", "hclassno", |c| {
        // Skip hclassno(-3), hclassno(-2), hclassno(-1): unsigned native input.
        if c.args[0].starts_with('-') {
            return;
        }
        c.check(hclassno(c.arg(0)));
=======
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
>>>>>>> origin/dev-001
    });
}

#[test]
<<<<<<< HEAD
fn numtoperm_oracle() {
    each("combinat", "numtoperm", |c| {
        // Skip numtoperm(5, -1) and numtoperm(5, 100000000000000000000):
        // the native rank is unsigned and limited to u64.
        let Ok(rank) = c.args[1].parse::<u64>() else {
            return;
        };
        let value = numtoperm(c.arg(0), rank)
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ");
        c.check(Some(value));
=======
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
            assert!(permtonum(&perm).is_err(), "fixture line {}", case.line);
            continue;
        }
        let got = permtonum(&perm).expect("valid fixture permutation");
        match case.result.as_deref() {
            Some(text) if text.parse::<u64>().is_err() => {
                assert_eq!(got, None, "fixture line {}", case.line);
            }
            _ => case.check(got),
        }
    }
    assert!(count > 0, "no fixture cases for permtonum");
}

#[test]
fn consecutive_integer_lcm_oracle() {
    each("combinat", "consecutive_integer_lcm", |case| {
        if let Ok(n) = case.args[0].parse() {
            // MPU promotes overflow to bigint; this API returns None.
            let expected = case.result.as_ref().and_then(|r| r.parse::<u64>().ok());
            assert_eq!(consecutive_integer_lcm(n), expected, "{case:?}");
        }
        // Skip bigint arguments outside the u64 API.
    });
}

#[test]
fn frobenius_number_oracle() {
    each("combinat", "frobenius_number", |case| {
        let values: Vec<u64> = case.args.iter().map(|a| a.parse().unwrap()).collect();
        // Skip the oracle resource failure for the ten-billion-entry table.
        // The fixture records a resource failure, outside the numeric contract.
        if case.error && values.first() == Some(&10_000_000_000) {
            return;
        }
        let result = std::panic::catch_unwind(|| frobenius_number(&values));
        if case.error {
            assert!(result.is_err(), "{case:?}");
            return;
        }
        // MPU promotes overflow to bigint; this API returns None.
        let expected = case
            .result
            .as_ref()
            .and_then(|r| r.parse::<i128>().ok())
            .filter(|&n| n == -1 || u64::try_from(n).is_ok());
        assert_eq!(result.unwrap(), expected, "{case:?}");
    });
}

#[test]
fn is_congruent_number_oracle() {
    each("combinat", "is_congruent_number", |case| {
        // Skip -3: signed inputs are outside the u64 API.
        if let Ok(n) = case.args[0].parse() {
            case.check(Some(u8::from(is_congruent_number(n))));
        }
    });
}

#[test]
fn is_polygonal_oracle() {
    each("combinat", "is_polygonal", |case| {
        // Skip signed n and bigint n or k: these are outside the u64 API.
        if let (Ok(n), Ok(k)) = (case.args[0].parse(), case.args[1].parse()) {
            let result = std::panic::catch_unwind(|| is_polygonal(n, k));
            if case.error {
                assert!(result.is_err(), "{case:?}");
            } else {
                case.check(Some(u8::from(result.unwrap())));
            }
        }
>>>>>>> origin/dev-001
    });
}

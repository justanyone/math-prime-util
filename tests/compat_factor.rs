//! Compatibility tests for MPU's bounded factor methods and the factor predicates from MPU 0.75.
mod common;

// The u64 API skips signed arguments (including -0) and the bigint cases
// 18450931744652885087, 36803912137803496523, and 36832540935240707603.
use common::each;
use math_prime_util::{
    is_carmichael, is_cyclic, is_perfect_number, is_quasi_carmichael, pbrent_factor,
    pminus1_factor, pplus1_factor, prho_factor,
};

fn check(name: &str, method: impl Fn(u64, Option<u64>, Option<u64>) -> Vec<u64>) {
    common::each("factor", name, |c| {
        // Signed and wider-than-u64 cases cannot be passed to this API.
        // pbrent_factor(91, "10foo") is a Perl parsing error; Rust bounds are u64.
        if c.error {
            assert_eq!(c.func, "pbrent_factor");
            assert_eq!(c.args, ["91", "10foo"]);
            return;
        }
        let Ok(n) = c.args[0].parse::<u64>() else {
            return;
        };
        let a = c.args.get(1).map(|s| s.parse().unwrap());
        let b = c.args.get(2).map(|s| s.parse().unwrap());
        let factors = method(n, a, b);
        let result = (!factors.is_empty()).then(|| {
            factors
                .iter()
                .map(u64::to_string)
                .collect::<Vec<_>>()
                .join(" ")
        });
        c.check(result);
    });
}

#[test]
fn prho_factor_oracle() {
    check("prho_factor", |n, a, _| prho_factor(n, a));
}
#[test]
fn pbrent_factor_oracle() {
    check("pbrent_factor", pbrent_factor);
}
#[test]
fn pminus1_factor_oracle() {
    check("pminus1_factor", pminus1_factor);
}
#[test]
fn pplus1_factor_oracle() {
    check("pplus1_factor", |n, a, _| pplus1_factor(n, a));
}

#[test]
fn is_carmichael_oracle() {
    each("factor", "is_carmichael", |c| {
        if let Ok(n) = c.args[0].parse::<u64>() {
            c.check(Some(u8::from(is_carmichael(n))));
        }
    });
}

#[test]
fn is_quasi_carmichael_oracle() {
    each("factor", "is_quasi_carmichael", |c| {
        if let Ok(n) = c.args[0].parse::<u64>() {
            c.check(Some(is_quasi_carmichael(n)));
        }
    });
}

#[test]
fn is_cyclic_oracle() {
    each("factor", "is_cyclic", |c| {
        if let Ok(n) = c.args[0].parse::<u64>() {
            c.check(Some(u8::from(is_cyclic(n))));
        }
    });
}

#[test]
fn is_perfect_number_oracle() {
    each("factor", "is_perfect_number", |c| {
        if let Ok(n) = c.args[0].parse::<u64>() {
            c.check(Some(u8::from(is_perfect_number(n))));
        }
    });
}

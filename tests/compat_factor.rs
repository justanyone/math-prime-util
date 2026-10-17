//! Compatibility tests for MPU's bounded factor methods.
mod common;
use math_prime_util::{pbrent_factor, pminus1_factor, pplus1_factor, prho_factor};

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

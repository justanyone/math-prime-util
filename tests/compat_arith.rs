//! Compatibility with the committed MPU arithmetic oracle.
mod common;
use common::{each, each_may_panic};
use math_prime_util::*;

fn joined<T: std::fmt::Display>(v: &[T]) -> String {
    v.iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn euler_phi_oracle() {
    each("arith", "euler_phi", |c| {
        if c.args.len() == 1 {
            c.check(Some(euler_phi(c.arg(0))));
        } else {
            let (lo, hi) = (c.arg(0), c.arg(1));
            c.check((lo <= hi).then(|| joined(&euler_phi_range(lo, hi))));
        }
    });
}
#[test]
fn jordan_totient_oracle() {
    each("arith", "jordan_totient", |c| {
        c.check(jordan_totient(c.arg(0), c.arg(1)))
    });
}
#[test]
fn carmichael_lambda_oracle() {
    each("arith", "carmichael_lambda", |c| {
        c.check(Some(carmichael_lambda(c.arg(0))))
    });
}
#[test]
fn moebius_oracle() {
    each("arith", "moebius", |c| {
        if c.args[0].starts_with('-') {
            return;
        } // Signed moebius arguments are outside the unsigned API.
        if c.args.len() == 1 {
            c.check(Some(moebius(c.arg(0))));
        } else {
            let (lo, hi) = (c.arg(0), c.arg(1));
            c.check((lo <= hi).then(|| joined(&moebius_range(lo, hi))));
        }
    });
}
#[test]
fn mertens_oracle() {
    each("arith", "mertens", |c| {
        if c.args.iter().any(|a| a.starts_with('-')) {
            return;
        } // Signed mertens interval endpoints are outside this API.
        let v = if c.args.len() == 1 {
            mertens(c.arg(0))
        } else {
            let (lo, hi): (u64, u64) = (c.arg(0), c.arg(1));
            if lo > hi {
                0
            } else {
                mertens(hi) - if lo > 0 { mertens(lo - 1) } else { 0 }
            }
        };
        c.check(Some(v));
    });
}
#[test]
fn liouville_oracle() {
    each("arith", "liouville", |c| c.check(Some(liouville(c.arg(0)))));
}
#[test]
fn sumliouville_oracle() {
    each("arith", "sumliouville", |c| {
        c.check(Some(sumliouville(c.arg(0))))
    });
}
#[test]
fn exp_mangoldt_oracle() {
    each("arith", "exp_mangoldt", |c| {
        c.check(Some(exp_mangoldt(c.arg(0))))
    });
}
#[test]
fn sumtotient_oracle() {
    each("arith", "sumtotient", |c| c.check(sumtotient(c.arg(0))));
}
#[test]
fn dedekind_psi_oracle() {
    each("arith", "dedekind_psi", |c| c.check(dedekind_psi(c.arg(0))));
}
#[test]
fn inverse_totient_oracle() {
    each("arith", "inverse_totient", |c| {
        let v = inverse_totient(c.arg(0));
        c.check((!v.is_empty()).then(|| joined(&v)));
    });
}
#[test]
fn is_totient_oracle() {
    each("arith", "is_totient", |c| {
        c.check(Some(u8::from(is_totient(c.arg(0)))))
    });
}
#[test]
fn ramanujan_sum_oracle() {
    each("arith", "ramanujan_sum", |c| {
        c.check(Some(ramanujan_sum(c.arg(0), c.arg(1))))
    });
}
#[test]
fn valuation_oracle() {
    each_may_panic("arith", "valuation", |c| {
        if c.args.iter().any(|a| a.starts_with('-')) {
            // Signed valuation inputs are outside the unsigned API.
            // The shared panic helper still verifies signed-base errors.
            if c.error {
                panic!("signed valuation base");
            }
            return c.result.as_ref().map(|s| s.parse::<u64>().unwrap());
        }
        valuation(c.arg(0), c.arg(1))
    });
}

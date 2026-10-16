//! Compat tests against MPU for `t/18-50-sqrtint.t`, `t/18-52-rootint.t`,
//! `t/18-60-logint.t`, `t/18-24-powint.t`, `t/26-ispower.t`, `t/19-gcd.t`
//! and the unsigned half of `t/26-modops.t`. Fixture: `tests/fixtures/int.tsv`.

mod common;

use common::{each, each_may_panic};
use math_prime_util::*;

#[test]
fn sqrtint_oracle() {
    each("int", "sqrtint", |c| c.check(Some(sqrtint(c.arg(0)))));
}

#[test]
fn is_square_oracle() {
    each("int", "is_square", |c| {
        c.check(Some(u8::from(is_square(c.arg(0)))));
    });
}

#[test]
fn rootint_oracle() {
    each_may_panic("int", "rootint", |c| Some(rootint(c.arg(0), c.arg(1))));
}

#[test]
fn crootint_oracle() {
    each_may_panic("int", "crootint", |c| Some(crootint(c.arg(0), c.arg(1))));
}

#[test]
fn is_power_oracle() {
    each("int", "is_power", |c| {
        c.check(Some(u8::from(is_power(c.arg(0), c.arg(1)))));
    });
}

#[test]
fn is_power_exp_oracle() {
    each("int", "is_power_exp", |c| {
        c.check(Some(int::is_power_exp(c.arg(0))));
    });
}

#[test]
fn powint_oracle() {
    each("int", "powint", |c| c.check(powint(c.arg(0), c.arg(1))));
}

#[test]
fn logint_oracle() {
    each_may_panic("int", "logint", |c| Some(logint(c.arg(0), c.arg(1))));
}

#[test]
fn gcd_oracle() {
    each("int", "gcd", |c| c.check(Some(gcd(c.arg(0), c.arg(1)))));
}

#[test]
fn lcm_oracle() {
    each("int", "lcm", |c| c.check(lcm(c.arg(0), c.arg(1))));
}

#[test]
fn gcdext_oracle() {
    each("int", "gcdext", |c| {
        let (g, u, v) = gcdext(c.arg(0), c.arg(1));
        c.check(Some(format!("{g} {u} {v}")));
    });
}

#[test]
fn modops_oracle() {
    each("int", "negmod", |c| c.check(negmod(c.arg(0), c.arg(1))));
    each("int", "invmod", |c| c.check(invmod(c.arg(0), c.arg(1))));
    each("int", "sqrmod", |c| c.check(sqrmod(c.arg(0), c.arg(1))));
    each("int", "addmod", |c| {
        c.check(addmod(c.arg(0), c.arg(1), c.arg(2)));
    });
    each("int", "submod", |c| {
        c.check(submod(c.arg(0), c.arg(1), c.arg(2)));
    });
    each("int", "mulmod", |c| {
        c.check(mulmod(c.arg(0), c.arg(1), c.arg(2)));
    });
    each("int", "powmod", |c| {
        c.check(powmod(c.arg(0), c.arg(1), c.arg(2)));
    });
    each("int", "muladdmod", |c| {
        c.check(muladdmod(c.arg(0), c.arg(1), c.arg(2), c.arg(3)));
    });
    each("int", "mulsubmod", |c| {
        c.check(mulsubmod(c.arg(0), c.arg(1), c.arg(2), c.arg(3)));
    });
}

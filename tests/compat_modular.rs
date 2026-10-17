//! MPU modular fixtures for the native unsigned API.
mod common;
use math_prime_util::{allrootmod, allsqrtmod, cornacchia, divmod, rootmod, sqrtmod};

fn cases(name: &str, f: impl Fn(common::Case)) {
    let mut count = 0;
    for c in common::load("modular")
        .into_iter()
        .filter(|c| c.func == name)
    {
        // Skip signed arguments and bigint arguments: the native API takes u64.
        if c.args.iter().any(|a| a.parse::<u64>().is_err()) {
            continue;
        }
        f(c);
        count += 1;
    }
    assert!(count > 0);
}
fn roots(c: &common::Case, values: &[u64]) {
    if c.arg::<u64>(c.args.len() - 1) == 0 {
        c.check::<String>(None);
    } else {
        c.check(Some(
            values
                .iter()
                .map(u64::to_string)
                .collect::<Vec<_>>()
                .join(" "),
        ));
    }
}
#[test]
fn sqrtmod_oracle() {
    cases("sqrtmod", |c| c.check(sqrtmod(c.arg(0), c.arg(1))));
}
#[test]
fn allsqrtmod_oracle() {
    cases("allsqrtmod", |c| roots(&c, &allsqrtmod(c.arg(0), c.arg(1))));
}
#[test]
fn rootmod_oracle() {
    cases("rootmod", |c| {
        c.check(rootmod(c.arg(0), c.arg(1), c.arg(2)))
    });
}
#[test]
fn allrootmod_oracle() {
    cases("allrootmod", |c| {
        roots(&c, &allrootmod(c.arg(0), c.arg(1), c.arg(2)))
    });
}
#[test]
fn divmod_oracle() {
    cases("divmod", |c| c.check(divmod(c.arg(0), c.arg(1), c.arg(2))));
}
#[test]
fn cornacchia_oracle() {
    cases("cornacchia", |c| {
        c.check(cornacchia(c.arg(0), c.arg(1)).map(|(x, y)| format!("{x} {y}")))
    });
}

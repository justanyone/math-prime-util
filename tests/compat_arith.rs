//! Arithmetic compatibility with the committed MPU oracle.
mod common;
use common::each;

macro_rules! unary_option {
    ($($f:ident),*) => {$ (
        #[test]
        fn $f() { each("arith", stringify!($f), |c| {
            // ramanujan_tau(-5) and negative znprimroot moduli need signed APIs.
            if c.args[0].starts_with('-') { return; }
            c.check(math_prime_util::$f(c.arg(0)));
        }); }
    )*};
}
unary_option!(ramanujan_tau, aliquot_sum, abundance, znprimroot, qnr);
#[test]
fn divisor_sum() {
    each("arith", "divisor_sum", |c| {
        // Results above u64::MAX need the future bigint divisor_sum API.
        if c.result
            .as_ref()
            .is_some_and(|s| s.parse::<u128>().is_ok_and(|v| v > u128::from(u64::MAX)))
        {
            return;
        }
        c.check(math_prime_util::divisor_sum(
            c.arg(0),
            if c.args.len() == 1 { 1 } else { c.arg(1) },
        ));
    });
}
#[test]
fn sopfr() {
    each("arith", "sopfr", |c| {
        c.check(Some(math_prime_util::sopfr(c.arg(0))));
    });
}
#[test]
fn sopf() {
    each("arith", "sopf", |c| {
        c.check(Some(math_prime_util::sopf(c.arg(0))));
    });
}
#[test]
fn kronecker() {
    each("arith", "kronecker", |c| {
        // Unsigned values above i64::MAX need a separate unsigned Kronecker API.
        if c.args.iter().any(|s| s.parse::<i64>().is_err()) {
            return;
        }
        c.check(Some(math_prime_util::kronecker(c.arg(0), c.arg(1))));
    });
}
#[test]
fn is_fundamental() {
    each("arith", "is_fundamental", |c| {
        // Values above i64::MAX need an unsigned discriminant API.
        if c.args[0].parse::<i64>().is_err() {
            return;
        }
        c.check(Some(u8::from(math_prime_util::is_fundamental(c.arg(0)))));
    });
}
#[test]
fn znorder() {
    each("arith", "znorder", |c| {
        c.check(math_prime_util::znorder(c.arg(0), c.arg(1)));
    });
}
#[test]
fn is_primitive_root() {
    each("arith", "is_primitive_root", |c| {
        // Negative primitive-root residues need a signed-residue API.
        if c.args.iter().any(|s| s.starts_with('-')) {
            return;
        }
        c.check(math_prime_util::is_primitive_root(c.arg(0), c.arg(1)).map(u8::from));
    });
}
#[test]
fn is_qr() {
    each("arith", "is_qr", |c| {
        // Negative quadratic residues need a signed-residue API.
        if c.args.iter().any(|s| s.starts_with('-')) {
            return;
        }
        c.check(Some(u8::from(math_prime_util::is_qr(c.arg(0), c.arg(1)))));
    });
}
fn pairs(c: &common::Case) -> Option<Vec<(u64, u64)>> {
    // Negative Chinese residues need a signed-residue API.
    c.args
        .chunks_exact(2)
        .map(|p| Some((p[0].parse().ok()?, p[1].parse().ok()?)))
        .collect()
}
#[test]
fn chinese() {
    each("arith", "chinese", |c| {
        if let Some(p) = pairs(c) {
            c.check(math_prime_util::chinese(&p));
        }
    });
}
#[test]
fn chinese2() {
    each("arith", "chinese2", |c| {
        if let Some(p) = pairs(c) {
            c.check(math_prime_util::chinese2(&p).map(|(r, m)| format!("{r} {m}")));
        }
    });
}

/// `inverse_sigma0` cases: `k hi` uses `lo = 1`; `k lo hi` is explicit.
fn inverse_range(c: &common::Case) -> (u64, u64, u64) {
    if c.args.len() == 2 {
        (c.arg(0), 1, c.arg(1))
    } else {
        (c.arg(0), c.arg(1), c.arg(2))
    }
}

#[test]
fn inverse_sigma0_oracle() {
    each("arith", "inverse_sigma0", |c| {
        let (k, lo, hi) = inverse_range(c);
        let got = math_prime_util::inverse_sigma0(k, lo, hi)
            .map(|v| v.iter().map(u64::to_string).collect::<Vec<_>>().join(" "));
        c.check(got);
    });
}

#[test]
fn inverse_sigma0_count_oracle() {
    each("arith", "inverse_sigma0_count", |c| {
        let (k, lo, hi) = inverse_range(c);
        c.check(math_prime_util::inverse_sigma0_count(k, lo, hi));
    });
}

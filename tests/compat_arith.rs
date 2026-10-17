//! Compatibility with the committed MPU arithmetic oracle.
//! Arithmetic compatibility with the committed MPU oracle.
mod common;
use common::each;
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
        if c.args[0].starts_with('-') {
            return;
        } // Signed euler_phi inputs are outside this API.
        if c.args.len() == 1 {
            c.check(Some(euler_phi(c.arg(0))));
        } else {
            let (lo, hi) = (c.arg(0), c.arg(1));
            c.check((lo <= hi).then(|| joined(&euler_phi_range(lo, hi))));
        }
    });
}

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
fn jordan_totient_oracle() {
    each("arith", "jordan_totient", |c| {
        c.check(jordan_totient(c.arg(0), c.arg(1)));
    });
}
#[test]
fn carmichael_lambda_oracle() {
    each("arith", "carmichael_lambda", |c| {
        if c.args[0].starts_with('-') {
            return;
        } // Signed carmichael_lambda inputs are outside this API.
        c.check(Some(carmichael_lambda(c.arg(0))));
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
        c.check(Some(sumliouville(c.arg(0))));
    });
}
#[test]
fn exp_mangoldt_oracle() {
    each("arith", "exp_mangoldt", |c| {
        c.check(Some(exp_mangoldt(c.arg(0))));
    });
}
#[test]
fn sumtotient_oracle() {
    each("arith", "sumtotient", |c| c.check(sumtotient(c.arg(0))));
}
#[test]
fn dedekind_psi_oracle() {
    each("arith", "dedekind_psi", |c| {
        if c.args[0].starts_with('-') {
            return;
        } // Signed dedekind_psi inputs are outside this API.
        c.check(dedekind_psi(c.arg(0)));
    });
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
        if c.args[0].starts_with('-') {
            return;
        } // Signed is_totient inputs are outside this API.
        c.check(Some(u8::from(is_totient(c.arg(0)))));
    });
}
#[test]
fn ramanujan_sum_oracle() {
    each("arith", "ramanujan_sum", |c| {
        c.check(Some(ramanujan_sum(c.arg(0), c.arg(1))));
    });
}
#[test]
fn valuation_oracle() {
    for c in common::load("arith")
        .into_iter()
        .filter(|c| c.func == "valuation")
    {
        if c.args.iter().any(|a| a.starts_with('-')) {
            continue;
        } // Signed valuation arguments are outside this API.
        if c.error {
            assert!(std::panic::catch_unwind(|| valuation(c.arg(0), c.arg(1))).is_err());
        } else {
            c.check(valuation(c.arg(0), c.arg(1)));
        }
    }
}

#[test]
fn chinese2() {
    each("arith", "chinese2", |c| {
        if let Some(p) = pairs(c) {
            c.check(math_prime_util::chinese2(&p).map(|(r, m)| format!("{r} {m}")));
        }
    });
}
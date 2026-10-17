//! Compatibility tests for the factor family against the MPU 0.75 fixtures.
mod common;

// The u64 API skips signed arguments (including -0) and the bigint cases
// 18450931744652885087, 36803912137803496523, and 36832540935240707603.
use common::{Case, each};
use math_prime_util::{
    cheb_factor, divisors, factor, factor_exp, fermat_factor, holf_factor, is_almost_prime,
    is_carmichael, is_cyclic, is_omega_prime, is_perfect_number, is_quasi_carmichael, is_semiprime,
    is_square_free, lehman_factor, pbrent_factor, pminus1_factor, pplus1_factor, prho_factor,
    prime_bigomega, prime_omega, prime_signature, squfof_factor, trial_factor,
};

fn check(name: &str, method: impl Fn(u64, Option<u64>) -> Vec<u64>) {
    let mut count = 0;
    for case in common::load("factor")
        .into_iter()
        .filter(|c| c.func == name)
    {
        // Skip signed arguments and bigint arguments above u64::MAX.
        let Ok(n) = case.args[0].parse::<u64>() else {
            continue;
        };
        let rounds = case.args.get(1).map(|r| r.parse::<u64>().unwrap());
        let got = method(n, rounds);
        if got.is_empty() {
            case.check::<String>(None);
        } else {
            case.check(Some(
                got.iter().map(u64::to_string).collect::<Vec<_>>().join(" "),
            ));
        }
        count += 1;
    }
    assert!(count > 0);
}

#[test]
fn fermat_factor_oracle() {
    check("fermat_factor", fermat_factor);
}
#[test]
fn holf_factor_oracle() {
    check("holf_factor", holf_factor);
}
#[test]
fn lehman_factor_oracle() {
    check("lehman_factor", |n, trial| {
        lehman_factor(n, trial.map(|t| t != 0))
    });
}
#[test]
fn squfof_factor_oracle() {
    check("squfof_factor", squfof_factor);
}

#[test]
fn factor_oracle() {
    for case in common::load("factor")
        .into_iter()
        .filter(|c| c.func == "factor")
    {
        let n = case.arg::<u128>(0);
        let factors = factor_u128(n).expect("fixture must split");
        if let Ok(small) = u64::try_from(n) {
            assert_eq!(
                math_prime_util::factor::factor(small)
                    .unwrap()
                    .into_iter()
                    .map(u128::from)
                    .collect::<Vec<_>>(),
                factors
            );
        }
        let result = factors
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ");
        case.check((!result.is_empty()).then_some(result));
        for p in factors {
            if p > 1 {
                assert_ne!(is_prime_u128(p), 0);
            }
        }
    }
}

#[test]
fn is_prime_oracle() {
    // The factor family has no is_prime rows. Check its inputs against u64,
    // and use the existing primality oracle for direct primality cases.
    for case in common::load("factor")
        .into_iter()
        .filter(|c| c.func == "factor")
    {
        if let Ok(n) = case.args[0].parse::<u64>() {
            assert_eq!(is_prime_u128(u128::from(n)), is_prime(n));
        }
    }
    for case in common::load("primality")
        .into_iter()
        .filter(|c| c.func == "is_prime")
    {
        // Negative inputs need the signed API, which this u128 path cannot take.
        if let Ok(n) = case.args[0].parse::<u128>() {
            case.check(Some(is_prime_u128(n)));
        }
    }
    assert_eq!(is_prime_u128((1u128 << 127) - 1), 1);
    assert_eq!(is_prime_u128(u128::MAX), 0);
}

fn check_bounded(name: &str, method: impl Fn(u64, Option<u64>, Option<u64>) -> Vec<u64>) {
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
    check_bounded("prho_factor", |n, a, _| prho_factor(n, a));
}
#[test]
fn pbrent_factor_oracle() {
    check_bounded("pbrent_factor", pbrent_factor);
}
#[test]
fn pminus1_factor_oracle() {
    check_bounded("pminus1_factor", pminus1_factor);
}
#[test]
fn pplus1_factor_oracle() {
    check_bounded("pplus1_factor", |n, a, _| pplus1_factor(n, a));
}

fn list<T: ToString>(values: &[T]) -> Option<String> {
    (!values.is_empty()).then(|| {
        values
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    })
}

#[test]
fn cheb_factor_oracle() {
    each("factor", "cheb_factor", |c| {
        // Negative and wider-than-u64 arguments have no native Rust API.
        let Ok(n) = c.args[0].parse::<u64>() else {
            return;
        };
        let bound = c.args.get(1).map_or(0, |s| s.parse().unwrap());
        let init_x = c.args.get(2).map_or(0, |s| s.parse().unwrap());
        c.check(list(&cheb_factor(n, bound, init_x)));
    });
}

#[test]
fn prime_bigomega_oracle() {
    each("factor", "prime_bigomega", |c| {
        // Negative and wider-than-u64 arguments have no native Rust API.
        if let Ok(n) = c.args[0].parse::<u64>() {
            c.check(Some(prime_bigomega(n)));
        }
    });
}

#[test]
fn prime_omega_oracle() {
    each("factor", "prime_omega", |c| {
        // Negative and wider-than-u64 arguments have no native Rust API.
        if let Ok(n) = c.args[0].parse::<u64>() {
            c.check(Some(prime_omega(n)));
        }
    });
}

#[test]
fn prime_signature_oracle() {
    each("factor", "prime_signature", |c| {
        // Negative and wider-than-u64 arguments have no native Rust API.
        if let Ok(n) = c.args[0].parse::<u64>() {
            c.check(list(&prime_signature(n)));
        }
    });
}

#[test]
fn is_square_free_oracle() {
    each("factor", "is_square_free", |c| {
        // Skip bigint and signed arguments outside this native unsigned API.
        // Examples include n=18450931744652885087 and k=10^30.
        if c.args.iter().any(|arg| arg.parse::<u64>().is_err()) {
            return;
        }
        c.check(Some(u8::from(is_square_free(c.arg(0)))));
    });
}

#[test]
fn is_semiprime_oracle() {
    each("factor", "is_semiprime", |c| {
        // Skip bigint and signed arguments outside this native unsigned API.
        // Examples include n=18450931744652885087 and k=10^30.
        if c.args.iter().any(|arg| arg.parse::<u64>().is_err()) {
            return;
        }
        c.check(Some(u8::from(is_semiprime(c.arg(0)))));
    });
}

#[test]
fn is_almost_prime_oracle() {
    each("factor", "is_almost_prime", |c| {
        // Skip bigint and signed arguments outside this native unsigned API.
        // Examples include n=18450931744652885087 and k=10^30.
        if c.args.iter().any(|arg| arg.parse::<u64>().is_err()) {
            return;
        }
        c.check(Some(u8::from(is_almost_prime(c.arg(0), c.arg(1)))));
    });
}

#[test]
fn is_omega_prime_oracle() {
    each("factor", "is_omega_prime", |c| {
        // Skip bigint and signed arguments outside this native unsigned API.
        // Examples include n=18450931744652885087 and k=10^30.
        if c.args.iter().any(|arg| arg.parse::<u64>().is_err()) {
            return;
        }
        c.check(Some(u8::from(is_omega_prime(c.arg(0), c.arg(1)))));
    });
}

fn native(c: &Case) -> bool {
    // Skip signed inputs, values above u64::MAX, and invalid text such as
    // trial_factor(18446744073709551616, "10foo"). These need a string API.
    c.args.iter().all(|a| a.parse::<u64>().is_ok())
}

fn check_list(c: &Case, values: &[u64]) {
    c.check((!values.is_empty()).then(|| {
        values
            .iter()
            .map(u64::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    }));
}

#[test]
fn factor_oracle() {
    each("factor", "factor", |c| {
        if native(c) {
            check_list(c, &factor(c.arg(0)));
        }
    });
}

#[test]
fn factor_exp_oracle() {
    each("factor", "factor_exp", |c| {
        if native(c) {
            let pairs = factor_exp(c.arg(0));
            c.check((!pairs.is_empty()).then(|| {
                pairs
                    .iter()
                    .map(|(p, e)| format!("{p}^{e}"))
                    .collect::<Vec<_>>()
                    .join(" ")
            }));
        }
    });
}

#[test]
fn divisors_oracle() {
    each("factor", "divisors", |c| {
        if native(c) {
            let max = c.args.get(1).map(|_| c.arg(1));
            check_list(c, &divisors(c.arg(0), max));
        }
    });
}

#[test]
fn trial_factor_oracle() {
    each("factor", "trial_factor", |c| {
        if native(c) {
            let limit = c.args.get(1).map_or(0, |_| c.arg(1));
            check_list(c, &trial_factor(c.arg(0), limit));
        }
    });
}

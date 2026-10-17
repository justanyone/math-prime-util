//! Compatibility tests for the factor family against the MPU 0.75 fixtures.
mod common;

// The u64 API skips signed arguments (including -0) and the bigint cases
// 18450931744652885087, 36803912137803496523, and 36832540935240707603.
use common::each;
use math_prime_util::{
    cheb_factor, fermat_factor, holf_factor, is_almost_prime, is_carmichael, is_cyclic,
    is_omega_prime, is_perfect_number, is_quasi_carmichael, is_semiprime, is_square_free,
    lehman_factor, pbrent_factor, pminus1_factor, pplus1_factor, prho_factor, prime_bigomega,
    prime_omega, prime_signature, squfof_factor,
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

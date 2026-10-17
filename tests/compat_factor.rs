//! Compatibility tests for the factor family against the MPU 0.75 fixtures.
mod common;

// The u64 API skips signed arguments (including -0) and the bigint cases
// 18450931744652885087, 36803912137803496523, and 36832540935240707603.
use common::each;
use math_prime_util::{
    cheb_factor, ecm_factor, fermat_factor, holf_factor, is_carmichael, is_cyclic,
    is_perfect_number, is_quasi_carmichael, lehman_factor, pbrent_factor, pminus1_factor,
    pplus1_factor, prho_factor, prime_bigomega, prime_omega, prime_signature, squfof_factor,
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
fn ecm_factor_oracle() {
    common::each("factor", "ecm_factor", |c| {
        let factors = ecm_factor(
            c.arg(0),
            c.args.get(1).map(|_| c.arg(1)),
            c.args.get(2).map(|_| c.arg(2)),
            c.args.get(3).map(|_| c.arg(3)),
        );
        // The fixed-sigma factor.c implementation groups the two factors 7
        // together for 1819015037140. The oracle groups one 7 with 1856137793.
        // ECM may return composite factors. Check both exact partitions here;
        // skip only their string equality, not this oracle case.
        if c.arg::<u64>(0) == 1_819_015_037_140 {
            assert_eq!(factors, vec![2, 2, 5, 49, 1_856_137_793]);
            let oracle: Vec<u64> = c
                .result
                .as_ref()
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            assert_eq!(oracle, vec![2, 2, 5, 7, 12_992_964_551]);
            assert_eq!(oracle.iter().product::<u64>(), c.arg::<u64>(0));
            assert_eq!(factors.iter().product::<u64>(), c.arg::<u64>(0));
            assert_eq!(49 * 1_856_137_793_u64, 7 * 12_992_964_551);
            return;
        }
        let result = if factors.is_empty() {
            None
        } else {
            Some(
                factors
                    .iter()
                    .map(u64::to_string)
                    .collect::<Vec<_>>()
                    .join(" "),
            )
        };
        c.check(result);
    });
}

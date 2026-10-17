//! Compatibility tests for the factor family against the MPU 0.75 fixtures.
mod common;

// The u64 API skips signed arguments (including -0) and the bigint cases
// 18450931744652885087, 36803912137803496523, and 36832540935240707603.
use common::{Case, each};
use math_prime_util::{
    cheb_factor, divisors, factor, factor_exp, factor_u128, fermat_factor, holf_factor,
    is_almost_prime, is_carmichael, is_cyclic, is_omega_prime, is_perfect_number, is_prime_u128,
    is_quasi_carmichael, is_semiprime, is_square_free, lehman_factor, pbrent_factor,
    pminus1_factor, pplus1_factor, prho_factor, prime_bigomega, prime_omega, prime_signature,
    rough_count, squarefree_kernel, squfof_factor, trial_factor,
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

/// The 128-bit semiprimes of MPU `t/50-factoring.t` (subtest "128-bit factoring"):
/// each stage of `factorintp128` (p-1, SQUFOF, small ECM) and a 128-bit p-1 case.
/// The fixture family `factor` has no cases above 64 bits.
const WIDE: [(u128, [u128; 2]); 5] = [
    (36_803_912_137_803_496_523, [4_284_736_327, 8_589_539_549]),
    (36_832_540_935_240_707_603, [4_293_801_221, 8_578_073_143]),
    (
        595_424_139_679_831_880_741,
        [17_380_103_011, 34_258_953_431],
    ),
    (
        594_225_194_063_012_351_347,
        [17_560_455_931, 33_838_824_937],
    ),
    (
        283_587_383_656_828_693_023_910_077_800_818_570_783,
        [16_647_189_274_036_065_961, 17_035_151_038_928_128_903],
    ),
];

#[test]
fn factor_u128_oracle() {
    for (n, expect) in WIDE {
        assert_eq!(factor_u128(n), Ok(expect.to_vec()), "{n}");
    }
}

#[test]
fn is_prime_u128_oracle() {
    for (n, expect) in WIDE {
        assert_eq!(is_prime_u128(n), 0, "{n}");
        for p in expect {
            assert_ne!(is_prime_u128(p), 0, "{p}");
        }
    }
}

#[test]
fn u128_paths_agree_with_u64_on_the_fixtures() {
    each("factor", "factor", |c| {
        if native(c) {
            let n = c.arg(0);
            let wide: Vec<u64> = factor_u128(u128::from(n))
                .unwrap()
                .into_iter()
                .map(|p| u64::try_from(p).unwrap())
                .collect();
            assert_eq!(wide, factor(n), "factor({n})");
            assert_eq!(
                is_prime_u128(u128::from(n)),
                math_prime_util::is_prime(n),
                "{n}"
            );
        }
    });
}

#[test]
fn rough_count_oracle() {
    each("factor", "rough_count", |c| {
        c.check(Some(rough_count(c.arg(0), c.arg(1))));
    });
}

#[test]
fn factor_u128_ecm_medium() {
    // "ECM med" and "128-bit" cases of t/50-factoring.t (extra tests there).
    let cases: [(u128, [u128; 2]); 3] = [
        (
            1_089_212_718_153_507_466_819_331,
            [1_027_240_161_683, 1_060_329_179_857],
        ),
        (
            633_285_935_952_110_634_787_381,
            [690_039_519_251, 917_753_140_631],
        ),
        (
            202_911_193_152_450_441_348_811_714_985_016_716_279,
            [13_317_945_721_813_380_433, 15_235_922_821_047_653_063],
        ),
    ];
    for (n, expect) in cases {
        assert_eq!(factor_u128(n), Ok(expect.to_vec()), "{n}");
    }
}

#[test]
fn squarefree_kernel_oracle() {
    each("factor", "squarefree_kernel", |c| {
        // Skip the bigint arguments above u64::MAX.
        if c.args[0].parse::<u64>().is_ok() {
            c.check(Some(squarefree_kernel(c.arg(0))));
        }
    });
}

//! Native proving checks against MPU 0.75.
mod common;
use math_prime_util::*;
#[test]
fn is_aks_prime_oracle() {
    common::each("primality", "is_aks_prime", |c| {
        // Signed inputs such as -1 and -2, and bigint inputs, are outside the u64 API.
        if let Ok(n) = c.args[0].parse::<u64>() {
            c.check(Some(u8::from(is_aks_prime(n))));
        }
    });
}
#[test]
fn is_provable_prime_oracle() {
    common::each("primality", "is_provable_prime", |c| {
        // Signed and bigint arguments are outside the native u64 API.
        if let Ok(n) = c.args[0].parse::<u64>() {
            c.check(Some(is_provable_prime(n)));
        }
    });
}
// The committed primality fixture has no rows for the certificate APIs.
// The expected text follows MPU PrimalityProving.pm::_small_cert.
#[test]
fn prime_certificate_compat() {
    assert_eq!(
        prime_certificate(101),
        "[MPU - Primality Certificate]\nVersion 1.0\n\nProof for:\nN 101\n\nType Small\nN 101\n"
    );
    for n in [0, 1, 4, 561, 3_215_031_751, u64::MAX] {
        assert_eq!(prime_certificate(n), "");
    }
    for n in [2, 3, 4_294_967_311, u64::MAX - 58] {
        assert_eq!(
            prime_certificate(n),
            format!(
                "[MPU - Primality Certificate]\nVersion 1.0\n\nProof for:\nN {n}\n\nType Small\nN {n}\n"
            )
        );
    }
}
#[test]
fn is_provable_prime_with_cert_compat() {
    for n in [0, 1, 15, 561, 3_215_031_751, u64::MAX] {
        assert_eq!(is_provable_prime_with_cert(n), (0, String::new()));
    }
    assert_eq!(
        is_provable_prime_with_cert(101),
        (2, prime_certificate(101))
    );
}
#[test]
fn verify_prime_compat() {
    for n in [2, 3, 101, 4_294_967_311, u64::MAX - 58] {
        assert_eq!(verify_prime(&prime_certificate(n)), Ok(true));
    }
    let header = "[MPU - Primality Certificate]\nVersion 1.0\nProof for:\nN 101\n";
    for proof in [
        "Type BLS5\nN 101\nQ[1] 5\nA[0] 2\nA[1] 2\n----\n",
        "Type Lucas\nN 101\nQ[1] 2\nQ[2] 5\nA 2\n",
    ] {
        assert_eq!(verify_prime(&format!("{header}{proof}")), Ok(true));
    }
    for proof in [
        "Type Small\nN 15",
        "Type BLS5\nN 101\nA[0] 1\n----",
        "Type BLS5\nN 101\nQ[1] 4\n----",
    ] {
        assert_eq!(verify_prime(&format!("{header}{proof}")), Ok(false));
    }
    for proof in [
        "Type ECPP\nN 101",
        "Type BLS5\nN 101\nN 101",
        "Type Small\nN 18446744073709551616",
        "Type BLS5\nN 101\nQ[2] 5\n----",
    ] {
        assert!(verify_prime(&format!("{header}{proof}")).is_err());
    }
    assert_eq!(verify_prime(""), Ok(false));
    assert!(verify_prime("garbage").is_err());
}

#[test]
fn is_frobenius_pseudoprime_oracle() {
    common::each("primality", "is_frobenius_pseudoprime", |c| {
        // Negative and bigint n arguments are outside this unsigned native API.
        let Ok(n) = c.args[0].parse::<u64>() else {
            return;
        };
        // A lone P argument is a Perl arity error; Rust requires both P and Q.
        if c.args.len() == 2 {
            assert!(c.error);
            return;
        }
        let parameters = if c.args.len() == 3 {
            let (Ok(p), Ok(q)) = (c.args[1].parse::<i64>(), c.args[2].parse::<i64>()) else {
                return;
            };
            // MPU uses its bigint fallback outside the native C P,Q bound.
            if p.unsigned_abs() > 3_037_000_497 || q.unsigned_abs() > 3_037_000_497 {
                return;
            }
            Some((p, q))
        } else {
            None
        };
        let result = std::panic::catch_unwind(|| {
            parameters.map_or_else(
                || is_frobenius_pseudoprime(n),
                |(p, q)| is_frobenius_pseudoprime_pq(n, p, q),
            )
        });
        if c.error {
            assert!(result.is_err(), "{c:?}");
        } else {
            c.check(Some(u8::from(result.unwrap())));
        }
    });
}

#[test]
fn is_frobenius_khashin_pseudoprime_oracle() {
    common::each("primality", "is_frobenius_khashin_pseudoprime", |c| {
        // Negative and bigint n arguments are outside this unsigned native API.
        let Ok(n) = c.args[0].parse::<u64>() else {
            return;
        };
        c.check(Some(u8::from(is_frobenius_khashin_pseudoprime(n))));
    });
}

#[test]
fn is_frobenius_underwood_pseudoprime_oracle() {
    common::each("primality", "is_frobenius_underwood_pseudoprime", |c| {
        // Negative and bigint n arguments are outside this unsigned native API.
        let Ok(n) = c.args[0].parse::<u64>() else {
            return;
        };
        c.check(Some(u8::from(is_frobenius_underwood_pseudoprime(n))));
    });
}

#[test]
fn is_perrin_pseudoprime_oracle() {
    common::each("primality", "is_perrin_pseudoprime", |c| {
        // Negative and bigint n arguments are outside this unsigned native API.
        let Ok(n) = c.args[0].parse::<u64>() else {
            return;
        };
        // The optional second argument is the restriction level (default 0).
        let restricted = c.args.get(1).map_or(Ok(0), |r| r.parse::<u32>());
        let Ok(restricted) = restricted else {
            // A negative restriction is an error in MPU and unrepresentable here.
            assert!(c.error, "{c:?}");
            return;
        };
        let result = std::panic::catch_unwind(|| u8::from(is_perrin_pseudoprime(n, restricted)));
        if c.error {
            assert!(result.is_err(), "{c:?}");
        } else {
            c.check(Some(result.unwrap()));
        }
    });
}

#[test]
fn is_catalan_pseudoprime_oracle() {
    common::each("primality", "is_catalan_pseudoprime", |c| {
        // Negative and bigint n arguments are outside this unsigned native API.
        let Ok(n) = c.args[0].parse::<u64>() else {
            return;
        };
        c.check(Some(u8::from(is_catalan_pseudoprime(n))));
    });
}

#[test]
fn is_chen_prime_oracle() {
    common::each("primality", "is_chen_prime", |c| {
        // Negative and bigint n arguments are outside this unsigned native API.
        let Ok(n) = c.args[0].parse::<u64>() else {
            return;
        };
        c.check(Some(u8::from(is_chen_prime(n))));
    });
}

#[test]
fn is_delicate_prime_oracle() {
    common::each("primality", "is_delicate_prime", |c| {
        // Negative and bigint n arguments are outside this unsigned native API.
        let Ok(n) = c.args[0].parse::<u64>() else {
            return;
        };
        let b = c
            .args
            .get(1)
            .map_or(10, |b| b.parse::<u32>().expect("base"));
        let result = std::panic::catch_unwind(|| is_delicate_prime(n, b));
        if c.error {
            assert!(result.is_err(), "{c:?}");
        } else {
            // None: MPU leaves the result to its bigint backend; this native API cannot answer.
            if let Some(r) = result.unwrap() {
                c.check(Some(u8::from(r)));
            }
        }
    });
}

mod generic {
    use super::common;

    use math_prime_util::Int;

    fn replay(function: &str) {
        let mut count = 0;
        for family in ["primality", "factor", "modular", "int", "sieve"] {
            for case in common::load(family) {
                if case.func != function || (family == "sieve" && function != "next_prime") {
                    continue;
                }
                // Signed rows (modular: powmod with a negative base or exponent) are
                // outside these unsigned backends. A row above u128 (modular: the
                // 41-digit powmod) is replayed through BigUint when `bigint` is on.
                let Ok(args) = case
                    .args
                    .iter()
                    .map(|s| s.parse::<u128>())
                    .collect::<Result<Vec<_>, _>>()
                else {
                    #[cfg(feature = "bigint")]
                    {
                        if let Ok(big_args) = case
                            .args
                            .iter()
                            .map(|s| s.parse::<num_bigint::BigUint>())
                            .collect::<Result<Vec<_>, _>>()
                        {
                            assert_eq!(
                                evaluate(&big_args, function),
                                case.result,
                                "BigUint {family}:{}",
                                case.line
                            );
                            count += 1;
                        }
                    }
                    continue;
                };
                let result = evaluate(&args, function);
                #[cfg(feature = "bigint")]
                {
                    let big_args = args
                        .iter()
                        .copied()
                        .map(num_bigint::BigUint::from)
                        .collect::<Vec<_>>();
                    assert_eq!(
                        evaluate(&big_args, function),
                        result,
                        "BigUint {family}:{}",
                        case.line
                    );
                }
                // The sieve oracle uses the bounded native next_prime API.
                if function == "next_prime" && case.result.is_none() {
                    assert_eq!(u64::try_from(args[0]).unwrap().next_prime(), None);
                } else {
                    assert_eq!(result, case.result, "{family}:{}", case.line);
                }
                if let Ok(small) = args
                    .iter()
                    .copied()
                    .map(u64::try_from)
                    .collect::<Result<Vec<_>, _>>()
                {
                    // next_prime can leave the u64 range. The bounded backend returns None.
                    let generic = evaluate(&small, function);
                    if function != "next_prime" || generic.is_some() {
                        assert_eq!(generic, result, "{family}:{}", case.line);
                    }
                    match function {
                        "is_prime" => {
                            assert_eq!(small[0].is_prime(), math_prime_util::is_prime(small[0]));
                        }
                        "factor" => assert_eq!(
                            small[0].factor().unwrap(),
                            math_prime_util::factor(small[0])
                        ),
                        "powmod" => assert_eq!(
                            small[0].powmod(&small[1], &small[2]),
                            math_prime_util::powmod(small[0], small[1], small[2])
                        ),
                        "gcd" => assert_eq!(
                            small.iter().fold(0, |g, n| g.gcd(n)),
                            small.iter().fold(0, |g, n| math_prime_util::gcd(g, *n))
                        ),
                        "next_prime" => {
                            assert_eq!(
                                small[0].next_prime(),
                                math_prime_util::next_prime(small[0])
                            );
                        }
                        _ => unreachable!(),
                    }
                }
                count += 1;
            }
        }
        assert!(count > 0);
    }

    fn evaluate<T: Int + From<u8> + std::fmt::Display + std::fmt::Debug>(
        args: &[T],
        function: &str,
    ) -> Option<String> {
        match function {
            "is_prime" => Some(args[0].is_prime().to_string()),
            "factor" => {
                let factors = args[0].factor().unwrap();
                (!factors.is_empty()).then(|| {
                    factors
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(" ")
                })
            }
            "powmod" => args[0].powmod(&args[1], &args[2]).map(|n| n.to_string()),
            "gcd" => Some(args.iter().fold(T::from(0), |g, n| g.gcd(n)).to_string()),
            "next_prime" => args[0].next_prime().map(|n| n.to_string()),
            _ => unreachable!(),
        }
    }

    #[test]
    fn compat_primality_is_prime() {
        replay("is_prime");
    }
    #[test]
    fn compat_primality_factor() {
        replay("factor");
    }
    #[test]
    fn compat_primality_powmod() {
        replay("powmod");
    }
    #[test]
    fn compat_primality_gcd() {
        replay("gcd");
    }
    #[test]
    fn compat_primality_next_prime() {
        replay("next_prime");
    }
}

#[test]
fn verify_prime_native_bls15() {
    // MPU PrimalityProving.pm::_verify_bls15, with V_7(-1, 2) = 13.
    let header = "[MPU - Primality Certificate]\nVersion 1.0\nProof for:\nN 13\n";
    let proof = "Type BLS15\nN 13\nQ 7\nLP -1\nLQ 2\n";
    assert_eq!(verify_prime(&format!("{header}{proof}")), Ok(true));
    for proof in [
        "Type BLS15\nN 13\nQ 2\nLP 1\nLQ -1",
        "Type BLS15\nN 13\nQ 5\nLP 1\nLQ -1",
        "Type BLS15\nN 13\nQ 7\nLP 2\nLQ 1",
        "Type BLS15\nN 13\nQ 7\nLP 1\nLQ 1",
    ] {
        assert_eq!(verify_prime(&format!("{header}{proof}")), Ok(false));
    }
    assert!(verify_prime(&format!("{header}Type BLS15\nN 13\nQ 7\nLP 1")).is_err());
}

#[test]
fn verify_prime_field_validation() {
    let header = "[MPU - Primality Certificate]\nProof for:\nn\t101\n";
    assert_eq!(
        verify_prime(&format!("{header}Type Small\nn 101")),
        Ok(true)
    );
    let header = "[MPU - Primality Certificate]\nProof for:\nN 101\n";
    // MPU requires Q indices to arrive in order, even when all values are valid.
    for proof in [
        "Type BLS5\nN 101\nQ[2] 5\nQ[1] 2\n----",
        "Type Lucas\nN 101\nQ[2] 5\nQ[1] 2\nA 2",
        "Type BLS5\nN 101\nA[1] 2\nQ[1] 5\n----",
        "Type BLS5\nN 101\nQ[1] -5\n----",
    ] {
        assert!(
            verify_prime(&format!("{header}{proof}")).is_err(),
            "{proof}"
        );
    }
}

#[test]
fn verify_prime_bls15_native_boundaries() {
    let header = "[MPU - Primality Certificate]\nProof for:\nN 13\nType BLS15\n";
    for proof in [
        "N 2\nQ 7\nLP -1\nLQ 2",
        "N 14\nQ 7\nLP -1\nLQ 2",
        "N 17\nQ 9\nLP -1\nLQ 2",
        "N 101\nQ 3\nLP -1\nLQ 2",
        "N 13\nQ 7\nLP 1\nLQ -1",
    ] {
        assert_eq!(verify_prime(&format!("{header}{proof}")), Ok(false));
    }
    // Residues larger than i64::MAX are centered before the Lucas call.
    let proof = "N 18446744073709551565\nQ 9223372036854775783\nLP 18446744073709551564\nLQ 2";
    assert_eq!(verify_prime(&format!("{header}{proof}")), Ok(false));
    for proof in [
        "Q 7\nLP -1\nLQ 2\nA 2",
        "N -13\nQ 7\nLP -1\nLQ 2",
        "N 13\nQ 7\nLP -18446744073709551616\nLQ 2",
        "N 13\nQ 7\nLP 18446744073709551616\nLQ 2",
    ] {
        assert!(verify_prime(&format!("{header}{proof}")).is_err());
    }
}

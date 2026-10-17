//! Native proving checks against MPU 0.75.
mod common;
use math_prime_util::*;
#[test]
fn is_aks_prime_oracle() {
    common::each("primality", "is_aks_prime", |c| {
        // Signed inputs such as -1 and -2 are outside the u64 API.
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
// The committed primality fixture has no rows for these three APIs.
// The expected text follows MPU PrimalityProving.pm::_small_cert.
#[test]
fn prime_certificate_compat() {
    assert_eq!(
        prime_certificate(101),
        "[MPU - Primality Certificate]\nVersion 1.0\n\nProof for:\nN 101\n\nType Small\nN 101\n"
    );
    assert_eq!(prime_certificate(1), "");
}
#[test]
fn is_provable_prime_with_cert_compat() {
    assert_eq!(is_provable_prime_with_cert(15), (0, String::new()));
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

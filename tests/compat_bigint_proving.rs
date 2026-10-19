//! The arbitrary precision primality proofs and certificates against MPU 0.75
//! (pure Perl) on 65 to 664 bit inputs: `tests/fixtures/bigint_proving.tsv`.
//!
//! The fixture writes a space as `~` and a newline as `|` inside a
//! certificate; `decode` restores them. Pure Perl MPU proves only with the
//! BLS75 n-1 method and answers 1 (could not prove) where that fails; the
//! Rust side must then prove the number with ECPP.
//!
//! `mpu_verifies_our_certificates` runs `Math::Prime::Util::verify_prime` on
//! the certificates the Rust side writes when a Perl with the vendored MPU
//! tree loads, and skips with a named reason otherwise.
#![cfg(feature = "bigint")]
mod common;
use common::Case;
use math_prime_util::bigint::primality::is_prob_prime;
use math_prime_util::bigint::proving::{
    Certificate, Proof, is_aks_prime, is_provable_prime, is_provable_prime_with_cert,
    prime_certificate, verify_prime,
};
use num_bigint::BigUint;
use std::io::Write;
use std::process::{Command, Stdio};

const FAMILY: &str = "bigint_proving";

fn decode(text: &str) -> String {
    text.replace('~', " ").replace('|', "\n")
}

fn unsigned(c: &Case, i: usize) -> BigUint {
    c.arg(i)
}

fn oracle(c: &Case) -> &str {
    c.result.as_deref().unwrap_or_default()
}

#[test]
fn provable_oracle() {
    let mut unproven_by_perl = 0;
    common::each(FAMILY, "is_provable_prime", |c| {
        let n = unsigned(c, 0);
        let got = is_provable_prime(&n);
        if oracle(c) == "1" {
            // Pure Perl ran out of factoring methods for n - 1; ECPP proves it.
            assert_eq!(got, 2, "fixture line {}: {n}", c.line);
            unproven_by_perl += 1;
            return;
        }
        c.check(Some(got));
    });
    println!("primes pure Perl could not prove: {unproven_by_perl}");
}

#[test]
fn certificate_oracle() {
    let mut equal = 0;
    let mut proven = 0;
    common::each(FAMILY, "is_provable_prime_with_cert", |c| {
        let n = unsigned(c, 0);
        let (status, text) = oracle(c).split_once(' ').unwrap_or((oracle(c), ""));
        let perl_cert = decode(text);
        let (got, cert) = is_provable_prime_with_cert(&n);
        if status == "0" {
            assert_eq!((got, cert.as_str()), (0, ""), "fixture line {}", c.line);
            return;
        }
        assert_eq!(got, 2, "fixture line {}: {n}", c.line);
        assert_eq!(verify_prime(&cert), Ok(true), "fixture line {}", c.line);
        assert_eq!(
            Certificate::parse(&cert).map(|c| c.n),
            Ok(n.clone()),
            "fixture line {}",
            c.line
        );
        proven += 1;
        if status == "2" {
            // MPU's own certificate verifies here, and ours matches its
            // text when the same factors of n - 1 were found.
            assert_eq!(
                verify_prime(&perl_cert),
                Ok(true),
                "fixture line {}",
                c.line
            );
            if cert == perl_cert {
                equal += 1;
            }
        }
    });
    assert!(proven > 10);
    assert!(equal > 5, "only {equal} certificates equal MPU's text");
    common::each(FAMILY, "prime_certificate", |c| {
        let n = unsigned(c, 0);
        let got = prime_certificate(&n);
        if is_prob_prime(&n) == 0 {
            assert_eq!(got, "", "fixture line {}", c.line);
            assert_eq!(oracle(c), "", "fixture line {}", c.line);
        } else {
            assert_eq!(verify_prime(&got), Ok(true), "fixture line {}", c.line);
            if !oracle(c).is_empty() {
                assert_eq!(
                    verify_prime(&decode(oracle(c))),
                    Ok(true),
                    "fixture line {}",
                    c.line
                );
            }
        }
    });
}

#[test]
fn verify_oracle() {
    let mut accepted = 0;
    let mut types = std::collections::BTreeSet::new();
    common::each(FAMILY, "verify_prime", |c| {
        let cert = decode(c.args.first().map_or("", String::as_str));
        let got = verify_prime(&cert);
        let flag = u8::from(got == Ok(true));
        c.check(Some(flag));
        if flag == 1 {
            accepted += 1;
            if let Ok(parsed) = Certificate::parse(&cert) {
                for proof in parsed.proofs {
                    types.insert(format!("{proof}").lines().next().unwrap_or("").to_string());
                }
            }
        }
    });
    assert!(accepted > 20);
    for kind in [
        "Type BLS5",
        "Type Lucas",
        "Type BLS3",
        "Type Pocklington",
        "Type BLS15",
        "Type ECPP",
        "Type ECPP3",
    ] {
        assert!(types.contains(kind), "no accepted {kind} in the fixture");
    }
}

#[test]
fn aks_oracle() {
    common::each(FAMILY, "is_aks_prime", |c| {
        let n = unsigned(c, 0);
        if c.error {
            // Skipped: PP `is_aks_prime` dies with `toint: 'NaN'` for every
            // input above 2^64 that passes its trivial tests
            // (docs/upstream-findings.md entry 15). The three inputs are
            // composite, so the correct value is 0.
            assert!(!is_aks_prime(&n), "fixture line {}", c.line);
            return;
        }
        c.check(Some(u8::from(is_aks_prime(&n))));
    });
}

/// The primes of the fixture above 2^64 that MPU proved or gave up on.
fn fixture_primes() -> Vec<BigUint> {
    common::load(FAMILY)
        .iter()
        .filter(|c| c.func == "is_provable_prime" && oracle(c) != "0")
        .map(|c| unsigned(c, 0))
        .filter(|n| *n > BigUint::from(u64::MAX))
        .collect()
}

/// The first prime after the 200-digit repunit: n - 1 does not factor far
/// enough for the n - 1 proof, so the ECPP downrun carries the whole proof.
fn hard_200_digit_prime() -> BigUint {
    let mut n: BigUint = "1".repeat(200).parse().unwrap_or_default();
    loop {
        n += 2_u8;
        if is_prob_prime(&n) != 0 {
            return n;
        }
    }
}

#[test]
fn proves_200_digits() {
    let n = hard_200_digit_prime();
    let start = std::time::Instant::now();
    let cert = Certificate::prove(&n).unwrap_or_else(|| panic!("no proof for {n}"));
    println!(
        "200-digit proof: {} blocks in {:?}",
        cert.proofs.len(),
        start.elapsed()
    );
    assert_eq!(cert.verify(), Ok(true));
    assert!(cert.proofs.iter().any(|p| matches!(p, Proof::Ecpp { .. })));
    // Every ECPP step shrinks the number to prove.
    let mut last = n;
    for proof in &cert.proofs {
        if let Proof::Ecpp { n, q, .. } = proof {
            assert!(*n <= last);
            assert!(q < n);
            last = q.clone();
        }
    }
}

#[test]
fn mpu_verifies_our_certificates() {
    let root = env!("CARGO_MANIFEST_DIR");
    let lib = format!("{root}/reference/mpu/lib");
    let probe = Command::new("perl")
        .args(["-I", &lib, "-MMath::Prime::Util=verify_prime", "-e", "1"])
        .env("MPU_NO_XS", "1")
        .output();
    match probe {
        Ok(out) if out.status.success() => {}
        Ok(out) => {
            println!(
                "skipped: the vendored Math::Prime::Util does not load in this Perl: {}",
                String::from_utf8_lossy(&out.stderr)
            );
            return;
        }
        Err(e) => {
            println!("skipped: no perl on this host ({e})");
            return;
        }
    }
    // A sample across the sizes, plus one certificate that needs ECPP.
    let mut certs = Vec::new();
    let primes = fixture_primes();
    for n in primes.iter().step_by(3).take(12) {
        certs.push(prime_certificate(n));
    }
    let m89 = (BigUint::from(1_u8) << 89_usize) - 1_u8;
    let m107 = (BigUint::from(1_u8) << 107_usize) - 1_u8;
    let mut n = &m89 * &m107 * 2_u8 + 1_u8;
    while is_prob_prime(&n) == 0 {
        n += &m89 * &m107 * 2_u8;
    }
    certs.push(prime_certificate(&n));
    let kinds: std::collections::BTreeSet<String> = certs
        .iter()
        .flat_map(|c| {
            c.lines()
                .filter(|l| l.starts_with("Type "))
                .map(str::to_string)
                .collect::<Vec<_>>()
        })
        .collect();
    assert!(kinds.contains("Type ECPP"), "{kinds:?}");
    assert!(kinds.contains("Type BLS5"), "{kinds:?}");
    let script = "local $/; my @certs = split /^=====\\n/m, <STDIN>; print verify_prime($_) ? 1 : 0, \"\\n\" for @certs;";
    let mut child = Command::new("perl")
        .args(["-I", &lib, "-MMath::Prime::Util=verify_prime", "-e", script])
        .env("MPU_NO_XS", "1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("perl: {e}"));
    if let Some(mut stdin) = child.stdin.take() {
        let joined = certs.join("=====\n");
        stdin
            .write_all(joined.as_bytes())
            .unwrap_or_else(|e| panic!("perl stdin: {e}"));
    }
    let out = child
        .wait_with_output()
        .unwrap_or_else(|e| panic!("perl: {e}"));
    let results: Vec<&str> = std::str::from_utf8(&out.stdout)
        .unwrap_or("")
        .lines()
        .collect();
    assert_eq!(
        results,
        vec!["1"; certs.len()],
        "MPU verify_prime: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    println!(
        "MPU verify_prime accepted {} certificates with {kinds:?}",
        certs.len()
    );
}

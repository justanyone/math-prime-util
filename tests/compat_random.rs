//! Seeded native compatibility with MPU 0.75.
mod common;
use math_prime_util::*;

fn unhex(s: &str) -> Vec<u8> {
    s.as_bytes()
        .chunks_exact(2)
        .map(|c| u8::from_str_radix(std::str::from_utf8(c).unwrap(), 16).unwrap())
        .collect()
}

fn cases(name: &str, mut f: impl FnMut(&common::Case)) {
    for c in common::load("random").iter().filter(|c| c.func == name) {
        let seed = &c.args[0];
        let bytes: Vec<_> = (0..seed.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&seed[i..i + 2], 16).unwrap())
            .collect();
        csrand(&bytes);
        f(c);
    }
}

fn list<T: std::fmt::Display>(values: &[T]) -> Option<String> {
    (!values.is_empty()).then(|| {
        values
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    })
}

#[test]
fn random_unrestricted_semiprime_oracle() {
    cases("random_unrestricted_semiprime", |c| {
        let bits: u32 = c.arg(1);
        // MPU bigint bit sizes above 64 have no native equivalent yet.
        if bits > 64 {
            assert_eq!(random_unrestricted_semiprime(bits), None);
            return;
        }
        c.check(random_unrestricted_semiprime(bits));
    });
}

#[test]
fn random_factored_integer_oracle() {
    cases("random_factored_integer", |c| {
        // Bounds above u64::MAX require the bigint phase.
        let Ok(n) = c.args[1].parse::<u64>() else {
            return;
        };
        if c.error {
            assert!(std::panic::catch_unwind(|| random_factored_integer(n)).is_err());
        } else {
            let (r, factors) = random_factored_integer(n);
            let mut values = vec![r];
            values.extend(factors);
            c.check(list(&values));
        }
    });
}

#[test]
fn randperm_oracle() {
    cases("randperm", |c| {
        let n = c.arg(1);
        let k = if c.args.len() == 2 { n } else { c.arg(2) };
        c.check(list(&randperm(n, k)));
    });
}

#[test]
fn shuffle_oracle() {
    cases("shuffle", |c| {
        let mut items = c.args[1..].to_vec();
        shuffle(&mut items);
        c.check(list(&items));
    });
}

// Skip random_prime arguments -3, (2, -4), (2, a), a, and undef: the
// native API cannot take signed or malformed inputs. Also skip bounds above
// u64::MAX, random_ndigit_prime(20/25/30), and random_nbit_prime(65/80/100/128):
// these need the bigint phase. Map one-argument random_prime(n) to (2, n).
fn check_prime(f: &str) {
    let mut count = 0;
    for c in common::load("random").into_iter().filter(|c| c.func == f) {
        let Ok(first) = c.args[1].parse::<u64>() else {
            continue;
        };
        let second = if c.args.len() > 2 {
            let Ok(n) = c.args[2].parse::<u64>() else {
                continue;
            };
            Some(n)
        } else {
            None
        };
        if f != "random_prime"
            && (first > u64::from(u32::MAX)
                || (f == "random_ndigit_prime" && first >= 20)
                || (f != "random_ndigit_prime" && first > 64))
        {
            continue;
        }
        count += 1;
        csrand(&unhex(&c.args[0]));
        let got = std::panic::catch_unwind(|| match f {
            "random_prime" => {
                second.map_or_else(|| random_prime(2, first), |n| random_prime(first, n))
            }
            "random_ndigit_prime" => random_ndigit_prime(u32::try_from(first).unwrap()),
            "random_nbit_prime" => random_nbit_prime(u32::try_from(first).unwrap()),
            "random_safe_prime" => random_safe_prime(u32::try_from(first).unwrap()),
            _ => unreachable!(),
        });
        if c.error {
            assert!(got.is_err(), "line {} must panic", c.line);
        } else {
            c.check(got.unwrap());
        }
    }
    assert!(count > 0);
}

#[test]
fn random_prime_oracle() {
    check_prime("random_prime");
}
#[test]
fn random_ndigit_prime_oracle() {
    check_prime("random_ndigit_prime");
}
#[test]
fn random_nbit_prime_oracle() {
    check_prime("random_nbit_prime");
}
#[test]
fn random_safe_prime_oracle() {
    check_prime("random_safe_prime");
}

// MPU's native Maurer and proven paths are aliases of random_nbit_prime.
// random.tsv has no separate rows for these three names. Its nbit rows
// supply the native oracle without requiring Perl at test time.
fn native_proven_oracle(generate: impl Fn(u32) -> Option<u64>) {
    common::each("random", "random_nbit_prime", |case| {
        let bits: u32 = case.arg(1);
        csrand(&unhex(&case.args[0]));
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| generate(bits)));
        if case.error {
            assert!(result.is_err(), "fixture line {}", case.line);
        } else if bits > 64 {
            // Bigint random_nbit_prime cases are not native u64 inputs yet.
            assert_eq!(result.unwrap(), None);
        } else {
            case.check(result.unwrap());
        }
    });
}

#[test]
fn random_maurer_prime_oracle() {
    native_proven_oracle(random_maurer_prime);
}

#[test]
fn random_proven_prime_oracle() {
    native_proven_oracle(random_proven_prime);
}

#[test]
fn random_proven_prime_with_cert_oracle() {
    native_proven_oracle(|bits| {
        random_proven_prime_with_cert(bits).map(|(prime, cert)| {
            assert_eq!(cert, format!(
                "[MPU - Primality Certificate]\nVersion 1.0\n\nProof for:\nN {prime}\n\nType Small\nN {prime}\n"
            ));
            prime
        })
    });
}

#[test]
fn random_strong_prime_oracle() {
    common::each("random", "random_strong_prime", |case| {
        let bits = case.arg(1);
        let result = std::panic::catch_unwind(|| random_strong_prime(bits));
        if case.error {
            assert!(result.is_err());
        } else {
            // All valid random_strong_prime fixture cases require bigint
            // (128, 129, 200, 247, or 256 bits), beyond this native row.
            assert!(bits > 64);
            assert_eq!(result.unwrap(), None);
        }
    });
}

fn seed(hex: &str) {
    let bytes: Vec<_> = hex
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
        .collect();
    csrand(&bytes);
}

#[test]
fn compat_random_semiprime() {
    let mut count = 0;
    for case in common::load("random")
        .into_iter()
        .filter(|c| c.func == "random_semiprime")
    {
        let bits: u32 = case.arg(1);
        // The 65-, 81-, and 100-bit oracle cases need the bigint phase.
        if bits > 64 {
            assert_eq!(random_semiprime(bits), None);
            continue;
        }
        seed(&case.args[0]);
        if case.error {
            assert!(std::panic::catch_unwind(|| random_semiprime(bits)).is_err());
        } else {
            case.check(random_semiprime(bits));
        }
        count += 1;
    }
    assert!(count > 100);
}

#[test]
fn compat_random_maurer_prime_with_cert() {
    // random.tsv has no Maurer rows. Native MPU dispatch uses random_nbit_prime.
    for bits in 2..=64 {
        csrand(&[15, 0, 0, 0]);
        let (p, cert) = random_maurer_prime_with_cert(bits).unwrap();
        csrand(&[15, 0, 0, 0]);
        assert_eq!(Some(p), random_shawe_taylor_prime(bits));
        assert_eq!(is_prime(p), 2);
        assert!(cert.ends_with(&format!("Type Small\nN {p}\n")));
    }
}

#[test]
fn compat_random_shawe_taylor_prime() {
    // No Shawe–Taylor rows exist; the native XS wrapper uses random_nbit_prime.
    csrand(&[15, 0, 0, 0]);
    assert_eq!(random_shawe_taylor_prime(2), Some(3));
    assert_eq!(random_shawe_taylor_prime(65), None);
}

#[test]
fn compat_random_shawe_taylor_prime_with_cert() {
    // No certificate rows exist in random.tsv. Check the MPU certificate format.
    csrand(&[15, 0, 0, 0]);
    let (p, cert) = random_shawe_taylor_prime_with_cert(32).unwrap().unwrap();
    assert_eq!(is_prime(p), 2);
    assert!(cert.ends_with(&format!("Type Small\nN {p}\n")));
    assert_eq!(random_shawe_taylor_prime_with_cert(65), Ok(None));
}

//! Seeded compatibility with MPU 0.75, including t/35-cipher.t.
mod common;
use common::Case;
use math_prime_util::*;
use std::fmt::Write;

fn unhex(s: &str) -> Vec<u8> {
    s.as_bytes()
        .chunks_exact(2)
        .map(|c| u8::from_str_radix(std::str::from_utf8(c).unwrap(), 16).unwrap())
        .collect()
}
fn hex(bytes: &[u8]) -> String {
    bytes.iter().fold(String::new(), |mut s, b| {
        write!(s, "{b:02x}").unwrap();
        s
    })
}
fn floats(c: &Case, values: &[f64]) {
    let expected: Vec<f64> = c
        .result
        .as_ref()
        .unwrap()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();
    assert_eq!(values.len(), expected.len());
    for (&got, &want) in values.iter().zip(&expected) {
        assert!(
            (got - want).abs() <= 1e-14 * want.abs().max(1.0),
            "line {}: {got} != {want}",
            c.line
        );
    }
}
#[allow(clippy::too_many_lines)] // Keep the fixture dispatch in one match.
fn check(f: &str) {
    let mut count = 0;
    for c in common::load("random").into_iter().filter(|c| c.func == f) {
        count += 1;
        csrand(&unhex(&c.args[0]));
        match f {
            "mixed_seq" => {
                let a = irand();
                let b = irand64();
                let bytes = hex(&random_bytes(3));
                let d = irand32();
                let bits = urandomb(5);
                let last = irand();
                c.check(Some(format!("{a} {b} {bytes} {d} {bits} {last}")));
            }
            "irand" | "srand42_irand" | "blake_irand" => c.check(Some(irand())),
            "irand32" => c.check(Some(irand32())),
            "irand64" | "tag_irand64" => c.check(Some(irand64())),
            "irand_seq" | "irand32_seq" | "irand64_seq" | "urandomm_seq" => {
                let n = c.result.as_ref().unwrap().split_whitespace().count();
                let values: Vec<String> = (0..n)
                    .map(|_| {
                        match f {
                            "irand64_seq" => irand64(),
                            "urandomm_seq" => urandomm(c.arg(1)),
                            _ => irand(),
                        }
                        .to_string()
                    })
                    .collect();
                c.check(Some(values.join(" ")));
            }
            "drand" | "rand" => floats(
                &c,
                &[drand(c.args.get(1).map_or(1.0, |v| v.parse().unwrap()))],
            ),
            "drand_seq" => {
                let n = c.result.as_ref().unwrap().split_whitespace().count();
                floats(&c, &(0..n).map(|_| drand(1.0)).collect::<Vec<_>>());
            }
            "random_bytes"
            | "random_bytes_seq"
            | "srand15_random_bytes4"
            | "blake_random_bytes7" => {
                c.check(Some(hex(&random_bytes(c.arg(1)))));
            }
            "irand32_after_bytes" | "irand64_after_bytes" | "random_bytes_len_tail" => {
                random_bytes(c.arg(1));
                match f {
                    "irand32_after_bytes" => c.check(Some(irand32())),
                    "irand64_after_bytes" => c.check(Some(irand64())),
                    _ => c.check(Some(hex(&random_bytes(8)))),
                }
            }
            "urandomb" => {
                // MPU's bigint urandomb(65..=128) has no native u64 API.
                if c.arg::<u32>(1) > 64 {
                    continue;
                }
                c.check(Some(urandomb(c.arg(1))));
            }
            "urandomm" => {
                // Bigint moduli above u64::MAX need the future bigint API.
                let Ok(n) = c.args[1].parse::<u64>() else {
                    continue;
                };
                if c.error {
                    assert!(std::panic::catch_unwind(|| urandomm(n)).is_err());
                } else {
                    c.check(Some(urandomm(n)));
                }
            }
            "urandomr" => {
                // Signed bounds and bounds above u64::MAX need a wider API.
                let (Ok(low), Ok(high)) = (c.args[1].parse::<u64>(), c.args[2].parse::<u64>())
                else {
                    continue;
                };
                c.check(urandomr(low, high));
            }
            "blake_irand_after7" => {
                random_bytes(7);
                c.check(Some(irand()));
            }
            "blake_drand_after_irand" => {
                random_bytes(7);
                irand();
                floats(&c, &[drand(1.0)]);
            }
            "tag_irand_irand64" => {
                irand();
                c.check(Some(irand64()));
            }
            "tag_irand_irand32_rand" => {
                let a = irand();
                let b = irand32();
                let r = rand(1.0);
                let expected: Vec<&str> = c.result.as_ref().unwrap().split_whitespace().collect();
                assert_eq!(a.to_string(), expected[0]);
                assert_eq!(b.to_string(), expected[1]);
                assert!((r - expected[2].parse::<f64>().unwrap()).abs() < 1e-14);
            }
            _ => panic!("unhandled fixture {f}"),
        }
    }
    assert!(count > 0, "missing fixture {f}");
}
#[test]
fn csrand_oracle() {
    check("irand_seq");
    check("irand32_seq");
    check("irand64_seq");
}
#[test]
fn srand_oracle() {
    srand(42);
    assert_eq!(irand(), 445_265_827);
    check("srand15_random_bytes4");
}
#[test]
fn irand_oracle() {
    for f in [
        "irand",
        "srand42_irand",
        "blake_irand",
        "blake_irand_after7",
    ] {
        check(f);
    }
}
#[test]
fn irand32_oracle() {
    check("irand32");
    check("irand32_after_bytes");
}
#[test]
fn irand64_oracle() {
    for f in [
        "irand64",
        "irand64_after_bytes",
        "tag_irand64",
        "tag_irand_irand64",
    ] {
        check(f);
    }
}
#[test]
fn drand_oracle() {
    check("drand");
    check("drand_seq");
    check("blake_drand_after_irand");
}
#[test]
fn rand_oracle() {
    check("rand");
    check("tag_irand_irand32_rand");
}
#[test]
fn random_bytes_oracle() {
    check("mixed_seq");
    for f in [
        "random_bytes",
        "random_bytes_seq",
        "random_bytes_len_tail",
        "blake_random_bytes7",
    ] {
        check(f);
    }
}
#[test]
fn entropy_bytes_os() {
    assert_eq!(entropy_bytes(0).unwrap().len(), 0);
    assert_eq!(entropy_bytes(40).unwrap().len(), 40);
}
#[test]
fn urandomb_oracle() {
    check("urandomb");
}
#[test]
fn urandomm_oracle() {
    check("urandomm");
    check("urandomm_seq");
}
#[test]
fn urandomr_oracle() {
    check("urandomr");
}

#[test]
fn cipher_vectors() {
    let plain = "Now I’m not gonna make a lot of extravagant claims for this little machine. Sure, it’ll change your whole life for the better, but that’s all.".as_bytes();
    let mut count = 0;
    for c in common::load("random")
        .into_iter()
        .filter(|c| c.func.starts_with("cipher_"))
    {
        count += 1;
        csrand(&unhex(&c.args[0]));
        let stream = random_bytes(c.arg(1));
        assert_eq!(stream.len(), plain.len());
        let cipher: Vec<_> = stream.iter().zip(plain).map(|(a, b)| a ^ b).collect();
        if c.func.ends_with("_L") {
            c.check(Some(u32::from_le_bytes(cipher[..4].try_into().unwrap())));
        } else {
            c.check(Some(hex(&cipher)));
        }
        csrand(&unhex(&c.args[0]));
        let decoded: Vec<_> = random_bytes(cipher.len())
            .iter()
            .zip(cipher)
            .map(|(a, b)| a ^ b)
            .collect();
        assert_eq!(decoded, plain);
    }
    assert_eq!(count, 6);
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

//! Native random-prime checks against MPU 0.75.
mod common;
use math_prime_util::{
    csrand, is_prime, random_maurer_prime_with_cert, random_nbit_prime, random_ndigit_prime,
    random_prime, random_safe_prime, random_semiprime, random_shawe_taylor_prime,
    random_shawe_taylor_prime_with_cert,
};

fn unhex(s: &str) -> Vec<u8> {
    s.as_bytes()
        .chunks_exact(2)
        .map(|c| u8::from_str_radix(std::str::from_utf8(c).unwrap(), 16).unwrap())
        .collect()
}

fn seed(hex: &str) {
    csrand(&unhex(hex));
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

//! Native random-prime checks against MPU 0.75.
mod common;
use math_prime_util::{
    csrand, is_prime, random_maurer_prime_with_cert, random_semiprime, random_shawe_taylor_prime,
    random_shawe_taylor_prime_with_cert,
};

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

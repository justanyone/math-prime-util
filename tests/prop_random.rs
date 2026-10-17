//! Algebraic laws and state isolation for native random primes.
use math_prime_util::{
    csrand, is_prime, powmod, random_maurer_prime_with_cert, random_semiprime,
    random_shawe_taylor_prime, random_shawe_taylor_prime_with_cert,
};
use proptest::prelude::*;

fn verify(p: u64, cert: &str) {
    assert_eq!(is_prime(p), 2);
    assert!(cert.contains(&format!("Proof for:\nN {p}\n")));
    if cert.contains("Type Pocklington") {
        let field = |name: &str| {
            cert.lines()
                .find_map(|l| l.strip_prefix(name))
                .unwrap()
                .parse::<u64>()
                .unwrap()
        };
        let q = field("Q ");
        let a = field("A ");
        assert_eq!(is_prime(q), 2);
        assert_eq!((p - 1) % (2 * q), 0);
        assert!(u128::from(q) * u128::from(q) > u128::from(p));
        let z = powmod(a, (p - 1) / q, p).unwrap();
        assert_eq!(powmod(z, q, p), Some(1));
        let (mut x, mut y) = (z.wrapping_sub(1), p);
        while y != 0 {
            (x, y) = (y, x % y);
        }
        assert_eq!(x, 1);
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(32))]
    #[test]
    fn prime_bits_and_certificates(bits in 2_u32..=64, seed in any::<[u8;40]>()) {
        csrand(&seed);
        let (p, cert) = random_maurer_prime_with_cert(bits).unwrap();
        prop_assert_eq!(64 - p.leading_zeros(), bits);
        verify(p, &cert);
        let (p, cert) = random_shawe_taylor_prime_with_cert(bits).unwrap().unwrap();
        prop_assert_eq!(64 - p.leading_zeros(), bits);
        verify(p, &cert);
    }
    #[test]
    fn semiprime_factors(bits in 4_u32..=32, seed in any::<[u8;40]>()) {
        csrand(&seed);
        let n = random_semiprime(bits).unwrap();
        prop_assert_eq!(64 - n.leading_zeros(), bits);
        let p = (2..=(1_u64 << bits.div_ceil(2))).find(|p| n % p == 0).unwrap();
        prop_assert_eq!(is_prime(p), 2);
        prop_assert_eq!(is_prime(n/p), 2);
        prop_assert_eq!(64 - p.leading_zeros(), bits/2);
        prop_assert_eq!(64 - (n/p).leading_zeros(), bits - bits/2);
    }
}

#[test]
fn bounds_reseed_and_threads() {
    for bits in [0, 1] {
        assert!(std::panic::catch_unwind(|| random_maurer_prime_with_cert(bits)).is_err());
        assert!(std::panic::catch_unwind(|| random_shawe_taylor_prime(bits)).is_err());
        assert!(std::panic::catch_unwind(|| random_shawe_taylor_prime_with_cert(bits)).is_err());
    }
    for bits in 0..4 {
        assert!(std::panic::catch_unwind(|| random_semiprime(bits)).is_err());
    }
    assert_eq!(random_maurer_prime_with_cert(65), None);
    assert_eq!(random_shawe_taylor_prime_with_cert(u32::MAX), Ok(None));
    let run = || {
        csrand(&[42; 40]);
        (
            random_maurer_prime_with_cert(64),
            random_shawe_taylor_prime_with_cert(64),
            random_semiprime(64),
        )
    };
    let expected = run();
    assert_eq!(expected, run());
    let handles: Vec<_> = (0..4).map(|_| std::thread::spawn(run)).collect();
    for h in handles {
        assert_eq!(h.join().unwrap(), expected);
    }
    for bits in 2..=64 {
        csrand(&[u8::try_from(bits).unwrap(); 40]);
        let (p, cert) = random_shawe_taylor_prime_with_cert(bits).unwrap().unwrap();
        verify(p, &cert);
    }
}

#[test]
fn unsupported_sizes_leave_stream_unchanged() {
    csrand(&[7; 40]);
    let expected = math_prime_util::irand64();
    csrand(&[7; 40]);
    assert_eq!(random_maurer_prime_with_cert(65), None);
    assert_eq!(random_shawe_taylor_prime(65), None);
    assert_eq!(random_shawe_taylor_prime_with_cert(65), Ok(None));
    assert_eq!(random_semiprime(65), None);
    assert_eq!(math_prime_util::irand64(), expected);
}

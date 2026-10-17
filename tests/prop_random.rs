//! Algebraic laws and state isolation for native random primes.
use math_prime_util::{
    csrand, irand64, is_prime, powmod, random_maurer_prime_with_cert, random_nbit_prime,
    random_ndigit_prime, random_prime, random_safe_prime, random_semiprime,
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

proptest! {
    #[test]
    fn prime_size_laws(seed in any::<u64>(), bits in 2u32..=64, digits in 1u32..=19) {
        srand(seed);
        let p = random_nbit_prime(bits).unwrap();
        prop_assert_eq!(64 - p.leading_zeros(), bits);
        prop_assert_eq!(is_prime(p), 2);
        let p = random_ndigit_prime(digits).unwrap();
        prop_assert_eq!(p.ilog10() + 1, digits);
        prop_assert_eq!(is_prime(p), 2);
    }
    #[test]
    fn safe_prime_laws(seed in any::<u64>(), bits in 3u32..=64) {
        srand(seed);
        let p = random_safe_prime(bits).unwrap();
        prop_assert_eq!(64 - p.leading_zeros(), bits);
        prop_assert_eq!(is_prime(p), 2);
        prop_assert_eq!(is_prime((p - 1) / 2), 2);
    }
    #[test]
    fn prime_range_laws(seed in any::<u64>(), low in 0u64..10000, width in 0u64..100) {
        srand(seed);
        let high = low + width;
        let p = random_prime(low, high);
        prop_assert_eq!(p.is_some(), (low..=high).any(|n| is_prime(n) != 0));
        if let Some(p) = p {
            prop_assert!((low..=high).contains(&p));
            prop_assert_eq!(is_prime(p), 2);
        }
    }
}

fn prime_sequence() -> [Option<u64>; 4] {
    [
        random_prime(2, 1000),
        random_ndigit_prime(6),
        random_nbit_prime(32),
        random_safe_prime(16),
    ]
}

#[test]
fn prime_reseed_and_thread_state() {
    srand(15);
    let expected = prime_sequence();
    srand(42);
    prime_sequence();
    srand(15);
    assert_eq!(prime_sequence(), expected);
    let children: Vec<_> = (0..4)
        .map(|_| {
            std::thread::spawn(|| {
                srand(15);
                prime_sequence()
            })
        })
        .collect();
    for child in children {
        assert_eq!(child.join().unwrap(), expected);
    }
    srand(15);
    assert_eq!(prime_sequence(), expected);
}

#[test]
fn native_prime_boundaries() {
    srand(15);
    for bits in 2..=64 {
        assert_eq!(64 - random_nbit_prime(bits).unwrap().leading_zeros(), bits);
    }
    for bits in 3..=64 {
        assert_eq!(64 - random_safe_prime(bits).unwrap().leading_zeros(), bits);
    }
    let max_prime = 18_446_744_073_709_551_557;
    assert_eq!(random_prime(max_prime, u64::MAX), Some(max_prime));
    assert_eq!(random_prime(max_prime + 1, u64::MAX), None);
    assert_eq!(random_prime(0, 1), None);
    assert_eq!(random_prime(3, 2), None);
    assert_eq!(random_prime(14, 16), None);
    assert_eq!(random_prime(2, 2), Some(2));
    assert_eq!(random_nbit_prime(65), None);
    assert_eq!(random_nbit_prime(u32::MAX), None);
    assert_eq!(random_safe_prime(65), None);
    assert_eq!(random_ndigit_prime(20), None);
    assert_eq!(random_ndigit_prime(u32::MAX), None);
}

#[test]
fn empty_prime_calls_preserve_stream() {
    srand(15);
    let expected = irand64();
    srand(15);
    assert_eq!(random_prime(14, 16), None);
    assert_eq!(random_prime(3, 2), None);
    assert_eq!(random_prime(0, 1), None);
    assert_eq!(random_prime(2, 2), Some(2));
    assert_eq!(random_nbit_prime(65), None);
    assert_eq!(random_ndigit_prime(20), None);
    assert_eq!(random_safe_prime(65), None);
    assert_eq!(random_safe_prime(4), Some(11));
    assert_eq!(random_safe_prime(5), Some(23));
    assert_eq!(irand64(), expected);
}

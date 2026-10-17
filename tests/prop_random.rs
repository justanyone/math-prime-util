//! Stream laws, input bounds, and independent thread state.
use math_prime_util::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn split_stream(seed in any::<u64>(), a in 0usize..2200, b in 0usize..2200) {
        srand(seed);
        let whole = random_bytes(a+b);
        srand(seed);
        let mut split = random_bytes(a);
        split.extend(random_bytes(b));
        prop_assert_eq!(whole, split);
    }
    #[test]
    fn integer_order(seed in any::<u64>(), skip in 0usize..2050) {
        srand(seed); random_bytes(skip);
        let high = irand(); let low = irand();
        srand(seed); random_bytes(skip);
        prop_assert_eq!(irand64(), (high << 32) | low);
    }
    #[test]
    fn bounds(seed in any::<u64>(), bits in 0u32..=64, n in 1u64..=u64::MAX, low in any::<u64>(), high in any::<u64>()) {
        srand(seed);
        let r = urandomb(bits);
        prop_assert!(bits == 64 || u128::from(r) < (1u128 << bits));
        prop_assert!(urandomm(n) < n);
        let range = urandomr(low, high);
        prop_assert_eq!(range.is_some(), low <= high);
        if let Some(r) = range { prop_assert!((low..=high).contains(&r)); }
        prop_assert!((0.0..1.0).contains(&drand(1.0)));
    }
}
#[test]
fn repeated_seed_resets_partial_buffer() {
    csrand(b"same");
    let expected = random_bytes(1100);
    csrand(b"other");
    random_bytes(17);
    csrand(b"same");
    assert_eq!(random_bytes(1100), expected);
}
#[test]
fn threads_keep_separate_seeds() {
    srand(42);
    let expected = random_bytes(1500);
    srand(42);
    let children: Vec<_> = (0..4)
        .map(|_| {
            std::thread::spawn(|| {
                srand(15);
                random_bytes(1500)
            })
        })
        .collect();
    for child in children {
        assert_ne!(child.join().unwrap(), expected);
    }
    assert_eq!(random_bytes(1500), expected);
}
#[test]
fn empty_calls_do_not_advance() {
    srand(15);
    let expected = irand64();
    srand(15);
    assert_eq!(random_bytes(0), Vec::<u8>::new());
    assert_eq!(urandomb(0), 0);
    assert_eq!(urandomm(1), 0);
    assert_eq!(urandomr(7, 7), Some(7));
    assert_eq!(urandomr(7, 3), None);
    assert_eq!(irand64(), expected);
}
#[test]
#[should_panic(expected = "too many bits")]
fn excessive_bits() {
    urandomb(65);
}
#[test]
#[should_panic(expected = "modulus must be positive")]
fn zero_modulus() {
    urandomm(0);
}
#[test]
#[should_panic(expected = "byte count exceeds MPU limit")]
fn excessive_random_bytes() {
    random_bytes(2_147_483_647);
}
#[test]
#[should_panic(expected = "byte count exceeds MPU limit")]
fn excessive_entropy_bytes() {
    let _ = entropy_bytes(2_147_483_647);
}
#[test]
fn empty_and_short_seeds_repeat() {
    for length in 0..=41 {
        let seed = vec![3; length];
        csrand(&seed);
        let expected = random_bytes(1030);
        csrand(&seed);
        assert_eq!(random_bytes(1030), expected);
    }
}
#[test]
fn long_seeds_use_first_forty_bytes() {
    csrand(&[9; 40]);
    let expected = random_bytes(1050);
    csrand(&[9; 100]);
    assert_eq!(random_bytes(1050), expected);
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(128))]
    #[test]
    fn proven_prime_size_and_certificate(seed in any::<u64>(), bits in 2u32..=64) {
        srand(seed);
        let expected = random_maurer_prime(bits).unwrap();
        prop_assert_eq!(is_prime(expected), 2);
        prop_assert_eq!(64 - expected.leading_zeros(), bits);
        srand(seed);
        prop_assert_eq!(random_proven_prime(bits), Some(expected));
        srand(seed);
        let (prime, cert) = random_proven_prime_with_cert(bits).unwrap();
        prop_assert_eq!(prime, expected);
        prop_assert_eq!(cert, format!(
            "[MPU - Primality Certificate]\nVersion 1.0\n\nProof for:\nN {prime}\n\nType Small\nN {prime}\n"
        ));
    }
}

#[test]
fn proven_prime_all_native_sizes() {
    for bits in 2..=64 {
        srand(u64::from(bits));
        let prime = random_maurer_prime(bits).unwrap();
        assert_eq!(64 - prime.leading_zeros(), bits);
        assert_eq!(is_prime(prime), 2);
    }
    for bits in [65, 128, u32::MAX] {
        assert_eq!(random_maurer_prime(bits), None);
        assert_eq!(random_proven_prime(bits), None);
        assert_eq!(random_proven_prime_with_cert(bits), None);
    }
    for bits in [128, 129, u32::MAX] {
        assert_eq!(random_strong_prime(bits), None);
    }
}

#[test]
fn proven_prime_validation_keeps_stream_state() {
    for bits in [0, 1] {
        assert!(std::panic::catch_unwind(|| random_maurer_prime(bits)).is_err());
        assert!(std::panic::catch_unwind(|| random_proven_prime(bits)).is_err());
        assert!(std::panic::catch_unwind(|| random_proven_prime_with_cert(bits)).is_err());
    }
    for bits in [0, 1, 64, 127] {
        assert!(std::panic::catch_unwind(|| random_strong_prime(bits)).is_err());
    }
    srand(42);
    let expected = irand64();
    srand(42);
    assert_eq!(random_maurer_prime(65), None);
    assert_eq!(random_proven_prime(65), None);
    assert_eq!(random_proven_prime_with_cert(65), None);
    assert_eq!(random_strong_prime(128), None);
    assert!(std::panic::catch_unwind(|| random_maurer_prime(1)).is_err());
    assert_eq!(irand64(), expected);
}

#[test]
fn proven_prime_threads_and_repeated_launches() {
    let handles: Vec<_> = (0..4)
        .map(|_| {
            std::thread::spawn(|| {
                srand(15);
                let first = random_proven_prime_with_cert(64);
                srand(15);
                assert_eq!(first, random_proven_prime_with_cert(64));
                first
            })
        })
        .collect();
    let values: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    assert!(values.windows(2).all(|pair| pair[0] == pair[1]));
}

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

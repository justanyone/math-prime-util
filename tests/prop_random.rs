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
    assert!(random_bytes(0).is_empty());
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

//! Bounds, factor products, permutation laws, and random state isolation.
use math_prime_util::*;
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(128))]
    #[test]
    fn factored_product(seed in any::<u64>(), bound in 1u64..=u64::MAX) {
        srand(seed);
        let (n, factors) = random_factored_integer(bound);
        prop_assert!((1..=bound).contains(&n));
        prop_assert_eq!(factors.iter().product::<u64>(), n);
        prop_assert!(factors.windows(2).all(|w| w[0] <= w[1]));
        prop_assert!(factors.iter().all(|&p| is_prime(p) != 0));
    }
    #[test]
    fn semiprime_bits(seed in any::<u64>(), bits in 3u32..=24) {
        srand(seed);
        let n = random_unrestricted_semiprime(bits).unwrap();
        prop_assert_eq!(64 - n.leading_zeros(), bits);
        let mut rest = n;
        let mut count = 0;
        let mut p = 2;
        while p * p <= rest {
            while rest % p == 0 { count += 1; rest /= p; }
            p += 1;
        }
        if rest > 1 { count += 1; }
        prop_assert_eq!(count, 2);
    }
    #[test]
    fn permutation_laws(seed in any::<u64>(), n in 0u64..1000, k in 0u64..1100) {
        srand(seed);
        let values = randperm(n, k);
        prop_assert_eq!(values.len() as u64, k.min(n));
        prop_assert!(values.iter().all(|&v| v < n));
        let mut sorted = values.clone();
        sorted.sort_unstable(); sorted.dedup();
        prop_assert_eq!(sorted.len(), values.len());
        let mut items = values.clone();
        shuffle(&mut items);
        items.sort_unstable();
        prop_assert_eq!(items, sorted);
    }
}

#[test]
fn reseed_and_thread_isolation() {
    fn sequence() -> (Vec<u64>, (u64, Vec<u64>), Option<u64>) {
        srand(42);
        (
            randperm(100, 20),
            random_factored_integer(1_000_000),
            random_unrestricted_semiprime(16),
        )
    }
    let expected = sequence();
    assert_eq!(sequence(), expected);
    let handles: Vec<_> = (0..4).map(|_| std::thread::spawn(sequence)).collect();
    for handle in handles {
        assert_eq!(handle.join().unwrap(), expected);
    }
    assert_eq!(sequence(), expected);
}

#[test]
fn boundary_paths() {
    for bits in [0, 1, 2, 65, u32::MAX] {
        assert_eq!(random_unrestricted_semiprime(bits), None);
    }
    srand(7);
    for (n, k) in [
        (10, 2),
        (100, 3),
        (40_000, 30),
        (100_000, 70),
        (u64::MAX, 17),
        (100, 40),
    ] {
        let mut values = randperm(n, k);
        assert_eq!(values.len() as u64, k);
        assert!(values.iter().all(|&v| v < n));
        values.sort_unstable();
        values.dedup();
        assert_eq!(values.len() as u64, k);
    }
    let mut empty: [u64; 0] = [];
    shuffle(&mut empty);
    let mut one = [3];
    shuffle(&mut one);
    assert_eq!(one, [3]);
}

#[test]
#[should_panic(expected = "requested permutation is too large")]
fn oversized_permutation() {
    randperm(u64::MAX, u64::MAX);
}

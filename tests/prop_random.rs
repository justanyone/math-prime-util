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
        (70_000, 100),
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

fn sample_then_miller() -> (Vec<i32>, Option<u8>, Vec<i32>) {
    srand(15);
    let first = vecsample(4, &(1..=100).collect::<Vec<_>>());
    let verdict = miller_rabin_random(10_007, 3);
    let second = vecsample(2, &[1, 2, 3, 4, 5]);
    (first, verdict, second)
}

proptest! {
    #[test]
    fn sample_keeps_distinct_items(k in 0u64..25, len in 0usize..30) {
        let data: Vec<u16> = (0..u16::try_from(len).unwrap()).collect();
        srand(1);
        let got = vecsample(k, &data);
        prop_assert_eq!(u64::try_from(got.len()).unwrap(), k.min(u64::try_from(len).unwrap()));
        prop_assert!(got.iter().all(|v| data.contains(v)));
        prop_assert_eq!(got.iter().collect::<std::collections::BTreeSet<_>>().len(), got.len());
    }

    #[test]
    fn miller_accepts_primes(n in 0u64..2000, bases in 1u64..6) {
        srand(n ^ bases.wrapping_mul(0x9e37));
        let got = miller_rabin_random(u128::from(n), bases).unwrap();
        prop_assert!(got <= 1);
        if is_prime(n) == 2 {
            prop_assert_eq!(got, 1);
        }
        if bases >= 3 * (n / 4) {
            prop_assert_eq!(got, u8::from(is_prob_prime(n) != 0));
        }
    }
}

#[test]
fn sample_reseed_thread_and_repeat() {
    let expected = sample_then_miller();
    let children: Vec<_> = (0..4)
        .map(|_| std::thread::spawn(sample_then_miller))
        .collect();
    for child in children {
        assert_eq!(child.join().unwrap(), expected);
    }
    assert_eq!(sample_then_miller(), expected);
}

#[test]
fn sample_launch_advances_stream() {
    let list = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    srand(15);
    let virgin = irand64();
    srand(15);
    assert_eq!(vecsample(3, &list), vec![1, 3, 5]);
    assert_ne!(irand64(), virgin);
    srand(15);
    let _ = vecsample(3, &list);
    let mid = irand64();
    srand(15);
    let _ = vecsample(3, &list);
    let _ = vecsample(3, &list);
    assert_ne!(irand64(), mid);
}

#[test]
fn sample_edges_keep_stream_and_duplicates() {
    srand(15);
    let expected = irand64();
    srand(15);
    assert_eq!(vecsample::<u64>(0, &[1, 2, 3]), Vec::<u64>::new());
    assert_eq!(vecsample::<u64>(4, &[]), Vec::<u64>::new());
    assert_eq!(vecsample(9, &[7u64]), vec![7]);
    assert_eq!(irand64(), expected);
    let data = [3, 1, 4, 1, 5, 9];
    srand(8);
    let mut got = vecsample(10, &data);
    let mut want = data;
    got.sort_unstable();
    want.sort_unstable();
    assert_eq!(got, want);
}

#[test]
fn miller_validation_state_and_width() {
    assert!(std::panic::catch_unwind(|| miller_rabin_random(10_007, 0)).is_err());
    assert!(std::panic::catch_unwind(|| miller_rabin_random((1u128 << 80) + 3, 0)).is_err());
    srand(9);
    let virgin = irand64();
    srand(9);
    assert_eq!(miller_rabin_random(0, 1), Some(0));
    assert_eq!(miller_rabin_random(1, 4), Some(0));
    assert_eq!(miller_rabin_random(2, 1), Some(1));
    assert_eq!(miller_rabin_random(3, 9), Some(1));
    assert_eq!(miller_rabin_random(4, 1), Some(0));
    assert_eq!(miller_rabin_random(4, 3), Some(0));
    assert_eq!(miller_rabin_random(31, 3 * (31 / 4)), Some(1));
    assert_eq!(miller_rabin_random((1u128 << 80) + 3, 5), None);
    assert_eq!(irand64(), virgin);
    assert!(miller_rabin_random(u128::from(u64::MAX), 1).is_some());

    srand(9);
    assert_eq!(miller_rabin_random(31, 2), Some(1));
    assert_ne!(irand64(), virgin);
    srand(9);
    assert_eq!(miller_rabin_random(6, 1), Some(0));
    let once = irand64();
    srand(9);
    assert_eq!(miller_rabin_random(6, 2), Some(0));
    assert_eq!(irand64(), once);
    srand(9);
    assert_eq!(miller_rabin_random(6, 1), Some(0));
    miller_rabin_random(6, 1);
    assert_ne!(irand64(), once);
}

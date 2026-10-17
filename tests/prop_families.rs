//! Algebraic laws for the Ramanujan functions.
use math_prime_util::*;
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]
    #[test]
    fn ramanujan_bounds(n in 1_u64..1000) {
        let p = nth_ramanujan_prime(n).unwrap();
        prop_assert!(nth_ramanujan_prime_lower(n).unwrap() <= p);
        prop_assert!(is_prime(p) != 0);
        prop_assert!(ramanujan_prime_count_upper(p) >= n);
        prop_assert!(prime_count(p)-prime_count(p/2) >= n);
        prop_assert!(nth_ramanujan_prime_approx(n).unwrap() >= nth_ramanujan_prime_lower(n).unwrap()-1);
    }
}

#[test]
fn zero_and_repeat_calls() {
    assert_eq!(nth_ramanujan_prime(0), None);
    assert_eq!(nth_ramanujan_prime_lower(0), None);
    assert_eq!(nth_ramanujan_prime_approx(0), None);
    assert_eq!(ramanujan_prime_count_upper(0), 0);
    assert_eq!(nth_ramanujan_prime(u64::MAX), None);
    assert_eq!(nth_ramanujan_prime_lower(u64::MAX), None);
    assert_eq!(nth_ramanujan_prime_approx(u64::MAX), None);
    let handles: Vec<_> = (0..4)
        .map(|_| std::thread::spawn(|| nth_ramanujan_prime(100)))
        .collect();
    for h in handles {
        assert_eq!(h.join().unwrap(), Some(1439));
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]
    #[test]
    fn prime_power_exponent(n in any::<u64>()) {
        let k = is_prime_power(n);
        if k > 0 {
            let p = rootint(n, k);
            prop_assert_eq!(is_prob_prime(p), 2);
            prop_assert_eq!(p.checked_pow(k), Some(n));
        }
    }

    #[test]
    fn prime_power_ranges(lo in 0u64..20_000, width in 0u64..200) {
        let hi = lo + width;
        let expected: Vec<_> = (lo..=hi).filter(|&n| is_prime_power(n) > 0).collect();
        prop_assert_eq!(prime_powers(lo, hi), expected);
    }

    #[test]
    fn prime_power_neighbors(n in 2u64..1_000_000) {
        let next = next_prime_power(n).unwrap();
        prop_assert!(next > n);
        prop_assert!(is_prime_power(next) > 0);
        prop_assert!((n+1..next).all(|v| is_prime_power(v) == 0));
        prop_assert_eq!(prev_prime_power(next + 1), Some(next));
        if let Some(prev) = prev_prime_power(n) {
            prop_assert!(prev < n);
            prop_assert!(is_prime_power(prev) > 0);
            prop_assert!((prev+1..n).all(|v| is_prime_power(v) == 0));
            prop_assert_eq!(next_prime_power(prev - 1), Some(prev));
        }
    }

    #[test]
    fn known_prime_powers(index in 0usize..8, k in 1u32..64) {
        let p: u64 = [2, 3, 5, 7, 11, 13, 17, 101][index];
        if let Some(n) = p.checked_pow(k) {
            prop_assert_eq!(is_prime_power(n), k);
        }
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]
    #[test]
    fn prime_power_rank_and_bounds(k in 1u64..20_000) {
        let n = nth_prime_power(k).unwrap();
        let count = |x| prime_count(x) + (2..=x.ilog2()).map(|e| prime_count(rootint(x,e))).sum::<u64>();
        prop_assert_eq!(count(n), k);
        prop_assert_eq!(count(n-1), k-1);
        prop_assert!(nth_prime_power_lower(k).unwrap() <= n);
        prop_assert!(nth_prime_power_upper(k).unwrap() >= n);
        prop_assert!(nth_prime_power_approx(k).is_some());
    }
}

#[test]
fn prime_power_boundaries_and_repeated_calls() {
    const MAX_PRIME: u64 = 18_446_744_073_709_551_557;
    assert_eq!(next_prime_power(MAX_PRIME - 1), Some(MAX_PRIME));
    assert_eq!(next_prime_power(MAX_PRIME), None);
    assert_eq!(next_prime_power(u64::MAX), None);
    assert_eq!(prev_prime_power(u64::MAX), Some(MAX_PRIME));
    assert_eq!(prev_prime_power(0), None);
    assert_eq!(prev_prime_power(1), None);
    assert_eq!(prev_prime_power(2), None);
    assert_eq!(prime_powers(0, 1), []);
    assert_eq!(prime_powers(20, 10), []);
    assert_eq!(prime_powers(10, 20), [11, 13, 16, 17, 19]);
    assert_eq!(prime_powers(0, 10), [2, 3, 4, 5, 7, 8, 9]);
    assert_eq!(prime_powers(0, 2), [2]);
    assert_eq!(prime_powers(82, 96), [83, 89]);
    assert_eq!(prime_powers(MAX_PRIME, u64::MAX), [MAX_PRIME]);
    for k in 2..64 {
        let n = 1u64 << k;
        assert_eq!(next_prime_power(n - 1), Some(n));
        assert_eq!(prev_prime_power(n + 1), Some(n));
    }
    // A composite root must not be accepted as a prime root.
    assert_eq!(is_prime_power(121 * 169), 0);
    for p in [3u64, 5, 7] {
        assert_eq!(is_prime_power(p * 11), 0);
    }
    let expected = prime_powers(0, 10_000);
    let threads: Vec<_> = (0..4)
        .map(|_| std::thread::spawn(|| prime_powers(0, 10_000)))
        .collect();
    for thread in threads {
        assert_eq!(thread.join().unwrap(), expected);
    }
    assert_eq!(prime_powers(0, 10_000), expected);
}

#[test]
fn prime_power_repeated_and_concurrent_calls() {
    for f in [
        nth_prime_power,
        nth_prime_power_approx,
        nth_prime_power_lower,
        nth_prime_power_upper,
    ] {
        assert_eq!(f(0), None);
        let expected = f(4096);
        let threads: Vec<_> = (0..4)
            .map(|_| std::thread::spawn(move || f(4096)))
            .collect();
        for thread in threads {
            assert_eq!(thread.join().unwrap(), expected);
        }
        assert_eq!(f(4096), expected);
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]
    #[test]
    fn count_matches_list(n in 0u64..30_000) {
        let pairs = twin_primes(0, n);
        prop_assert_eq!(pairs.len() as u64, twin_prime_count(n));
        prop_assert!(pairs.iter().all(|&p| is_prime(p) != 0 && is_prime(p + 2) != 0));
        prop_assert!(pairs.windows(2).all(|p| p[0] < p[1]));
    }
    #[test]
    fn selection_inverts_count(n in 1u64..1000) {
        let p = nth_twin_prime(n).unwrap();
        prop_assert_eq!(twin_prime_count(p), n);
        prop_assert_eq!(twin_prime_count(p - 1), n - 1);
    }
    #[test]
    fn partition_preserves_pairs(lo in 0u64..5000, width in 0u64..1000) {
        let mid = lo + width / 2;
        let hi = lo + width;
        let mut pairs = twin_primes(lo, mid);
        pairs.extend(twin_primes(mid + 1, hi));
        prop_assert_eq!(pairs, twin_primes(lo, hi));
    }
}

#[test]
fn native_and_segment_boundaries() {
    assert_eq!(twin_primes(11, 10), []);
    assert_eq!(twin_primes(11, 11), vec![11]);
    assert_eq!(twin_primes(29, 29), vec![29]);
    assert_eq!(twin_primes(u64::MAX - 844, u64::MAX), vec![u64::MAX - 844]);
    assert_eq!(twin_primes(u64::MAX, u64::MAX), []);
    assert_eq!(nth_twin_prime(u64::MAX), None);
    let pairs = twin_primes(0, 1_000_100);
    assert_eq!(pairs.len() as u64, twin_prime_count(1_000_100));
    assert_eq!(twin_prime_count(10_000_000), 58_980);
    assert_eq!(
        twin_prime_count(10_000_019),
        twin_prime_count(10_000_000) + twin_primes(10_000_001, 10_000_019).len() as u64
    );
}

#[test]
fn correction_intervals_and_repeat_calls() {
    for n in [
        1999, 2000, 3999, 4000, 7999, 8000, 15_999, 16_000, 31_999, 32_000, 63_999, 64_000,
        127_999, 128_000, 255_999, 256_000, 599_999, 600_000, 999_999, 1_000_000, 3_999_999,
        4_000_000, 15_999_999, 16_000_000, 31_999_999, 32_000_000,
    ] {
        assert!(twin_prime_count_approx(n) > 0);
    }
    let handles: Vec<_> = (0..4)
        .map(|_| {
            std::thread::spawn(|| {
                assert_eq!(nth_twin_prime(100), Some(3821));
                assert_eq!(twin_prime_count(1000), 35);
                assert_eq!(twin_primes(29, 31), vec![29]);
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(128))]
    #[test]
    fn counts_and_sums(n in 1_u64..2000, k in 2_u32..8) {
        let powerful=powerful_numbers(1,n,k);
        prop_assert_eq!(powerful_count(n,k),powerful.len() as u64);
        prop_assert_eq!(sumpowerful(n,k),powerful.iter().map(|&v| u128::from(v)).sum::<u128>());
        let free: Vec<_>=(1..=n).filter(|&i| is_powerfree(i,k)).collect();
        prop_assert_eq!(powerfree_count(n,k),free.len() as u64);
        prop_assert_eq!(powerfree_sum(n,k),free.iter().map(|&v| u128::from(v)).sum::<u128>());
        prop_assert_eq!(powerfree_part_sum(n,k),(1..=n).map(|i| u128::from(powerfree_part(i,k))).sum::<u128>());
    }
    #[test]
    fn parts_and_inverse(n in 1_u64..1000, k in 2_u32..7) {
        let part=powerfree_part(n,k);
        prop_assert!(is_powerfree(part,k));
        prop_assert_eq!(n%part,0);
        prop_assert!(is_power(n/part,k));
        let p=nth_powerful(n,k).unwrap();
        prop_assert_eq!(powerful_count(p,k),n);
        prop_assert!(is_powerful(p,k));
        let q=nth_powerfree(n,k).unwrap();
        prop_assert_eq!(powerfree_count(q,k),n);
        prop_assert!(is_powerfree(q,k));
    }
}

#[test]
fn boundaries_and_repeat_calls() {
    assert!(!is_powerful(0, 0));
    assert!(is_powerful(1, 100));
    assert!(!is_powerful(2, 64));
    assert_eq!(powerful_count(0, 100), 0);
    assert_eq!(powerful_count(10, 100), 1);
    assert_eq!(powerful_numbers(10, 1, 2), Vec::<u64>::new());
    assert_eq!(powerful_numbers(2, 100, 64), Vec::<u64>::new());
    assert_eq!(powerful_numbers(0, 100, 64), vec![1]);
    assert_eq!(sumpowerful(10, 0), 55);
    assert_eq!(nth_powerful(0, 2), None);
    assert_eq!(nth_powerful(2, 64), None);
    assert_eq!(nth_powerful(10, 1), Some(10));
    assert_eq!(nth_powerfree(0, 2), None);
    assert_eq!(nth_powerfree(5, 1), None);
    assert_eq!(nth_powerfree(u64::MAX, 2), None);
    assert_eq!(nth_powerfree(u64::MAX, 64), Some(u64::MAX));
    for k in [0, 1, 2, 3, 64, 100] {
        assert_eq!(powerfree_count(0, k), 0);
        assert_eq!(powerfree_sum(0, k), 0);
        assert_eq!(powerfree_part(0, k), 0);
        assert_eq!(powerfree_part_sum(0, k), 0);
    }
    let threads: Vec<_> = (0..4)
        .map(|_| {
            std::thread::spawn(|| {
                for _ in 0..3 {
                    assert_eq!(powerful_numbers(1, 10, 2), vec![1, 4, 8, 9]);
                    assert_eq!(powerfree_count(10, 2), 7);
                }
            })
        })
        .collect();
    for thread in threads {
        thread.join().unwrap();
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(128))]
    #[test]
    fn semiprime_list_count(lo in 0u64..2000, width in 0u64..200) {
        let hi = lo + width;
        let values = semi_primes(lo, hi);
        let before = lo.checked_sub(1).map_or(0, semiprime_count);
        prop_assert_eq!(values.len() as u64, semiprime_count(hi)-before);
        prop_assert!(values.windows(2).all(|w| w[0] < w[1]));
        for n in values {
            prop_assert!((2..=math_prime_util::sqrtint(n)).any(|p| n%p == 0 && is_prime(p) != 0 && is_prime(n/p) != 0));
        }
    }
    #[test]
    fn small_approx_is_exact(n in 0u64..=623) {
        prop_assert_eq!(semiprime_count_approx(n), semiprime_count(n));
    }
    #[test]
    fn twin_estimate_grows(n in 6u64..100_000) {
        prop_assert!(nth_twin_prime_approx(n).unwrap() <= nth_twin_prime_approx(n+1).unwrap());
    }
}

#[test]
fn empty_ranges_and_repeat_calls() {
    assert_eq!(semi_primes(15, 4), []);
    assert_eq!(semi_primes(0, 3), []);
    assert_eq!(semi_primes(u64::MAX, u64::MAX), []);
    let expected = semi_primes(0, 2000);
    let workers: Vec<_> = (0..4)
        .map(|_| std::thread::spawn(|| semi_primes(0, 2000)))
        .collect();
    for worker in workers {
        assert_eq!(worker.join().unwrap(), expected);
    }
}

fn chen_semiprime(mut n: u64) -> bool {
    let mut factors = 0;
    let mut p = 2;
    while p <= n / p {
        while n % p == 0 {
            n /= p;
            factors += 1;
        }
        p += 1;
    }
    factors + u32::from(n > 1) == 2
}

proptest! {
    #[test]
    fn goldbach_agrees_with_exhaustive_search(n in 0_u64..5000) {
        let expected: Vec<_> = (2..=n / 2)
            .filter(|&p| is_prime(p) != 0 && is_prime(n - p) != 0)
            .collect();
        let pairs = goldbach_pairs(n);
        prop_assert_eq!(&pairs, &expected);
        prop_assert_eq!(goldbach_pair_count(n), pairs.len() as u64);
        prop_assert_eq!(minimal_goldbach_pair(n), pairs.first().copied());
    }

    #[test]
    fn chen_is_the_next_valid_prime(n in 0_u64..10_000) {
        let p = next_chen_prime(n).unwrap();
        prop_assert!(p > n);
        prop_assert!(is_prime(p) != 0);
        prop_assert!(is_prime(p + 2) != 0 || chen_semiprime(p + 2));
        for q in n + 1..p {
            prop_assert!(is_prime(q) == 0 || (is_prime(q + 2) == 0 && !chen_semiprime(q + 2)));
        }
    }
}

#[test]
fn repeat_and_concurrent_calls_agree() {
    let expected = goldbach_pairs(100_000);
    for handle in (0..4)
        .map(|_| {
            std::thread::spawn(|| {
                (
                    goldbach_pairs(100_000),
                    goldbach_pair_count(100_000),
                    next_chen_prime(167),
                )
            })
        })
        .collect::<Vec<_>>()
    {
        let (pairs, count, chen) = handle.join().unwrap();
        assert_eq!(pairs, expected);
        assert_eq!(count, expected.len() as u64);
        assert_eq!(chen, Some(179));
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(128))]
    #[test]
    fn sum_of_two_squares_matches_search(n in 0_u64..3000) {
        let found = (0..=n.isqrt()).any(|a| is_square(n - a * a));
        prop_assert_eq!(is_sum_of_squares(n, 2), found);
    }
    #[test]
    fn sum_of_squares_monotone_in_k(n in 0_u64..100_000) {
        prop_assert!(is_sum_of_squares(n, 4));
        prop_assert_eq!(is_sum_of_squares(n, 1), is_square(n));
        if is_sum_of_squares(n, 2) {
            prop_assert!(is_sum_of_squares(n, 3));
        }
        // Legendre: n is a sum of three squares unless n = 4^a (8b + 7).
        let tz = n.trailing_zeros();
        let legendre = n == 0 || tz % 2 == 1 || (n >> tz) % 8 != 7;
        prop_assert_eq!(is_sum_of_squares(n, 3), legendre);
    }
    #[test]
    fn gaussian_norm_law(a in 1_u64..2000, b in 1_u64..2000) {
        // For a, b both nonzero, a + bi is prime exactly when a^2 + b^2 is prime.
        prop_assert_eq!(is_gaussian_prime(a, b) == 2, is_prime(a * a + b * b) == 2);
        prop_assert_eq!(is_gaussian_prime(a, b), is_gaussian_prime(b, a));
    }
    #[test]
    fn gaussian_axis_law(p in 0_u64..100_000) {
        let expected = u8::from(p % 4 == 3 && is_prime(p) != 0) * 2;
        prop_assert_eq!(is_gaussian_prime(p, 0), expected);
        prop_assert_eq!(is_gaussian_prime(0, p), expected);
    }
}

const SMALL_RAM: [u64; 23] = [
    2, 11, 17, 29, 41, 47, 59, 67, 71, 97, 101, 107, 127, 149, 151, 167, 179, 181, 227, 229, 233,
    239, 241,
];

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]
    #[test]
    fn ramanujan_upper_is_at_least_the_prime_on_the_small_range(k in 1u64..24) {
        let prime = SMALL_RAM[usize::try_from(k).unwrap() - 1];
        prop_assert_eq!(nth_ramanujan_prime_upper(k), Some(prime));
        prop_assert!(is_ramanujan_prime(prime));
        prop_assert_eq!(is_prime(prime), 2);
    }

    #[test]
    fn ramanujan_upper_grows_past_the_table(k in 24u64..80) {
        let lo = nth_ramanujan_prime_upper(k).unwrap();
        let hi = nth_ramanujan_prime_upper(k + 1).unwrap();
        prop_assert!(lo <= hi);
        prop_assert!(lo > k);
    }
}

#[test]
fn ramanujan_validation_repeat_and_threads() {
    assert_eq!(nth_ramanujan_prime_upper(0), None);
    assert!(!is_ramanujan_prime(0));
    assert!(!is_ramanujan_prime(1));
    assert!(!is_ramanujan_prime(u64::MAX));
    let bound = nth_ramanujan_prime_upper(100);
    assert_eq!(bound, nth_ramanujan_prime_upper(100));
    assert_eq!(is_ramanujan_prime(97), is_ramanujan_prime(97));
    let threads: Vec<_> = (0..4)
        .map(|_| std::thread::spawn(|| (nth_ramanujan_prime_upper(80), is_ramanujan_prime(241))))
        .collect();
    for thread in threads {
        assert_eq!(thread.join().unwrap(), (Some(1123), true));
    }
    for (index, &prime) in SMALL_RAM.iter().enumerate() {
        assert!(is_ramanujan_prime(prime));
        if index + 1 < SMALL_RAM.len() {
            for n in prime + 1..SMALL_RAM[index + 1] {
                assert!(!is_ramanujan_prime(n), "{n}");
            }
        }
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]
    #[test]
    fn ranges_match_counts(a in 0_u64..5000, b in 0_u64..5000) {
        let lo=a.min(b); let hi=a.max(b);
        let values=ramanujan_primes(lo,hi);
        prop_assert_eq!(values.len() as u64, ramanujan_prime_count(hi)-ramanujan_prime_count(lo.saturating_sub(1)));
        prop_assert!(values.windows(2).all(|w| w[0]<w[1]));
        for p in values {prop_assert_ne!(is_prime(p),0);}
        prop_assert!(ramanujan_prime_count_lower(hi)<=ramanujan_prime_count(hi));
    }
    #[test]
    fn defining_inequality(n in 2_u64..3000, extra in 0_u64..1000) {
        let count=ramanujan_prime_count(n);
        let x=n+extra;
        prop_assert!(prime_count(x)-prime_count(x/2)>=count);
    }
}

#[test]
fn empty_and_singleton_ranges() {
    assert_eq!(ramanujan_primes(10, 2), Vec::<u64>::new());
    assert_eq!(ramanujan_primes(0, 1), Vec::<u64>::new());
    assert_eq!(ramanujan_primes(242, 250), Vec::<u64>::new());
    assert_eq!(ramanujan_primes(263, 263), vec![263]);
}

#[test]
fn repeat_and_concurrent_calls() {
    let expected = ramanujan_primes(250, 1000);
    let threads: Vec<_> = (0..4)
        .map(|_| std::thread::spawn(|| ramanujan_primes(250, 1000)))
        .collect();
    for thread in threads {
        assert_eq!(thread.join().unwrap(), expected);
    }
    assert_eq!(ramanujan_primes(250, 1000), expected);
    for k in 0_u32..=56 {
        assert!(ramanujan_prime_count(1_u64 << k) <= 1_u64 << k);
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(128))]
    #[test]
    fn nth_semiprime_is_increasing(k in 1u64..300) {
        let a = nth_semiprime(k).unwrap();
        let b = nth_semiprime(k + 1).unwrap();
        prop_assert!(a < b);
        prop_assert!(nth_semiprime_approx(k).is_some());
    }
    #[test]
    fn nth_semiprime_matches_table(k in 1u64..191) {
        prop_assert!(nth_semiprime(k).unwrap() >= nth_semiprime_approx(k).unwrap().min(nth_semiprime(k).unwrap()));
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]
    #[test]
    fn nth_perfect_power_bounds_ordered(n in 1_u64..100_000) {
        let lo = nth_perfect_power_lower(n).unwrap();
        let up = nth_perfect_power_upper(n).unwrap();
        let ap = nth_perfect_power_approx(n).unwrap();
        prop_assert!(lo <= up);
        prop_assert!(lo <= ap + 1 && ap <= up + 1);
    }

    #[test]
    fn nth_perfect_power_bounds_monotone(n in 1_u64..100_000) {
        prop_assert!(nth_perfect_power_lower(n).unwrap() <= nth_perfect_power_lower(n + 1).unwrap());
        prop_assert!(nth_perfect_power_upper(n).unwrap() <= nth_perfect_power_upper(n + 1).unwrap());
    }
}

proptest! {
    #[test]
    fn count_changes_at_positive_perfect_powers(n in 1..=u64::MAX) {
        prop_assert_eq!(perfect_power_count(n) - perfect_power_count(n - 1),
                        u64::from(is_perfect_power(n)));
    }

    #[test]
    fn neighbors_are_strict_and_adjacent(n in any::<u64>()) {
        if let Some(next) = next_perfect_power(n) {
            prop_assert!(next > n);
            prop_assert!(is_perfect_power(next));
            prop_assert!(prev_perfect_power(next).is_some_and(|prev| prev <= n));
            prop_assert_eq!(perfect_power_count(next), perfect_power_count(n) + 1);
        }
        if let Some(prev) = prev_perfect_power(n) {
            prop_assert!(prev < n);
            prop_assert!(is_perfect_power(prev));
            prop_assert!(next_perfect_power(prev).is_none_or(|next| next >= n));
        }
    }

    #[test]
    fn generated_powers_are_recognized(base in 2u64..=1000, exponent in 2u32..=63) {
        if let Some(power) = base.checked_pow(exponent) {
            prop_assert!(is_perfect_power(power));
            prop_assert_eq!(next_perfect_power(power - 1), Some(power));
            prop_assert_eq!(prev_perfect_power(power + 1), Some(power));
        }
    }
}

#[test]
fn small_values_match_enumeration() {
    let powers: Vec<u64> = (0u64..=1000)
        .filter(|&n| n <= 1 || (2u64..=31).any(|b| (2..=10).any(|k| b.checked_pow(k) == Some(n))))
        .collect();
    for n in 0..1000 {
        assert_eq!(is_perfect_power(n), powers.contains(&n));
        assert_eq!(
            next_perfect_power(n),
            powers.iter().copied().find(|&p| p > n)
        );
        assert_eq!(
            prev_perfect_power(n),
            powers.iter().copied().rev().find(|&p| p < n)
        );
        assert_eq!(
            perfect_power_count(n),
            u64::try_from(powers.iter().filter(|&&p| p != 0 && p <= n).count()).unwrap()
        );
    }
}

#[test]
fn native_boundaries_and_repeat_calls() {
    let largest = 18_446_744_065_119_617_025;
    for _ in 0..2 {
        assert_eq!(next_perfect_power(largest - 1), Some(largest));
        assert_eq!(next_perfect_power(largest), None);
        assert_eq!(next_perfect_power(u64::MAX), None);
        assert_eq!(prev_perfect_power(u64::MAX), Some(largest));
        assert_eq!(perfect_power_count(u64::MAX), 4_297_615_581);
        assert!(is_perfect_power(largest));
        assert!(!is_perfect_power(u64::MAX));
    }
}

#[test]
fn concurrent_calls_agree() {
    std::thread::scope(|scope| {
        for _ in 0..4 {
            scope.spawn(|| {
                assert_eq!(next_perfect_power(64), Some(81));
                assert_eq!(prev_perfect_power(64), Some(49));
                assert_eq!(perfect_power_count(64), 11);
                assert!(is_perfect_power(64));
            });
        }
    });
}

#[test]
fn all_binary_exponents_are_perfect_powers() {
    for k in 2..=63 {
        let n = 1u64 << k;
        assert!(is_perfect_power(n), "exponent {k}");
        assert_eq!(next_perfect_power(n - 1), Some(n));
        assert_eq!(prev_perfect_power(n + 1), Some(n));
    }
}

// Compare against factor exponents without using the powerful implementation.
fn powerful_by_trial_division(mut n: u64, k: u32) -> bool {
    if n == 0 {
        return false;
    }
    let mut p = 2;
    while p <= n / p {
        let mut exponent = 0;
        while n % p == 0 {
            n /= p;
            exponent += 1;
        }
        if exponent > 0 && exponent < k {
            return false;
        }
        p += 1;
    }
    n == 1 || k <= 1
}

#[test]
fn powerful_factor_exponents_and_prefixes() {
    for k in 0..=8 {
        let mut count = 0;
        let mut sum = 0_u128;
        for n in 0..=512 {
            let expected = powerful_by_trial_division(n, k);
            assert_eq!(is_powerful(n, k), expected, "n={n}, k={k}");
            count += u64::from(expected);
            sum += if expected { u128::from(n) } else { 0 };
            assert_eq!(powerful_count(n, k), count, "n={n}, k={k}");
            assert_eq!(sumpowerful(n, k), sum, "n={n}, k={k}");
        }
    }
}

#[test]
fn powerful_range_thresholds_and_native_edges() {
    for lo in [1, 499_999, 500_000, 399_999_999, 400_000_000] {
        for k in [2, 3, 4] {
            let hi = lo + 2500;
            let expected: Vec<_> = (lo..=hi).filter(|&n| is_powerful(n, k)).collect();
            let values = powerful_numbers(lo, hi, k);
            assert_eq!(values, expected);
            assert!(values.windows(2).all(|pair| pair[0] < pair[1]));
            assert_eq!(
                powerful_count(hi, k) - powerful_count(lo - 1, k),
                values.len() as u64
            );
        }
    }
    assert_eq!(
        powerful_numbers(u64::MAX - 1, u64::MAX, 1),
        vec![u64::MAX - 1, u64::MAX]
    );
    assert!(powerful_numbers(u64::MAX - 1, u64::MAX, 2).is_empty());
    for k in [63, 64, u32::MAX] {
        let expected = if k == 63 {
            vec![1, 1_u64 << 63]
        } else {
            vec![1]
        };
        assert_eq!(powerful_numbers(0, u64::MAX, k), expected);
        assert_eq!(powerful_count(u64::MAX, k), expected.len() as u64);
        assert_eq!(
            sumpowerful(u64::MAX, k),
            expected.iter().map(|&n| u128::from(n)).sum::<u128>()
        );
    }
    assert_eq!(
        sumpowerful(u64::MAX, 1),
        u128::from(u64::MAX) * (u128::from(u64::MAX) + 1) / 2
    );
}

#[test]
fn powerful_parallel_repeat_calls() {
    let threads: Vec<_> = (0..4)
        .map(|_| {
            std::thread::spawn(|| {
                for _ in 0..3 {
                    assert!(is_powerful(72, 2));
                    assert_eq!(powerful_numbers(1, 100, 3), vec![1, 8, 16, 27, 32, 64, 81]);
                    assert_eq!(powerful_count(100, 3), 7);
                    assert_eq!(sumpowerful(100, 3), 229);
                }
            })
        })
        .collect();
    for thread in threads {
        thread.join().unwrap();
    }
}

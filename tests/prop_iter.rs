//! Algebraic laws and state isolation for iterator loops.
use math_prime_util::iter::*;
use proptest::prelude::*;
use std::ops::ControlFlow;

proptest! {
    #[test]
    fn partition_sums(n in 0_u64..20) {
        for p in partitions_of(n, PartitionOptions::default()) {
            prop_assert_eq!(p.iter().sum::<u64>(), n);
            prop_assert!(p.windows(2).all(|v| v[0] <= v[1]));
        }
    }
    #[test]
    fn composition_count(n in 1_u64..12) {
        prop_assert_eq!(compositions_of(n, PartitionOptions::default()).count(), 1 << (n-1));
    }
    #[test]
    fn permutation_laws(n in 0_usize..8) {
        let mut count = 0;
        for mut p in permutations_of(n) { p.sort_unstable(); prop_assert_eq!(p, (0..n).collect::<Vec<_>>()); count += 1; }
        prop_assert_eq!(count, (1..=n).product::<usize>());
    }
    #[test]
    fn prime_composite_cover(hi in 0_u64..1000) {
        prop_assert_eq!(primes_in(0..=hi).count() + composites_in(0..=hi).count(), usize::try_from(hi.saturating_sub(1)).unwrap());
    }
}

#[test]
fn state_is_local_and_unwind_safe() {
    let mut outer = Vec::new();
    let result: ControlFlow<()> = forprimes(2..=11, |p| {
        outer.push(p);
        assert!(forprimes(2..=100, |_| lastfor()).is_break());
        ControlFlow::Continue(())
    });
    assert!(result.is_continue());
    assert_eq!(outer, [2, 3, 5, 7, 11]);
    assert!(
        std::panic::catch_unwind(|| {
            let _: ControlFlow<()> = forperm(3, |_| panic!("callback"));
        })
        .is_err()
    );
    assert_eq!(permutations_of(3).count(), 6);
    let workers: Vec<_> = (0..4)
        .map(|_| std::thread::spawn(|| primes_in(0..=100).count()))
        .collect();
    for worker in workers {
        assert_eq!(worker.join().unwrap(), 25);
    }
}
#[test]
fn empty_break_and_snapshot() {
    assert_eq!(set_product::<u8>(&[]).count(), 0);
    assert_eq!(set_product(&[vec![1], vec![]]).count(), 0);
    let mut sets = vec![vec![1, 2], vec![3, 4]];
    let mut iter = set_product(&sets);
    sets.clear();
    assert_eq!(iter.by_ref().count(), 4);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(combinations_of(2, Some(3)).count(), 0);
    for n in 0..8 {
        assert!(forpart(n, PartitionOptions::default(), |_| lastfor()).is_break());
        assert!(forcomp(n, PartitionOptions::default(), |_| lastfor()).is_break());
        assert!(forcomb(usize::try_from(n).unwrap(), Some(0), |_| lastfor()).is_break());
        assert!(forperm(usize::try_from(n).unwrap(), |_| lastfor()).is_break());
    }
    assert!(forderange(1, |_| lastfor()).is_continue());
    assert!(formultiperm::<u8, _>(&[], |_| lastfor()).is_break());
    assert!(forsetproduct(&[vec![1]], |_| lastfor()).is_break());
    assert!(forcomposites(4..=9, |_| lastfor()).is_break());
    assert!(foroddcomposites(4..=9, |_| lastfor()).is_break());
}
#[test]
#[should_panic(expected = "partition argument overflow")]
fn invalid_partition_bound() {
    let _ = partitions_of(u64::MAX, PartitionOptions::default());
}

#[test]
fn closure_streams_match_iterators() {
    macro_rules! same {
        ($function:path, ($($arg:expr),*), $iter:expr) => {{
            let mut values = Vec::new();
            let result: ControlFlow<()> = $function($($arg,)* |value| {
                values.push(value);
                ControlFlow::Continue(())
            });
            assert!(result.is_continue());
            assert_eq!(values, $iter.collect::<Vec<_>>());
        }};
    }
    let options = PartitionOptions::default();
    same!(forpart, (8, options), partitions_of(8, options));
    same!(forcomp, (8, options), compositions_of(8, options));
    same!(forcomb, (6, None), combinations_of(6, None));
    same!(forperm, (5), permutations_of(5));
    same!(forderange, (5), derangements_of(5));
    same!(
        formultiperm,
        (&[1, 1, 2, 3]),
        multipermutations_of(&[1, 1, 2, 3])
    );
    let sets = [vec![1, 2], vec![3, 4]];
    same!(forsetproduct, (&sets), set_product(&sets));
    same!(forprimes, (0..=50), primes_in(0..=50));
    same!(forcomposites, (0..=50), composites_in(0..=50));
    same!(foroddcomposites, (0..=50), odd_composites_in(0..=50));
}

#[test]
fn terminal_and_duplicate_state() {
    let mut first = prime_iterator(10);
    assert_eq!(first.next(), Some(11));
    let mut second = first.clone();
    assert_eq!(first.next(), Some(13));
    assert_eq!(second.next(), Some(13));
    let mut end = prime_iterator(u64::MAX);
    assert_eq!(end.next(), None);
    assert_eq!(end.next(), None);
    for n in 0..8 {
        let expected: Vec<_> = permutations_of(n)
            .filter(|p| p.iter().enumerate().all(|(i, &v)| i != v))
            .collect();
        assert_eq!(derangements_of(n).collect::<Vec<_>>(), expected);
        assert_eq!(combinations_of(n, None).count(), 1 << n);
    }
    let mut multiset = multipermutations_of(&[1, 1]);
    assert_eq!(multiset.next(), Some(vec![1, 1]));
    assert_eq!(multiset.next(), None);
    assert_eq!(multiset.next(), None);
    let options = PartitionOptions {
        nmax: Some(0),
        ..PartitionOptions::default()
    };
    assert_eq!(partitions_of(5, options).count(), 0);
    let options = PartitionOptions {
        amin: Some(10),
        ..PartitionOptions::default()
    };
    assert_eq!(partitions_of(5, options).count(), 0);
}

proptest! {
    #[test]
    fn semiprimes_have_two_factors(hi in 0_u64..2000) {
        for n in semiprimes_in(0..=hi) {
            let omega: u32 = factored_in(n..=n).next().unwrap().1.iter().map(|&(_, e)| e).sum();
            prop_assert_eq!(omega, 2);
        }
        let mut count = 0;
        let _ = forsemiprimes(0..=hi, |_| { count += 1; ControlFlow::<()>::Continue(()) });
        prop_assert_eq!(count, semiprimes_in(0..=hi).count());
    }
    #[test]
    fn almost_primes_match_omega(k in 1_u32..5, hi in 0_u64..500) {
        for n in almost_primes_in(k, 0..=hi) {
            let omega: u32 = factored_in(n..=n).next().unwrap().1.iter().map(|&(_, e)| e).sum();
            prop_assert_eq!(omega, k);
        }
        let expected = (1..=hi).filter(|&n| factored_in(n..=n).next().unwrap().1.iter().map(|&(_, e)| e).sum::<u32>() == k).count();
        prop_assert_eq!(almost_primes_in(k, 0..=hi).count(), expected);
    }
    #[test]
    fn factored_products_rebuild_n(lo in 1_u64..1000, len in 0_u64..50) {
        for (n, factors) in factored_in(lo..=lo + len) {
            let product: u64 = factors.iter().map(|&(p, e)| p.pow(e)).product();
            prop_assert_eq!(product, n);
            prop_assert!(factors.windows(2).all(|w| w[0].0 < w[1].0));
        }
        prop_assert_eq!(factored_in(lo..=lo + len).count(), usize::try_from(len + 1).unwrap());
    }
    #[test]
    fn squarefree_has_unit_exponents(hi in 1_u64..2000) {
        let mut seen = 0_usize;
        for (n, factors) in squarefree_in(1..=hi) {
            prop_assert!(factors.iter().all(|&(_, e)| e == 1));
            prop_assert_eq!(factors.iter().map(|&(p, _)| p).product::<u64>(), n);
            seen += 1;
        }
        let expected = (1..=hi).filter(|&n| factored_in(n..=n).next().unwrap().1.iter().all(|&(_, e)| e == 1)).count();
        prop_assert_eq!(seen, expected);
    }
    #[test]
    fn divisors_divide_and_ascend(n in 1_u64..100_000) {
        let divs: Vec<u64> = divisors_of(n).collect();
        prop_assert!(divs.windows(2).all(|w| w[0] < w[1]));
        prop_assert!(divs.iter().all(|&d| n % d == 0));
        prop_assert_eq!(divs.first().copied(), Some(1));
        prop_assert_eq!(divs.last().copied(), Some(n));
        let mut visited = 0_usize;
        let flow = fordivisors(n, |d| { visited += 1; let _ = d; ControlFlow::<()>::Continue(()) });
        prop_assert_eq!(flow, ControlFlow::Continue(()));
        prop_assert_eq!(visited, divs.len());
    }
}

#[test]
fn requested_iterator_bounds_and_state() {
    assert_eq!(almost_primes_in(64, 0..=u64::MAX).next(), None);
    assert_eq!(almost_primes_in(u32::MAX, 0..=u64::MAX).next(), None);
    assert_eq!(
        almost_primes_in(63, 0..=u64::MAX).collect::<Vec<_>>(),
        [1_u64 << 63, 3_u64 << 62]
    );
    assert_eq!(almost_primes_in(0, 0..=10).count(), 0);
    let lo = 10;
    let hi = 2;
    assert_eq!(almost_primes_in(2, lo..=hi).count(), 0);
    assert_eq!(factored_in(lo..=hi).count(), 0);
    assert_eq!(squarefree_integers_in(lo..=hi).count(), 0);
    assert_eq!(squarefree_integers_in(0..=0).count(), 0);
    for (n, factors) in factored_in(0..=1000) {
        assert_eq!(factors, math_prime_util::factor_exp(n));
    }
    let mut a = factored_in(1..=300);
    assert_eq!(a.next(), Some((1, vec![])));
    let b = a.clone();
    assert_eq!(a.collect::<Vec<_>>(), b.collect::<Vec<_>>());
    let mut a = squarefree_integers_in(0..=100);
    assert_eq!(a.next(), Some(1));
    let b = a.clone();
    assert_eq!(a.collect::<Vec<_>>(), b.collect::<Vec<_>>());
    let mut empty = squarefree_integers_in(4..=4);
    assert_eq!(empty.next(), None);
    assert_eq!(empty.next(), None);
    assert_eq!(
        foralmostprimes(2, 0..=100, ControlFlow::Break),
        ControlFlow::Break(4)
    );
    assert_eq!(
        forfactored(1..=100, |n, _| ControlFlow::Break(n)),
        ControlFlow::Break(1)
    );
    assert_eq!(
        forsquarefree(1..=100, |n, _| ControlFlow::Break(n)),
        ControlFlow::Break(1)
    );
    assert_eq!(
        forsquarefreeint(1..=100, ControlFlow::Break),
        ControlFlow::Break(1)
    );
    let workers: Vec<_> = (0..4)
        .map(|_| {
            std::thread::spawn(|| {
                let mut seen = Vec::new();
                let _: ControlFlow<()> = forsquarefreeint(0..=100, |n| {
                    assert!(forfactored(n..=n, |_, _| ControlFlow::Break(())).is_break());
                    seen.push(n);
                    ControlFlow::Continue(())
                });
                seen
            })
        })
        .collect();
    for worker in workers {
        assert_eq!(
            worker.join().unwrap(),
            squarefree_in(0..=100).map(|(n, _)| n).collect::<Vec<_>>()
        );
    }
}

proptest! {
    #[test]
    fn multiset_matches_unique_index_permutations(values in prop::collection::vec(0_u8..4, 0..7)) {
        let expected: std::collections::BTreeSet<_> = permutations_of(values.len())
            .map(|indices| indices.into_iter().map(|i| values[i]).collect::<Vec<_>>())
            .collect();
        let actual: Vec<_> = multipermutations_of(&values).collect();
        prop_assert_eq!(&actual, &expected.into_iter().collect::<Vec<_>>());
        prop_assert!(actual.windows(2).all(|pair| pair[0] < pair[1]));
    }

    #[test]
    fn product_preserves_order_and_duplicates(
        left in prop::collection::vec(0_u8..3, 0..5),
        right in prop::collection::vec(0_u8..3, 0..5),
    ) {
        let expected: Vec<_> = left.iter().flat_map(|&a| right.iter().map(move |&b| vec![a, b])).collect();
        prop_assert_eq!(set_product(&[left, right]).collect::<Vec<_>>(), expected);
    }

    #[test]
    fn derangement_count_obeys_recurrence(n in 2_usize..9) {
        let expected = (n - 1) * (derangements_of(n - 1).count() + derangements_of(n - 2).count());
        prop_assert_eq!(derangements_of(n).count(), expected);
    }
}

#[test]
fn arrangement_state_is_independent() {
    macro_rules! independent {
        ($iterator:expr) => {{
            let mut first = $iterator;
            let saved = first.next().unwrap();
            let mut duplicate = first.clone();
            assert_eq!(first.next(), duplicate.next());
            let tail = first.collect::<Vec<_>>();
            assert_eq!(tail, duplicate.by_ref().collect::<Vec<_>>());
            assert_eq!(duplicate.next(), None);
            assert_eq!(duplicate.next(), None);
            assert!(!saved.is_empty());
        }};
    }
    independent!(permutations_of(4));
    independent!(derangements_of(4));
    independent!(multipermutations_of(&[1, 1, 2, 3]));
    independent!(set_product(&[vec![1, 2], vec![3, 4]]));
}

#[test]
fn arrangement_callbacks_are_reentrant_and_stop() {
    macro_rules! callback {
        ($function:ident, $argument:expr, $expected:expr) => {{
            let mut calls = 0;
            let result = $function($argument, |mut value| {
                calls += 1;
                value.clear();
                assert_eq!(
                    $function($argument, |_| ControlFlow::Break(17)),
                    ControlFlow::Break(17)
                );
                ControlFlow::Break(23)
            });
            assert_eq!(result, ControlFlow::Break(23));
            assert_eq!(calls, 1);
            assert!(
                std::panic::catch_unwind(|| {
                    let _: ControlFlow<()> = $function($argument, |_| panic!("callback"));
                })
                .is_err()
            );
            let mut count = 0;
            let result: ControlFlow<()> = $function($argument, |_| {
                count += 1;
                ControlFlow::Continue(())
            });
            assert!(result.is_continue());
            assert_eq!(count, $expected);
        }};
    }
    callback!(forperm, 4, 24);
    callback!(forderange, 4, 9);
    callback!(formultiperm, &[1, 1, 2], 3);
    callback!(forsetproduct, &[vec![1, 2], vec![3, 4]], 4);
    let workers: Vec<_> = (0..4)
        .map(|_| {
            std::thread::spawn(|| {
                (
                    permutations_of(5).count(),
                    derangements_of(5).count(),
                    multipermutations_of(&[1, 1, 2, 2]).count(),
                    set_product(&[vec![1, 2], vec![3, 4]]).count(),
                )
            })
        })
        .collect();
    for worker in workers {
        assert_eq!(worker.join().unwrap(), (120, 44, 6, 4));
    }
}

#[test]
fn requested_iterator_unwind_and_relaunch() {
    assert!(
        std::panic::catch_unwind(|| {
            let _: ControlFlow<()> = forfactored(0..=300, |_, _| panic!("callback"));
        })
        .is_err()
    );
    assert_eq!(factored_in(0..=300).count(), 300);
    assert!(
        std::panic::catch_unwind(|| {
            let _: ControlFlow<()> = forsquarefreeint(1..=100, |_| panic!("callback"));
        })
        .is_err()
    );
    assert_eq!(squarefree_integers_in(1..=100).count(), 61);
    let mut end = factored_in(u64::MAX..=u64::MAX);
    assert_eq!(end.next().unwrap().0, u64::MAX);
    assert_eq!(end.next(), None);
    assert_eq!(end.next(), None);
    let lo = 100_000_000_000_000;
    for (n, factors) in factored_in(lo..=lo + 100) {
        assert_eq!(factors, math_prime_util::factor_exp(n));
    }
}

proptest! {
    #[test]
    fn squarefree_integer_sieve_matches_factorization(lo in 0_u64..500, len in 0_u64..100) {
        let expected: Vec<_> = (lo..=lo + len).filter(|&n| {
            n > 0 && math_prime_util::factor_exp(n).iter().all(|&(_, e)| e == 1)
        }).collect();
        prop_assert_eq!(squarefree_integers_in(lo..=lo + len).collect::<Vec<_>>(), expected);
    }
}

#[test]
fn arrangement_empty_and_singleton_boundaries() {
    assert_eq!(
        permutations_of(0).collect::<Vec<_>>(),
        [Vec::<usize>::new()]
    );
    assert_eq!(permutations_of(1).collect::<Vec<_>>(), [vec![0]]);
    assert_eq!(
        derangements_of(0).collect::<Vec<_>>(),
        [Vec::<usize>::new()]
    );
    assert_eq!(derangements_of(1).next(), None);
    assert_eq!(derangements_of(2).collect::<Vec<_>>(), [vec![1, 0]]);
    let mut empty = multipermutations_of::<u8>(&[]);
    assert_eq!(empty.next(), Some(vec![]));
    assert_eq!(empty.next(), None);
    assert_eq!(empty.next(), None);
    assert_eq!(multipermutations_of(&[7]).collect::<Vec<_>>(), [vec![7]]);
    assert_eq!(set_product::<u8>(&[]).next(), None);
    for sets in [vec![vec![]], vec![vec![], vec![1]], vec![vec![1], vec![]]] {
        let mut calls = 0;
        let result: ControlFlow<()> = forsetproduct(&sets, |_| {
            calls += 1;
            ControlFlow::Continue(())
        });
        assert!(result.is_continue());
        assert_eq!(calls, 0);
    }
    assert_eq!(
        set_product(&[vec![2, 2]]).collect::<Vec<_>>(),
        [vec![2], vec![2]]
    );
}

proptest! {
    #[test]
    fn range_loop_laws(lo in 0_u64..500, width in 0_u64..200) {
        let hi = lo + width;
        let composites = composites_in(lo..=hi).collect::<Vec<_>>();
        prop_assert_eq!(odd_composites_in(lo..=hi).collect::<Vec<_>>(),
            composites.iter().copied().filter(|n| n % 2 == 1).collect::<Vec<_>>());
        prop_assert_eq!(semiprimes_in(lo..=hi).collect::<Vec<_>>(),
            composites.into_iter().filter(|&n| math_prime_util::is_semiprime(n)).collect::<Vec<_>>());
    }
}

#[test]
fn range_loops_stop_and_keep_local_state() {
    type Loop = fn(std::ops::RangeInclusive<u64>, fn(u64) -> ControlFlow<u64>) -> ControlFlow<u64>;
    let loops: [Loop; 4] = [forprimes, forcomposites, foroddcomposites, forsemiprimes];
    for (run, first) in loops.into_iter().zip([2, 4, 9, 4]) {
        assert_eq!(run(0..=30, ControlFlow::Break), ControlFlow::Break(first));
        assert_eq!(
            run(std::ops::RangeInclusive::new(30, 0), |_| panic!(
                "empty callback"
            )),
            ControlFlow::Continue(())
        );
        assert!(std::panic::catch_unwind(|| run(0..=30, |_| panic!("callback"))).is_err());
        assert_eq!(run(0..=30, ControlFlow::Break), ControlFlow::Break(first));
    }
    let mut outer = Vec::new();
    let result: ControlFlow<()> = forsemiprimes(0..=10, |n| {
        outer.push(n);
        assert_eq!(
            forsemiprimes(0..=10, ControlFlow::Break),
            ControlFlow::Break(4)
        );
        ControlFlow::Continue(())
    });
    assert!(result.is_continue());
    assert_eq!(outer, [4, 6, 9, 10]);
    let workers: Vec<_> = (0..4)
        .map(|_| {
            std::thread::spawn(|| {
                (
                    composites_in(0..=100).count(),
                    odd_composites_in(0..=100).count(),
                    semiprimes_in(0..=100).count(),
                )
            })
        })
        .collect();
    for worker in workers {
        assert_eq!(worker.join().unwrap(), (74, 25, 34));
    }
}

#[test]
fn range_iterators_end_at_native_bounds() {
    let max = u64::MAX;
    let mut composites = composites_in(max..=max);
    assert_eq!(composites.next(), Some(max));
    assert_eq!(composites.next(), None);
    assert_eq!(composites.next(), None);
    assert_eq!(odd_composites_in(max..=max).collect::<Vec<_>>(), [max]);
    assert_eq!(semiprimes_in(max..=max).next(), None);
    let last = 18_446_744_073_709_551_601;
    let mut semis = semiprimes_in(last..=max);
    assert_eq!(semis.next(), Some(last));
    assert_eq!(semis.next(), None);
    assert_eq!(semis.next(), None);
    assert_eq!(semiprimes_in(0..=3).next(), None);
    assert_eq!(composites_in(0..=3).next(), None);
    assert_eq!(odd_composites_in(0..=8).next(), None);
    assert_eq!(primes_in(max..=max).next(), None);
}

#[test]
fn semiprime_exhausted_input_stays_empty() {
    let mut range = 4..=4;
    assert_eq!(range.next(), Some(4));
    assert_eq!(semiprimes_in(range).next(), None);
    assert_eq!(
        semiprimes_in(std::ops::RangeInclusive::new(10, 4)).next(),
        None
    );
    let mut iter = semiprimes_in(0..=10);
    assert_eq!(iter.next(), Some(4));
    assert_eq!(iter.clone().collect::<Vec<_>>(), [6, 9, 10]);
    assert_eq!(iter.collect::<Vec<_>>(), [6, 9, 10]);
}

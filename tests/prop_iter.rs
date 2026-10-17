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

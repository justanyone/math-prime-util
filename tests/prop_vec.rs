//! Algebraic laws, native boundaries, and independent call state.
use math_prime_util::*;
use proptest::prelude::*;
proptest! {
    #[test]
    fn set_laws(a in proptest::collection::vec(-100_i64..100,0..80),
                b in proptest::collection::vec(-100_i64..100,0..80)) {
        prop_assert_eq!(setunion(&a,&b),setunion(&b,&a));
        prop_assert_eq!(setintersect(&a,&b),setintersect(&b,&a));
        prop_assert_eq!(setdelta(&a,&b),setdelta(&b,&a));
        prop_assert_eq!(setunion(&a,&a),toset(&a));
        prop_assert!(setminus(&a,&a).is_empty());
        prop_assert_eq!(setdelta(&setdelta(&a,&b),&b),toset(&a));
        prop_assert!(set_is_subset(&setunion(&a,&b),&a));
        prop_assert_eq!(sumset(&a,&b),sumset(&b,&a));
    }
    #[test]
    fn sort_matches_standard(a in proptest::collection::vec(any::<i64>(),0..1200)) {
        let mut expected=a.clone(); expected.sort_unstable();
        prop_assert_eq!(vecsort(&a),expected);
    }
}
#[test]
fn empty_and_overflow() {
    assert_eq!(vecsum(&[]), 0);
    assert_eq!(vecsum(&[u64::MAX, 1]), 1_u128 << 64);
    assert_eq!(
        vecprefixsum(&[u64::MAX, 1]),
        [u128::from(u64::MAX), 1_u128 << 64]
    );
    assert_eq!(vecprod(&[]), 1);
    assert_eq!(vecprod(&[u64::MAX, u64::MAX, u64::MAX, 0]), 0);
    assert!(std::panic::catch_unwind(|| vecprod(&[u64::MAX; 3])).is_err());
    assert!(std::panic::catch_unwind(|| sumset(&[i64::MAX], &[1])).is_err());
    assert!(!is_sumfree_set(&[i64::MIN, 0]));
    assert!(is_sumfree_set(&[i64::MAX]));
    assert!(!is_sidon_set(&[-1]));
    assert_eq!(vecreduce::<i32>(&[], |a, b| a + b), None);
    assert!(vecall::<i32>(&[], |_| false));
    assert!(vecnone::<i32>(&[], |_| true));
    assert_eq!(vecfirstidx(&[1, 2], |&x| x == 3), None);
}
#[test]
fn windows_and_extraction() {
    assert!(std::panic::catch_unwind(|| vecwindow(&[1], 0, 1, |_| [1])).is_err());
    assert!(std::panic::catch_unwind(|| vecwindow(&[1], 1, 0, |_| [1])).is_err());
    assert!(vecwindow(&[1], 1, 2, |_| [1]).is_empty());
    assert_eq!(vecwindow(&[1, 2, 3, 4], 2, 2, |w| w.to_vec()), [1, 2, 3, 4]);
    assert_eq!(
        vecextract(&[1, 2], VecSelection::Indices(&[-1, -3, 2, 0])),
        [Some(2), None, None, Some(1)]
    );
    assert_eq!(vecextract(&[1], VecSelection::Mask(3)), [Some(1), None]);
    assert_eq!(vecpairwise(&[1, 2], &[3], |a, b| [a + b, a * b]), [4, 3]);
}
#[test]
fn duplicate_calls_and_threads() {
    let check = || {
        let mut a = vec![1, 2];
        assert_eq!(setinsert(&mut a, &[2, 3, 3]), 1);
        assert_eq!(setinsert(&mut a, &[2, 3, 3]), 0);
        assert_eq!(setremove(&mut a, &[3, 3]), 1);
        assert_eq!(setremove(&mut a, &[3, 3]), 0);
        assert_eq!(setinvert(&mut a, &[2, 3, 3]), 0);
        assert_eq!(a, [1, 3]);
        assert_eq!(vecuniq(&[2, 1, 2]), [2, 1]);
        assert_eq!(vecsingleton(&[2, 1, 2]), [1]);
    };
    check();
    let threads: Vec<_> = (0..4).map(|_| std::thread::spawn(check)).collect();
    for t in threads {
        t.join().unwrap();
    }
}
#[test]
fn short_circuit_and_reentry() {
    let mut calls = 0;
    assert!(vecany(&[1, 2, 3], |&x| {
        calls += 1;
        x == 1
    }));
    assert_eq!(calls, 1);
    assert_eq!(
        setbinop(&[1, 2], &[3], |a, b| {
            assert_eq!(vecmex(&[0]), 1);
            a + b
        }),
        [4, 5]
    );
}
#[test]
fn radix_narrow_signed() {
    for scale in [1, 128, 32768, 1_i64 << 40] {
        let a: Vec<_> = (0..900).map(|i| ((i * 73) % 255 - 127) * scale).collect();
        let mut expected = a.clone();
        expected.sort_unstable();
        assert_eq!(vecsort(&a), expected);
    }
}

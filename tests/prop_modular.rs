//! Algebraic laws and invalid-modulus regressions for modular roots.
use math_prime_util::*;
use proptest::prelude::*;
proptest! {
    #[test]
    fn complete_roots(a in any::<u64>(), n in 1u64..200, k in 0u64..12) {
        let expected: Vec<_> = (0..n).filter(|&r| powmod(r,k,n) == Some(a % n)).collect();
        prop_assert_eq!(allrootmod(a,k,n), expected);
        let square: Vec<_> = (0..n).filter(|&r| mulmod(r,r,n) == Some(a % n)).collect();
        prop_assert_eq!(allsqrtmod(a,n), square.clone());
        prop_assert_eq!(sqrtmod(a,n).is_some(), !square.is_empty());
        if let Some(r) = rootmod(a,k,n) { prop_assert_eq!(powmod(r,k,n), Some(a % n)); }
    }
    #[test]
    fn division_law(a in any::<u64>(), b in any::<u64>(), n in 1u64..) {
        if let Some(q) = divmod(a,b,n) { prop_assert_eq!(mulmod(q,b,n), Some(a%n)); }
        else { prop_assert!(gcd(b,n)>1); }
    }
    #[test]
    fn cornacchia_law(d in 0u64..100, n in 0u64..10000) {
        if let Some((x,y)) = cornacchia(d,n) { prop_assert_eq!(x*x+d*y*y,n); }
    }
}
#[test]
fn zero_modulus_and_repeated_calls() {
    assert_eq!(sqrtmod(1, 0), None);
    assert_eq!(rootmod(1, 3, 0), None);
    assert!(allsqrtmod(1, 0).is_empty());
    assert!(allrootmod(1, 3, 0).is_empty());
    assert_eq!(divmod(1, 1, 0), None);
    assert_eq!(cornacchia(0, 0), Some((0, 0)));
    assert_eq!(cornacchia(0, 4), Some((2, 0)));
    assert_eq!(cornacchia(0, 3), None);
    for _ in 0..2 {
        assert_eq!(allrootmod(1, 3, 7), vec![1, 2, 4]);
    }
}
#[test]
fn concurrent_calls() {
    let threads: Vec<_> = (0..4)
        .map(|_| {
            std::thread::spawn(|| {
                assert_eq!(allsqrtmod(1, 72), vec![1, 17, 19, 35, 37, 53, 55, 71]);
            })
        })
        .collect();
    for thread in threads {
        thread.join().unwrap();
    }
}
#[test]
#[should_panic(expected = "Maximum returned roots exceeded")]
fn root_count_limit() {
    allrootmod(1, 0, 600_000_001);
}

#[test]
fn native_word_edges() {
    let prime = u64::MAX - 58;
    let a = u64::MAX - 123;
    let square = mulmod(a, a, prime).unwrap();
    let r = sqrtmod(square, prime).unwrap();
    assert_eq!(mulmod(r, r, prime), Some(square));
    assert_eq!(allsqrtmod(square, prime), vec![r, prime - r]);
    for n in [1 << 63, 1 << 62, 1 << 40] {
        assert_eq!(sqrtmod(17, n).map(|r| mulmod(r, r, n)), Some(Some(17)));
        for root in allsqrtmod(17, n) {
            assert_eq!(mulmod(root, root, n), Some(17));
        }
    }
    for n in 1..150 {
        for k in 0..10 {
            for a in 0..n {
                let roots = allrootmod(a, k, n);
                assert_eq!(
                    rootmod(a, k, n).is_some(),
                    !roots.is_empty(),
                    "a={a}, k={k}, n={n}"
                );
            }
        }
    }
}

#[test]
#[should_panic(expected = "Maximum returned roots exceeded")]
fn square_root_count_limit() {
    allsqrtmod(0, 1 << 62);
}

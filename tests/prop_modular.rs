//! Property tests: algebraic laws for `cornacchia` and `divmod`.

use math_prime_util::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn cornacchia_solution_satisfies_equation(d in 1u64..1000, p in 0u64..100_000) {
        if let Some((x, y)) = cornacchia(d, p) {
            let lhs = u128::from(x) * u128::from(x) + u128::from(d) * u128::from(y) * u128::from(y);
            prop_assert_eq!(lhs, u128::from(p));
        }
    }

    #[test]
    fn divmod_undoes_mulmod(a in 0u64..1_000_000, b in 1u64..1_000_000, n in 2u64..1_000_000) {
        if let Some(q) = divmod(a, b, n) {
            if gcd(b, n) == 1 {
                prop_assert_eq!(mulmod(q, b, n), Some(a % n));
            }
        }
    }

    #[test]
    fn divmod_by_one_is_reduction(a in any::<u64>(), n in 1u64..u64::MAX) {
        prop_assert_eq!(divmod(a, 1, n), Some(a % n));
    }
}

mod roots {
    //! Algebraic, exhaustive, and concurrency tests for modular roots.
    use math_prime_util::{allrootmod, allsqrtmod, powmod, rootmod, sqrtmod};
    use proptest::prelude::*;

    #[test]
    fn exhaustive_roots() {
        for n in 1..80 {
            for a in 0..n {
                for k in 0..9 {
                    let expected: Vec<_> = (0..n).filter(|&r| powmod(r, k, n) == Some(a)).collect();
                    assert_eq!(allrootmod(a, k, n), expected, "a={a}, k={k}, n={n}");
                    let one = rootmod(a, k, n);
                    assert_eq!(one.is_some(), !expected.is_empty(), "a={a}, k={k}, n={n}");
                    if let Some(r) = one {
                        assert!(expected.contains(&r));
                    }
                    if k == 2 {
                        assert_eq!(allsqrtmod(a, n), expected);
                        assert_eq!(sqrtmod(a, n).is_some(), !expected.is_empty());
                    }
                }
            }
        }
    }
    #[test]
    fn invalid_modulus_and_limits() {
        assert_eq!(sqrtmod(1, 0), None);
        assert_eq!(rootmod(1, 2, 0), None);
        assert_eq!(allsqrtmod(1, 0), Vec::<u64>::new());
        assert_eq!(allrootmod(1, 2, 0), Vec::<u64>::new());
        assert!(std::panic::catch_unwind(|| allrootmod(1, 0, 600_000_001)).is_err());
        assert!(std::panic::catch_unwind(|| allsqrtmod(0, 1_u64 << 60)).is_err());
    }
    #[test]
    fn concurrent_repeated_calls() {
        let threads: Vec<_> = (0..4)
            .map(|_| {
                std::thread::spawn(|| {
                    for _ in 0..10 {
                        assert_eq!(allrootmod(249, 50, 625).len(), 50);
                    }
                })
            })
            .collect();
        for thread in threads {
            thread.join().unwrap();
        }
    }
    proptest! {
        #[test]
        fn square_roots_square_back(a in any::<u64>(), n in 1_u64..100_000) {
            let square = math_prime_util::mulmod(a,a,n).unwrap();
            let r = sqrtmod(square,n).unwrap();
            prop_assert_eq!(math_prime_util::mulmod(r,r,n),Some(square));
        }
        #[test]
        fn roots_raise_back(a in any::<u64>(), k in 1_u64..20, n in 1_u64..1000) {
            let residue = powmod(a,k,n).unwrap();
            let roots = allrootmod(residue,k,n);
            prop_assert!(roots.contains(&(a % n)));
            prop_assert!(roots.windows(2).all(|w| w[0] < w[1]));
        }
    }

    #[test]
    fn word_boundaries() {
        for n in [1_u64 << 63, u64::MAX, 18_446_744_073_709_551_557] {
            for a in [1, 4, 9] {
                let roots = allsqrtmod(a, n);
                assert_ne!(roots, Vec::<u64>::new());
                for r in roots {
                    assert_eq!(math_prime_util::mulmod(r, r, n), Some(a));
                }
                assert_eq!(powmod(sqrtmod(a, n).unwrap(), 2, n), Some(a));
            }
        }
        let n = 1_u64 << 63;
        assert_eq!(allrootmod(27, 3, n), vec![3]);
        assert_eq!(rootmod(27, 3, n), Some(3));
        assert_eq!(allrootmod(2, u64::MAX, 3), vec![2]);
        assert!(std::panic::catch_unwind(|| allrootmod(0, 3, n)).is_err());
    }
}

//! Property tests: algebraic laws for the modular functions, including
//! `cornacchia` and `divmod`.

use math_prime_util::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn factorial_recurrence(n in 1u64..1500, m in 1u64..10000) {
        prop_assert_eq!(factorialmod(n, m), mulmod(factorialmod(n - 1, m).unwrap(), n, m));
    }
    #[test]
    fn binomial_pascal(n in 1u64..1000, k in 1u64..1000, m in 1u64..2000) {
        let k = k.min(n);
        prop_assert_eq!(
            binomialmod(n, k, m),
            addmod(
                binomialmod(n - 1, k - 1, m).unwrap(),
                binomialmod(n - 1, k, m).unwrap(),
                m
            )
        );
        prop_assert_eq!(binomialmod(n, k, m), binomialmod(n, n - k, m));
    }
    #[test]
    fn logarithm_roundtrip(g in 1u64..200, n in 2u64..300, k in 0u64..400) {
        let a = powmod(g, k, n).unwrap();
        let x = znlog(a, g, n).unwrap();
        prop_assert_eq!(powmod(g, x, n), Some(a));
        prop_assert!(x <= k);
        if gcd(g, n) == 1 {
            prop_assert!(x < n);
        }
    }
    #[test]
    fn pisano_returns_first_period(n in 2u64..300) {
        let period = pisano_period(n);
        let (mut a, mut b, mut k) = (0, 1, 0u128);
        loop {
            (a, b) = (b, (a + b) % n);
            k += 1;
            if a == 0 && b == 1 {
                break;
            }
        }
        prop_assert_eq!(period, k);
    }
}

#[test]
fn zero_moduli_and_repeat_calls() {
    assert_eq!(factorialmod(0, 0), None);
    assert_eq!(binomialmod(0, 0, 0), None);
    assert_eq!(znlog(0, 0, 0), None);
    assert_eq!(pisano_period(0), 0);
    for _ in 0..2 {
        assert_eq!(znlog(0, 2, 8), Some(3));
        assert_eq!(znlog(12, 98, 100), Some(19));
        assert_eq!(binomialmod(500_000, 200_000, 1024), Some(640));
    }
}

#[test]
fn concurrent_calls() {
    let workers: Vec<_> = (0..4)
        .map(|_| {
            std::thread::spawn(|| {
                assert_eq!(znlog(5, 2, 1019), Some(10));
                assert_eq!(factorialmod(1000, 1009), Some(782));
                assert_eq!(binomialmod(10, 3, 13), Some(3));
                assert_eq!(pisano_period(1000), 1500);
            })
        })
        .collect();
    for worker in workers {
        worker.join().unwrap();
    }
}

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

/// Without the prime Kronecker rejection, `2^61 - 1` walks about `2^30.5`
/// candidates. The assertion times out on that scan.
#[test]
fn cornacchia_rejects_large_prime_without_scanning() {
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let _ = tx.send(cornacchia(1, (1_u64 << 61) - 1));
    });
    assert_eq!(rx.recv_timeout(std::time::Duration::from_secs(2)), Ok(None));
}

/// `p = 134217757 * 134217869`. The plain scan returns
/// `(132396848, 22033973)`; MPU `corn_all` returns the other pair.
#[test]
fn cornacchia_large_composite_matches_root_path() {
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let _ = tx.send(cornacchia(1, 18_014_421_326_499_833));
    });
    assert_eq!(
        rx.recv_timeout(std::time::Duration::from_secs(2)),
        Ok(Some((114_145_427, 70_606_252)))
    );
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

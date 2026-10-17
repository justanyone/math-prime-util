//! Algebraic laws for bounded factor methods. Factors need not be prime.
use math_prime_util::{fermat_factor, holf_factor, lehman_factor, squfof_factor};
use proptest::prelude::*;

type Method = fn(u64, Option<u64>) -> Vec<u64>;
const METHODS: [Method; 3] = [fermat_factor, holf_factor, squfof_factor];

fn validate(n: u64, factors: &[u64]) {
    assert_eq!(
        factors.iter().map(|&f| u128::from(f)).product::<u128>(),
        u128::from(n)
    );
    if n > 1 {
        assert!(factors.iter().all(|&f| f > 1 && n % f == 0));
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]
    #[test]
    fn product_is_input(n in any::<u64>(), rounds in 0u64..100) {
        for method in METHODS { validate(n, &method(n, Some(rounds))); }
        validate(n, &lehman_factor(n, Some(false)));
        validate(n, &lehman_factor(n, Some(true)));
    }
    #[test]
    fn bounded_small_products(a in 1u64..1000, b in 1u64..1000) {
        let n = a*b;
        for method in METHODS { validate(n, &method(n, Some(1000))); }
        validate(n, &lehman_factor(n, None));
    }
}

#[test]
fn edge_inputs_and_zero_budget() {
    for method in METHODS {
        assert_eq!(method(0, Some(0)), vec![0]);
        assert_eq!(method(1, Some(0)), Vec::<u64>::new());
        assert_eq!(method(2 * 3 * 5 * 7, Some(0)), vec![2, 3, 5, 7]);
        assert_eq!(method(77, Some(0)), vec![77]);
        assert_eq!(method(49, None), vec![7, 7]);
        validate(u64::MAX, &method(u64::MAX, Some(1)));
    }
    assert_eq!(lehman_factor(0, None), vec![0]);
    assert_eq!(lehman_factor(1, None), Vec::<u64>::new());
    assert_eq!(lehman_factor(77, Some(true)), vec![7, 11]);
    assert_eq!(lehman_factor(89_u64.pow(3), None), vec![89, 89, 89]);
    assert_eq!(fermat_factor(49, Some(0)), vec![7, 7]);
    assert_eq!(holf_factor(49, Some(0)), vec![7, 7]);
    assert_eq!(squfof_factor(49, Some(0)), vec![49]);
    assert_eq!(
        squfof_factor(9_223_371_873_002_223_329, None),
        vec![9_223_371_873_002_223_329]
    );
}

#[test]
fn concurrent_repeated_calls_have_no_shared_state() {
    std::thread::scope(|scope| {
        for method in METHODS {
            scope.spawn(move || {
                for _ in 0..4 {
                    assert_eq!(method(8051, None), vec![83, 97]);
                }
            });
        }
        scope.spawn(|| {
            for _ in 0..4 {
                assert_eq!(lehman_factor(8051, None), vec![83, 97]);
            }
        });
    });
}

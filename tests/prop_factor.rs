//! Algebraic laws for factoring.
use math_prime_util::{factor_u128, is_prime_u128};
use proptest::prelude::*;
proptest! {
    #[test]
    fn reconstruct(n in 1u64..1_000_000_000) {
        let factors = factor_u128(u128::from(n)).unwrap();
        prop_assert_eq!(factors.iter().product::<u128>(), u128::from(n));
        prop_assert!(factors.windows(2).all(|pair| pair[0] <= pair[1]));
        prop_assert!(factors.iter().all(|&p| is_prime_u128(p) != 0));
    }
}
#[test]
fn repeat_and_parallel_calls() {
    let n = 2017 * 18_446_744_073_709_551_629u128;
    let expected = vec![2017, 18_446_744_073_709_551_629];
    let handles: Vec<_> = (0..4)
        .map(|_| std::thread::spawn(move || factor_u128(n)))
        .collect();
    for handle in handles {
        assert_eq!(handle.join().unwrap(), Ok(expected.clone()));
    }
    assert_eq!(factor_u128(0), Ok(vec![0]));
    assert_eq!(factor_u128(1), Ok(vec![]));
}

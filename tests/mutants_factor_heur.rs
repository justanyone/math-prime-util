//! Public-API mutation tests for the heuristic factoring methods.
use math_prime_util::{lehman_factor, pminus1_factor, pplus1_factor, prho_factor};

#[test]
fn small_prime_cofactors_are_stripped_by_every_method() {
    // 30 = 2*3*5 leaves the cofactor 1, which no method may be asked to split.
    assert_eq!(prho_factor(30, None), vec![2, 3, 5]);
    assert_eq!(pminus1_factor(30, None, None), vec![2, 3, 5]);
    assert_eq!(pplus1_factor(30, None), vec![2, 3, 5]);
    assert_eq!(lehman_factor(30, None), vec![2, 3, 5]);
    assert_eq!(lehman_factor(30, Some(false)), vec![2, 3, 5]);
    assert_eq!(prho_factor(1, None), Vec::<u64>::new());
    assert_eq!(pminus1_factor(1, None, None), Vec::<u64>::new());
    assert_eq!(pplus1_factor(1, None), Vec::<u64>::new());
    assert_eq!(lehman_factor(1, None), Vec::<u64>::new());
}

#[test]
fn lehman_cutoff_and_trial_flag() {
    // A semiprime above the 8,796,393,022,207 cutoff is not searched.
    let p = math_prime_util::next_prime(3_000_000).expect("small");
    let q = math_prime_util::next_prime(p + 1).expect("small");
    assert!(p * q > 8_796_393_022_207);
    assert_eq!(lehman_factor(p * q, Some(false)), vec![p * q]);
    let n = 89_u64 * 1_000_003 * 1_000_033;
    assert_eq!(lehman_factor(n, Some(true)).iter().product::<u64>(), n);
}

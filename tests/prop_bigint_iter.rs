#![cfg(feature = "bigint")]
use math_prime_util::bigint::iter::*;
use num_bigint::BigUint;

#[test]
fn small_range_results_match_native() {
    for lo in 0..=20_u64 {
        for hi in lo..=40_u64 {
            let (a, b) = (BigUint::from(lo), BigUint::from(hi));
            assert_eq!(
                forsemiprimes(&a, &b)
                    .unwrap()
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>(),
                math_prime_util::semiprimes_in(lo..=hi)
                    .map(|n| n.to_string())
                    .collect::<Vec<_>>()
            );
            assert_eq!(
                forsquarefreeint(&a, &b)
                    .unwrap()
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>(),
                math_prime_util::squarefree_in(lo..=hi)
                    .map(|n| n.to_string())
                    .collect::<Vec<_>>()
            );
        }
    }
}

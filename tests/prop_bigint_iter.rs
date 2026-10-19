#![cfg(feature = "bigint")]
use math_prime_util::bigint::iter::*;
use num_bigint::BigUint;

fn strs(v: &[BigUint]) -> Vec<String> {
    v.iter().map(ToString::to_string).collect()
}

#[test]
fn small_range_results_match_native() {
    for lo in 0..=20_u64 {
        for hi in lo..=40_u64 {
            let (a, b) = (BigUint::from(lo), BigUint::from(hi));
            let semi: Vec<String> = math_prime_util::semiprimes_in(lo..=hi)
                .map(|n| n.to_string())
                .collect();
            assert_eq!(strs(&forsemiprimes(&a, &b).unwrap()), semi);
            let sqf: Vec<String> = math_prime_util::squarefree_in(lo..=hi)
                .map(|(n, _)| n.to_string())
                .collect();
            assert_eq!(strs(&forsquarefreeint(&a, &b).unwrap()), sqf);
            assert_eq!(strs(&forsquarefree(&a, &b).unwrap()), sqf);
            for k in 0..=4_u32 {
                let alm: Vec<String> = math_prime_util::almost_primes_in(k, lo..=hi)
                    .map(|n| n.to_string())
                    .collect();
                assert_eq!(
                    strs(&foralmostprimes(u64::from(k), &a, &b).unwrap()),
                    alm,
                    "k={k} {lo}..{hi}"
                );
            }
            let fact = forfactored(&a, &b).unwrap();
            let native: Vec<usize> = math_prime_util::factored_in(lo..=hi)
                .map(|(_, f)| f.iter().map(|&(_, e)| e as usize).sum())
                .collect();
            assert_eq!(fact.iter().map(Vec::len).collect::<Vec<_>>(), native);
        }
    }
}

#[test]
fn divisors_match_native() {
    for n in 0..=200_u64 {
        let got = fordivisors(&BigUint::from(n)).unwrap();
        let want: Vec<String> = math_prime_util::divisors(n, None)
            .iter()
            .map(ToString::to_string)
            .collect();
        assert_eq!(strs(&got), want, "n={n}");
    }
}

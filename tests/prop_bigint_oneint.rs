//! The `BigUint` forms of `inverse_sigma0`, `inverse_sigma0_count`,
//! `farey_rank` and `is_congruent_number` agree with the native forms.
#![cfg(feature = "bigint")]
use math_prime_util::{
    self as native,
    bigint::{functions, predicates, rational},
};
use num_bigint::BigUint;
use proptest::prelude::*;

fn big(n: u64) -> BigUint {
    BigUint::from(n)
}

fn sigma0_agrees(k: u64, lo: u64, hi: u64) -> Result<(), TestCaseError> {
    let want: Vec<BigUint> = native::inverse_sigma0(k, lo, hi)
        .unwrap_or_default()
        .into_iter()
        .map(BigUint::from)
        .collect();
    let (bk, blo, bhi) = (big(k), big(lo), big(hi));
    prop_assert_eq!(&functions::inverse_sigma0(&bk, &blo, &bhi).unwrap(), &want);
    let count = native::inverse_sigma0_count(k, lo, hi).unwrap();
    prop_assert_eq!(
        functions::inverse_sigma0_count(&bk, &blo, &bhi).unwrap(),
        big(count)
    );
    prop_assert_eq!(count, want.len() as u64);
    Ok(())
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(48))]
    #[test]
    fn inverse_sigma0_narrow(k in 0_u64..40, lo in 0_u64..1_000_000, width in 0_u64..400) {
        sigma0_agrees(k, lo, lo + width)?;
    }

    #[test]
    fn inverse_sigma0_wide(k in 0_u64..64, lo in 0_u64..2_000, hi in 1_u64..200_000) {
        sigma0_agrees(k, lo, hi)?;
    }

    #[test]
    fn farey_rank_agrees(n in 1_u32..3_000, p in 0_u32..5_000, q in 1_u32..5_000) {
        let got = rational::farey_rank(u64::from(n), &p.into(), &q.into());
        prop_assert_eq!(got, big(native::farey_rank(n, p, q)));
    }

    #[test]
    fn is_congruent_number_agrees(n in 0_u64..100_000, core in 1_u64..2_000, s in 1_u64..1_000_000) {
        for n in [n, core * s * s] {
            prop_assert_eq!(predicates::is_congruent_number(&big(n)).unwrap(), native::is_congruent_number(n));
        }
    }
}

#[test]
fn inverse_sigma0_edges() {
    for k in 0..12 {
        for (lo, hi) in [(0, 0), (0, 1), (1, 1), (5, 4), (0, 64), (60, 64), (1, 200)] {
            sigma0_agrees(k, lo, hi).unwrap();
        }
    }
    // k past 2^64: no integer this small has that many divisors. Such a k
    // takes the factored scan, so the window is short.
    let k = big(1) << 64_usize;
    let r = functions::inverse_sigma0(&k, &k, &(&k + 10_u8)).unwrap();
    assert_eq!(r, Vec::<BigUint>::new());
    let r = functions::inverse_sigma0(&(&k * 2_u8), &big(1), &k).unwrap();
    assert_eq!(r, Vec::<BigUint>::new());
}

#[test]
fn farey_rank_mertens_path() {
    // Orders past 10^6 use Mertens values, as MPU does.
    for n in [1_000_001_u32, 1_500_007] {
        for (p, q) in [
            (1_u32, 2_u32),
            (1, 3),
            (2, 7),
            (999, 1000),
            (5, 5),
            (7, 3),
            (0, 1),
        ] {
            let got = rational::farey_rank(u64::from(n), &p.into(), &q.into());
            assert_eq!(got, big(native::farey_rank(n, p, q)), "{n} {p}/{q}");
        }
    }
    assert_eq!(rational::farey_rank(0, &big(1), &big(2)), big(0));
    assert_eq!(rational::farey_rank(5, &big(1), &big(0)), big(0));
}

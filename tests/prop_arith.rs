//! Algebraic laws for `inverse_sigma0` and `inverse_sigma0_count`.

use math_prime_util::*;

#[test]
fn count_equals_list_length() {
    for k in 1..=12 {
        let list = inverse_sigma0(k, 1, 3000).unwrap();
        assert_eq!(inverse_sigma0_count(k, 1, 3000), Some(list.len() as u64));
    }
}

#[test]
fn list_is_sorted_and_in_range() {
    for k in 1..=12 {
        let list = inverse_sigma0(k, 500, 3000).unwrap();
        assert!(list.windows(2).all(|w| w[0] < w[1]));
        assert!(list.iter().all(|&n| (500..=3000).contains(&n)));
    }
}

#[test]
fn members_have_k_divisors() {
    for k in 1..=12 {
        for n in inverse_sigma0(k, 1, 2000).unwrap() {
            assert_eq!(divisor_sum(n, 0), Some(k));
        }
    }
}

#[test]
fn counts_add_over_split_ranges() {
    for k in 1..=8 {
        let whole = inverse_sigma0_count(k, 1, 4000).unwrap();
        let left = inverse_sigma0_count(k, 1, 1999).unwrap();
        let right = inverse_sigma0_count(k, 2000, 4000).unwrap();
        assert_eq!(whole, left + right);
    }
}

#[test]
fn zero_k_is_undef() {
    assert_eq!(inverse_sigma0(0, 1, 100), None);
    assert_eq!(inverse_sigma0_count(0, 1, 100), None);
}

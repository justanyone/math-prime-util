//! `BigUint` nth values and powerful number lists against MPU 0.75 past 2^64.
#![cfg(feature = "bigint")]
mod common;
use math_prime_util::bigint::family_nth::{
    nth_almost_prime, nth_omega_prime, nth_powerful, powerful_numbers,
};
use num_bigint::BigUint;

fn list(v: &[BigUint]) -> Option<String> {
    let text = v
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" ");
    (!text.is_empty()).then_some(text)
}

#[test]
fn bigint_family_nth_oracle() {
    let rows = common::load("bigint_family_nth");
    assert!(rows.len() >= 70, "fixture rows: {}", rows.len());
    for c in rows {
        let got = match c.func.as_str() {
            "nth_almost_prime" => nth_almost_prime(c.arg(0), &c.arg(1)),
            "nth_omega_prime" => nth_omega_prime(c.arg(0), &c.arg(1)),
            "nth_powerful" => nth_powerful(&c.arg(0), c.arg(1)),
            "powerful_numbers" => {
                c.check(list(&powerful_numbers(&c.arg(0), &c.arg(1), c.arg(2))));
                continue;
            }
            _ => panic!("unknown row {}", c.func),
        };
        c.check(got);
    }
}

/// `nth_semiprime` and `nth_prime_power` have no `BigUint` form: past the
/// native range MPU's Perl path needs prime counts near 2^64 and does not
/// return. The native forms return MPU C's boundary values there, and the
/// `BigUint` forms that would reach them return `None`.
#[test]
fn semiprime_and_prime_power_stop_at_the_native_range() {
    const MAX_SEMI_PRIME_IDX: u64 = 1_701_748_900_850_019_777;
    const MAX_PRIME_IDX: u64 = 425_656_284_035_217_743;
    const MAX_PRIME: u64 = 18_446_744_073_709_551_557;
    assert_eq!(
        math_prime_util::nth_semiprime(MAX_SEMI_PRIME_IDX),
        Some(18_446_744_073_709_551_601)
    );
    assert_eq!(
        math_prime_util::nth_semiprime(MAX_SEMI_PRIME_IDX + 1),
        Some(0)
    );
    assert_eq!(math_prime_util::nth_semiprime(u64::MAX), Some(0));
    assert_eq!(
        math_prime_util::nth_prime_power(MAX_PRIME_IDX + 1),
        Some(MAX_PRIME)
    );
    assert_eq!(math_prime_util::nth_prime_power(u64::MAX), Some(MAX_PRIME));
    let past_semi = BigUint::from(MAX_SEMI_PRIME_IDX) + 1_u8;
    assert_eq!(nth_almost_prime(2, &past_semi), None);
    assert_eq!(
        nth_almost_prime(2, &(BigUint::from(1_u8) << 64_usize)),
        None
    );
    let past_power = BigUint::from(MAX_PRIME_IDX) + 1_u8;
    assert_eq!(nth_omega_prime(1, &past_power), None);
    // Inside the native range the BigUint forms hand over to them.
    assert_eq!(
        nth_almost_prime(2, &BigUint::from(1234_u16)),
        math_prime_util::nth_semiprime(1234).map(BigUint::from)
    );
    assert_eq!(
        nth_omega_prime(1, &BigUint::from(10_u8)),
        math_prime_util::nth_prime_power(10).map(BigUint::from)
    );
}

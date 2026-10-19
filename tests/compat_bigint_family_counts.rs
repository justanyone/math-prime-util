//! `BigUint` family counts against MPU 0.75 (pure Perl) on inputs above 2^64.
#![cfg(feature = "bigint")]
use math_prime_util::bigint::family_counts as f;
use num_bigint::BigUint;

fn big(s: &str) -> BigUint {
    s.parse().expect("integer")
}

/// Replay one fixture row; the result is the decimal text of the Rust value.
fn replay(name: &str, a: &[&str]) -> String {
    match (name, a.len()) {
        ("perfect_power_count", 1) => f::perfect_power_count(&big(a[0])).to_string(),
        ("powerfree_count", 2) => f::powerfree_count(&big(a[0]), small(a[1]))
            .expect("factors")
            .to_string(),
        ("powerfree_sum", 2) => f::powerfree_sum(&big(a[0]), small(a[1]))
            .expect("factors")
            .to_string(),
        ("powerfree_part_sum", 2) => f::powerfree_part_sum(&big(a[0]), small(a[1]))
            .expect("factors")
            .to_string(),
        ("legendre_phi", 2) => f::legendre_phi(&big(a[0]), u64::from(small(a[1]))).to_string(),
        ("rough_count", 2) => f::rough_count(&big(a[0]), u64::from(small(a[1]))).to_string(),
        ("smooth_count", 2) => f::smooth_count(&big(a[0]), u64::from(small(a[1]))).to_string(),
        ("nth_perfect_power", 1) => f::nth_perfect_power(&big(a[0])).expect("rank").to_string(),
        ("nth_powerfree", 2) => f::nth_powerfree(&big(a[0]), small(a[1]))
            .expect("factors")
            .expect("rank")
            .to_string(),
        ("prime_count", 2) => f::prime_count(&big(a[0]), &big(a[1]))
            .expect("prime")
            .to_string(),
        ("sum_primes", 2) => f::sum_primes(&big(a[0]), &big(a[1]))
            .expect("prime")
            .to_string(),
        ("twin_prime_count", 2) => f::twin_prime_count(&big(a[0]), &big(a[1])).to_string(),
        ("prime_power_count", 2) => f::prime_power_count(&big(a[0]), &big(a[1])).to_string(),
        ("semiprime_count", 2) => f::semiprime_count(&big(a[0]), &big(a[1]))
            .expect("factors")
            .to_string(),
        ("sumtotient", 2) => f::sumtotient(&big(a[0]), &big(a[1]))
            .expect("factors")
            .to_string(),
        ("mertens", 2) => f::mertens(&big(a[0]), &big(a[1]))
            .expect("factors")
            .to_string(),
        ("sumliouville", 2) => f::sumliouville(&big(a[0]), &big(a[1]))
            .expect("factors")
            .to_string(),
        ("almost_prime_count", 3) => {
            f::almost_prime_count(u64::from(small(a[0])), &big(a[1]), &big(a[2]))
                .expect("factors")
                .to_string()
        }
        ("omega_prime_count", 3) => {
            f::omega_prime_count(u64::from(small(a[0])), &big(a[1]), &big(a[2]))
                .expect("factors")
                .to_string()
        }
        ("powerful_count", 3) => f::powerful_count(small(a[0]), &big(a[1]), &big(a[2]))
            .expect("factors")
            .to_string(),
        ("sumpowerful", 3) => f::sumpowerful(small(a[0]), &big(a[1]), &big(a[2]))
            .expect("factors")
            .to_string(),
        _ => panic!("unknown row {name} with {} arguments", a.len()),
    }
}

fn small(s: &str) -> u32 {
    s.parse().expect("small parameter")
}

#[test]
fn replay_bigint_family_counts() {
    let mut rows = 0;
    for line in include_str!("fixtures/bigint_family_counts.tsv").lines() {
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        let row: Vec<_> = line.split('\t').collect();
        let args: Vec<_> = row[1].split_whitespace().collect();
        assert_eq!(replay(row[0], &args), row[2], "{line}");
        rows += 1;
    }
    assert!(rows > 130, "fixture rows: {rows}");
}

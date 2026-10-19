#![cfg(feature = "bigint")]
use math_prime_util::bigint::family_counts as f;
use num_bigint::BigUint;

#[test]
fn replay_bigint_family_counts() {
    for line in include_str!("fixtures/bigint_family_counts.tsv").lines() {
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        let row: Vec<_> = line.split('\t').collect();
        let args: Vec<_> = row[1].split_whitespace().collect();
        let n: BigUint = args[0].parse().expect("input");
        let k = || args[1].parse().expect("small parameter");
        let result = match row[0] {
            "perfect_power_count" => f::perfect_power_count(&n).to_string(),
            "powerfree_count" => f::powerfree_count(&n, k()).expect("factors").to_string(),
            "powerfree_sum" => f::powerfree_sum(&n, k()).expect("factors").to_string(),
            "powerfree_part_sum" => f::powerfree_part_sum(&n, k()).expect("factors").to_string(),
            "legendre_phi" => f::legendre_phi(&n, args[1].parse().expect("a")).to_string(),
            "rough_count" => f::rough_count(&n, args[1].parse().expect("k")).to_string(),
            "smooth_count" => f::smooth_count(&n, args[1].parse().expect("k")).to_string(),
            name => {
                let high: BigUint = args.last().expect("high").parse().expect("high integer");
                match name {
                    "semiprime_count" => {
                        f::semiprime_count(&n, &high).expect("factors").to_string()
                    }
                    "prime_count" => f::prime_count(&n, &high).expect("prime").to_string(),
                    "sum_primes" => f::sum_primes(&n, &high).expect("prime").to_string(),
                    "sumtotient" => f::sumtotient(&n, &high).expect("factors").to_string(),
                    "mertens" => f::mertens(&n, &high).expect("factors").to_string(),
                    "sumliouville" => f::sumliouville(&n, &high).expect("factors").to_string(),
                    name => {
                        let low: BigUint = args[1].parse().expect("low");
                        match name {
                            "almost_prime_count" => {
                                f::almost_prime_count(args[0].parse().expect("k"), &low, &high)
                            }
                            "omega_prime_count" => {
                                f::omega_prime_count(args[0].parse().expect("k"), &low, &high)
                            }
                            "powerful_count" => {
                                f::powerful_count(args[0].parse().expect("k"), &low, &high)
                            }
                            "sumpowerful" => {
                                f::sumpowerful(args[0].parse().expect("k"), &low, &high)
                            }
                            _ => panic!("unknown row {name}"),
                        }
                        .expect("factors")
                        .to_string()
                    }
                }
            }
        };
        assert_eq!(result, row[2], "{line}");
    }
}

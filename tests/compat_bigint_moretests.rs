//! Replay the MPU 0.75 pure Perl bigint oracle.
#![cfg(feature = "bigint")]
use math_prime_util::bigint::{factor, primality};
use num_bigint::BigUint;

#[test]
fn bigint_moretests_fixtures() {
    for row in include_str!("fixtures/bigint_moretests.tsv").lines() {
        if row.starts_with('#') || row.is_empty() {
            continue;
        }
        let fields: Vec<_> = row.split('\t').collect();
        let args: Vec<_> = fields[1].split_whitespace().collect();
        let n: BigUint = args[0].parse().unwrap();
        let result = match fields[0] {
            "is_safe_prime" => u8::from(primality::is_safe_prime(&n)).to_string(),
            "is_chen_prime" => u8::from(primality::is_chen_prime(&n)).to_string(),
            "is_frobenius_pseudoprime" => {
                u8::from(primality::is_frobenius_pseudoprime(&n)).to_string()
            }
            "is_frobenius_khashin_pseudoprime" => {
                u8::from(primality::is_frobenius_khashin_pseudoprime(&n)).to_string()
            }
            "is_frobenius_underwood_pseudoprime" => {
                u8::from(primality::is_frobenius_underwood_pseudoprime(&n)).to_string()
            }
            "is_delicate_prime" => primality::is_delicate_prime(&n, args[1].parse().unwrap())
                .map_or_else(|| "undef".into(), |r| u8::from(r).to_string()),
            "is_perrin_pseudoprime" => u8::from(primality::is_perrin_pseudoprime(
                &n,
                args[1].parse().unwrap(),
            ))
            .to_string(),
            "is_gaussian_prime" => {
                primality::is_gaussian_prime(&n, &args[1].parse().unwrap()).to_string()
            }
            "fermat_factor" | "cheb_factor" => {
                let values = if fields[0] == "fermat_factor" {
                    factor::fermat_factor(&n, Some(args[1].parse().unwrap()))
                } else {
                    factor::cheb_factor(&n, args[1].parse().unwrap(), args[2].parse().unwrap())
                };
                values
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" ")
            }
            name => panic!("unknown fixture function {name}"),
        };
        assert_eq!(result, fields[2], "{row}");
    }
}

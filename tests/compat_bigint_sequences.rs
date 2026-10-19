//! Replay every row from the MPU 0.75 pure-Perl sequence oracle.
#![cfg(feature = "bigint")]
mod common;
use math_prime_util::bigint::sequences as s;
use num_bigint::BigUint;

#[test]
fn bigint_sequences_oracle() {
    for c in common::load("bigint_sequences") {
        let values: Vec<BigUint> = if c.func.starts_with("lucasu") || c.func == "lucasv" {
            Vec::new()
        } else {
            c.args.iter().map(|v| v.parse().unwrap()).collect()
        };
        let run = || -> Option<String> {
            let result = match c.func.as_str() {
                "fibonacci" => s::fibonacci(&values[0]).to_string(),
                "lucas_number" => s::lucas_number(&values[0]).to_string(),
                "lucasu" => s::lucasu(&c.arg(0), &c.arg(1), &c.arg(2)).to_string(),
                "lucasv" => s::lucasv(&c.arg(0), &c.arg(1), &c.arg(2)).to_string(),
                "lucasuv" => {
                    let (u, v) = s::lucasuv(&c.arg(0), &c.arg(1), &c.arg(2));
                    format!("{u} {v}")
                }
                "primorial" => s::primorial(&values[0]).to_string(),
                "pn_primorial" => s::pn_primorial(&values[0]).to_string(),
                "consecutive_integer_lcm" => s::consecutive_integer_lcm(&values[0]).to_string(),
                "powersum" => s::powersum(&values[0], &values[1]).to_string(),
                "vecsum" => s::vecsum(&values).to_string(),
                "vecprod" => s::vecprod(&values).to_string(),
                "vecprefixsum" => {
                    let sums = s::vecprefixsum(&values);
                    return (!sums.is_empty()).then(|| {
                        sums.iter()
                            .map(ToString::to_string)
                            .collect::<Vec<_>>()
                            .join(" ")
                    });
                }
                "floor_sum" => {
                    s::floor_sum(&values[0], &values[1], &values[2], &values[3]).to_string()
                }
                "frobenius_number" => return s::frobenius_number(&values).map(|v| v.to_string()),
                name => panic!("unknown fixture function {name}"),
            };
            Some(result)
        };
        if c.error {
            assert!(std::panic::catch_unwind(run).is_err(), "{c:?}");
        } else {
            c.check(run());
        }
    }
}

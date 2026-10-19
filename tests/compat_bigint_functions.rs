//! Replay the pure Perl bigint integer function oracle.
#![cfg(feature = "bigint")]
mod common;
use math_prime_util::bigint::functions::*;
use num_bigint::BigUint;

fn list<T: ToString>(v: Vec<T>) -> Option<String> {
    (!v.is_empty()).then(|| {
        v.into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    })
}

#[test]
fn bigint_functions_oracle() {
    for c in common::load("bigint_functions") {
        let n: BigUint = c.arg(0);
        let got = match c.func.as_str() {
            "aliquot_sum" => Some(aliquot_sum(&n).unwrap().to_string()),
            "abundance" => Some(abundance(&n).unwrap().to_string()),
            "dedekind_psi" => Some(dedekind_psi(&n).unwrap().to_string()),
            "exp_mangoldt" => Some(exp_mangoldt(&n).unwrap().to_string()),
            "liouville" => Some(liouville(&n).unwrap().to_string()),
            "prime_signature" => list(prime_signature(&n).unwrap()),
            "sopf" => Some(sopf(&n).unwrap().to_string()),
            "sopfr" => Some(sopfr(&n).unwrap().to_string()),
            "squarefree_kernel" => Some(squarefree_kernel(&n).unwrap().to_string()),
            "crootint" => Some(crootint(&n, c.arg(1)).to_string()),
            "powerfree_part" => Some(powerfree_part(&n, c.arg(1)).unwrap().to_string()),
            "jordan_totient" => Some(jordan_totient(c.arg(0), &c.arg(1)).unwrap().to_string()),
            "ramanujan_sum" => Some(ramanujan_sum(&n, &c.arg(1)).unwrap().to_string()),
            "remove_factors" => remove_factors(&n, &c.arg(1)).map(|x| x.to_string()),
            "remove_factors_exp" => {
                remove_factors_exp(&n, &c.arg(1)).map(|(r, e)| format!("{r} {e}"))
            }
            "inverse_totient" => list(inverse_totient(&n).unwrap()),
            other => panic!("unhandled fixture {other}"),
        };
        c.check(got);
    }
}

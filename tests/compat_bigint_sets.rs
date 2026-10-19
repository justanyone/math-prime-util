//! Replay every MPU pure-Perl bigint set fixture row.
#![cfg(feature = "bigint")]
use math_prime_util::bigint::sets::*;
use num_bigint::{BigInt, BigUint};

fn values(s: &str) -> Vec<BigInt> {
    if s == "-" {
        return Vec::new();
    }
    s.split(',').map(|x| x.parse().expect("integer")).collect()
}
fn render(a: &[BigInt]) -> String {
    if a.is_empty() {
        return "undef".to_owned();
    }
    a.iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" ")
}
#[test]
fn bigint_sets_oracle() {
    let mut rows = 0;
    for line in include_str!("fixtures/bigint_sets.tsv").lines() {
        if line.starts_with('#') {
            continue;
        }
        let parts: Vec<_> = line.split('\t').collect();
        let args: Vec<_> = parts[1].split_whitespace().collect();
        let a = values(args[0]);
        let b = args.get(1).map_or_else(Vec::new, |s| values(s));
        let expected = parts[2];
        let got = match parts[0] {
            "toset" => render(&toset(&a)),
            "vecuniq" => render(&vecuniq(&a)),
            "vecsingleton" => render(&vecsingleton(&a)),
            "vecsort" => render(&vecsort(&a)),
            "vecrsort" => render(&vecrsort(&a)),
            "setunion" => render(&setunion(&a, &b)),
            "setintersect" => render(&setintersect(&a, &b)),
            "setminus" => render(&setminus(&a, &b)),
            "setdelta" => render(&setdelta(&a, &b)),
            "sumset" => render(&sumset(&a, &b)),
            "setcontains" => u8::from(setcontains(&a, &b)).to_string(),
            "setcontainsany" => u8::from(setcontainsany(&a, &b)).to_string(),
            "set_is_disjoint" => u8::from(set_is_disjoint(&a, &b)).to_string(),
            "set_is_equal" => u8::from(set_is_equal(&a, &b)).to_string(),
            "set_is_subset" => u8::from(set_is_subset(&a, &b)).to_string(),
            "set_is_superset" => u8::from(set_is_superset(&a, &b)).to_string(),
            "set_is_proper_subset" => u8::from(set_is_proper_subset(&a, &b)).to_string(),
            "set_is_proper_superset" => u8::from(set_is_proper_superset(&a, &b)).to_string(),
            "set_is_proper_intersection" => {
                u8::from(set_is_proper_intersection(&a, &b)).to_string()
            }
            "is_sidon_set" => u8::from(is_sidon_set(&a)).to_string(),
            "is_sumfree_set" => u8::from(is_sumfree_set(&a)).to_string(),
            "vecmin" => vecmin(&a).map_or_else(|| "undef".to_owned(), |x| x.to_string()),
            "vecmax" => vecmax(&a).map_or_else(|| "undef".to_owned(), |x| x.to_string()),
            "vecsorti" => {
                let mut a = a.clone();
                vecsorti(&mut a);
                render(&a)
            }
            "vecrsorti" => {
                let mut a = a.clone();
                vecrsorti(&mut a);
                render(&a)
            }
            "vecmex" => {
                let a: Vec<BigUint> = a
                    .iter()
                    .map(|x| x.to_biguint().expect("unsigned"))
                    .collect();
                vecmex(&a).to_string()
            }
            "vecpmex" => {
                let a: Vec<BigUint> = a
                    .iter()
                    .map(|x| x.to_biguint().expect("unsigned"))
                    .collect();
                if a.contains(&BigUint::from(0_u8)) {
                    assert_eq!(expected, "ERROR");
                    rows += 1;
                    continue;
                }
                vecpmex(&a).to_string()
            }
            "setinsert" => {
                let mut a = toset(&a);
                let delta = setinsert(&mut a, &b);
                let list = a
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(",");
                format!("{delta};{list}")
            }
            "setremove" => {
                let mut a = toset(&a);
                let delta = setremove(&mut a, &b);
                let list = a
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(",");
                format!("{delta};{list}")
            }
            "setinvert" => {
                let mut a = toset(&a);
                let delta = setinvert(&mut a, &b);
                let list = a
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(",");
                format!("{delta};{list}")
            }
            "setbinop" => render(&setbinop(&a, &b, |x, y| x + y)),
            "vecfreq" => vecfreq(&a)
                .iter()
                .map(|(x, n)| format!("{x}:{n}"))
                .collect::<Vec<_>>()
                .join(" "),
            name => panic!("unknown function {name}"),
        };
        assert_eq!(got, expected, "{line}");
        rows += 1;
    }
    assert!(rows > 1000);
}

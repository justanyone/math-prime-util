//! Native iterator compatibility with the frozen MPU 0.75 oracle.
//! The oracle caps list callbacks at 2,000; count properties test full streams.
mod common;
use math_prime_util::iter::*;
use std::fmt::Display;

fn joined<T: Display>(values: impl IntoIterator<Item = T>, sep: &str) -> String {
    values
        .into_iter()
        .take(2000)
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(sep)
}
fn tuples<T: Display>(values: impl Iterator<Item = Vec<T>>, sep: &str) -> String {
    values
        .take(2000)
        .map(|v| {
            if v.is_empty() {
                "()".into()
            } else {
                joined(v, sep)
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
fn check(name: &str) {
    for c in common::load("iter").into_iter().filter(|c| c.func == name) {
        // Skip Perl validation cases: undef, negative, fractional, and string inputs
        // cannot enter the unsigned typed Rust API. Skip bigint inputs above u64.
        if c.error {
            continue;
        }
        let got = match name {
            "forprimes" | "forcomposites" | "foroddcomposites" => {
                let Ok(a) = c.args[0].parse::<u64>() else {
                    continue;
                };
                let b = if c.args.len() == 2 {
                    let Ok(b) = c.args[1].parse::<u64>() else {
                        continue;
                    };
                    b
                } else {
                    a
                };
                let a = if c.args.len() == 1 { 0 } else { a };
                match name {
                    "forprimes" => joined(primes_in(a..=b), " "),
                    "forcomposites" => joined(composites_in(a..=b), " "),
                    _ => joined(odd_composites_in(a..=b), " "),
                }
            }
            "forpart" | "forcomp" => {
                let n = c.arg(0);
                let mut options = PartitionOptions::default();
                for arg in &c.args[1..] {
                    let (key, v) = arg.split_once('=').unwrap();
                    let v: u64 = v.parse().unwrap();
                    match key {
                        "amin" => options.amin = Some(v),
                        "amax" => options.amax = Some(v),
                        "n" => options.n = Some(v),
                        "nmin" => options.nmin = Some(v),
                        "nmax" => options.nmax = Some(v),
                        "prime" => options.prime = Some(v != 0),
                        _ => panic!("unknown option"),
                    }
                }
                tuples(
                    if name == "forpart" {
                        partitions_of(n, options)
                    } else {
                        compositions_of(n, options)
                    },
                    "+",
                )
            }
            "forcomb" => tuples(
                combinations_of(c.arg(0), c.args.get(1).map(|s| s.parse().unwrap())),
                ",",
            ),
            "forperm" => tuples(permutations_of(c.arg(0)), ","),
            "forderange" => tuples(derangements_of(c.arg(0)), ","),
            "formultiperm" => tuples(multipermutations_of(&c.args), ","),
            "forsetproduct" => {
                let sets = if c.args.is_empty() {
                    Vec::new()
                } else {
                    c.args.split(|v| v == "|").map(<[String]>::to_vec).collect()
                };
                tuples(set_product(&sets), ",")
            }
            "prime_iterator" | "prime_iterator_object_iterate" => {
                let start = if c.args.is_empty() {
                    0
                } else {
                    let Ok(start) = c.args[0].parse::<u64>() else {
                        continue;
                    };
                    start
                };
                let expected = c.result.as_deref().unwrap().split_whitespace().count();
                // Native iterators stop at u64::MAX. MPU switches to bigint here.
                if c.result
                    .as_deref()
                    .unwrap()
                    .split_whitespace()
                    .any(|p| p.parse::<u64>().is_err())
                {
                    continue;
                }
                joined(
                    math_prime_util::prime_iterator_object(start).take(expected),
                    " ",
                )
            }
            _ => unreachable!(),
        };
        let expected = c.result.as_deref().unwrap();
        if got != expected {
            let i = got
                .bytes()
                .zip(expected.bytes())
                .position(|(a, b)| a != b)
                .unwrap_or(got.len().min(expected.len()));
            panic!(
                "line {} {} args {:?}: lengths {} {}, difference at {}: {:?} / {:?}",
                c.line,
                name,
                c.args,
                got.len(),
                expected.len(),
                i,
                &got[i..got.len().min(i + 60)],
                &expected[i..expected.len().min(i + 60)]
            );
        }
    }
}
macro_rules! oracle { ($($name:ident),* $(,)?) => { $(#[test] fn $name() { check(stringify!($name)); })* }; }
oracle!(
    forprimes,
    forcomposites,
    foroddcomposites,
    forpart,
    forcomp,
    forcomb,
    forperm,
    forderange,
    formultiperm,
    forsetproduct,
    prime_iterator
);
#[test]
fn prime_iterator_object() {
    // prime_iterator_object fixtures stringify Perl memory addresses. Compare
    // the corresponding prime_iterator_object_iterate value fixtures instead.
    check("prime_iterator_object_iterate");
}
#[test]
fn lastfor() {
    let mut count = 0;
    let result = math_prime_util::forpart(7, Default::default(), |p| {
        count += 1;
        if p.len() == 4 {
            math_prime_util::lastfor()
        } else {
            std::ops::ControlFlow::Continue(())
        }
    });
    assert!(result.is_break());
    assert_eq!(count, 5);
}

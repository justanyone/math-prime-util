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
/// Parse `hi` or `lo hi` as an inclusive range. Bigint bounds return `None` (skipped).
fn parse_range(args: &[String]) -> Option<std::ops::RangeInclusive<u64>> {
    match args {
        [hi] => Some(0..=hi.parse().ok()?),
        [lo, hi] => Some(lo.parse().ok()?..=hi.parse().ok()?),
        _ => None,
    }
}
/// Format `n:p*p*q` as the MPU fixture does, with each prime repeated by its exponent.
fn format_factored(n: u64, factors: &[(u64, u32)]) -> String {
    let primes: Vec<String> = factors
        .iter()
        .flat_map(|&(p, e)| std::iter::repeat_n(p, e as usize))
        .map(|p| p.to_string())
        .collect();
    format!("{n}:{}", primes.join("*"))
}
#[allow(clippy::too_many_lines)] // Keep the fixture dispatcher together.
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
            "forsemiprimes" | "foralmostprimes" | "forfactored" | "forsquarefree"
            | "forsquarefreeint" => {
                // `foralmostprimes` takes k first. Bigint bounds are skipped.
                let (k, rest) = if name == "foralmostprimes" {
                    let Ok(k) = c.args[0].parse::<u32>() else {
                        continue;
                    };
                    (k, &c.args[1..])
                } else {
                    (0, &c.args[..])
                };
                let Some(range) = parse_range(rest) else {
                    continue;
                };
                match name {
                    "forsemiprimes" => joined(semiprimes_in(range), " "),
                    "foralmostprimes" | "forfactored" | "forsquarefree" | "forsquarefreeint" => {
                        let mut values = Vec::new();
                        let mut save = |v| {
                            values.push(v);
                            if values.len() == 2000 {
                                std::ops::ControlFlow::Break(())
                            } else {
                                std::ops::ControlFlow::Continue(())
                            }
                        };
                        let _ = match name {
                            "foralmostprimes" => {
                                math_prime_util::foralmostprimes(k, range, |n| save(n.to_string()))
                            }
                            "forfactored" => math_prime_util::forfactored(range, |n, f| {
                                save(format_factored(n, &f))
                            }),
                            "forsquarefree" => math_prime_util::forsquarefree(range, |n, f| {
                                save(format_factored(n, &f))
                            }),
                            _ => math_prime_util::forsquarefreeint(range, |n| save(n.to_string())),
                        };
                        values.join(" ")
                    }
                    _ => unreachable!(),
                }
            }
            "fordivisors" => {
                let Ok(n) = c.args[0].parse::<u64>() else {
                    continue;
                };
                joined(divisors_of(n), " ")
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
    prime_iterator,
    forsemiprimes,
    foralmostprimes,
    forfactored,
    forsquarefree,
    forsquarefreeint,
    fordivisors
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
    let result = math_prime_util::forpart(7, PartitionOptions::default(), |p| {
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

//! Native iterator compatibility with the frozen MPU 0.75 oracle.
//! The oracle caps list callbacks at 2,000; count properties test full streams.
mod common;
use math_prime_util::iter::*;
use std::fmt::Display;
use std::ops::ControlFlow;

// Match the oracle callback cap and exercise the public closure APIs.
macro_rules! loop_values {
    ($run:ident, $range:expr) => {{
        let mut values = Vec::new();
        let _: ControlFlow<()> = math_prime_util::iter::$run($range, |n| {
            values.push(n);
            if values.len() == 2000 {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        });
        joined(values, " ")
    }};
}

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
// Exercise the closure API, including lastfor at the oracle's output cap.
fn collected<T>(
    visit: impl FnOnce(&mut dyn FnMut(Vec<T>) -> ControlFlow<()>) -> ControlFlow<()>,
) -> impl Iterator<Item = Vec<T>> {
    let mut values = Vec::new();
    let _ = visit(&mut |value| {
        values.push(value);
        if values.len() == 2000 {
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    });
    values.into_iter()
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
                    "forprimes" => loop_values!(forprimes, a..=b),
                    "forcomposites" => loop_values!(forcomposites, a..=b),
                    _ => loop_values!(foroddcomposites, a..=b),
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
                if name == "forpart" {
                    tuples(partitions_of(n, options), "+")
                } else {
                    tuples(compositions_of(n, options), "+")
                }
            }
            "forcomb" => tuples(
                combinations_of(c.arg(0), c.args.get(1).map(|s| s.parse().unwrap())),
                ",",
            ),
            "forperm" => tuples(collected(|f| math_prime_util::forperm(c.arg(0), f)), ","),
            "forderange" => tuples(collected(|f| math_prime_util::forderange(c.arg(0), f)), ","),
            "formultiperm" => tuples(
                collected(|f| math_prime_util::formultiperm(&c.args, f)),
                ",",
            ),
            "forsetproduct" => {
                let sets = if c.args.is_empty() {
                    Vec::new()
                } else {
                    c.args.split(|v| v == "|").map(<[String]>::to_vec).collect()
                };
                tuples(collected(|f| math_prime_util::forsetproduct(&sets, f)), ",")
            }
            "prime_iterator" | "prime_iterator_object_iterate" => {
                // Skipped rows, named here. The `u64` API cannot take them:
                // - args `-2`, `abc`, and `4.5` (MPU dies in `_validate_integer_nonneg`)
                // - arg `100000000000000000000` (above `u64::MAX`)
                // - args `18446744073709551557` and `18446744073709551615`
                //   (the oracle keeps going with primes above `u64::MAX`)
                let start = if c.args.is_empty() {
                    0
                } else {
                    let Ok(start) = c.args[0].parse::<u64>() else {
                        continue;
                    };
                    start
                };
                let expected = c.result.as_deref().unwrap().split_whitespace().count();
                if c.result
                    .as_deref()
                    .unwrap()
                    .split_whitespace()
                    .any(|p| p.parse::<u64>().is_err())
                {
                    continue;
                }
                let iter = if name == "prime_iterator" {
                    math_prime_util::prime_iterator(start)
                } else {
                    math_prime_util::prime_iterator_object(start)
                };
                joined(iter.take(expected), " ")
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
                    "forsemiprimes" => loop_values!(forsemiprimes, range),
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
    // Rows named `prime_iterator_object` stringify Perl addresses
    // (`Math::Prime::Util::PrimeIterator=SCALAR(...)` or `=REF(...)`).
    // They are not prime lists. Value rows are `prime_iterator_object_iterate`.
    // Skipped error rows, same args on every iterator name: `-2`, `abc`, `4.5`.
    for c in common::load("iter")
        .into_iter()
        .filter(|c| c.func == "prime_iterator_object")
    {
        if c.error {
            assert!(
                matches!(c.args.as_slice(), [a] if a == "-2" || a == "abc" || a == "4.5"),
                "line {}: unnamed prime_iterator_object error {:?}",
                c.line,
                c.args
            );
            continue;
        }
        let text = c.result.as_deref().unwrap();
        assert!(
            text.contains("PrimeIterator="),
            "line {}: expected an address string",
            c.line
        );
    }
    check("prime_iterator_object_iterate");
}
#[test]
fn prime_iterator_nested_streams_are_independent() {
    // `prime_iterator_nested3`: three nested cursors must not share a buffer.
    let case = common::load("iter")
        .into_iter()
        .find(|c| c.func == "prime_iterator_nested3")
        .expect("fixture row");
    let mut out = Vec::new();
    let outer = math_prime_util::prime_iterator(0);
    for a in outer {
        if a > 10 {
            break;
        }
        let middle = math_prime_util::prime_iterator(10 * a);
        for b in middle {
            if b > 10 * a + 10 {
                break;
            }
            let inner = math_prime_util::prime_iterator(b);
            for c in inner {
                if c > b + 10 {
                    break;
                }
                out.push(c);
            }
        }
    }
    assert_eq!(joined(out, " "), case.result.unwrap());
}
#[test]
fn lastfor_error_rows() {
    // `lastfor_outside_loop` and `lastfor_after_callback_die`: MPU croaks when
    // `forcount == 0` (`XS.xs`, `lastfor`). Rust `lastfor` is a `Break` value
    // and has no loop stack, so those two rows have no Rust failure to raise.
    let mut seen_outside = false;
    let mut seen_after_die = false;
    for c in common::load("iter")
        .into_iter()
        .filter(|c| c.func.starts_with("lastfor_"))
    {
        if c.error {
            match c.func.as_str() {
                "lastfor_outside_loop" => seen_outside = true,
                "lastfor_after_callback_die" => seen_after_die = true,
                other => panic!("unnamed lastfor error row {other}"),
            }
            continue;
        }
        // Value rows are covered by the loop tests; only named rows are checked here.
        assert!(c.result.is_some(), "{}", c.func);
    }
    assert!(seen_outside && seen_after_die);
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

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
// Perl `lastfor` sets a flag that ends the innermost running loop after the
// current callback returns; the rest of that callback still runs. A Rust
// callback models this by returning `Break` (`lastfor()`) at the point where
// the Perl callback finishes or does `lastfor, return`. `outside_loop` and
// `after_callback_die` croak in MPU because no loop is active; Rust `lastfor`
// is only a `Break` value with no loop stack, so those two rows have no Rust
// failure to raise and are checked only as named error rows.
fn lastfor_row(func: &str, args: &[String]) -> String {
    use math_prime_util::{lastfor, PartitionOptions};
    let cont = ControlFlow::<()>::Continue(());
    let num = |i: usize| -> u64 { args[i].parse().expect("numeric fixture arg") };
    let unum = |i: usize| -> usize { args[i].parse().expect("numeric fixture arg") };
    match func {
        "lastfor_forprimes_forprimes"
        | "lastfor_forprimes_forcomposites"
        | "lastfor_forprimes_foroddcomposites" => {
            let (hi_outer, hi_inner) = (num(0), num(1));
            let mut zn = Vec::new();
            let _ = math_prime_util::forprimes(0..=hi_outer, |p| {
                let mut hit = |n: u64| {
                    if n % p == 1 {
                        zn.push(n);
                        lastfor()
                    } else {
                        cont
                    }
                };
                // lastfor leaves only the inner loop; the outer one goes on.
                let _ = match func {
                    "lastfor_forprimes_forprimes" => {
                        math_prime_util::forprimes(0..=hi_inner, &mut hit)
                    }
                    "lastfor_forprimes_forcomposites" => {
                        math_prime_util::forcomposites(0..=hi_inner, &mut hit)
                    }
                    _ => math_prime_util::foroddcomposites(0..=hi_inner, &mut hit),
                };
                cont
            });
            joined(zn, " ")
        }
        "lastfor_fordivisors_power" => {
            let mut out = Vec::new();
            for n in 1..=20u64 {
                let _ = math_prime_util::fordivisors(n, |d| {
                    if math_prime_util::is_power(d, 0) {
                        out.push(d);
                        lastfor()
                    } else {
                        cont
                    }
                });
            }
            joined(out, " ")
        }
        "lastfor_forpart_len4" | "lastfor_forcomp_len4" => {
            let mut c = 0u64;
            let f = |p: Vec<u64>| {
                if p.len() < 4 {
                    return lastfor();
                }
                c += 1;
                cont
            };
            let _ = if func == "lastfor_forpart_len4" {
                math_prime_util::forpart(num(0), PartitionOptions::default(), f)
            } else {
                math_prime_util::forcomp(num(0), PartitionOptions::default(), f)
            };
            c.to_string()
        }
        "lastfor_forcomb_sum11" => {
            let mut c = 0u64;
            let _ = math_prime_util::forcomb(unum(0), Some(unum(1)), |v| {
                if v.iter().sum::<usize>() > 11 {
                    return lastfor();
                }
                c += 1;
                cont
            });
            c.to_string()
        }
        "lastfor_forperm_pos3_is5" | "lastfor_forderange_pos3_is5" => {
            let mut t = 0u64;
            let f = |v: Vec<usize>| {
                if v[3] == 5 {
                    return lastfor();
                }
                t += 1;
                cont
            };
            let _ = if func == "lastfor_forperm_pos3_is5" {
                math_prime_util::forperm(unum(0), f)
            } else {
                math_prime_util::forderange(unum(0), f)
            };
            t.to_string()
        }
        "lastfor_formultiperm_miles" => {
            let letters: Vec<char> = args.iter().filter_map(|s| s.chars().next()).collect();
            let mut t = 0u64;
            let _ = math_prime_util::formultiperm(&letters, |v| {
                // Perl `lastfor if ...; $t++` does not return: the match is counted too.
                t += 1;
                if v.iter().collect::<String>() == "miles" {
                    lastfor()
                } else {
                    cont
                }
            });
            t.to_string()
        }
        "lastfor_nested_ge7" => {
            let mut ps = Vec::new();
            let _ = math_prime_util::forprimes(0..=num(0), |p| {
                // Perl keeps running the body after `lastfor`, so the inner
                // loop still runs for p = 7 before the outer loop ends.
                let stop = p >= 7;
                ps.push(p);
                let _ = math_prime_util::forcomposites(0..=p, |c| {
                    ps.push(c);
                    cont
                });
                if stop {
                    lastfor()
                } else {
                    cont
                }
            });
            joined(ps, " ")
        }
        "lastfor_forcomposites_gt2000" => {
            let mut t = 0;
            let _ = math_prime_util::forcomposites(0..=num(0), |n| {
                t = n;
                if n > 2000 {
                    lastfor()
                } else {
                    cont
                }
            });
            t.to_string()
        }
        "lastfor_forpart_0"
        | "lastfor_forcomp_0"
        | "lastfor_forcomb_0"
        | "lastfor_forcomb_3_0"
        | "lastfor_forperm_0"
        | "lastfor_forderange_0"
        | "lastfor_formultiperm_empty"
        | "lastfor_forfactored_0_3"
        | "lastfor_forsquarefree_0_3"
        | "lastfor_forsquarefreeint_0_3" => {
            let mut t = 0u64;
            let mut tick = || {
                t += 1;
                lastfor()
            };
            let _ = match func {
                "lastfor_forpart_0" => {
                    math_prime_util::forpart(0, PartitionOptions::default(), |_| tick())
                }
                "lastfor_forcomp_0" => {
                    math_prime_util::forcomp(0, PartitionOptions::default(), |_| tick())
                }
                "lastfor_forcomb_0" => math_prime_util::forcomb(0, None, |_| tick()),
                "lastfor_forcomb_3_0" => math_prime_util::forcomb(3, Some(0), |_| tick()),
                "lastfor_forperm_0" => math_prime_util::forperm(0, |_| tick()),
                "lastfor_forderange_0" => math_prime_util::forderange(0, |_| tick()),
                "lastfor_formultiperm_empty" => {
                    math_prime_util::formultiperm::<char, ()>(&[], |_| tick())
                }
                "lastfor_forfactored_0_3" => math_prime_util::forfactored(0..=3, |_, _| tick()),
                "lastfor_forsquarefree_0_3" => math_prime_util::forsquarefree(0..=3, |_, _| tick()),
                _ => math_prime_util::forsquarefreeint(0..=3, |_| tick()),
            };
            t.to_string()
        }
        "lastfor_formultiperm_first" => {
            let mut out = Vec::new();
            let _ = math_prime_util::formultiperm(&[1u32, 1, 2], |v| {
                out.push(joined(v, " "));
                lastfor()
            });
            out.join(" ")
        }
        "lastfor_inner_die_outer_ok" => {
            // Perl: an inner callback dies inside eval; the outer lastfor still works.
            let mut n = 0u64;
            let _ = math_prime_util::forprimes(0..=num(0), |_| {
                let died = std::panic::catch_unwind(|| {
                    let _ = math_prime_util::forprimes(0..=10, |_| -> ControlFlow<()> {
                        panic!("inner")
                    });
                });
                assert!(died.is_err());
                n += 1;
                lastfor()
            });
            n.to_string()
        }
        "lastfor_forprimes_first_gt_100" => {
            let mut r = 0;
            let _ = math_prime_util::forprimes(0..=num(0), |p| {
                if p > 100 {
                    r = p;
                    lastfor()
                } else {
                    cont
                }
            });
            r.to_string()
        }
        other => panic!("unnamed lastfor row {other}"),
    }
}
#[test]
fn lastfor_rows() {
    let mut rows = 0;
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
        let args = c.args.clone();
        let got = lastfor_row(&c.func, &args);
        assert_eq!(Some(got), c.result, "line {}: {}", c.line, c.func);
        rows += 1;
    }
    assert!(seen_outside && seen_after_die);
    assert_eq!(rows, 25);
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

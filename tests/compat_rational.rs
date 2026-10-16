//! Oracle checks for the rational family.
mod common;
use math_prime_util::*;

fn pairs<T: std::fmt::Display>(values: Vec<(T, T)>) -> String {
    values
        .into_iter()
        .map(|(a, b)| format!("{a} {b}"))
        .collect::<Vec<_>>()
        .join(" ")
}
fn check(name: &str) {
    for c in common::load("rational")
        .into_iter()
        .filter(|c| c.func == name || (name == "convergents" && c.func == "convergents_list"))
    {
        // Skip MPU bigint fallback cases whose arguments or results exceed i128/u128.
        let unsupported_args = name != "bestrational"
            && c.args
                .iter()
                .filter(|s| !s.contains('/'))
                .any(|s| s.parse::<i128>().is_err() && s.parse::<u128>().is_err());
        let unsupported_result = c.result.as_ref().is_some_and(|s| {
            s.split_whitespace()
                .any(|v| v.parse::<i128>().is_err() && v.parse::<u128>().is_err())
        });
        if unsupported_args || unsupported_result {
            continue;
        }
        let result = std::panic::catch_unwind(|| {
            let u = |i: usize| c.args[i].parse::<u128>().expect("unsigned argument");
            let i = |j: usize| c.args[j].parse::<i128>().unwrap();
            let pair = || {
                let (a, b) = c.args[1].split_once('/').unwrap();
                (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap())
            };
            Some(match name {
                "contfrac" => contfrac(i(0), i(1))
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" "),
                "from_contfrac" => {
                    let v = c
                        .args
                        .iter()
                        .map(|s| s.parse().unwrap())
                        .collect::<Vec<_>>();
                    let (a, b) = from_contfrac(&v);
                    format!("{a} {b}")
                }
                "convergents" => {
                    let v = if c.func == "convergents_list" {
                        c.args.iter().map(|s| s.parse().unwrap()).collect()
                    } else if c.args.is_empty() {
                        Vec::new()
                    } else {
                        contfrac(i(0), i(1))
                    };
                    pairs(convergents(&v))
                }
                "bestrational" => {
                    let (a, b) = bestrational(c.args[0].parse().unwrap_or(f64::NAN), c.arg(1));
                    format!("{a} {b}")
                }
                "next_calkin_wilf" => {
                    let (a, b) = next_calkin_wilf(u(0), u(1));
                    format!("{a} {b}")
                }
                "next_stern_brocot" => {
                    let (a, b) = next_stern_brocot(u(0), u(1));
                    format!("{a} {b}")
                }
                "calkin_wilf_n" => calkin_wilf_n(u(0), u(1)).to_string(),
                "stern_brocot_n" => stern_brocot_n(u(0), u(1)).to_string(),
                "nth_calkin_wilf" => {
                    let (a, b) = nth_calkin_wilf(u(0));
                    format!("{a} {b}")
                }
                "nth_stern_brocot" => {
                    let (a, b) = nth_stern_brocot(u(0));
                    format!("{a} {b}")
                }
                "nth_stern_diatomic" => nth_stern_diatomic(u(0)).to_string(),
                "farey" => {
                    let v = farey(c.arg(0));
                    if c.args.len() == 1 {
                        pairs(v)
                    } else {
                        return v.get(c.arg::<usize>(1)).map(|(a, b)| format!("{a} {b}"));
                    }
                }
                "next_farey" => {
                    let (a, b) = pair();
                    return next_farey(c.arg(0), a, b).map(|(a, b)| format!("{a} {b}"));
                }
                "farey_rank" => {
                    let (a, b) = pair();
                    farey_rank(c.arg(0), a, b).to_string()
                }
                _ => unreachable!(),
            })
        });
        if c.error {
            assert!(result.is_err(), "expected error: {c:?}");
        } else {
            c.check(result.unwrap_or_else(|_| panic!("unexpected panic: {c:?}")));
        }
    }
}
macro_rules! oracle { ($($name:ident),*) => { $(#[test] fn $name() { check(stringify!($name)); })* }; }
oracle!(
    contfrac,
    from_contfrac,
    convergents,
    bestrational,
    next_calkin_wilf,
    next_stern_brocot,
    calkin_wilf_n,
    stern_brocot_n,
    nth_calkin_wilf,
    nth_stern_brocot,
    nth_stern_diatomic,
    farey,
    next_farey,
    farey_rank
);

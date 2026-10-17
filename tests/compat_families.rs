//! MPU powerful and powerfree oracle cases.
mod common;
use math_prime_util::*;

#[test]
fn is_powerful_oracle() {
    for c in common::load("families")
        .into_iter()
        .filter(|c| c.func == "is_powerful")
    {
        // Skip signed inputs, k outside u32, and bigint inputs/results: native API only.
        if c.args.iter().any(|a| a.parse::<u64>().is_err()) {
            continue;
        }
        let n = c.arg::<u64>(0);
        let k = if c.args.len() > 1 {
            let Ok(k) = c.args.last().unwrap().parse::<u32>() else {
                continue;
            };
            k
        } else {
            2
        };
        c.check(Some(u8::from(is_powerful(n, k))));
    }
}

#[test]
fn powerful_numbers_oracle() {
    for c in common::load("families")
        .into_iter()
        .filter(|c| c.func == "powerful_numbers")
    {
        // Skip signed inputs, k outside u32, and bigint inputs/results: native API only.
        if c.args.iter().any(|a| a.parse::<u64>().is_err()) {
            continue;
        }
        let n = c.arg::<u64>(0);
        let k = if c.args.len() > 1 {
            let Ok(k) = c.args.last().unwrap().parse::<u32>() else {
                continue;
            };
            k
        } else {
            2
        };
        let k = if c.args.len() > 2 { c.arg(2) } else { 2 };
        let result = powerful_numbers(n, c.arg(1), k)
            .iter()
            .map(u64::to_string)
            .collect::<Vec<_>>()
            .join(" ");
        c.check(Some(result));
    }
}

#[test]
fn powerful_count_oracle() {
    for c in common::load("families")
        .into_iter()
        .filter(|c| c.func == "powerful_count")
    {
        // Skip signed inputs, k outside u32, and bigint inputs/results: native API only.
        if c.args.iter().any(|a| a.parse::<u64>().is_err()) {
            continue;
        }
        let n = c.arg::<u64>(0);
        let k = if c.args.len() > 1 {
            let Ok(k) = c.args.last().unwrap().parse::<u32>() else {
                continue;
            };
            k
        } else {
            2
        };
        c.check(Some(powerful_count(n, k)));
    }
}

#[test]
fn sumpowerful_oracle() {
    for c in common::load("families")
        .into_iter()
        .filter(|c| c.func == "sumpowerful")
    {
        // Skip signed inputs, k outside u32, and bigint inputs/results: native API only.
        if c.args.iter().any(|a| a.parse::<u64>().is_err()) {
            continue;
        }
        let n = c.arg::<u64>(0);
        let k = if c.args.len() > 1 {
            let Ok(k) = c.args.last().unwrap().parse::<u32>() else {
                continue;
            };
            k
        } else {
            2
        };
        c.check(Some(sumpowerful(n, k)));
    }
}

#[test]
fn nth_powerful_oracle() {
    for c in common::load("families")
        .into_iter()
        .filter(|c| c.func == "nth_powerful")
    {
        // Skip signed inputs, k outside u32, and bigint inputs/results: native API only.
        if c.args.iter().any(|a| a.parse::<u64>().is_err()) {
            continue;
        }
        let n = c.arg::<u64>(0);
        let k = if c.args.len() > 1 {
            let Ok(k) = c.args.last().unwrap().parse::<u32>() else {
                continue;
            };
            k
        } else {
            2
        };
        if c.result.as_ref().is_some_and(|r| r.parse::<u64>().is_err()) {
            continue;
        }
        c.check(nth_powerful(n, k));
    }
}

#[test]
fn is_powerfree_oracle() {
    for c in common::load("families")
        .into_iter()
        .filter(|c| c.func == "is_powerfree")
    {
        // Skip signed inputs, k outside u32, and bigint inputs/results: native API only.
        if c.args.iter().any(|a| a.parse::<u64>().is_err()) {
            continue;
        }
        let n = c.arg::<u64>(0);
        let k = if c.args.len() > 1 {
            let Ok(k) = c.args.last().unwrap().parse::<u32>() else {
                continue;
            };
            k
        } else {
            2
        };
        c.check(Some(u8::from(is_powerfree(n, k))));
    }
}

#[test]
fn powerfree_count_oracle() {
    for c in common::load("families")
        .into_iter()
        .filter(|c| c.func == "powerfree_count")
    {
        // Skip signed inputs, k outside u32, and bigint inputs/results: native API only.
        if c.args.iter().any(|a| a.parse::<u64>().is_err()) {
            continue;
        }
        let n = c.arg::<u64>(0);
        let k = if c.args.len() > 1 {
            let Ok(k) = c.args.last().unwrap().parse::<u32>() else {
                continue;
            };
            k
        } else {
            2
        };
        c.check(Some(powerfree_count(n, k)));
    }
}

#[test]
fn nth_powerfree_oracle() {
    for c in common::load("families")
        .into_iter()
        .filter(|c| c.func == "nth_powerfree")
    {
        // Skip signed inputs, k outside u32, and bigint inputs/results: native API only.
        if c.args.iter().any(|a| a.parse::<u64>().is_err()) {
            continue;
        }
        let n = c.arg::<u64>(0);
        let k = if c.args.len() > 1 {
            let Ok(k) = c.args.last().unwrap().parse::<u32>() else {
                continue;
            };
            k
        } else {
            2
        };
        if c.result.as_ref().is_some_and(|r| r.parse::<u64>().is_err()) {
            continue;
        }
        c.check(nth_powerfree(n, k));
    }
}

#[test]
fn powerfree_sum_oracle() {
    for c in common::load("families")
        .into_iter()
        .filter(|c| c.func == "powerfree_sum")
    {
        // Skip signed inputs, k outside u32, and bigint inputs/results: native API only.
        if c.args.iter().any(|a| a.parse::<u64>().is_err()) {
            continue;
        }
        let n = c.arg::<u64>(0);
        let k = if c.args.len() > 1 {
            let Ok(k) = c.args.last().unwrap().parse::<u32>() else {
                continue;
            };
            k
        } else {
            2
        };
        c.check(Some(powerfree_sum(n, k)));
    }
}

#[test]
fn powerfree_part_oracle() {
    for c in common::load("families")
        .into_iter()
        .filter(|c| c.func == "powerfree_part")
    {
        // Skip signed inputs, k outside u32, and bigint inputs/results: native API only.
        if c.args.iter().any(|a| a.parse::<u64>().is_err()) {
            continue;
        }
        let n = c.arg::<u64>(0);
        let k = if c.args.len() > 1 {
            let Ok(k) = c.args.last().unwrap().parse::<u32>() else {
                continue;
            };
            k
        } else {
            2
        };
        c.check(Some(powerfree_part(n, k)));
    }
}

#[test]
fn powerfree_part_sum_oracle() {
    for c in common::load("families")
        .into_iter()
        .filter(|c| c.func == "powerfree_part_sum")
    {
        // Skip signed inputs, k outside u32, and bigint inputs/results: native API only.
        if c.args.iter().any(|a| a.parse::<u64>().is_err()) {
            continue;
        }
        let n = c.arg::<u64>(0);
        let k = if c.args.len() > 1 {
            let Ok(k) = c.args.last().unwrap().parse::<u32>() else {
                continue;
            };
            k
        } else {
            2
        };
        c.check(Some(powerfree_part_sum(n, k)));
    }
}

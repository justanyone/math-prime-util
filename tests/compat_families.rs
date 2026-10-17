//! Ramanujan prime cases from the committed MPU oracle.
mod common;
use common::each;
use math_prime_util::*;

#[test]
fn nth_ramanujan_prime_oracle() {
    each("families", "nth_ramanujan_prime", |c| {
        c.check(nth_ramanujan_prime(c.arg(0)));
    });
}
#[test]
fn nth_ramanujan_prime_lower_oracle() {
    each("families", "nth_ramanujan_prime_lower", |c| {
        c.check(nth_ramanujan_prime_lower(c.arg(0)));
    });
}
#[test]
fn nth_ramanujan_prime_approx_oracle() {
    each("families", "nth_ramanujan_prime_approx", |c| {
        c.check(nth_ramanujan_prime_approx(c.arg(0)));
    });
}
#[test]
fn ramanujan_prime_count_upper_oracle() {
    each("families", "ramanujan_prime_count_upper", |c| {
        c.check(Some(ramanujan_prime_count_upper(c.arg(0))));
    });
}

#[test]
fn prime_powers_oracle() {
    each("families", "prime_powers", |c| {
        let (lo, hi) = if c.args.len() == 1 {
            (0, c.arg(0))
        } else {
            (c.arg(0), c.arg(1))
        };
        let values = prime_powers(lo, hi);
        c.check((!values.is_empty()).then(|| {
            values
                .iter()
                .map(u64::to_string)
                .collect::<Vec<_>>()
                .join(" ")
        }));
    });
}

#[test]
fn is_prime_power_oracle() {
    each("families", "is_prime_power", |c| {
        c.check(Some(is_prime_power(c.arg(0))));
    });
}

#[test]
fn next_prime_power_oracle() {
    each("families", "next_prime_power", |c| {
        // MPU promotes these results to bigint. The native u64 API cannot
        // return 2^64 for MAX_PRIME, u64::MAX - 1, or u64::MAX.
        if c.result.as_ref().is_some_and(|v| v.parse::<u64>().is_err()) {
            assert_eq!(next_prime_power(c.arg(0)), None);
            return;
        }
        c.check(next_prime_power(c.arg(0)));
    });
}

#[test]
fn prev_prime_power_oracle() {
    each("families", "prev_prime_power", |c| {
        c.check(prev_prime_power(c.arg(0)));
    });
}
#[test]
fn nth_prime_power_oracle() {
    each("families", "nth_prime_power", |c| {
        c.check(nth_prime_power(c.arg(0)));
    });
}
#[test]
fn nth_prime_power_approx_oracle() {
    each("families", "nth_prime_power_approx", |c| {
        c.check(nth_prime_power_approx(c.arg(0)));
    });
}
#[test]
fn nth_prime_power_lower_oracle() {
    each("families", "nth_prime_power_lower", |c| {
        c.check(nth_prime_power_lower(c.arg(0)));
    });
}
#[test]
fn nth_prime_power_upper_oracle() {
    each("families", "nth_prime_power_upper", |c| {
        c.check(nth_prime_power_upper(c.arg(0)));
    });
}

#[test]
fn twin_primes_oracle() {
    each("families", "twin_primes", |c| {
        let (lo, hi) = if c.args.len() == 1 {
            (0, c.arg(0))
        } else {
            (c.arg(0), c.arg(1))
        };
        let values = twin_primes(lo, hi);
        c.check((!values.is_empty()).then(|| {
            values
                .iter()
                .map(u64::to_string)
                .collect::<Vec<_>>()
                .join(" ")
        }));
    });
}

#[test]
fn twin_prime_count_oracle() {
    each("families", "twin_prime_count", |c| {
        let value = if c.args.len() == 1 {
            twin_prime_count(c.arg(0))
        } else {
            let lo: u64 = c.arg(0);
            let hi: u64 = c.arg(1);
            if lo > hi {
                0
            } else {
                twin_prime_count(hi) - lo.checked_sub(1).map_or(0, twin_prime_count)
            }
        };
        c.check(Some(value));
    });
}

#[test]
fn twin_prime_count_approx_oracle() {
    each("families", "twin_prime_count_approx", |c| {
        c.check(Some(twin_prime_count_approx(c.arg(0))));
    });
}

#[test]
fn nth_twin_prime_oracle() {
    each("families", "nth_twin_prime", |c| {
        c.check(nth_twin_prime(c.arg(0)));
    });
}

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
        // Skip signed and bigint inputs; the API takes native unsigned integers.
        if c.args.iter().any(|a| a.parse::<u64>().is_err()) {
            continue;
        }
        let (lo, hi) = if c.args.len() == 1 {
            (1, c.arg(0))
        } else {
            (c.arg(0), c.arg(1))
        };
        let k = if c.args.len() > 2 {
            u32::try_from(c.arg::<u64>(2)).unwrap_or(u32::MAX)
        } else {
            2
        };
        let result = powerful_numbers(lo, hi, k)
            .iter()
            .map(u64::to_string)
            .collect::<Vec<_>>()
            .join(" ");
        c.check(if result.is_empty() {
            None
        } else {
            Some(result)
        });
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

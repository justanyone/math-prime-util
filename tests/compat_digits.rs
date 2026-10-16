//! Compare fixed width helpers with MPU fixtures.
mod common;
use common::Case;
use math_prime_util::digits::*;

fn run(name: &str, call: impl Fn(&Case) -> Option<String>) {
    common::each("digits", name, |c| {
        // Skip bases above u64::MAX for all native base-taking functions.
        if matches!(
            name,
            "todigits"
                | "todigitstring"
                | "fromdigits"
                | "reverse_digits"
                | "is_palindrome"
                | "is_harshad"
                | "digital_root"
                | "mult_digital_root"
        ) && c.args.get(1).is_some_and(|s| s.parse::<u64>().is_err())
        {
            return;
        }
        // fromdigits returns u64; skip its bigint result cases.
        if name == "fromdigits"
            && !c.error
            && c.result.as_ref().is_some_and(|s| s.parse::<u64>().is_err())
        {
            return;
        }
        // Skip bigint arguments or results outside the i128/u64 API.
        // Each skipped function and argument is recorded by this comment
        // and the fixture case itself; no signed i64 case is skipped.
        if !matches!(name, "sumdigits" | "fromdigits" | "toint")
            && c.args.iter().any(|a| {
                a.parse::<i128>().is_err()
                    && a.bytes()
                        .all(|b| b.is_ascii_digit() || b == b'-' || b == b'+')
            })
        {
            return;
        }
        if c.result.as_ref().is_some_and(|s| {
            s.split_whitespace().any(|a| {
                a.parse::<i128>().is_err() && a.bytes().all(|b| b.is_ascii_digit() || b == b'-')
            })
        }) {
            return;
        }
        let got = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| call(c)));
        match got {
            Err(_) => {
                if !c.error {
                    println!("unexpected panic: {c:?}");
                }
                assert!(c.error, "{c:?}");
            }
            Ok(v) => {
                if c.error || v != c.result {
                    println!("mismatch: {c:?}; got {v:?}");
                }
                c.check(v);
            }
        }
    });
}
fn number(c: &Case, i: usize) -> i128 {
    c.arg(i)
}
fn base(c: &Case) -> u64 {
    c.args.get(1).map_or(10, |s| s.parse().unwrap())
}
#[test]
fn absint_oracle() {
    run("absint", |c| absint(number(c, 0)).map(|v| v.to_string()));
}
#[test]
fn negint_oracle() {
    run("negint", |c| negint(number(c, 0)).map(|v| v.to_string()));
}
#[test]
fn add1int_oracle() {
    run("add1int", |c| add1int(number(c, 0)).map(|v| v.to_string()));
}
#[test]
fn sub1int_oracle() {
    run("sub1int", |c| sub1int(number(c, 0)).map(|v| v.to_string()));
}
#[test]
fn signint_oracle() {
    run("signint", |c| Some(signint(number(c, 0)).to_string()));
}
#[test]
fn hammingweight_oracle() {
    run("hammingweight", |c| {
        Some(hammingweight(number(c, 0)).to_string())
    });
}
#[test]
fn addint_oracle() {
    run("addint", |c| {
        addint(number(c, 0), number(c, 1)).map(|v| v.to_string())
    });
}
#[test]
fn subint_oracle() {
    run("subint", |c| {
        subint(number(c, 0), number(c, 1)).map(|v| v.to_string())
    });
}
#[test]
fn mulint_oracle() {
    run("mulint", |c| {
        mulint(number(c, 0), number(c, 1)).map(|v| v.to_string())
    });
}
#[test]
fn divint_oracle() {
    run("divint", |c| {
        divint(number(c, 0), number(c, 1)).map(|v| v.to_string())
    });
}
#[test]
fn modint_oracle() {
    run("modint", |c| {
        modint(number(c, 0), number(c, 1)).map(|v| v.to_string())
    });
}
#[test]
fn cdivint_oracle() {
    run("cdivint", |c| {
        cdivint(number(c, 0), number(c, 1)).map(|v| v.to_string())
    });
}
#[test]
fn cmpint_oracle() {
    run("cmpint", |c| {
        Some(cmpint(number(c, 0), number(c, 1)).to_string())
    });
}
#[test]
fn muladdint_oracle() {
    run("muladdint", |c| {
        muladdint(number(c, 0), number(c, 1), number(c, 2)).map(|v| v.to_string())
    });
}
#[test]
fn mulsubint_oracle() {
    run("mulsubint", |c| {
        mulsubint(number(c, 0), number(c, 1), number(c, 2)).map(|v| v.to_string())
    });
}
#[test]
fn divrem_oracle() {
    run("divrem", |c| {
        divrem(number(c, 0), number(c, 1)).map(|(q, r)| format!("{q} {r}"))
    });
}
#[test]
fn tdivrem_oracle() {
    run("tdivrem", |c| {
        tdivrem(number(c, 0), number(c, 1)).map(|(q, r)| format!("{q} {r}"))
    });
}
#[test]
fn fdivrem_oracle() {
    run("fdivrem", |c| {
        fdivrem(number(c, 0), number(c, 1)).map(|(q, r)| format!("{q} {r}"))
    });
}
#[test]
fn cdivrem_oracle() {
    run("cdivrem", |c| {
        cdivrem(number(c, 0), number(c, 1)).map(|(q, r)| format!("{q} {r}"))
    });
}
#[test]
fn lshiftint_oracle() {
    run("lshiftint", |c| {
        lshiftint(
            number(c, 0),
            c.args.get(1).map_or(1, |s| s.parse().unwrap()),
        )
        .map(|v| v.to_string())
    });
}
#[test]
fn rshiftint_oracle() {
    run("rshiftint", |c| {
        rshiftint(
            number(c, 0),
            c.args.get(1).map_or(1, |s| s.parse().unwrap()),
        )
        .map(|v| v.to_string())
    });
}
#[test]
fn rashiftint_oracle() {
    run("rashiftint", |c| {
        rashiftint(
            number(c, 0),
            c.args.get(1).map_or(1, |s| s.parse().unwrap()),
        )
        .map(|v| v.to_string())
    });
}
#[test]
fn reverse_digits_oracle() {
    run("reverse_digits", |c| {
        reverse_digits(number(c, 0), base(c)).map(|v| v.to_string())
    });
}
#[test]
fn digital_root_oracle() {
    run("digital_root", |c| {
        Some(digital_root(number(c, 0), base(c)).to_string())
    });
}
#[test]
fn mult_digital_root_oracle() {
    run("mult_digital_root", |c| {
        Some(mult_digital_root(number(c, 0), base(c)).to_string())
    });
}
#[test]
fn is_harshad_oracle() {
    run("is_harshad", |c| {
        Some(u8::from(is_harshad(number(c, 0), base(c))).to_string())
    });
}
#[test]
fn is_palindrome_oracle() {
    run("is_palindrome", |c| {
        Some(u8::from(is_palindrome(number(c, 0), base(c))).to_string())
    });
}
#[test]
fn todigits_oracle() {
    run("todigits", |c| {
        Some(
            todigits(
                number(c, 0),
                base(c),
                c.args.get(2).map(|s| s.parse().unwrap()),
            )
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(" "),
        )
    });
}
#[test]
fn todigitstring_oracle() {
    run("todigitstring", |c| {
        Some(todigitstring(
            number(c, 0),
            base(c),
            c.args.get(2).map(|s| s.parse().unwrap()),
        ))
    });
}
#[test]
fn sumdigits_oracle() {
    run("sumdigits", |c| {
        Some(sumdigits(&c.args[0], c.args.get(1).map(|s| s.parse().unwrap())).to_string())
    });
}
#[test]
fn fromdigits_oracle() {
    run("fromdigits", |c| {
        // The shared loader drops the empty first argument in " 10".
        if c.args == ["10"] && c.result.as_deref() == Some("0") {
            return fromdigits(&[], 10).map(|v| v.to_string());
        }
        let b = base(c);
        let d: Vec<_> = c.args[0]
            .chars()
            .map(|ch| u64::from(ch.to_digit(36).expect("digit")))
            .collect();
        fromdigits(&d, b).map(|v| v.to_string())
    });
}
#[test]
fn toint_oracle() {
    run("toint", |c| {
        toint(
            c.args
                .first()
                .map_or("", |s| if s == "(empty)" { "" } else { s }),
        )
        .map(|v| v.to_string())
    });
}
#[test]
fn tozeckendorf_oracle() {
    common::each("digits", "tozeckendorf", |c| {
        // Skip tozeckendorf bigint arguments above u64::MAX.
        if let Ok(n) = c.args[0].parse::<u64>() {
            c.check(Some(tozeckendorf(n)));
        } else if c.error {
            assert!(c.args[0].starts_with('-'));
        }
    });
}
#[test]
fn fromzeckendorf_oracle() {
    common::each("digits", "fromzeckendorf", |c| {
        // Skip fromzeckendorf bigint results above u64::MAX.
        if !c.error && c.result.as_ref().is_some_and(|s| s.parse::<u64>().is_err()) {
            return;
        }
        let got = std::panic::catch_unwind(|| fromzeckendorf(&c.args[0]));
        match got {
            Err(_) => assert!(c.error),
            Ok(v) => c.check(v),
        }
    });
}

fn predicate_oracle(name: &str, call: impl Fn(&[i128]) -> bool) {
    common::each("combinat", name, |c| {
        // Skip is_divisible varargs: the Rust helper takes one divisor.
        if name == "is_divisible" && c.args.len() != 2 {
            return;
        }
        // Skip is_odd, is_even, is_divisible and is_congruent bigint
        // arguments outside i128. Signed i64 cases are all supported.
        let Ok(args) = c
            .args
            .iter()
            .map(|s| s.parse())
            .collect::<Result<Vec<i128>, _>>()
        else {
            return;
        };
        c.check(Some(u8::from(call(&args))));
    });
}

#[test]
fn is_odd_oracle() {
    predicate_oracle("is_odd", |a| is_odd(a[0]));
}
#[test]
fn is_even_oracle() {
    predicate_oracle("is_even", |a| is_even(a[0]));
}
#[test]
fn is_divisible_oracle() {
    predicate_oracle("is_divisible", |a| is_divisible(a[0], a[1]));
}
#[test]
fn is_congruent_oracle() {
    predicate_oracle("is_congruent", |a| is_congruent(a[0], a[1], a[2]));
}

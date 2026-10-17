//! Seeded native compatibility with MPU 0.75.
mod common;
use math_prime_util::*;

fn cases(name: &str, mut f: impl FnMut(&common::Case)) {
    for c in common::load("random").iter().filter(|c| c.func == name) {
        let seed = &c.args[0];
        let bytes: Vec<_> = (0..seed.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&seed[i..i + 2], 16).unwrap())
            .collect();
        csrand(&bytes);
        f(c);
    }
}

fn list<T: std::fmt::Display>(values: &[T]) -> Option<String> {
    (!values.is_empty()).then(|| {
        values
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    })
}

#[test]
fn random_unrestricted_semiprime_oracle() {
    cases("random_unrestricted_semiprime", |c| {
        let bits: u32 = c.arg(1);
        // MPU bigint bit sizes above 64 have no native equivalent yet.
        if bits > 64 {
            assert_eq!(random_unrestricted_semiprime(bits), None);
            return;
        }
        c.check(random_unrestricted_semiprime(bits));
    });
}

#[test]
fn random_factored_integer_oracle() {
    cases("random_factored_integer", |c| {
        // Bounds above u64::MAX require the bigint phase.
        let Ok(n) = c.args[1].parse::<u64>() else {
            return;
        };
        if c.error {
            assert!(std::panic::catch_unwind(|| random_factored_integer(n)).is_err());
        } else {
            let (r, factors) = random_factored_integer(n);
            let mut values = vec![r];
            values.extend(factors);
            c.check(list(&values));
        }
    });
}

#[test]
fn randperm_oracle() {
    cases("randperm", |c| {
        let n = c.arg(1);
        let k = if c.args.len() == 2 { n } else { c.arg(2) };
        c.check(list(&randperm(n, k)));
    });
}

#[test]
fn shuffle_oracle() {
    cases("shuffle", |c| {
        let mut items = c.args[1..].to_vec();
        shuffle(&mut items);
        c.check(list(&items));
    });
}

// Skip random_prime arguments -3, (2, -4), (2, a), a, and undef: the
// native API cannot take signed or malformed inputs. Also skip bounds above
// u64::MAX, random_ndigit_prime(20/25/30), and random_nbit_prime(65/80/100/128):
// these need the bigint phase. Map one-argument random_prime(n) to (2, n).
fn check_prime(f: &str) {
    let mut count = 0;
    for c in common::load("random").into_iter().filter(|c| c.func == f) {
        let Ok(first) = c.args[1].parse::<u64>() else {
            continue;
        };
        let second = if c.args.len() > 2 {
            let Ok(n) = c.args[2].parse::<u64>() else {
                continue;
            };
            Some(n)
        } else {
            None
        };
        if f != "random_prime"
            && (first > u64::from(u32::MAX)
                || (f == "random_ndigit_prime" && first >= 20)
                || (f != "random_ndigit_prime" && first > 64))
        {
            continue;
        }
        count += 1;
        csrand(&unhex(&c.args[0]));
        let got = std::panic::catch_unwind(|| match f {
            "random_prime" => {
                second.map_or_else(|| random_prime(2, first), |n| random_prime(first, n))
            }
            "random_ndigit_prime" => random_ndigit_prime(u32::try_from(first).unwrap()),
            "random_nbit_prime" => random_nbit_prime(u32::try_from(first).unwrap()),
            "random_safe_prime" => random_safe_prime(u32::try_from(first).unwrap()),
            _ => unreachable!(),
        });
        if c.error {
            assert!(got.is_err(), "line {} must panic", c.line);
        } else {
            c.check(got.unwrap());
        }
    }
    assert!(count > 0);
}

#[test]
fn random_prime_oracle() {
    check_prime("random_prime");
}
#[test]
fn random_ndigit_prime_oracle() {
    check_prime("random_ndigit_prime");
}
#[test]
fn random_nbit_prime_oracle() {
    check_prime("random_nbit_prime");
}
#[test]
fn random_safe_prime_oracle() {
    check_prime("random_safe_prime");
}

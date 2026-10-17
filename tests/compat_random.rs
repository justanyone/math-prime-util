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

//! MPU 0.75 primality oracle checks.
mod common;
use math_prime_util::*;

#[test]
fn is_pseudoprime_oracle() {
    common::each("primality", "is_pseudoprime", |c| {
        // Negative and bigint n arguments are outside this unsigned native API.
        let Ok(n) = c.args[0].parse::<u64>() else {
            return;
        };
        let result = std::panic::catch_unwind(|| {
            if c.args.len() == 1 {
                return Some(u8::from(is_pseudoprime(n, 2)));
            }
            Some(u8::from(
                c.args
                    .iter()
                    .skip(1)
                    .all(|a| is_pseudoprime(n, a.parse().unwrap())),
            ))
        });
        if c.error {
            assert!(result.is_err(), "{c:?}");
        } else {
            c.check(result.unwrap());
        }
    });
}

#[test]
fn is_euler_pseudoprime_oracle() {
    common::each("primality", "is_euler_pseudoprime", |c| {
        // Negative and bigint n arguments are outside this unsigned native API.
        let Ok(n) = c.args[0].parse::<u64>() else {
            return;
        };
        let result = std::panic::catch_unwind(|| {
            if c.args.len() == 1 {
                return Some(u8::from(is_euler_pseudoprime(n, 2)));
            }
            Some(u8::from(
                c.args
                    .iter()
                    .skip(1)
                    .all(|a| is_euler_pseudoprime(n, a.parse().unwrap())),
            ))
        });
        if c.error {
            assert!(result.is_err(), "{c:?}");
        } else {
            c.check(result.unwrap());
        }
    });
}

#[test]
fn is_strong_pseudoprime_oracle() {
    common::each("primality", "is_strong_pseudoprime", |c| {
        // Negative and bigint n arguments are outside this unsigned native API.
        let Ok(n) = c.args[0].parse::<u64>() else {
            return;
        };
        let result = std::panic::catch_unwind(|| {
            if c.args.len() == 1 {
                return Some(u8::from(is_strong_pseudoprime(n, 2)));
            }
            Some(u8::from(
                c.args
                    .iter()
                    .skip(1)
                    .all(|a| is_strong_pseudoprime(n, a.parse().unwrap())),
            ))
        });
        if c.error {
            assert!(result.is_err(), "{c:?}");
        } else {
            c.check(result.unwrap());
        }
    });
}

#[test]
fn is_euler_plumb_pseudoprime_oracle() {
    common::each("primality", "is_euler_plumb_pseudoprime", |c| {
        // Negative and bigint n arguments are outside this unsigned native API.
        let Ok(n) = c.args[0].parse::<u64>() else {
            return;
        };
        let result = std::panic::catch_unwind(|| Some(u8::from(is_euler_plumb_pseudoprime(n))));
        if c.error {
            assert!(result.is_err(), "{c:?}");
        } else {
            c.check(result.unwrap());
        }
    });
}

#[test]
fn is_lucas_pseudoprime_oracle() {
    common::each("primality", "is_lucas_pseudoprime", |c| {
        // Negative and bigint n arguments are outside this unsigned native API.
        let Ok(n) = c.args[0].parse::<u64>() else {
            return;
        };
        let result = std::panic::catch_unwind(|| Some(u8::from(is_lucas_pseudoprime(n))));
        if c.error {
            assert!(result.is_err(), "{c:?}");
        } else {
            c.check(result.unwrap());
        }
    });
}

#[test]
fn is_strong_lucas_pseudoprime_oracle() {
    common::each("primality", "is_strong_lucas_pseudoprime", |c| {
        // Negative and bigint n arguments are outside this unsigned native API.
        let Ok(n) = c.args[0].parse::<u64>() else {
            return;
        };
        let result = std::panic::catch_unwind(|| Some(u8::from(is_strong_lucas_pseudoprime(n))));
        if c.error {
            assert!(result.is_err(), "{c:?}");
        } else {
            c.check(result.unwrap());
        }
    });
}

#[test]
fn is_extra_strong_lucas_pseudoprime_oracle() {
    common::each("primality", "is_extra_strong_lucas_pseudoprime", |c| {
        // Negative and bigint n arguments are outside this unsigned native API.
        let Ok(n) = c.args[0].parse::<u64>() else {
            return;
        };
        let result =
            std::panic::catch_unwind(|| Some(u8::from(is_extra_strong_lucas_pseudoprime(n))));
        if c.error {
            assert!(result.is_err(), "{c:?}");
        } else {
            c.check(result.unwrap());
        }
    });
}

#[test]
fn is_almost_extra_strong_lucas_pseudoprime_oracle() {
    common::each(
        "primality",
        "is_almost_extra_strong_lucas_pseudoprime",
        |c| {
            // Negative and bigint n arguments are outside this unsigned native API.
            let Ok(n) = c.args[0].parse::<u64>() else {
                return;
            };
            let result = std::panic::catch_unwind(|| {
                Some(u8::from(is_almost_extra_strong_lucas_pseudoprime(
                    n,
                    c.args.get(1).map_or(1, |a| a.parse().unwrap()),
                )))
            });
            if c.error {
                assert!(result.is_err(), "{c:?}");
            } else {
                c.check(result.unwrap());
            }
        },
    );
}

#[test]
fn is_bpsw_prime_oracle() {
    common::each("primality", "is_bpsw_prime", |c| {
        // Negative and bigint n arguments are outside this unsigned native API.
        let Ok(n) = c.args[0].parse::<u64>() else {
            return;
        };
        let result = std::panic::catch_unwind(|| Some(is_bpsw_prime(n)));
        if c.error {
            assert!(result.is_err(), "{c:?}");
        } else {
            c.check(result.unwrap());
        }
    });
}

#[test]
fn is_prob_prime_oracle() {
    common::each("primality", "is_prob_prime", |c| {
        // Negative and bigint n arguments are outside this unsigned native API.
        let Ok(n) = c.args[0].parse::<u64>() else {
            return;
        };
        let result = std::panic::catch_unwind(|| Some(is_prob_prime(n)));
        if c.error {
            assert!(result.is_err(), "{c:?}");
        } else {
            c.check(result.unwrap());
        }
    });
}

#[test]
fn is_safe_prime_oracle() {
    common::each("primality", "is_safe_prime", |c| {
        // Negative and bigint n arguments are outside this unsigned native API.
        let Ok(n) = c.args[0].parse::<u64>() else {
            return;
        };
        let result = std::panic::catch_unwind(|| Some(u8::from(is_safe_prime(n))));
        if c.error {
            assert!(result.is_err(), "{c:?}");
        } else {
            c.check(result.unwrap());
        }
    });
}

#[test]
fn is_mersenne_prime_oracle() {
    common::each("primality", "is_mersenne_prime", |c| {
        // Negative and bigint n arguments are outside this unsigned native API.
        let Ok(n) = c.args[0].parse::<u64>() else {
            return;
        };
        let result = std::panic::catch_unwind(|| is_mersenne_prime(n).map(u8::from));
        if c.error {
            assert!(result.is_err(), "{c:?}");
        } else {
            c.check(result.unwrap());
        }
    });
}

#[test]
fn is_prime_oracle() {
    common::each("primality", "is_prime", |c| {
        // Negative and bigint n arguments are outside this unsigned native API.
        let Ok(n) = c.args[0].parse::<u64>() else {
            return;
        };
        let result = std::panic::catch_unwind(|| is_prime(n));
        if c.error {
            assert!(result.is_err(), "{c:?}");
        } else {
            c.check(Some(result.unwrap()));
        }
    });
}

#[test]
fn is_frobenius_pseudoprime_oracle() {
    common::each("primality", "is_frobenius_pseudoprime", |c| {
        // Negative and bigint n arguments are outside this unsigned native API.
        let Ok(n) = c.args[0].parse::<u64>() else {
            return;
        };
        // A lone P argument is a Perl arity error; Rust requires both P and Q.
        if c.args.len() == 2 {
            assert!(c.error);
            return;
        }
        let parameters = if c.args.len() == 3 {
            let (Ok(p), Ok(q)) = (c.args[1].parse::<i64>(), c.args[2].parse::<i64>()) else {
                return;
            };
            // MPU uses its bigint fallback outside the native C P,Q bound.
            if p.unsigned_abs() > 3_037_000_497 || q.unsigned_abs() > 3_037_000_497 {
                return;
            }
            Some((p, q))
        } else {
            None
        };
        let result = std::panic::catch_unwind(|| {
            parameters.map_or_else(
                || is_frobenius_pseudoprime(n),
                |(p, q)| is_frobenius_pseudoprime_pq(n, p, q),
            )
        });
        if c.error {
            assert!(result.is_err(), "{c:?}");
        } else {
            c.check(Some(u8::from(result.unwrap())));
        }
    });
}

#[test]
fn is_frobenius_khashin_pseudoprime_oracle() {
    common::each("primality", "is_frobenius_khashin_pseudoprime", |c| {
        // Negative and bigint n arguments are outside this unsigned native API.
        let Ok(n) = c.args[0].parse::<u64>() else {
            return;
        };
        c.check(Some(u8::from(is_frobenius_khashin_pseudoprime(n))));
    });
}

#[test]
fn is_frobenius_underwood_pseudoprime_oracle() {
    common::each("primality", "is_frobenius_underwood_pseudoprime", |c| {
        // Negative and bigint n arguments are outside this unsigned native API.
        let Ok(n) = c.args[0].parse::<u64>() else {
            return;
        };
        c.check(Some(u8::from(is_frobenius_underwood_pseudoprime(n))));
    });
}

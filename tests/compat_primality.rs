//! Replay the unsigned oracle cases through the generic integer surface.
mod common;

use math_prime_util::Int;

fn replay(function: &str) {
    let mut count = 0;
    for family in ["primality", "factor", "modular", "int", "sieve"] {
        for case in common::load(family) {
            if case.func != function || (family == "sieve" && function != "next_prime") {
                continue;
            }
            // Signed inputs and values above u128 are not supported by these backends.
            let Ok(args) = case
                .args
                .iter()
                .map(|s| s.parse::<u128>())
                .collect::<Result<Vec<_>, _>>()
            else {
                continue;
            };
            let result = evaluate(&args, function);
            // The sieve oracle uses the bounded native next_prime API.
            if function == "next_prime" && case.result.is_none() {
                assert_eq!(u64::try_from(args[0]).unwrap().next_prime(), None);
            } else {
                assert_eq!(result, case.result, "{family}:{}", case.line);
            }
            if let Ok(small) = args
                .iter()
                .copied()
                .map(u64::try_from)
                .collect::<Result<Vec<_>, _>>()
            {
                // next_prime can leave the u64 range. The bounded backend returns None.
                let generic = evaluate(&small, function);
                if function != "next_prime" || generic.is_some() {
                    assert_eq!(generic, result, "{family}:{}", case.line);
                }
                match function {
                    "is_prime" => {
                        assert_eq!(small[0].is_prime(), math_prime_util::is_prime(small[0]))
                    }
                    "factor" => assert_eq!(
                        small[0].factor().unwrap(),
                        math_prime_util::factor(small[0])
                    ),
                    "powmod" => assert_eq!(
                        small[0].powmod(&small[1], &small[2]),
                        math_prime_util::powmod(small[0], small[1], small[2])
                    ),
                    "gcd" => assert_eq!(
                        small.iter().fold(0, |g, n| g.gcd(n)),
                        small.iter().fold(0, |g, n| math_prime_util::gcd(g, *n))
                    ),
                    "next_prime" => {
                        assert_eq!(small[0].next_prime(), math_prime_util::next_prime(small[0]))
                    }
                    _ => unreachable!(),
                }
            }
            count += 1;
        }
    }
    assert!(count > 0);
}

fn evaluate<T: Int + From<u8> + std::fmt::Display + std::fmt::Debug>(
    args: &[T],
    function: &str,
) -> Option<String> {
    match function {
        "is_prime" => Some(args[0].is_prime().to_string()),
        "factor" => {
            let factors = args[0].factor().unwrap();
            (!factors.is_empty()).then(|| {
                factors
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" ")
            })
        }
        "powmod" => args[0].powmod(&args[1], &args[2]).map(|n| n.to_string()),
        "gcd" => Some(args.iter().fold(T::from(0), |g, n| g.gcd(n)).to_string()),
        "next_prime" => args[0].next_prime().map(|n| n.to_string()),
        _ => unreachable!(),
    }
}

#[test]
fn compat_primality_is_prime() {
    replay("is_prime");
}
#[test]
fn compat_primality_factor() {
    replay("factor");
}
#[test]
fn compat_primality_powmod() {
    replay("powmod");
}
#[test]
fn compat_primality_gcd() {
    replay("gcd");
}
#[test]
fn compat_primality_next_prime() {
    replay("next_prime");
}

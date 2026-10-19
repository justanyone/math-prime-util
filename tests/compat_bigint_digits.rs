//! Replay the MPU 0.75 pure-Perl bigint digit oracle.
#![cfg(feature = "bigint")]
mod common;
use math_prime_util::bigint::digits;
use num_bigint::BigUint;

#[test]
fn bigint_digits_oracle() {
    for c in common::load("bigint_digits") {
        let run = || -> Option<String> {
            let n = || c.arg::<BigUint>(0);
            let base = || c.arg::<u64>(1);
            let len = || c.args.get(2).map(|s| s.parse().unwrap());
            Some(match c.func.as_str() {
                "digital_root" => digits::digital_root(&n(), base()).to_string(),
                "is_harshad" => u8::from(digits::is_harshad(&n(), base())).to_string(),
                "is_palindrome" => u8::from(digits::is_palindrome(&n(), base())).to_string(),
                "mult_digital_root" => digits::mult_digital_root(&n(), base()).to_string(),
                "reverse_digits" => digits::reverse_digits(&n(), base())?.to_string(),
                "todigits" => digits::todigits(&n(), base(), len())
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" "),
                "todigitstring" => digits::todigitstring(&n(), base(), len()),
                "fromdigits" => digits::fromdigits(
                    &c.args[1..]
                        .iter()
                        .map(|s| s.parse().unwrap())
                        .collect::<Vec<_>>(),
                    c.arg(0),
                )?
                .to_string(),
                "is_happy" => digits::is_happy(&n(), c.arg(1), c.arg(2)).to_string(),
                "tozeckendorf" => digits::tozeckendorf(&n()),
                "fromzeckendorf" => digits::fromzeckendorf(&c.args[0])?.to_string(),
                "toint" => digits::toint(&c.args[0])?.to_string(),
                other => panic!("unknown fixture function: {other}"),
            })
        };
        if c.error {
            assert!(std::panic::catch_unwind(run).is_err(), "line {}", c.line);
        } else {
            c.check(run());
        }
    }
}

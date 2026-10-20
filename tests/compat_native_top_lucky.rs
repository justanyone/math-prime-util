//! The native lucky, Ramanujan and twin-prime forms at the top of the `u64`
//! range, against MPU 0.75 (XS). These names have no `BigUint` form (`n.a.` in
//! docs/bigint-coverage.md); where MPU died, the native form returns `None`.
mod common;
use math_prime_util::{is_lucky, is_ramanujan_prime, nth_ramanujan_prime, nth_twin_prime};

#[test]
fn native_top_lucky_oracle() {
    let rows = common::load("native_top_lucky");
    assert_eq!(rows.len(), 10, "the oracle must cover every input");
    for c in rows {
        let n: u64 = c.arg(0);
        let got = match c.func.as_str() {
            "is_lucky" => Some(u8::from(is_lucky(n))),
            "is_ramanujan_prime" => Some(u8::from(is_ramanujan_prime(n))),
            "nth_twin_prime" => {
                assert!(c.error, "line {}: MPU died on this index", c.line);
                assert_eq!(nth_twin_prime(n), None, "nth_twin_prime({n})");
                continue;
            }
            "nth_ramanujan_prime" => {
                assert!(c.error, "line {}: MPU died on this index", c.line);
                assert_eq!(nth_ramanujan_prime(n), None, "nth_ramanujan_prime({n})");
                continue;
            }
            _ => panic!("unknown row {}", c.func),
        };
        c.check(got);
    }
}

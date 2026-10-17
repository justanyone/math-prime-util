//! ECM compatibility with the committed MPU oracle.
mod common;

use math_prime_util::ecm_factor;

#[test]
fn ecm_factor_oracle() {
    common::each("factor", "ecm_factor", |c| {
        let factors = ecm_factor(
            c.arg(0),
            c.args.get(1).map(|_| c.arg(1)),
            c.args.get(2).map(|_| c.arg(2)),
            c.args.get(3).map(|_| c.arg(3)),
        );
        let result = if factors.is_empty() {
            None
        } else {
            Some(
                factors
                    .iter()
                    .map(u64::to_string)
                    .collect::<Vec<_>>()
                    .join(" "),
            )
        };
        c.check(result);
    });
}

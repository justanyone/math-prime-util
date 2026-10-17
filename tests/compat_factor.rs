//! Compatibility tests for the four factor predicates from MPU 0.75.
mod common;

// The u64 API skips signed arguments (including -0) and the bigint cases
// 18450931744652885087, 36803912137803496523, and 36832540935240707603.
use common::each;
use math_prime_util::{
    factor, factor_exp, is_carmichael, is_cyclic, is_perfect_number, is_quasi_carmichael,
};

fn u64_arg(c: &common::Case) -> Option<u64> {
    c.args[0].parse().ok()
}

#[test]
fn factor_oracle() {
    each("factor", "factor", |c| {
        let Some(n) = u64_arg(c) else {
            return;
        };
        let got = factor(n);
        if got.is_empty() {
            c.check(None::<u64>);
        } else {
            let text = got
                .iter()
                .map(u64::to_string)
                .collect::<Vec<_>>()
                .join(" ");
            c.check(Some(text));
        }
    });
}

#[test]
fn factor_exp_oracle() {
    each("factor", "factor_exp", |c| {
        let Some(n) = u64_arg(c) else {
            return;
        };
        let got = factor_exp(n);
        if got.is_empty() {
            c.check(None::<u64>);
        } else {
            let text = got
                .iter()
                .map(|(prime, exponent)| format!("{prime}^{exponent}"))
                .collect::<Vec<_>>()
                .join(" ");
            c.check(Some(text));
        }
    });
}

#[test]
fn is_carmichael_oracle() {
    each("factor", "is_carmichael", |c| {
        if let Ok(n) = c.args[0].parse::<u64>() {
            c.check(Some(u8::from(is_carmichael(n))));
        }
    });
}

#[test]
fn is_quasi_carmichael_oracle() {
    each("factor", "is_quasi_carmichael", |c| {
        if let Ok(n) = c.args[0].parse::<u64>() {
            c.check(Some(is_quasi_carmichael(n)));
        }
    });
}

#[test]
fn is_cyclic_oracle() {
    each("factor", "is_cyclic", |c| {
        if let Ok(n) = c.args[0].parse::<u64>() {
            c.check(Some(u8::from(is_cyclic(n))));
        }
    });
}

#[test]
fn is_perfect_number_oracle() {
    each("factor", "is_perfect_number", |c| {
        if let Ok(n) = c.args[0].parse::<u64>() {
            c.check(Some(u8::from(is_perfect_number(n))));
        }
    });
}

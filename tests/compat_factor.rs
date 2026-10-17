//! Compat tests against MPU for `rough_count` and `squarefree_kernel`
//! (`factor.c`, `powerfree.c`). Fixture: `tests/fixtures/factor.tsv`.

mod common;

use common::each;
use math_prime_util::*;

#[test]
fn rough_count_oracle() {
    each("factor", "rough_count", |c| {
        c.check(Some(rough_count(c.arg(0), c.arg(1))));
    });
}

#[test]
fn squarefree_kernel_oracle() {
    each("factor", "squarefree_kernel", |c| {
        c.check(Some(squarefree_kernel(c.arg(0))));
    });
}

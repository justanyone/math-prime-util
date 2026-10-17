//! Property tests: algebraic laws for `cornacchia` and `divmod`.

use math_prime_util::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn cornacchia_solution_satisfies_equation(d in 1u64..1000, p in 0u64..100_000) {
        if let Some((x, y)) = cornacchia(d, p) {
            let lhs = u128::from(x) * u128::from(x) + u128::from(d) * u128::from(y) * u128::from(y);
            prop_assert_eq!(lhs, u128::from(p));
        }
    }

    #[test]
    fn divmod_undoes_mulmod(a in 0u64..1_000_000, b in 1u64..1_000_000, n in 2u64..1_000_000) {
        if let Some(q) = divmod(a, b, n) {
            if gcd(b, n) == 1 {
                prop_assert_eq!(mulmod(q, b, n), Some(a % n));
            }
        }
    }

    #[test]
    fn divmod_by_one_is_reduction(a in any::<u64>(), n in 1u64..u64::MAX) {
        prop_assert_eq!(divmod(a, 1, n), Some(a % n));
    }
}

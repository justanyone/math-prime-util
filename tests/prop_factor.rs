//! Algebraic laws for `rough_count` and `squarefree_kernel`.

use math_prime_util::*;

#[test]
fn rough_count_is_bounded_by_n() {
    for n in 0..200 {
        for k in 0..12 {
            assert!(rough_count(n, k) <= n, "n={n} k={k}");
        }
    }
}

#[test]
fn rough_count_is_monotone_in_n() {
    for k in 0..10 {
        for n in 0..200 {
            assert!(rough_count(n, k) <= rough_count(n + 1, k), "n={n} k={k}");
        }
    }
}

#[test]
fn rough_count_grows_as_k_shrinks() {
    // Fewer excluded primes never lowers the count.
    for n in 0..150 {
        for k in 3..12 {
            assert!(rough_count(n, k) <= rough_count(n, k - 1), "n={n} k={k}");
        }
    }
}

#[test]
fn squarefree_kernel_divides_n() {
    for n in 1..2000 {
        assert_eq!(n % squarefree_kernel(n), 0, "n={n}");
    }
}

#[test]
fn squarefree_kernel_is_idempotent() {
    for n in 0..2000 {
        let s = squarefree_kernel(n);
        assert_eq!(squarefree_kernel(s), s, "n={n}");
    }
}

#[test]
fn squarefree_kernel_is_multiplicative_on_coprimes() {
    // gcd(a, b) = 1 gives kernel(a * b) = kernel(a) * kernel(b).
    for a in 1..60u64 {
        for b in 1..60u64 {
            if gcd(a, b) == 1 {
                assert_eq!(
                    squarefree_kernel(a * b),
                    squarefree_kernel(a) * squarefree_kernel(b),
                    "a={a} b={b}"
                );
            }
        }
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

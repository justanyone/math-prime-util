//! Mutation-killing tests for `squfof_factor`.
//!
//! SQUFOF may fail or find a different factor, so each check only demands a
//! non-trivial divisor, and exact pairs only where the input is a semiprime.
use math_prime_util::{is_prime, squfof_factor};

fn next_prime(n: u64) -> u64 {
    math_prime_util::next_prime(n).expect("next prime fits in u64")
}

fn semiprime_cases() -> Vec<(u64, u64)> {
    let mut cases = Vec::new();
    for (lo, hi_start) in [
        (101u64, 103u64),
        (1_009, 10_007),
        (10_007, 65_537),
        (65_537, 1_000_003),
        (1_000_003, 1_000_033),
        (100_003, 3_000_017),
        (999_983, 2_147_483_659),
        (1_000_003, 1_000_000_007),
        (2_147_483_647, 2_147_483_659),
    ] {
        let p = next_prime(lo);
        let mut q = next_prime(hi_start.max(p));
        for _ in 0..4 {
            if q != p && p.checked_mul(q).is_some_and(|n| n >> 2 <= u64::MAX >> 4) {
                cases.push((p, q));
            }
            q = next_prime(q);
        }
    }
    cases
}

#[test]
fn semiprimes_split_exactly() {
    let cases = semiprime_cases();
    assert!(cases.len() > 20);
    let mut ok = 0;
    for (p, q) in cases {
        let n = p * q;
        let got = squfof_factor(n, None);
        if got.len() == 2 {
            assert_eq!(got, vec![p.min(q), p.max(q)], "n={n}");
            ok += 1;
        } else {
            assert_eq!(got, vec![n], "n={n}");
        }
    }
    assert!(ok >= 20, "only {ok} split");
}

#[test]
fn composites_give_divisor() {
    let mut ok = 0;
    for n in (10_001u64..60_000).step_by(37).chain([
        1_000_036_000_099,
        4_611_686_014_132_420_609,
        4_611_685_468_671_474_689,
        9_223_371_873_002_223_329,
        3_037_000_493 * 3_037_000_453,
        999_999_000_001 * 7,
        600_851_475_143,
        1_522_605_027_922_533_360,
        3_215_031_751,
    ]) {
        let got = squfof_factor(n, None);
        assert_eq!(got.iter().product::<u64>(), n, "n={n}");
        if is_prime(n) == 0 && got.len() > 1 {
            ok += 1;
            assert!(got.iter().all(|&f| f > 1 && n % f == 0));
        }
    }
    assert!(ok > 100);
}

#[test]
fn perfect_squares_and_primes() {
    for r in [7u64, 101, 65_537, 1_000_003, 2_147_483_647, 3_037_000_493] {
        let n = r * r;
        let got = squfof_factor(n, None);
        assert_eq!(got.iter().product::<u64>(), n);
        assert!(got.iter().all(|&f| f == r || f > 1 && n % f == 0));
    }
    for p in [1_000_000_007u64, 2_147_483_647, 4_611_686_018_427_387_847] {
        assert_eq!(squfof_factor(p, None), vec![p]);
    }
}

#[test]
fn above_62_bits_is_kept() {
    let n = (1u64 << 62) + 1;
    assert_eq!(squfof_factor(n, None).iter().product::<u64>(), n);
    let n = u64::MAX - 4;
    assert_eq!(squfof_factor(n, None).iter().product::<u64>(), n);
}

#[test]
fn round_limits_are_deterministic() {
    // Smallest round budget that splits each semiprime; pins the accounting.
    let mut sum = 0u64;
    for (p, q) in [(1_000_003u64, 1_000_033u64), (65_537, 999_983), (10_007, 10_009)] {
        let n = p * q;
        let mut r = 0;
        while squfof_factor(n, Some(r)) == vec![n] {
            r += 1;
            assert!(r < 100_000);
        }
        assert_eq!(squfof_factor(n, Some(r + 5000)), vec![p, q]);
        sum += r;
    }
    assert_eq!(sum, 1450);
}

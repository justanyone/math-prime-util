//! Algebraic laws for multiplicative arithmetic.
//! Arithmetic identities and boundary regressions.
use math_prime_util::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn divisor_identities(n in 1_u64..5000) {
        let ds: Vec<_> = (1..=n).filter(|d|n%d==0).collect();
        prop_assert_eq!(ds.iter().map(|&d|euler_phi(d)).sum::<u64>(),n);
        prop_assert_eq!(ds.iter().map(|&d|i64::from(moebius(d))).sum::<i64>(),i64::from(n==1));
        prop_assert_eq!(jordan_totient(1,n),Some(euler_phi(n)));
        prop_assert_eq!(euler_phi(n)%carmichael_lambda(n),0);
        prop_assert_eq!(ds.iter().map(|&d|i64::from(liouville(d))).sum::<i64>(),i64::from(is_square(n)));
    }
    #[test]
    fn multiplicative(a in 1_u64..1000,b in 1_u64..1000) {
        if gcd(a,b)==1 {
            prop_assert_eq!(euler_phi(a*b),euler_phi(a)*euler_phi(b));
            prop_assert_eq!(moebius(a*b),moebius(a)*moebius(b));
            prop_assert_eq!(dedekind_psi(a*b),Some(dedekind_psi(a).unwrap()*dedekind_psi(b).unwrap()));
        }
        prop_assert_eq!(liouville(a*b),liouville(a)*liouville(b));
    }
}
proptest! {
    #[test]
    fn divisor_identities_wide(n in 1_u64..10000) {
        let s = divisor_sum(n, 1).unwrap();
        prop_assert_eq!(aliquot_sum(n), Some(s - n));
        prop_assert_eq!(abundance(n), Some(i64::try_from(s).unwrap() - 2 * i64::try_from(n).unwrap()));
        prop_assert!(sopf(n) <= sopfr(n));
    }
    #[test]
    fn crt_reconstructs(a in 0_u64..10000, b in 0_u64..10000) {
        let (r, m) = chinese2(&[(a, 64), (b, 81)]).unwrap();
        prop_assert_eq!(m, 5184);
        prop_assert_eq!(r % 64, a % 64);
        prop_assert_eq!(r % 81, b % 81);
        prop_assert_eq!(chinese2(&[(b, 81), (a, 64)]), Some((r,m)));
    }
    #[test]
    fn squares_are_residues(a in 0_u64..10000, n in 1_u64..10000) {
        prop_assert!(is_qr(a*a, n));
    }
    #[test]
    fn order_annihilates(a in 0_u64..1000, n in 2_u64..1000) {
        if let Some(k) = znorder(a,n) { prop_assert_eq!(powmod(a,k,n), Some(1)); }
    }
    #[test]
    fn kronecker_multiplicative(a in -1000_i64..1000, b in -1000_i64..1000, n in -1000_i64..1000) {
        prop_assert_eq!(kronecker(a*b,n), kronecker(a,n)*kronecker(b,n));
    }
}
#[test]
fn boundaries_and_repeated_calls() {
    assert_eq!(divisor_sum(2, u64::MAX), None);
    assert_eq!(divisor_sum(0, u64::MAX), Some(0));
    assert_eq!(ramanujan_tau(u64::MAX), None);
    assert_eq!(kronecker(i64::MIN, i64::MIN), 0);
    assert_eq!(kronecker(-1, 0), 1);
    assert!(!is_fundamental(i64::MIN));
    assert_eq!(chinese2(&[]), Some((0, 0)));
    assert_eq!(chinese2(&[(0, 0)]), None);
    assert_eq!(chinese2(&[(0, 2), (1, 2)]), None);
    assert_eq!(chinese2(&[(0, u64::MAX), (0, 2)]), None);
    for _ in 0..2 {
        assert_eq!(chinese2(&[(5, 12), (17, 18), (5, 12)]), Some((17, 36)));
    }
    assert_eq!(znorder(1, 0), None);
    assert_eq!(znorder(1, 1), Some(1));
    for n in 1..200 {
        if let Some(a) = znprimroot(n) {
            assert_eq!(is_primitive_root(a, n), Some(true));
        }
    }
}

proptest! {
    #[test]
    fn tau_multiplicative(a in 1_u64..40, b in 1_u64..40) {
        if gcd(a,b) == 1 {
            prop_assert_eq!(ramanujan_tau(a*b), Some(ramanujan_tau(a).unwrap()*ramanujan_tau(b).unwrap()));
        }
    }
}

#[test]
fn range_and_prefix_agree() {
    for (lo, hi) in [
        (0, 0),
        (0, 1),
        (0, 2),
        (0, 15),
        (0, 500),
        (1, 500),
        (100, 500),
        (65530, 65600),
    ] {
        assert_eq!(
            euler_phi_range(lo, hi),
            (lo..=hi).map(euler_phi).collect::<Vec<_>>()
        );
        assert_eq!(
            moebius_range(lo, hi),
            (lo..=hi).map(moebius).collect::<Vec<_>>()
        );
    }
    let (mut m, mut l, mut t) = (0, 0, 0);
    for n in 1..200 {
        m += i64::from(moebius(n));
        l += i64::from(liouville(n));
        t += euler_phi(n);
        assert_eq!(mertens(n), m);
        assert_eq!(sumliouville(n), l);
        assert_eq!(sumtotient(n), Some(t));
        for v in inverse_totient(n) {
            assert_eq!(euler_phi(v), n);
        }
        assert_eq!(is_totient(n), !inverse_totient(n).is_empty());
    }
}

#[test]
fn repeated_and_concurrent_calls() {
    let threads: Vec<_> = (0..4)
        .map(|_| {
            std::thread::spawn(|| {
                for _ in 0..2 {
                    assert_eq!(mertens(10000), -23);
                    assert_eq!(sumtotient(100), Some(3044));
                }
            })
        })
        .collect();
    for t in threads {
        t.join().unwrap();
    }
}

#[test]
#[should_panic(expected = "invalid or oversized range")]
fn reversed_phi_range() {
    euler_phi_range(5, 4);
}
#[test]
#[should_panic(expected = "invalid or oversized range")]
fn reversed_mu_range() {
    moebius_range(5, 4);
}
#[test]
#[should_panic(expected = "invalid or oversized range")]
fn oversized_range() {
    moebius_range(0, u64::MAX);
}

#[test]
fn nonresidues_and_parallel_calls() {
    for n in 3..300 {
        let a = qnr(n).unwrap();
        assert!(!is_qr(a, n), "qnr({n}) = {a}");
    }
    std::thread::scope(|scope| {
        for _ in 0..4 {
            scope.spawn(|| {
                assert_eq!(divisor_sum(360_360, 1), Some(1_572_480));
                assert_eq!(chinese2(&[(2, 3), (3, 5)]), Some((8, 15)));
                assert_eq!(ramanujan_tau(1000), Some(-30_328_412_970_240_000));
            });
        }
    });
}

#[test]
fn jordan_totient_power_of_two_boundary() {
    // Regression: 2^64 overflowed u64 before the subtraction; the result u64::MAX is representable.
    assert_eq!(jordan_totient(64, 2), Some(u64::MAX));
    assert_eq!(jordan_totient(63, 2), Some(u64::MAX >> 1));
    assert_eq!(jordan_totient(128, 2), None);
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]
    #[test]
    fn inverse_sigma0_matches_divisor_count(k in 1_u64..40, lo in 0_u64..2000, width in 0_u64..2000) {
        let hi = lo + width;
        let want: Vec<u64> = (lo.max(1)..=hi)
            .filter(|&n| divisor_sum(n, 0) == Some(k))
            .collect();
        prop_assert_eq!(inverse_sigma0(k, lo, hi), Some(want.clone()));
        prop_assert_eq!(inverse_sigma0_count(k, lo, hi), Some(want.len() as u64));
    }
}

proptest! {
    #[test]
    fn prime_factor_sums_add(a in 1_u64..10000, b in 1_u64..10000) {
        prop_assert_eq!(sopfr(a * b), sopfr(a) + sopfr(b));
        if gcd(a, b) == 1 {
            prop_assert_eq!(sopf(a * b), sopf(a) + sopf(b));
        }
        prop_assert_eq!(sopf(a * a), sopf(a));
    }

    #[test]
    fn chinese_general_congruences(a in any::<u64>(), m in 1_u64..500, n in 1_u64..500) {
        let pairs = [(a, m), (a, n)];
        let result = chinese(&pairs).unwrap();
        prop_assert_eq!(result % m, a % m);
        prop_assert_eq!(result % n, a % n);
        prop_assert!(result < m / gcd(m, n) * n);
        prop_assert_eq!(chinese(&[(a, n), (a, m)]), Some(result));
        prop_assert_eq!(chinese(&[(a, m), (a, n), (a, m)]), Some(result));
    }

    #[test]
    fn kronecker_denominator_product(a in -1000_i64..1000, m in -1000_i64..1000, n in -1000_i64..1000) {
        prop_assert_eq!(kronecker(a, m * n), kronecker(a, m) * kronecker(a, n));
    }
}

#[test]
fn divisor_sums_native_edges() {
    for (n, repeated, distinct) in [
        (0, 0, 0),
        (1, 0, 0),
        (2, 2, 2),
        (3, 3, 3),
        (4, 4, 2),
        (5, 5, 5),
        (64, 12, 2),
    ] {
        assert_eq!(sopfr(n), repeated);
        assert_eq!(sopf(n), distinct);
    }
    assert_eq!(sopfr(1 << 63), 126);
    assert_eq!(sopf(1 << 63), 2);
    assert_eq!(
        sopfr(18_446_744_073_709_551_557),
        18_446_744_073_709_551_557
    );
    assert_eq!(sopf(18_446_744_073_709_551_557), 18_446_744_073_709_551_557);
}

#[test]
fn chinese_native_fallbacks_and_validation() {
    assert_eq!(chinese(&[]), Some(0));
    assert_eq!(chinese(&[(8, 1)]), Some(0));
    assert_eq!(chinese(&[(0, 0)]), None);
    assert_eq!(chinese(&[(0, 2), (1, 2)]), None);
    // Wide, coprime moduli use util.c::_simple_chinese.
    let m = (1_u64 << 62) + 1;
    assert_eq!(chinese(&[(17, m), (17, 2)]), Some(17));
    assert_eq!(chinese(&[(19, u64::MAX)]), Some(19));
    assert_eq!(chinese(&[(0, u64::MAX), (0, 2)]), None);
    // The C fallback refuses noncoprime moduli even when a solution exists.
    assert_eq!(chinese(&[(1, 1_u64 << 63), (1, 2)]), None);
    assert_eq!(chinese(&[(1, u64::MAX), (0, 1)]), None);
}

#[test]
fn kronecker_signed_boundaries_and_concurrent_calls() {
    for (a, b, expected) in [
        (0, 0, 0),
        (1, 0, 1),
        (-1, 0, 1),
        (2, 0, 0),
        (i64::MIN, -1, -1),
        (i64::MIN, 1, 1),
        (i64::MIN, i64::MIN, 0),
        (1, i64::MIN, 1),
        (-1, i64::MIN, -1),
        (3, 2, -1),
        (-3, 2, -1),
    ] {
        assert_eq!(kronecker(a, b), expected);
    }
    let threads: Vec<_> = (0..4)
        .map(|_| {
            std::thread::spawn(|| {
                for _ in 0..3 {
                    assert_eq!(sopfr(360), 17);
                    assert_eq!(sopf(360), 10);
                    let inputs = [(5, 12), (17, 18), (5, 12)];
                    assert_eq!(chinese(&inputs), Some(17));
                    assert_eq!(inputs, [(5, 12), (17, 18), (5, 12)]);
                    assert_eq!(kronecker(-7, 19), -1);
                }
            })
        })
        .collect();
    for thread in threads {
        thread.join().unwrap();
    }
}

proptest! {
    #[test]
    fn valuation_is_exact_power(n in 1_u64..50_000, k in 2_u64..300) {
        let v = valuation(n, k).unwrap();
        let mut rest = n;
        for _ in 0..v {
            prop_assert_eq!(rest % k, 0);
            rest /= k;
        }
        prop_assert!(rest % k != 0);
        // A second call uses no saved state.
        prop_assert_eq!(valuation(n, k), Some(v));
        if k == 2 {
            prop_assert_eq!(v, u64::from(n.trailing_zeros()));
        }
    }

    #[test]
    fn valuation_adds_over_products(a in 1_u64..2_000, b in 1_u64..2_000, k in 2_u64..40) {
        let prod = a * b;
        let va = valuation(a, k).unwrap();
        let vb = valuation(b, k).unwrap();
        prop_assert_eq!(valuation(prod, k), Some(va + vb));
    }

    #[test]
    fn ramanujan_sum_depends_on_gcd(k in 1_u64..400, n in 1_u64..400) {
        if gcd(k, n) == 1 {
            prop_assert_eq!(ramanujan_sum(k, n), i64::from(moebius(k)));
        }
        prop_assert_eq!(ramanujan_sum(k, n), ramanujan_sum(k, gcd(k, n)));
        let sum = (1..=k)
            .filter(|d| k % d == 0)
            .map(|d| ramanujan_sum(d, n))
            .sum::<i64>();
        let expect = if n % k == 0 { i64::try_from(k).unwrap() } else { 0 };
        prop_assert_eq!(sum, expect);
    }

    #[test]
    fn ramanujan_sum_multiplicative(a in 1_u64..60, b in 1_u64..60, n in 1_u64..120) {
        if gcd(a, b) == 1 {
            prop_assert_eq!(
                ramanujan_sum(a * b, n),
                ramanujan_sum(a, n) * ramanujan_sum(b, n)
            );
        }
    }
}

#[test]
fn valuation_rejects_bad_base_and_stays_pure() {
    for k in [0, 1] {
        assert!(std::panic::catch_unwind(|| valuation(12, k)).is_err());
    }
    // A rejected call must not change a later call.
    assert_eq!(valuation(12, 2), Some(2));
    assert_eq!(valuation(12, 2), Some(2));
    assert_eq!(valuation(0, 9), None);
    assert_eq!(valuation(1, 9), Some(0));
    assert_eq!(valuation(1_u64 << 63, 2), Some(63));
    assert_eq!(valuation(1_u64 << 62, 4), Some(31));
    assert_eq!(valuation(u64::MAX, u64::MAX), Some(1));
    assert_eq!(valuation(1000, 10), Some(3));
    assert_eq!(valuation(96552, 6), Some(3));
}

#[test]
fn ramanujan_sum_edges_and_repeat() {
    assert_eq!(ramanujan_sum(0, 0), 0);
    assert_eq!(ramanujan_sum(1, 1), 1);
    assert_eq!(ramanujan_sum(12, 36), 4);
    assert_eq!(ramanujan_sum(12, 36), ramanujan_sum(12, 36));
    // gcd(k, n) > 1 and moebius(g) == 0.
    assert_eq!(ramanujan_sum(12, 2), 0);
    let p = next_prime(1_u64 << 63).unwrap();
    assert_ne!(is_prime(p), 0);
    assert_eq!(
        ramanujan_sum(p, p),
        i64::from_ne_bytes((p - 1).to_ne_bytes())
    );
}

#[test]
fn valuation_and_ramanujan_sum_concurrent() {
    let threads: Vec<_> = (1_u64..=4)
        .map(|n| {
            std::thread::spawn(move || {
                for _ in 0..3 {
                    assert_eq!(valuation(72, 3), Some(2));
                    assert_eq!(valuation(1_u64 << 40, 2), Some(40));
                    assert_eq!(ramanujan_sum(12, 36), 4);
                    assert_eq!(ramanujan_sum(n, 1), i64::from(moebius(n)));
                }
            })
        })
        .collect();
    for thread in threads {
        thread.join().unwrap();
    }
}

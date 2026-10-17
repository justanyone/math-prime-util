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

#[test]
fn tau_prime_does_not_require_the_prime_power_term() {
    // From the q-series recurrence for Delta, independent of the class-number path.
    assert_eq!(ramanujan_tau(3011), Some(-6_602_349_165_399_272_388));
}

proptest! {
    #[test]
    fn divisor_power_sums_match_enumeration(n in 0_u64..300, k in 0_u32..5) {
        let expected = (1..=n).filter(|d| n % d == 0).map(|d| d.pow(k)).sum();
        prop_assert_eq!(divisor_sum(n, u64::from(k)), Some(expected));
    }

    #[test]
    fn divisor_sums_are_multiplicative(a in 1_u64..100, b in 1_u64..100, k in 0_u64..4) {
        if gcd(a, b) == 1 {
            prop_assert_eq!(divisor_sum(a * b, k), Some(divisor_sum(a, k).unwrap() * divisor_sum(b, k).unwrap()));
        }
    }
}

#[test]
fn divisor_sum_native_boundaries_and_prime_powers() {
    assert_eq!(divisor_sum(1, u64::MAX), Some(1));
    assert_eq!(divisor_sum(2, 63), Some((1_u64 << 63) + 1));
    assert_eq!(divisor_sum(2, 64), None);
    assert_eq!(divisor_sum(2, 128), None);
    assert_eq!(divisor_sum(4, 127), None);
    assert_eq!(divisor_sum(6, 63), None);
    assert_eq!(divisor_sum(1_u64 << 63, 0), Some(64));
    assert_eq!(divisor_sum(1_u64 << 63, 1), Some(u64::MAX));
    assert_eq!(aliquot_sum(1_u64 << 63), Some((1_u64 << 63) - 1));
    assert_eq!(aliquot_sum(18_446_744_073_709_551_557), Some(1));
    // Sigma exceeds u64, but the proper divisor sum still fits.
    assert_eq!(aliquot_sum(5_u64 << 61), Some(16_140_901_064_495_857_658));
}

#[test]
fn divisor_sums_have_no_shared_call_state() {
    std::thread::scope(|scope| {
        for _ in 0..4 {
            scope.spawn(|| {
                for _ in 0..2 {
                    assert_eq!(divisor_sum(12, 2), Some(210));
                    assert_eq!(aliquot_sum(27), Some(13));
                    assert_eq!(abundance(28), Some(0));
                    assert_eq!(ramanujan_tau(49), Some(-1696965207));
                }
            });
        }
    });
}

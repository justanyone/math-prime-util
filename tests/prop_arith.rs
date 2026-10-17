//! Algebraic laws for multiplicative arithmetic.
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

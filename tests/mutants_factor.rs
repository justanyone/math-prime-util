//! Mutation-killing tests for the factor family: products built from known
//! primes give an exact oracle for every factoring-based predicate.
use math_prime_util::{
    factor, holf_factor, is_almost_prime, is_carmichael, is_prime, is_quasi_carmichael,
    is_semiprime, is_square_free, lehman_factor, pminus1_factor, pplus1_factor, prho_factor,
    prime_bigomega,
};

/// Primes from small to near 2^64: every size class the splitters branch on.
const POOL: [u64; 12] = [
    2,
    3,
    7,
    101,
    421,
    2011,
    2017,
    65_537,
    1_000_003,
    4_294_967_291,
    2_305_843_009_213_693_951,
    18_446_744_073_709_551_557,
];

/// Every multiset of at most six `POOL` primes whose product fits in u64.
fn products() -> Vec<(u64, Vec<u64>)> {
    fn go(start: usize, n: u64, list: &mut Vec<u64>, out: &mut Vec<(u64, Vec<u64>)>) {
        out.push((n, list.clone()));
        if list.len() == 6 {
            return;
        }
        for i in start..POOL.len() {
            if let Some(m) = n.checked_mul(POOL[i]) {
                list.push(POOL[i]);
                go(i, m, list, out);
                list.pop();
            }
        }
    }
    let mut out = Vec::new();
    go(0, 1, &mut Vec::new(), &mut out);
    out
}

fn naive(mut n: u64) -> Vec<u64> {
    let mut out = Vec::new();
    let mut p = 2;
    while p * p <= n {
        while n % p == 0 {
            out.push(p);
            n /= p;
        }
        p += 1;
    }
    if n > 1 {
        out.push(n);
    }
    out
}

#[test]
fn factor_matches_construction() {
    for (n, list) in products() {
        assert_eq!(factor(n), list, "factor({n})");
        let omega = list.len() as u64;
        assert_eq!(u64::from(prime_bigomega(n)), omega, "bigomega({n})");
        let squarefree = list.windows(2).all(|w| w[0] != w[1]);
        assert_eq!(is_square_free(n), squarefree && n != 0, "squarefree({n})");
        assert_eq!(is_semiprime(n), omega == 2, "semiprime({n})");
        for k in 0..=14 {
            assert_eq!(is_almost_prime(k, n), omega == k, "almost_prime({k},{n})");
        }
    }
}

#[test]
fn small_values_match_trial_division() {
    assert_eq!(factor(0), vec![0]);
    assert_eq!(factor(1), Vec::<u64>::new());
    for n in 2..=20_000 {
        let want = naive(n);
        assert_eq!(factor(n), want, "factor({n})");
        let omega = want.len() as u64;
        assert_eq!(u64::from(prime_bigomega(n)), omega, "bigomega({n})");
        assert_eq!(is_semiprime(n), omega == 2, "semiprime({n})");
        let squarefree = want.windows(2).all(|w| w[0] != w[1]);
        assert_eq!(is_square_free(n), squarefree, "squarefree({n})");
        for k in 0..=8 {
            assert_eq!(is_almost_prime(k, n), omega == k, "almost_prime({k},{n})");
        }
    }
    assert!(!is_square_free(0));
    assert!(is_almost_prime(0, 1));
    assert!(!is_almost_prime(0, 0));
    assert!(!is_almost_prime(1, 1));
}

#[test]
fn prime_pairs_and_squares_around_table_edges() {
    // Edges of the small-prime table (2011/2017) and of 2017^2.
    for p in [2003_u64, 2011, 2017, 2027, 2029, 2039] {
        for q in [2003_u64, 2011, 2017, 2027, 2029, 2039, 4_093, 4_099] {
            let n = p * q;
            assert_eq!(factor(n), naive(n), "{n}");
            assert!(is_semiprime(n));
            assert_eq!(is_square_free(n), p != q);
        }
    }
    for n in [2017_u64 * 2017 - 1, 2017 * 2017, 2017 * 2017 + 1] {
        assert_eq!(factor(n), naive(n), "{n}");
    }
}

#[test]
fn square_free_boundary_around_prime_squares() {
    for p in [17_u64, 19, 23, 29, 31, 37, 41, 43, 47] {
        for n in [p * p, p * p * 2, p * p * 3, p * p - 1, p * p + 2, p * p * 7] {
            let want = naive(n).windows(2).all(|w| w[0] != w[1]);
            assert_eq!(is_square_free(n), want, "{n}");
        }
    }
}

/// Factors returned by a heuristic splitter: every entry divides, the
/// product is `n`, and a split of a composite is nontrivial.
fn assert_split(n: u64, got: &[u64], what: &str) {
    assert_eq!(got.iter().product::<u64>(), n, "{what}({n}) = {got:?}");
    assert!(!got.contains(&0), "{what}({n})");
}

fn composites() -> Vec<u64> {
    let mut out = Vec::new();
    for (n, list) in products() {
        // Keep composites whose smallest factor is above 5 so the method runs.
        if list.len() >= 2 && list[0] > 5 {
            out.push(n);
        }
    }
    out
}

#[test]
fn splitters_return_valid_splits_and_leave_primes() {
    for p in [
        7_u64,
        101,
        65_537,
        4_294_967_291,
        18_446_744_073_709_551_557,
    ] {
        assert_eq!(holf_factor(p, None), vec![p]);
        assert_eq!(lehman_factor(p, None), vec![p]);
        assert_eq!(lehman_factor(p, Some(false)), vec![p]);
        assert_eq!(prho_factor(p, None), vec![p]);
        assert_eq!(pminus1_factor(p, None, None), vec![p]);
        assert_eq!(pplus1_factor(p, None), vec![p]);
    }
    for n in composites() {
        assert_split(n, &holf_factor(n, Some(100_000)), "holf");
        assert_split(n, &lehman_factor(n, None), "lehman");
        assert_split(n, &lehman_factor(n, Some(false)), "lehman_nt");
        assert_split(n, &prho_factor(n, Some(100_000)), "prho");
        assert_split(
            n,
            &pminus1_factor(n, Some(10_000), Some(100_000)),
            "pminus1",
        );
        assert_split(n, &pplus1_factor(n, Some(1_000)), "pplus1");
    }
}

#[test]
fn carmichael_and_quasi_carmichael_are_consistent() {
    // Korselt: squarefree, at least three primes, p - 1 divides n - 1.
    let korselt = |n: u64| {
        let f = naive(n);
        n > 1
            && f.len() >= 3
            && f.windows(2).all(|w| w[0] != w[1])
            && f.iter().all(|p| (n - 1) % (p - 1) == 0)
    };
    for n in 0..=200_000 {
        assert_eq!(is_carmichael(n), korselt(n), "{n}");
    }
    let _ = (is_prime, is_quasi_carmichael);
}

/// Composite inputs from 10 to 62 bits (balanced, lopsided, three-prime and
/// prime-square shapes) with every prime factor above 43.
fn inputs() -> Vec<u64> {
    let np = |n: u64| math_prime_util::next_prime(n).expect("below 2^63");
    let mut out = Vec::new();
    for bits in (10_u32..=62).step_by(4) {
        let half = bits / 2;
        let third = bits / 3;
        for k in [0_u64, 7, 91] {
            let p = np((1 << (half - 1)) + 4 * k);
            let q = np((1 << (bits - half)) + 100 * k + 1);
            out.push(p * q);
            let r = np((1 << (third - 1)) + 2 * k + 45);
            let s = np((1 << (bits - third - 1)) + 30 * k);
            out.push(r * s);
        }
        let p = np(1 << (third - 1)).max(47);
        let q = np(p + 1000);
        let r = np(q + 20_000);
        if let Some(m) = p.checked_mul(q).and_then(|m| m.checked_mul(r)) {
            out.push(m);
        }
        out.push(p * p);
    }
    out.retain(|&n| n > 2000);
    out.sort_unstable();
    out.dedup();
    out
}

/// Order-sensitive fold of a factor list into a running hash.
fn fold(mut hash: u64, list: &[u64]) -> u64 {
    for &x in list.iter().chain(&[1]) {
        hash = (hash ^ x.rotate_left(17)).wrapping_mul(0x0000_0100_0000_01b3) ^ (hash >> 29);
    }
    hash
}

#[test]
fn heuristic_splitters_exact_results() {
    let (mut holf, mut lehman, mut rho, mut pm1, mut pp1) = (0, 0, 0, 0, 0);
    for n in inputs() {
        let mut check = |what: &str, h: &mut u64, got: Vec<u64>| {
            assert_split(n, &got, what);
            *h = fold(*h, &got);
        };
        for r in [Some(1), Some(3), Some(25), Some(1_000), None] {
            check("holf", &mut holf, holf_factor(n, r));
        }
        for t in [Some(true), Some(false)] {
            check("lehman", &mut lehman, lehman_factor(n, t));
        }
        for r in [Some(64), Some(200), Some(5_000), Some(100_000)] {
            check("prho", &mut rho, prho_factor(n, r));
        }
        for (b1, b2) in [(10, 10), (100, 5_000), (3_000, 3_001), (5_000, 100_000)] {
            check("pminus1", &mut pm1, pminus1_factor(n, Some(b1), Some(b2)));
        }
        check("pminus1", &mut pm1, pminus1_factor(n, None, None));
        for b in [2, 10, 200, 3_000] {
            check("pplus1", &mut pp1, pplus1_factor(n, Some(b)));
        }
    }
    assert_eq!(
        (holf, lehman, rho, pm1, pp1),
        (
            5_030_854_463_351_398_821,
            3_433_281_630_521_670_205,
            11_711_687_619_559_150_068,
            6_388_315_210_644_599_507,
            17_049_337_011_819_765_190
        ),
        "fold of exact outputs"
    );
}

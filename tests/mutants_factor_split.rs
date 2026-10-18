//! Mutation-killing tests for the split family (`factor`, `holf_factor`,
//! `is_almost_prime`, `is_semiprime`, `is_square_free`): exact oracles from
//! products of known primes placed at the edges of the small-prime table
//! (2011/2017) and of the cube-root bound.
use math_prime_util::{
    factor, holf_factor, is_almost_prime, is_prime, is_semiprime, is_square_free, next_prime,
};

fn primes_from(start: u64, count: usize) -> Vec<u64> {
    let mut out = Vec::new();
    let mut p = start;
    while out.len() < count {
        p = next_prime(p).expect("below 2^64");
        out.push(p);
    }
    out
}

/// Primes around 2017 and around several larger cube-root bounds.
fn pool() -> Vec<u64> {
    let mut v = primes_from(1990, 14);
    v.extend(primes_from(4_000, 3));
    v.extend(primes_from(65_000, 3));
    v.extend(primes_from(1_000_000, 3));
    v.extend(primes_from(2_000_000, 2));
    v.extend(primes_from(4_000_000, 2));
    v.extend(primes_from(65_000_000, 1));
    v
}

#[test]
fn semiprimes_and_non_semiprimes_from_known_primes() {
    let pool = pool();
    for (i, &p) in pool.iter().enumerate() {
        for &q in &pool[i..] {
            let Some(n) = p.checked_mul(q) else { continue };
            assert!(is_semiprime(n), "semiprime({p}*{q})");
            assert_eq!(is_square_free(n), p != q, "squarefree({p}*{q})");
            assert_eq!(factor(n), if p == q { vec![p, p] } else { vec![p, q] });
            assert!(is_almost_prime(2, n));
            assert!(!is_almost_prime(3, n) && !is_almost_prime(1, n));
            for &r in &pool {
                let Some(m) = n.checked_mul(r) else { continue };
                assert!(!is_semiprime(m), "semiprime({p}*{q}*{r})");
                assert!(is_almost_prime(3, m), "almost3({p}*{q}*{r})");
                assert!(!is_almost_prime(2, m) && !is_almost_prime(4, m));
                let mut want = vec![p, q, r];
                want.sort_unstable();
                assert_eq!(factor(m), want);
                let dup = p == q || q == r || p == r;
                assert_eq!(is_square_free(m), !dup, "squarefree({p}*{q}*{r})");
            }
        }
    }
}

#[test]
fn prime_squares_and_cubes() {
    let mut ps = primes_from(1990, 40);
    ps.extend(primes_from(65_000, 2));
    ps.extend([4_294_967_291, 4_294_967_279, 2_147_483_647, 1_000_003]);
    for p in ps {
        let n = p * p;
        assert!(is_semiprime(n), "{p}^2");
        assert_eq!(factor(n), vec![p, p]);
        assert!(!is_square_free(n));
        assert!(is_prime(p) != 0);
        if let Some(c) = n.checked_mul(p) {
            assert!(!is_semiprime(c), "{p}^3");
            assert_eq!(factor(c), vec![p, p, p]);
            assert!(is_almost_prime(3, c));
        }
        // Squares of semiprimes are not semiprimes.
        if let Some(sq) = (p * 3).checked_mul(p * 3) {
            assert!(!is_semiprime(sq));
        }
    }
}

#[test]
fn square_free_beyond_the_table() {
    // 37^2 < n: the trial loop must keep testing past the early return.
    for p in [41_u64, 43, 47, 53, 101, 2003, 2017, 65_537] {
        for m in [1_u64, 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 1_009] {
            let n = p * p * m;
            assert!(!is_square_free(n), "{n}");
            let n = p * m * 1_000_003;
            assert_eq!(is_square_free(n), m != p, "{n}");
        }
    }
    // Small n at and just above 37^2 and below the next square.
    for n in 256..2_000_u64 {
        let f = factor(n);
        assert_eq!(is_square_free(n), f.windows(2).all(|w| w[0] != w[1]), "{n}");
    }
    assert!(is_square_free(1_369 + 4));
}

#[test]
fn almost_prime_big_k_path() {
    // k >= 5 takes the small-prime stripping loop; mix factors above and
    // below the 11..=2011 range and leave a prime, semiprime or larger tail.
    let ps = [
        2_u64, 3, 5, 7, 11, 13, 17, 31, 61, 101, 211, 1009, 2003, 2017, 65_537, 1_000_003,
    ];
    for a in 0..ps.len() {
        for b in a..ps.len() {
            for c in b..ps.len() {
                for d in c..ps.len() {
                    for e in d..ps.len() {
                        let mut n = 1_u64;
                        let list = [ps[a], ps[b], ps[c], ps[d], ps[e]];
                        let mut ok = true;
                        for x in list {
                            match n.checked_mul(x) {
                                Some(m) => n = m,
                                None => ok = false,
                            }
                        }
                        if !ok {
                            continue;
                        }
                        for k in 0..=9 {
                            assert_eq!(is_almost_prime(k, n), k == 5, "k={k} n={n}");
                        }
                        for x in [2_u64, 2017, 65_537] {
                            if let Some(m) = n.checked_mul(x) {
                                for k in 4..=7 {
                                    assert_eq!(is_almost_prime(k, m), k == 6, "k={k} n={m}");
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn holf_factor_exact_oracle() {
    let pool = pool();
    for (i, &p) in pool.iter().enumerate() {
        for &q in &pool[i..] {
            let Some(n) = p.checked_mul(q) else { continue };
            let got = holf_factor(n, None);
            assert_eq!(got.iter().product::<u64>(), n);
            let want = if p == q { vec![p, p] } else { vec![p, q] };
            for r in [Some(1), Some(5), Some(100), Some(100_000), None] {
                let got = holf_factor(n, r);
                assert_eq!(got.iter().product::<u64>(), n, "holf({n},{r:?})");
                if r.is_none() && n < 1 << 50 {
                    assert_eq!(got, want, "holf({n})");
                }
            }
        }
    }
}

fn omega_naive(mut n: u64) -> u64 {
    let mut k = 0;
    let mut p = 2;
    while p * p <= n {
        while n % p == 0 {
            n /= p;
            k += 1;
        }
        p += 1;
    }
    k + u64::from(n > 1)
}

#[test]
fn almost_prime_exhaustive_range() {
    for n in 1..=200_000_u64 {
        let w = omega_naive(n);
        for k in 0..=19 {
            assert_eq!(is_almost_prime(k, n), k == w, "almost_prime({k},{n})");
        }
    }
}

fn fold(hash: u64, x: u64) -> u64 {
    (hash ^ x.rotate_left(17)).wrapping_mul(0x0000_0100_0000_01b3) ^ (hash >> 29)
}

/// Semiprimes and prime squares in every multiplier band of `holf_factor`
/// (2^51, 2^53, 2^54, 2^56, 2^58 and above), with short and long round limits.
fn holf_inputs() -> Vec<u64> {
    let mut out: Vec<u64> = (9..30_000_u64).step_by(2).collect();
    for bits in 40_u32..=63 {
        for k in 0..4_u64 {
            let p = next_prime((1_u64 << (bits / 2 - 1)) + 40 * k).expect("below 2^64");
            let q =
                next_prime((1_u64 << (bits - bits / 2 - 1)) + 1_000 * k + 3).expect("below 2^64");
            out.extend(p.checked_mul(q));
            out.extend(p.checked_mul(p));
            let r = next_prime(q + 100_000).expect("below 2^64");
            out.extend(p.checked_mul(r));
        }
    }
    out
}

#[test]
fn holf_factor_exact_results_across_bands() {
    let mut h = 0;
    for n in holf_inputs() {
        for r in [1_u64, 3, 40, 2_000, 100_000] {
            let got = holf_factor(n, Some(r));
            assert_eq!(got.iter().product::<u64>(), n, "holf({n},{r})");
            for f in got {
                h = fold(h, f);
            }
            h = fold(h, 0);
        }
    }
    println!("GOLD {h}");
    assert_eq!(h, 0);
}

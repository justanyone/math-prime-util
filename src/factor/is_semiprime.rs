use crate::{gcd, is_prime, mulmod, rootint, sqrtint};

/// Return whether `n` has exactly two prime factors, with multiplicity.
///
/// Port of MPU 0.75 `util.c::is_semiprime`, with factoring support from
/// `factor.c::prho_factor` and `factor.c::trial_factor`.
///
/// ```
/// use math_prime_util::is_semiprime;
/// assert!(is_semiprime(49));
/// assert!(!is_semiprime(12));
/// ```
pub fn is_semiprime(n: u64) -> bool {
    if n < 6 {
        return n == 4;
    }
    for p in [2, 3, 5] {
        if n % p == 0 {
            return is_prime(n / p) != 0;
        }
    }
    let n3 = rootint(n, 3);
    let mut p = 7;
    for &prime in &SMALL_PRIMES[4..60] {
        p = prime;
        if p > n3 {
            break;
        }
        if n % p == 0 {
            return is_prime(n / p) != 0;
        }
    }
    if is_prime(n) != 0 {
        return false;
    }
    if p > n3 {
        return true;
    }
    let r = sqrtint(n);
    if r * r == n {
        return is_prime(r) != 0;
    }
    let f = split(n);
    is_prime(f) != 0 && is_prime(n / f) != 0
}

// MPU util.c::primes_small, including its zero sentinel.
pub(super) const SMALL_PRIMES: &[u64] = &[
    0, 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
    97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191,
    193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293,
    307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419,
    421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541,
    547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653,
    659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787,
    797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919,
    929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997, 1009, 1013, 1019, 1021, 1031, 1033,
    1039, 1049, 1051, 1061, 1063, 1069, 1087, 1091, 1093, 1097, 1103, 1109, 1117, 1123, 1129, 1151,
    1153, 1163, 1171, 1181, 1187, 1193, 1201, 1213, 1217, 1223, 1229, 1231, 1237, 1249, 1259, 1277,
    1279, 1283, 1289, 1291, 1297, 1301, 1303, 1307, 1319, 1321, 1327, 1361, 1367, 1373, 1381, 1399,
    1409, 1423, 1427, 1429, 1433, 1439, 1447, 1451, 1453, 1459, 1471, 1481, 1483, 1487, 1489, 1493,
    1499, 1511, 1523, 1531, 1543, 1549, 1553, 1559, 1567, 1571, 1579, 1583, 1597, 1601, 1607, 1609,
    1613, 1619, 1621, 1627, 1637, 1657, 1663, 1667, 1669, 1693, 1697, 1699, 1709, 1721, 1723, 1733,
    1741, 1747, 1753, 1759, 1777, 1783, 1787, 1789, 1801, 1811, 1823, 1831, 1847, 1861, 1867, 1871,
    1873, 1877, 1879, 1889, 1901, 1907, 1913, 1931, 1933, 1949, 1951, 1973, 1979, 1987, 1993, 1997,
    1999, 2003, 2011,
];

// MPU factor.c::prho_factor: batches of 64 Floyd steps and GCD backtracking.
// The caller retries with a new polynomial, then uses trial division.
fn rho(n: u64, mut a: u64) -> Option<u64> {
    let mut u = 7 % n;
    let mut v = u;
    let mut fails = 3;
    for _ in 0..4096 {
        let (old_u, old_v) = (u, v);
        let mut m = 1;
        for _ in 0..64 {
            u = step(u, a, n);
            v = step(step(v, a, n), a, n);
            m = mulmod(m, u.abs_diff(v), n).expect("nonzero modulus");
        }
        let mut f = gcd(m, n);
        if f == 1 {
            continue;
        }
        if f == n {
            u = old_u;
            v = old_v;
            for _ in 0..=64 {
                u = step(u, a, n);
                v = step(step(v, a, n), a, n);
                f = gcd(u.abs_diff(v), n);
                if f != 1 {
                    break;
                }
            }
        }
        if f == n || f == 0 {
            if fails == 0 {
                return None;
            }
            fails -= 1;
            u = crate::addmod(u, 2, n).expect("nonzero modulus");
            v = u;
            a += 2;
        } else {
            return Some(f);
        }
    }
    None
}

fn step(x: u64, a: u64, n: u64) -> u64 {
    crate::addmod(mulmod(x, x, n).expect("nonzero modulus"), a % n, n).expect("nonzero modulus")
}

fn split(n: u64) -> u64 {
    for &p in &SMALL_PRIMES[1..] {
        if n % p == 0 {
            return p;
        }
    }
    for a in [1, 3, 5, 7] {
        if let Some(f) = rho(n, a) {
            return f;
        }
    }
    trial_split(n)
}

// MPU factor.c::trial_factor, final exact fallback after the small table.
fn trial_split(n: u64) -> u64 {
    let mut p = 2017;
    while p <= n / p {
        if n % p == 0 {
            return p;
        }
        p += 2;
    }
    n
}

pub(super) fn factors(n: u64) -> Vec<u64> {
    fn collect(n: u64, out: &mut Vec<u64>) {
        if n <= 1 {
            return;
        }
        if is_prime(n) != 0 {
            out.push(n);
            return;
        }
        let f = split(n);
        collect(f, out);
        collect(n / f, out);
    }
    let mut out = Vec::new();
    collect(n, &mut out);
    out.sort_unstable();
    out
}

#[cfg(test)]
mod tests {
    use super::{factors, rho, split, step, trial_split};

    #[test]
    fn factoring_support() {
        assert_eq!(factors(0), [] as [u64; 0]);
        assert_eq!(factors(1), [] as [u64; 0]);
        assert_eq!(factors(2 * 3 * 3 * 17), [2, 3, 3, 17]);
        assert_eq!(split(2027), 2027);
        assert_eq!(trial_split(2017 * 2027), 2017);
        assert_eq!(trial_split(2027 * 2029), 2027);
        assert_eq!(trial_split(2027), 2027);
        assert_eq!(rho(13, 1), None);
        assert_eq!(rho(18_446_744_073_709_551_557, 1), None);
        let n = 1_000_003 * 1_000_033;
        let f = rho(n, 1).expect("composite splits");
        assert!(f > 1 && f < n && n % f == 0);
        assert_eq!(factors(n), [1_000_003, 1_000_033]);
    }

    #[test]
    fn rho_exact_results() {
        use crate::factor::test_support::{fold_opt, inputs};
        let mut h = 0;
        for n in inputs() {
            for a in [1, 3, 5, 7] {
                let got = rho(n, a);
                if let Some(f) = got {
                    assert!(f > 1 && f < n && n % f == 0, "rho({n},{a})");
                }
                h = fold_opt(h, got);
            }
        }
        assert_eq!(h, 14_561_977_967_123_951_357);
    }

    #[test]
    fn rho_small_and_split_edges() {
        use crate::factor::test_support::{extra_inputs, fold, fold_opt};
        let (mut h, mut hs) = (0, 0);
        for n in extra_inputs() {
            for a in [1, 3, 5, 7, 9, 21] {
                let got = rho(n, a);
                if let Some(f) = got {
                    assert!(f > 1 && f < n && n % f == 0, "rho({n},{a})");
                }
                h = fold_opt(h, got);
            }
            hs = fold(hs, split(n));
        }
        assert_eq!(
            (h, hs),
            (11_462_009_608_631_017_843, 12_067_257_513_942_240_652)
        );
        assert_eq!(split(11 * 13), 11);
        assert_eq!(split(2011 * 2017), 2011);
        assert_eq!(split(13 * 2017 * 2027), 13);
        assert_eq!(trial_split(15), 15);
        assert_eq!(trial_split(3 * 2017), 3 * 2017);
        assert_eq!(trial_split(2029), 2029);
        assert_eq!(trial_split(2027 * 2029), 2027);
        assert_eq!(trial_split(2017 * 2017), 2017);
        assert_eq!(factors(2027 * 2029 * 2039), [2027, 2029, 2039]);
        assert_eq!(factors(2), [2]);
        assert_eq!(factors(0), [] as [u64; 0]);
        assert_eq!(step(5, 7, 3), 2);
    }
}

/// Return trial factors and the remaining cofactor.
///
/// Port of `factor.c::trial_factor` in MPU 0.75, with `first = 2`.
/// The bound is inclusive. A zero bound means no limit.
/// The last value can be composite. One returns an empty list; zero returns `[0]`.
///
/// ```
/// use math_prime_util::trial_factor;
/// assert_eq!(trial_factor(175, 5), vec![5, 5, 7]);
/// assert_eq!(trial_factor(403, 10), vec![403]);
/// ```
pub fn trial_factor(n: u64, limit: u64) -> Vec<u64> {
    trial_from(n, 2, limit)
}

pub(super) fn trial_from(mut n: u64, first: u64, mut last: u64) -> Vec<u64> {
    let first = first.max(2);
    if last == 0 || last > n / last {
        last = u64::MAX;
    }
    if n < 4 || last < first {
        return if n == 1 { Vec::new() } else { vec![n] };
    }
    let mut factors = Vec::new();
    for p in [2, 3, 5] {
        if first <= p && p <= last {
            while n % p == 0 {
                factors.push(p);
                n /= p;
            }
        }
    }
    // MPU primes_small[4..NPRIMES_SMALL], from 7 through 2011.
    const SMALL_PRIMES: &[u64] = &[
        7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101,
        103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
        197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293,
        307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409,
        419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521,
        523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641,
        643, 647, 653, 659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757,
        761, 769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881,
        883, 887, 907, 911, 919, 929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997, 1009, 1013,
        1019, 1021, 1031, 1033, 1039, 1049, 1051, 1061, 1063, 1069, 1087, 1091, 1093, 1097, 1103,
        1109, 1117, 1123, 1129, 1151, 1153, 1163, 1171, 1181, 1187, 1193, 1201, 1213, 1217, 1223,
        1229, 1231, 1237, 1249, 1259, 1277, 1279, 1283, 1289, 1291, 1297, 1301, 1303, 1307, 1319,
        1321, 1327, 1361, 1367, 1373, 1381, 1399, 1409, 1423, 1427, 1429, 1433, 1439, 1447, 1451,
        1453, 1459, 1471, 1481, 1483, 1487, 1489, 1493, 1499, 1511, 1523, 1531, 1543, 1549, 1553,
        1559, 1567, 1571, 1579, 1583, 1597, 1601, 1607, 1609, 1613, 1619, 1621, 1627, 1637, 1657,
        1663, 1667, 1669, 1693, 1697, 1699, 1709, 1721, 1723, 1733, 1741, 1747, 1753, 1759, 1777,
        1783, 1787, 1789, 1801, 1811, 1823, 1831, 1847, 1861, 1867, 1871, 1873, 1877, 1879, 1889,
        1901, 1907, 1913, 1931, 1933, 1949, 1951, 1973, 1979, 1987, 1993, 1997, 1999, 2003, 2011,
    ];
    let mut f = first.max(7);
    for &p in SMALL_PRIMES.iter().skip_while(|&&p| p < first.max(7)) {
        f = p;
        if f > n / f || f > last {
            break;
        }
        while n % f == 0 {
            factors.push(f);
            n /= f;
        }
        f = 2017;
    }
    if f > 2011 {
        const DISTANCE: [u64; 30] = [
            1, 0, 5, 4, 3, 2, 1, 0, 3, 2, 1, 0, 1, 0, 3, 2, 1, 0, 1, 0, 3, 2, 1, 0, 5, 4, 3, 2, 1,
            0,
        ];
        const ADVANCE: [u64; 30] = [
            1, 6, 5, 4, 3, 2, 1, 4, 3, 2, 1, 2, 1, 4, 3, 2, 1, 2, 1, 4, 3, 2, 1, 6, 5, 4, 3, 2, 1,
            2,
        ];
        if let Some(start) = f.checked_add(DISTANCE[(f % 30) as usize]) {
            f = start;
            while f <= n / f && f <= last {
                while n % f == 0 {
                    factors.push(f);
                    n /= f;
                }
                f += ADVANCE[(f % 30) as usize];
            }
        }
    }
    if n != 1 {
        factors.push(n);
    }
    factors
}

#[cfg(test)]
mod tests {
    use super::trial_from;

    #[test]
    fn first_bound_and_wheel() {
        assert_eq!(trial_from(30, 3, 0), [3, 10]);
        assert_eq!(trial_from(49, 0, 0), [7, 7]);
        assert_eq!(trial_from(25, 7, 5), [25]);
        assert_eq!(trial_from(2017 * 2027, 2012, 0), [2017, 2027]);
        assert_eq!(trial_from(u64::MAX, u64::MAX, 0), [u64::MAX]);
        assert_eq!(trial_from(u64::MAX, u64::MAX - 1, 0), [u64::MAX]);
    }
}

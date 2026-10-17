//! Two-stage Montgomery-curve ECM from MPU `factor.c`.

use crate::int::mont::Mont;
use crate::modular::{addmod_u, mulmod_u};
use crate::{gcd, invmod, is_prime, primes, sqrtint};

#[derive(Clone, Copy, Debug, Default)]
struct Point {
    x: u64,
    z: u64,
}

fn sub(a: u64, b: u64, n: u64) -> u64 {
    if a >= b { a - b } else { n - (b - a) }
}

/// Port of `factor.c:ecm_double64`.
fn double(p: Point, a24: u64, c: Mont) -> Point {
    let u = c.sqr(sub(p.x, p.z, c.n));
    let v = c.sqr(addmod_u(p.x, p.z, c.n));
    let w = sub(v, u, c.n);
    Point {
        x: c.mul(u, v),
        z: c.mul(w, addmod_u(u, c.mul(a24, w), c.n)),
    }
}

/// Port of `factor.c:ecm_dadd64`.
fn dadd(p: Point, q: Point, difference: Point, c: Mont) -> Point {
    let u = c.mul(sub(p.x, p.z, c.n), addmod_u(q.x, q.z, c.n));
    let v = c.mul(addmod_u(p.x, p.z, c.n), sub(q.x, q.z, c.n));
    Point {
        x: c.mul(c.sqr(addmod_u(u, v, c.n)), difference.z),
        z: c.mul(c.sqr(sub(u, v, c.n)), difference.x),
    }
}

/// Port of `factor.c:ecm_mul64`. The scalar is positive.
fn multiply(p: Point, k: u64, a24: u64, c: Mont) -> Point {
    if k == 1 {
        return p;
    }
    let mut r0 = p;
    let mut r1 = double(p, a24, c);
    if k == 2 {
        return r1;
    }
    let mut bit = (1_u64 << k.ilog2()) >> 1;
    while bit != 0 {
        if k & bit != 0 {
            r0 = dadd(r0, r1, p, c);
            r1 = double(r1, a24, c);
        } else {
            r1 = dadd(r0, r1, p, c);
            r0 = double(r0, a24, c);
        }
        bit >>= 1;
    }
    r0
}

/// Port of `factor.c:ecm_batch_normalize_x64`.
/// An error carries a proper factor, or zero for a collapsed curve.
fn normalize(points: &[Point], c: Mont) -> Result<Vec<u64>, u64> {
    let mut acc = c.one;
    let prefix: Vec<_> = points
        .iter()
        .map(|p| {
            acc = c.mul(acc, p.z);
            acc
        })
        .collect();
    let g = gcd(acc, c.n);
    if g > 1 {
        return Err(if g < c.n {
            g
        } else {
            points
                .iter()
                .map(|p| gcd(p.z, c.n))
                .find(|&g| g > 1 && g < c.n)
                .unwrap_or(0)
        });
    }
    let Some(inverse) = invmod(c.recover(acc), c.n) else {
        return Err(0);
    };
    let mut inverse = c.to_mont(inverse);
    let mut out = vec![0; points.len()];
    for j in (0..points.len()).rev() {
        let prev = if j == 0 { c.one } else { prefix[j - 1] };
        let zinv = c.mul(inverse, prev);
        inverse = c.mul(inverse, points[j].z);
        out[j] = c.mul(points[j].x, zinv);
    }
    Ok(out)
}

/// Port of `factor.c:tinyecm64_stage2`.
fn stage2(q: Point, a24: u64, c: Mont, b1: u64, b2: u64) -> u64 {
    if b2 <= b1 {
        return 0;
    }
    let d = sqrtint(b2 >> 1);
    let d = d + (d & 1);
    if d == 0 {
        return 0;
    }
    let two_d = 2 * d;
    let end = b2.saturating_add(d);
    let windows = (end - 1).div_ceil(two_d);
    // These lengths fit usize on all supported 64-bit targets.
    let d = usize::try_from(d).expect("ECM table too large");
    let windows = usize::try_from(windows).expect("ECM table too large");
    let mut nq = vec![Point::default(); 2 * d + 1];
    nq[1] = q;
    for i in 2..nq.len() {
        nq[i] = if i & 1 == 1 {
            dadd(nq[(i - 1) / 2], nq[i.div_ceil(2)], q, c)
        } else {
            double(nq[i / 2], a24, c)
        };
    }
    let nqx = match normalize(&nq[1..=d], c) {
        Ok(x) => x,
        Err(f) => return f,
    };
    let mut s = vec![q; windows];
    let mut xm = nq[2 * d - 1];
    for w in 1..windows {
        s[w] = dadd(nq[2 * d], s[w - 1], xm, c);
        xm = s[w - 1];
    }
    let sx = match normalize(&s, c) {
        Ok(x) => x,
        Err(f) => return f,
    };
    let mut product = c.one;
    let mut m = 1_u64;
    for x in sx {
        let hi = m.saturating_add(d as u64).min(b2);
        let lo = m.saturating_sub(d as u64).max(b1.saturating_add(1)).max(2);
        if lo <= hi {
            for p in primes(lo, hi) {
                if p == m {
                    continue;
                }
                if p > m && m <= u64::MAX / 2 && p <= 2 * m {
                    let mirror = 2 * m - p;
                    if mirror > b1 && mirror >= lo && is_prime(mirror) != 0 {
                        continue;
                    }
                }
                let idx = usize::try_from(p.abs_diff(m)).expect("ECM index too large");
                if idx <= d {
                    // nqx[0] in C is unused: p == m was skipped above.
                    product = c.mul(product, sub(x, nqx[idx - 1], c.n));
                }
            }
            let g = gcd(c.recover(product), c.n);
            if g > 1 {
                return if g < c.n { g } else { 0 };
            }
        }
        let Some(next) = m.checked_add(two_d) else {
            break;
        };
        m = next;
    }
    let g = gcd(c.recover(product), c.n);
    if g > 1 && g < c.n { g } else { 0 }
}

/// Port of `factor.c:tinyecm64` with its fixed Suyama sigma sequence.
fn find_factor(n: u64, b1: u64, b2: u64, curves: u64) -> u64 {
    if curves == 0 {
        return 0;
    }
    let c = Mont::new(n);
    let stage1_primes = primes(2, b1);
    'curve: for &sigma in SIGMAS
        .iter()
        .take(usize::try_from(curves).unwrap_or(usize::MAX))
    {
        let sigma = u64::from(sigma);
        let u = sigma * sigma - 5;
        let v = 4 * sigma;
        let umv = u - v;
        let u3 = mulmod_u(mulmod_u(u % n, u % n, n), u % n, n);
        let v3 = mulmod_u(mulmod_u(v % n, v % n, n), v % n, n);
        let umv3 = mulmod_u(mulmod_u(umv % n, umv % n, n), umv % n, n);
        let abs_num = mulmod_u(umv3, (3 * u + v) % n, n);
        let num = if abs_num == 0 { 0 } else { n - abs_num };
        let den = mulmod_u(mulmod_u(u3, v % n, n), 16 % n, n);
        let Some(inverse) = invmod(den, n) else {
            let g = gcd(den, n);
            if g > 1 && g < n {
                return g;
            }
            continue;
        };
        let a24 = c.mul(c.to_mont(num), c.to_mont(inverse));
        let mut p = Point {
            x: c.to_mont(u3),
            z: c.to_mont(v3),
        };
        for (j, &prime) in stage1_primes.iter().enumerate() {
            let mut k = prime;
            if prime <= sqrtint(b1) {
                while k <= b1 / prime {
                    k *= prime;
                }
            }
            p = multiply(p, k, a24, c);
            if j % 64 == 0 {
                let g = gcd(c.recover(p.z), n);
                if g > 1 && g < n {
                    return g;
                }
                if g == n {
                    continue 'curve;
                }
            }
        }
        let g = gcd(c.recover(p.z), n);
        if g > 1 && g < n {
            return g;
        }
        if b2 > b1 {
            let g = stage2(p, a24, c, b1, b2);
            if g > 1 && g < n {
                return g;
            }
        }
    }
    0
}

/// Splits `n` with two-stage elliptic curve factoring.
///
/// Port of MPU 0.75 `factor.c:tinyecm64_factor`, `tinyecm64`, and its
/// `ecm_*64` helpers. Small factors follow `XS.xs:trial_factor` (ECM alias).
/// Bounds default to 4000 and twenty times the first bound. The default
/// curve count is 100. A zero second bound selects the default second bound.
/// Factors need not be prime. An unsuccessful search returns the cofactor.
/// The input 1 returns an empty vector. The input 0 returns `[0]`.
///
/// # Panics
/// Panics if `b2` is set without `b1`, or an ECM table cannot fit in memory.
///
/// ```
/// use math_prime_util::ecm_factor;
/// assert_eq!(ecm_factor(403, None, None, None), vec![13, 31]);
/// assert!(ecm_factor(1, None, None, None).is_empty());
/// ```
pub fn ecm_factor(mut n: u64, b1: Option<u64>, b2: Option<u64>, curves: Option<u64>) -> Vec<u64> {
    assert!(
        b1.is_some() || b2.is_none(),
        "ecm_factor: B1 must be specified if B2 is specified"
    );
    if n == 0 {
        return vec![0];
    }
    let mut result = Vec::new();
    for p in [2, 3, 5] {
        while n % p == 0 {
            result.push(p);
            n /= p;
        }
    }
    if n == 1 {
        return result;
    }
    if is_prime(n) == 0 {
        let b1 = b1.unwrap_or(4000);
        let b2 = b2
            .filter(|&b| b != 0)
            .unwrap_or_else(|| b1.saturating_mul(20));
        let f = find_factor(n, b1, b2, curves.unwrap_or(100));
        if f > 1 && f < n {
            result.extend([f.min(n / f), f.max(n / f)]);
            return result;
        }
    }
    result.push(n);
    result
}

const SIGMAS: &[u16] = &[
    11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 103, 127, 139,
    149, 151, 157, 163, 181, 191, 197, 199, 211, 223, 227, 233, 239, 257, 271, 293, 313, 331, 337,
    347, 359, 379, 389, 397, 401, 409, 421, 443, 449, 457, 479, 487, 509, 521, 523, 547, 557, 587,
    641, 653, 659, 673, 677, 683, 691, 719, 727, 739, 751, 769, 797, 809, 853, 919, 929, 941, 997,
    1049, 1051, 1063, 1091, 1093, 1109, 1117, 1129, 1153, 1201, 1217, 1229, 1283, 1327, 1361, 1381,
    1427, 1447, 1459, 1471, 1481, 1489, 1543, 1549, 1571, 1621, 1709, 1723, 1753, 1759, 1801, 1811,
    1867, 1987, 2039, 2099, 2113, 2131, 2251, 2309, 2347, 2381, 2399, 2447, 2473, 2551, 2557, 2663,
    2677, 2689, 2713, 2719, 2749, 2857, 2879, 2887, 2939, 3001, 3061, 3067, 3121, 3137, 3187, 3251,
    3259, 3271, 3307, 3359, 3371, 3373, 3467, 3593, 3607, 3623, 3643, 3709, 3733, 3793, 3851, 3923,
    3989, 4019, 4049, 4129, 4231, 4253, 4283, 4339, 4349, 4441, 4523, 4649, 4787, 4987, 4999, 5171,
    5237, 5273, 5297, 5333, 5387, 5471, 5479, 5647, 5749, 5791, 6101, 6163, 6257, 6299, 6337, 6451,
    6491, 6659, 6793, 6823, 6967, 7013, 7229, 7253, 7333, 7369, 7477, 7621, 7793, 7817, 8059, 8167,
    8209, 8263, 8311, 8377, 8573, 8641, 8741, 8837, 8863, 8963, 9001, 9151, 9203, 9433, 9697, 9743,
    9781, 9883, 10007, 10069, 10099, 10139, 10163, 10193, 10267, 10429, 10457, 10487, 10691, 10837,
    10949, 11087, 11243, 11321, 11411, 11681, 11813, 11903, 12011, 12263, 12277, 12401, 12409,
    12437, 12479, 12569, 12619, 12739, 12911, 13331, 13367, 13537, 13721, 13789, 13841, 13873,
    14051, 14149, 14221, 14419, 14431, 14827, 14887, 15077, 15289, 15467, 15511, 15649, 15773,
    15797, 15859, 15901, 16057, 16141, 16217, 16529, 16547, 16553, 16619, 17299, 17393, 17419,
    17449, 17737, 17921, 18049, 18223, 19073, 19183, 19477, 20021, 20323, 20347, 20759, 20929,
    21023, 21157, 21587, 21611, 21613, 21673, 21751, 21799, 21821, 22109, 22469, 22651, 22943,
    23327, 23459, 23567, 23767, 23911, 23957, 24001, 24197, 24281, 24407, 24799, 24851, 25147,
    25183, 25469, 25679, 25703, 26561, 26683, 26701, 26821, 27073, 27191, 27271, 27277, 27427,
    27487, 27539, 27617, 27647, 27673, 27749, 27983, 28319, 28789, 28843, 29017, 29123, 29209,
    29669, 29803, 29921, 30323, 30809, 30851, 30911, 30983, 31397, 31541, 31963, 32369, 32561,
    32771, 32969, 33029, 33083, 33487, 33637, 33757, 34057, 34381, 34513, 34613, 34807, 35083,
    35171, 35311, 35381, 36013, 36251, 36493, 36529, 36551, 36913, 36919, 37363, 37517, 37699,
    37907, 38047, 38177, 38273, 38749, 38903,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn curve_ladder_and_normalization() {
        let c = Mont::new(101);
        let p = Point {
            x: c.to_mont(7),
            z: c.one,
        };
        let a = c.to_mont(3);
        assert_eq!(normalize(&[], c), Ok(vec![]));
        assert_eq!(normalize(&[p], c).unwrap(), vec![c.to_mont(7)]);
        let twice = double(p, a, c);
        let thrice = dadd(p, twice, p, c);
        for (k, q) in [(1, p), (2, twice), (3, thrice), (4, double(twice, a, c))] {
            let r = multiply(p, k, a, c);
            assert_eq!(c.mul(r.x, q.z), c.mul(q.x, r.z));
        }
        assert_eq!(stage2(p, a, c, 10, 10), 0);
        assert_eq!(stage2(p, a, c, 0, 1), 0);
        let _ = stage2(p, a, c, 2, 100);
    }

    #[test]
    fn normalization_recovers_individual_factors() {
        let c = Mont::new(35);
        let point = |z| Point {
            x: c.one,
            z: c.to_mont(z),
        };
        assert_eq!(normalize(&[point(5)], c), Err(5));
        assert_eq!(normalize(&[point(5), point(7)], c), Err(5));
        assert_eq!(normalize(&[point(0)], c), Err(0));
    }

    #[test]
    fn debug_ambiguous_split() {
        let n = 90_950_751_857_u64;
        let mut msg = String::new();
        for &sigma in SIGMAS.iter().take(8) {
            let sigma = u64::from(sigma);
            let u = sigma * sigma - 5;
            let v = 4 * sigma;
            let umv = u - v;
            let u3 = mulmod_u(mulmod_u(u % n, u % n, n), u % n, n);
            let umv3 = mulmod_u(mulmod_u(umv % n, umv % n, n), umv % n, n);
            let den = mulmod_u(mulmod_u(u3, v % n, n), 16 % n, n);
            let gden = gcd(den, n);
            let gz = gcd(u3, n);
            let gv = gcd(v, n);
            msg.push_str(&format!(
                "sigma={sigma} gcd(den)={gden} gcd(u3)={gz} gcd(v)={gv} den={den}\n"
            ));
        }
        // Stage-1 only: B2 <= B1 skips stage 2.
        let f = find_factor(n, 100, 100, 1);
        msg.push_str(&format!("stage1-only first curve f={f}\n"));
        panic!("{msg}");
    }

    #[test]
    fn bounds_and_unsuccessful_search() {
        assert_eq!(ecm_factor(0, None, None, None), vec![0]);
        assert_eq!(ecm_factor(1, None, None, None), vec![]);
        assert_eq!(ecm_factor(91, Some(0), Some(0), Some(0)), vec![91]);
        assert_eq!(
            ecm_factor(91, Some(0), Some(0), Some(1))
                .iter()
                .product::<u64>(),
            91
        );
        assert_eq!(ecm_factor(101, None, None, None), vec![101]);
        assert_eq!(
            ecm_factor(91, Some(u64::MAX), Some(u64::MAX), Some(0)),
            vec![91]
        );
        for b1 in [0, 2, 10, 50, 100] {
            let factors = ecm_factor(8051, Some(b1), Some(b1 * 20), Some(400));
            assert_eq!(factors.iter().product::<u64>(), 8051);
        }
    }
}

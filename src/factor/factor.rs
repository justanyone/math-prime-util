use crate::{gcd, is_prime, rootint, sqrtint};

/// Return prime factors with multiplicity, in ascending order.
///
/// Port of MPU 0.75 `factor.c::factor`, `_small_trial_factor`,
/// `_power_factor`, and `factor_one`. The splitter follows that
/// function's Montgomery order: `holf32`, Brent, SQUFOF through 62 bits,
/// p-1, rho, then Chebyshev. `tinyecm64` is the `HAS_ECM64` arm and SIQS
/// has no source in MPU 0.75, so those stages are not called.
/// One returns an empty list. Zero returns `[0]`.
/// Inputs above 64 bits use [`factor_u128`](crate::factor_u128).
///
/// ```
/// use math_prime_util::factor;
/// assert_eq!(factor(360), vec![2, 2, 2, 3, 3, 5]);
/// assert_eq!(factor(1), Vec::<u64>::new());
/// ```
pub fn factor(n: u64) -> Vec<u64> {
    if n < 4 {
        return if n == 1 { Vec::new() } else { vec![n] };
    }
    let mut factors = super::trial_factor(n, 421);
    let remainder = factors.pop().unwrap_or(1);
    let limit = if remainder < 2017 * 2017 { 2011 } else { 421 };
    let mut rest = super::trial_factor::trial_from(remainder, 431, limit);
    let remainder = rest.pop().unwrap_or(1);
    factors.extend(rest);
    let mut stack = vec![remainder];
    while let Some(n) = stack.pop() {
        if n == 1 {
            continue;
        }
        if is_prime(n) != 0 {
            factors.push(n);
            continue;
        }
        let power = crate::int::is_power_exp(n);
        if power > 1 {
            let root = rootint(n, power);
            let roots = factor(root);
            for _ in 0..power {
                factors.extend_from_slice(&roots);
            }
            continue;
        }
        let divisor = split(n);
        stack.push(divisor);
        stack.push(n / divisor);
    }
    factors.sort_unstable();
    factors
}

// MPU factor.c::holf32, with its 5040 multiplier and 10000-round bound.
fn holf(n: u64) -> Option<u64> {
    let step = n * 5040;
    let mut ni = step;
    for _ in 0..10_000 {
        let s = sqrtint(ni) + 1;
        let m = s * s - ni;
        let root = sqrtint(m);
        if root * root == m {
            let f = gcd(n, s - root);
            if f > 1 && f < n {
                return Some(f);
            }
        }
        ni = ni.checked_add(step)?;
    }
    None
}

/// One nontrivial factor from a method that returns a factor list.
fn one_factor(n: u64, factors: &[u64]) -> Option<u64> {
    let f = *factors.first()?;
    (f > 1 && f < n && n % f == 0).then_some(f)
}

/// `holf32` only for values that fit in 32 bits (`factor_one`).
fn holf_stage(n: u64) -> Option<u64> {
    if u32::try_from(n).is_err() {
        return None;
    }
    holf(n)
}

/// SQUFOF while `n` has at most 62 bits. The ECM build stops at 48 bits
/// and runs `tinyecm64` instead; without that function the `#else` arm
/// in `factor_one` keeps SQUFOF through 62 bits.
fn squfof_stage(n: u64) -> Option<u64> {
    if 64 - n.leading_zeros() > 62 {
        return None;
    }
    one_factor(n, &super::squfof_factor(n, Some(200_000)))
}

/// Portable `factor_one` chain. `start` skips earlier stages so tests can
/// reach each arm. Production calls this with `start == 0`.
fn dispatcher(n: u64, start: u8) -> Option<u64> {
    if start == 0 {
        if let Some(f) = holf_stage(n) {
            return Some(f);
        }
    }
    let bits = 64 - n.leading_zeros();
    let extra = if bits <= 45 { 0 } else { u64::from(bits - 45) };
    // USE_MONTMATH round schedule in factor_one.
    let rounds = 8_000 + 9_000 * extra;
    if start <= 1 {
        if let Some(f) = one_factor(n, &super::pbrent_factor(n, Some(rounds), Some(1))) {
            return Some(f);
        }
    }
    if start <= 2 {
        if let Some(f) = one_factor(
            n,
            &super::pbrent_factor(n, Some(rounds.saturating_mul(2)), Some(3)),
        ) {
            return Some(f);
        }
    }
    if start <= 3 {
        if let Some(f) = squfof_stage(n) {
            return Some(f);
        }
    }
    if start <= 4 {
        if let Some(f) = one_factor(n, &super::pminus1_factor(n, Some(8_000), Some(120_000))) {
            return Some(f);
        }
    }
    if start <= 5 {
        if let Some(f) = one_factor(n, &super::pbrent_factor(n, Some(500_000), Some(5))) {
            return Some(f);
        }
    }
    if start <= 6 {
        if let Some(f) = one_factor(n, &super::prho_factor(n, Some(180_000))) {
            return Some(f);
        }
    }
    if start <= 7 {
        if let Some(f) = one_factor(n, &super::cheb_factor(n, 1_000_000, 0)) {
            return Some(f);
        }
    }
    None
}

fn split(n: u64) -> u64 {
    if let Some(f) = dispatcher(n, 0) {
        return f;
    }
    // factor_one croaks when every stage misses. Keep searching with the
    // non-Montgomery Brent polynomial so a composite still splits.
    let mut a = 7;
    loop {
        if let Some(f) = brent(n, 500_000, a) {
            return f;
        }
        a += 2;
    }
}

// MPU factor.c::pbrent_factor, non-Montgomery u128 arithmetic path.
#[allow(clippy::cast_possible_truncation)] // A residue modulo a u64 fits u64.
fn brent(n: u64, mut rounds: u64, mut a: u64) -> Option<u64> {
    let inner = if n <= 4_000_000_000 { 32 } else { 160 };
    let mut fails = 6;
    let (mut r, mut xi, mut xm) = (1u64, 1u64, 1u64);
    while rounds > 0 {
        let mut left = r.min(rounds);
        let mut saved = xi;
        let mut f = 1;
        while left > 0 {
            let count = left.min(inner);
            saved = xi;
            left -= count;
            rounds -= count;
            xi = square_add(xi, a, n);
            let mut m = xi.abs_diff(xm);
            for _ in 1..count {
                xi = square_add(xi, a, n);
                m = mul_mod(m, xi.abs_diff(xm), n);
            }
            f = gcd(m, n);
            if f != 1 {
                break;
            }
        }
        if f == 1 {
            r *= 2;
            xm = xi;
            continue;
        }
        if f == n {
            xi = saved;
            loop {
                xi = square_add(xi, a, n);
                f = gcd(xi.abs_diff(xm), n);
                if f != 1 || r == 0 {
                    break;
                }
                r -= 1;
            }
        }
        if f == 0 || f == n {
            if fails == 0 {
                break;
            }
            fails -= 1;
            xm = ((u128::from(xm) + 11) % u128::from(n)) as u64;
            xi = xm;
            a += 1;
            continue;
        }
        return Some(f);
    }
    None
}

#[allow(clippy::cast_possible_truncation)] // A residue modulo a u64 fits u64.
fn mul_mod(a: u64, b: u64, n: u64) -> u64 {
    (u128::from(a) * u128::from(b) % u128::from(n)) as u64
}

#[allow(clippy::cast_possible_truncation)] // A residue modulo a u64 fits u64.
fn square_add(x: u64, a: u64, n: u64) -> u64 {
    ((u128::from(x) * u128::from(x) + u128::from(a)) % u128::from(n)) as u64
}

#[cfg(test)]
mod tests {
    use super::{brent, dispatcher, holf, holf_stage, one_factor, split, squfof_stage};

    #[test]
    fn split_paths_and_round_limits() {
        assert_eq!(brent(403, 0, 1), None);
        assert_eq!(brent(9, 1, 1), None);
        assert_eq!(brent(3, 10_000, 0), None);
        assert_eq!(brent(101, 10_000, 1), None);
        assert_eq!(holf(1), None);
        for n in [403, 1_000_036_000_099, 18_446_743_979_220_271_189] {
            let f = split(n);
            assert!(f > 1 && f < n, "{n}");
            assert_eq!(n % f, 0);
        }
        for a in 1..=10 {
            if let Some(f) = brent(1_000_036_000_099, 100_000, a) {
                assert!(f > 1 && f < 1_000_036_000_099);
                assert_eq!(1_000_036_000_099 % f, 0);
            }
        }
    }

    #[test]
    fn dispatcher_reaches_each_stage() {
        assert_eq!(one_factor(15, &[]), None);
        assert_eq!(one_factor(15, &[15]), None);
        assert_eq!(one_factor(15, &[3, 5]), Some(3));
        assert_eq!(holf_stage(1 << 40), None);
        assert_eq!(holf_stage(101), None);
        let f = holf_stage(403).unwrap();
        assert_eq!(403 % f, 0);
        assert_eq!(squfof_stage(u64::MAX), None);
        let f = squfof_stage(91).unwrap();
        assert_eq!(91 % f, 0);
        for start in 0..8 {
            let f = dispatcher(403, start).unwrap();
            assert!(f > 1 && f < 403 && 403 % f == 0, "start {start} gave {f}");
        }
        // Above 45 bits the Montgomery schedule adds rounds; 2^50 is even.
        assert_eq!(dispatcher(1 << 50, 1), Some(2));
        assert_eq!(dispatcher(1, 8), None);
    }

    #[test]
    fn brent_and_holf_exact_results() {
        use crate::factor::test_support::{fold_opt, inputs};
        let (mut hb, mut hh) = (0, 0);
        for n in inputs() {
            if let Ok(small) = u32::try_from(n) {
                hh = fold_opt(hh, holf(u64::from(small)));
            }
            for a in [1, 2, 3, 5] {
                for rounds in [100, 5_000, 500_000] {
                    let got = brent(n, rounds, a);
                    if let Some(f) = got {
                        assert!(f > 1 && f < n && n % f == 0, "brent({n},{rounds},{a})");
                    }
                    hb = fold_opt(hb, got);
                }
            }
        }
        assert_eq!(
            (hb, hh),
            (4_434_182_252_870_372_899, 7_831_995_469_746_149_911)
        );
    }
}

//! The 128-bit paths of `factor` and `is_prime`, ported from MPU `factor128.c`.
//!
//! The arithmetic is Montgomery with R = 2^128 (`mont_*` in the C), the
//! primality test is BPSW with an extra-strong Lucas test, and the factoring
//! loop is `factorintp128`: trial division, then p-1, SQUFOF and ECM on the
//! cofactors.  MPU 0.75 has no `tinysiqs128.c`; its largest splitter is the
//! tiny ECM, which this file ports as well.

use std::sync::OnceLock;

/// Primes 2..=2011, the trial division table of the C (`primes_small`).
fn small_primes() -> &'static [u64] {
    static TABLE: OnceLock<Vec<u64>> = OnceLock::new();
    TABLE.get_or_init(|| primes_upto(2011).into_iter().map(u64::from).collect())
}

/// Primes up to and including `limit` (sieve of Eratosthenes).
#[allow(clippy::cast_possible_truncation)] // Callers pass limits below 2^32.
fn primes_upto(limit: u64) -> Vec<u32> {
    let flags = prime_flags(limit);
    (2..=limit as usize)
        .filter(|&i| flags[i])
        .map(|i| i as u32)
        .collect()
}

#[allow(clippy::cast_possible_truncation)] // Callers pass limits that fit memory.
fn prime_flags(limit: u64) -> Vec<bool> {
    let limit = limit as usize;
    let mut flags = vec![true; limit + 1];
    flags[0] = false;
    if limit >= 1 {
        flags[1] = false;
    }
    let mut i = 2;
    while i * i <= limit {
        if flags[i] {
            for j in (i * i..=limit).step_by(i) {
                flags[j] = false;
            }
        }
        i += 1;
    }
    flags
}

fn add_mod(a: u128, b: u128, n: u128) -> u128 {
    let nb = n - b;
    if a >= nb { a - nb } else { a + b }
}

fn sub_mod(a: u128, b: u128, n: u128) -> u128 {
    if a >= b { a - b } else { n - (b - a) }
}

/// `a / 2 mod n` for odd `n`; preserves Montgomery form.
fn half_mod(a: u128, n: u128) -> u128 {
    if a & 1 == 1 {
        (a >> 1) + (n >> 1) + 1
    } else {
        a >> 1
    }
}

/// Binary double-and-add product, used where the modulus varies.
fn mul_mod(a: u128, b: u128, n: u128) -> u128 {
    let (a, b) = (a % n, b % n);
    if let (Ok(x), Ok(y)) = (u64::try_from(a), u64::try_from(b)) {
        return u128::from(x) * u128::from(y) % n;
    }
    let (mut x, mut y) = if a >= b { (a, b) } else { (b, a) };
    let mut r = 0;
    while y > 0 {
        if y & 1 == 1 {
            r = add_mod(r, x, n);
        }
        x = add_mod(x, x, n);
        y >>= 1;
    }
    r
}

fn pow2_mod(e: u32, n: u128) -> u128 {
    let mut r = 1 % n;
    for _ in 0..e {
        r = add_mod(r, r, n);
    }
    r
}

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

/// `a^-1 mod n`, or 0 when `gcd(a, n) > 1` (C `modinv128`).
fn mod_inv(a: u128, n: u128) -> u128 {
    let (mut r0, mut r1) = (n, a % n);
    let (mut t0, mut t1) = (0u128, 1u128);
    if r1 == 0 {
        return 0;
    }
    while r1 != 0 {
        let q = r0 / r1;
        let r = r0 - q * r1;
        let mut qt = q % n;
        if t1 != 1 {
            qt = mul_mod(qt, t1, n);
        }
        let t = sub_mod(t0, qt, n);
        (r0, r1) = (r1, r);
        (t0, t1) = (t1, t);
    }
    if r0 == 1 { t0 } else { 0 }
}

/// Full 256-bit product as `(low, high)` halves (C `mul256`).
fn mul256(a: u128, b: u128) -> (u128, u128) {
    let mask = u128::from(u64::MAX);
    let (a0, a1) = (a & mask, a >> 64);
    let (b0, b1) = (b & mask, b >> 64);
    let p00 = a0 * b0;
    let p01 = a0 * b1;
    let p10 = a1 * b0;
    let p11 = a1 * b1;
    let mid = p01.wrapping_add(p10);
    let mid_carry = u128::from(mid < p01);
    let lo = p00.wrapping_add(mid << 64);
    let lo_carry = u128::from(lo < p00);
    (lo, p11 + (mid >> 64) + (mid_carry << 64) + lo_carry)
}

/// Montgomery context for an odd modulus (C `mont128_t`).
struct Mont {
    n: u128,
    ninv: u128,
    r2: u128,
}

impl Mont {
    fn new(n: u128) -> Self {
        let mut x = n.wrapping_mul(3) ^ 2;
        for _ in 0..5 {
            x = x.wrapping_mul(2u128.wrapping_sub(n.wrapping_mul(x)));
        }
        Mont {
            n,
            ninv: x.wrapping_neg(),
            r2: pow2_mod(256, n),
        }
    }

    /// REDC: `a * b / R mod n` for `a, b < n`.
    fn mul(&self, a: u128, b: u128) -> u128 {
        let (lo, hi) = mul256(a, b);
        let m = lo.wrapping_mul(self.ninv);
        let carry = u128::from(lo != 0);
        let mn_hi = mul256(m, self.n).1;
        let (t, o1) = hi.overflowing_add(mn_hi);
        let (t, o2) = t.overflowing_add(carry);
        if o1 || o2 || t >= self.n {
            t.wrapping_sub(self.n)
        } else {
            t
        }
    }

    fn sqr(&self, a: u128) -> u128 {
        self.mul(a, a)
    }

    fn enter(&self, a: u128) -> u128 {
        self.mul(a % self.n, self.r2)
    }

    fn exit(&self, a: u128) -> u128 {
        self.mul(a, 1)
    }

    fn pow(&self, a: u128, mut k: u128) -> u128 {
        let mut a = a;
        let mut r = self.enter(1);
        while k > 0 {
            if k & 1 == 1 {
                r = self.mul(r, a);
            }
            k >>= 1;
            if k > 0 {
                a = self.sqr(a);
            }
        }
        r
    }
}

/// Jacobi symbol `(a/n)` for odd `n`.
fn jacobi(a: u128, n: u128) -> i32 {
    let (mut a, mut n) = (a % n, n);
    let mut t = 1;
    while a != 0 {
        while a % 2 == 0 {
            a /= 2;
            if matches!(n % 8, 3 | 5) {
                t = -t;
            }
        }
        (a, n) = (n, a);
        if a % 4 == 3 && n % 4 == 3 {
            t = -t;
        }
        a %= n;
    }
    if n == 1 { t } else { 0 }
}

fn is_square(n: u128) -> bool {
    let r = n.isqrt();
    r * r == n
}

/// C `miller_rabin128` for odd `n > 2`.
fn miller_rabin(n: u128, base: u128) -> bool {
    let s = (n - 1).trailing_zeros();
    let d = (n - 1) >> s;
    let ctx = Mont::new(n);
    let one = ctx.enter(1);
    let base = if base == 2 {
        add_mod(one, one, n)
    } else {
        ctx.enter(base % n)
    };
    let mut x = ctx.pow(base, d);
    let minus_one = n - one;
    if x == one || x == minus_one {
        return true;
    }
    for _ in 1..s {
        x = ctx.sqr(x);
        if x == minus_one {
            return true;
        }
    }
    false
}

/// Baillie's extra-strong Lucas parameter `P` (C `select_extra_strong_parameters128`).
fn extra_strong_parameter(n: u128) -> Option<u128> {
    let mut p = 3u128;
    loop {
        let d = p * p - 4;
        let j = jacobi(d, n);
        if j == 0 {
            let g = gcd(d, n);
            if g != 1 && g != n {
                return None;
            }
        }
        if j == -1 {
            return Some(p);
        }
        if p == 23 && is_square(n) {
            return None;
        }
        p += 1;
        assert!(p <= 65535, "lucas_extrastrong_params: P exceeded 65535");
    }
}

/// C `is_extra_strong_lucas128` for odd `n > 2` below `u128::MAX`.
fn extra_strong_lucas(n: u128) -> bool {
    let Some(p) = extra_strong_parameter(n) else {
        return false;
    };
    let d = p * p - 4;
    let s = (n + 1).trailing_zeros();
    let odd = (n + 1) >> s;
    let ctx = Mont::new(n);
    let one = ctx.enter(1);
    let two = add_mod(one, one, n);
    let mont_p = ctx.enter(p);
    let mont_d = ctx.enter(d);
    let (mut u, mut v) = (one, mont_p);
    for bit in (0..odd.ilog2()).rev() {
        u = ctx.mul(u, v);
        v = sub_mod(ctx.sqr(v), two, n);
        if (odd >> bit) & 1 == 1 {
            let t2 = ctx.mul(u, mont_d);
            let u_next = add_mod(ctx.mul(u, mont_p), v, n);
            let v_next = add_mod(ctx.mul(v, mont_p), t2, n);
            u = half_mod(u_next, n);
            v = half_mod(v_next, n);
        }
    }
    if u == 0 && (v == two || v == n - two) {
        return true;
    }
    for r in 1..s {
        if v == 0 {
            return true;
        }
        if r + 1 < s {
            v = sub_mod(ctx.sqr(v), two, n);
        }
    }
    false
}

/// C `is_bpsw128`: no trial division; `n` should be odd.
fn is_bpsw(n: u128) -> bool {
    if n < 7 {
        return matches!(n, 2 | 3 | 5);
    }
    if n & 1 == 0 || n == u128::MAX {
        return false;
    }
    miller_rabin(n, 2) && extra_strong_lucas(n)
}

/// Tests whether `n` is prime, on the 128-bit path of `is_prime`.
///
/// Port of MPU 0.75 `factor128.c::is_prime128` (trial division by the
/// primes to 2011, then `is_bpsw128`: a base-2 Miller-Rabin test and an
/// extra-strong Lucas test, both in Montgomery arithmetic). A value that fits
/// in 64 bits goes to [`is_prime`](crate::is_prime). The result follows that
/// function: 0 for composite, 2 for a proven prime, and 1 for a probable
/// prime, which is what every prime above 64 bits gets.
///
/// ```
/// use math_prime_util::is_prime_u128;
/// assert_eq!(is_prime_u128((1u128 << 127) - 1), 1);
/// assert_eq!(is_prime_u128(1u128 << 100), 0);
/// assert_eq!(is_prime_u128(97), 2);
/// ```
#[must_use]
pub fn is_prime_u128(n: u128) -> u8 {
    if let Ok(small) = u64::try_from(n) {
        return crate::is_prime(small);
    }
    if n & 1 == 0 {
        return 0;
    }
    for &p in &small_primes()[1..] {
        if n % u128::from(p) == 0 {
            return 0;
        }
    }
    u8::from(is_bpsw(n))
}

/// State of one SQUFOF multiplier in the race (C `squfof128_mult_t`).
#[derive(Clone, Copy, Default)]
struct Squfof {
    valid: i8,
    p: u64,
    bn: u64,
    qn: u64,
    q0: u64,
    b0: u64,
    it: u64,
    imax: u64,
}

impl Squfof {
    fn step(&mut self) {
        let t1 = self.p;
        self.p = self.bn.wrapping_mul(self.qn).wrapping_sub(self.p);
        let t2 = self.qn;
        self.qn = self
            .q0
            .wrapping_add(self.bn.wrapping_mul(t1.wrapping_sub(self.p)));
        self.q0 = t2;
        self.bn = (self.b0 + self.p) / self.qn;
        self.it += 1;
    }

    /// One slice of the race (C `squfof_unit128`): a factor of `nn` or 0.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // C casts.
    #[allow(clippy::cast_precision_loss)] // The square test is a float filter.
    fn unit(&mut self, nn: u128) -> u64 {
        let imax = self.it + self.imax;
        if self.it & 1 == 1 {
            self.step();
        }
        let root = loop {
            if self.it >= imax {
                return 0;
            }
            self.step();
            let t2 = self.qn & 127;
            if t2.wrapping_mul(0x8bc4_0d7d) & t2.wrapping_mul(0xa1e2_f5d1) & 0x14_020a == 0 {
                let t1 = u64::from((self.qn as f64).sqrt() as u32);
                if self.qn == t1.wrapping_mul(t1) {
                    break t1;
                }
            }
            self.step();
        };
        let mut s = root;
        let mut ro = self
            .p
            .wrapping_add(s.wrapping_mul(self.b0.wrapping_sub(self.p) / s));
        let mut so = ((nn - u128::from(ro) * u128::from(ro)) / u128::from(s)) as u64;
        let mut bbn = (self.b0 + ro) / so;
        let mut j = 0u64;
        'walk: loop {
            for _ in 0..4 {
                let t1 = ro;
                ro = bbn.wrapping_mul(so).wrapping_sub(ro);
                if ro == t1 {
                    break 'walk;
                }
                let t2 = so;
                so = s.wrapping_add(bbn.wrapping_mul(t1.wrapping_sub(ro)));
                s = t2;
                bbn = (self.b0 + ro) / so;
            }
            let over = j > self.imax;
            j += 1;
            if over {
                self.valid = 0;
                return 0;
            }
        }
        let g = gcd(u128::from(ro), nn) as u64;
        if g > 1 { g } else { 0 }
    }
}

/// Multiplier table of Gower and Wagstaff, plus the 1680 series.
const SQUFOF_MULTIPLIERS: [u64; 58] = [
    33 * 1680,
    11 * 1680,
    66 * 1680,
    3 * 1680,
    2 * 1680,
    6 * 1680,
    22 * 1680,
    78 * 1680,
    1680,
    26 * 1680,
    39 * 1680,
    13 * 1680,
    102 * 1680,
    30 * 1680,
    34 * 1680,
    10 * 1680,
    15 * 1680,
    51 * 1680,
    5 * 1680,
    57 * 1680,
    17 * 1680,
    19 * 1680,
    3 * 5 * 7 * 11,
    3 * 5 * 7,
    3 * 5 * 7 * 11 * 13,
    3 * 5 * 7 * 13,
    3 * 5 * 7 * 11 * 17,
    3 * 5 * 11,
    3 * 5 * 7 * 17,
    3 * 5,
    3 * 5 * 7 * 11 * 19,
    3 * 5 * 11 * 13,
    3 * 5 * 7 * 19,
    3 * 5 * 7 * 13 * 17,
    3 * 5 * 13,
    3 * 7 * 11,
    3 * 7,
    5 * 7 * 11,
    3 * 7 * 13,
    5 * 7,
    3 * 5 * 17,
    5 * 7 * 13,
    3 * 5 * 19,
    3 * 11,
    3 * 7 * 17,
    3,
    3 * 11 * 13,
    5 * 11,
    3 * 7 * 19,
    3 * 13,
    5,
    5 * 11 * 13,
    5 * 7 * 19,
    5 * 13,
    7 * 11,
    7,
    3 * 17,
    7 * 13,
];

/// C `squfof128`: a nontrivial factor of `n` below 126 bits, within `rounds`.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // C casts.
#[allow(clippy::cast_precision_loss)] // Bit counts are tiny.
fn squfof(n: u128, rounds: u64) -> Option<u128> {
    const MAX_NN: u128 = (1 << 126) - 1;
    let mut ms = [Squfof {
        valid: -1,
        ..Squfof::default()
    }; SQUFOF_MULTIPLIERS.len()];
    let count = SQUFOF_MULTIPLIERS.len();
    let mut racing = count;
    let mut done = 0u64;
    while racing > 0 && done < rounds {
        let mut i = 0;
        while i < count && done < rounds {
            if ms[i].valid == 0 {
                i += 1;
                continue;
            }
            let mult = SQUFOF_MULTIPLIERS[i];
            if ms[i].valid == -1 && n > MAX_NN / u128::from(mult) {
                ms[i].valid = 0;
                racing -= 1;
                i += 1;
                continue;
            }
            let nn = n * u128::from(mult);
            if ms[i].valid == -1 {
                let sq = nn.isqrt() as u64;
                let rem = nn - u128::from(sq) * u128::from(sq);
                if rem == 0 {
                    let f = gcd(u128::from(sq), n) as u64;
                    if f > 1 && u128::from(f) < n {
                        return Some(u128::from(f));
                    }
                    // The C divides by zero here; retire the multiplier.
                    ms[i].valid = 0;
                    racing -= 1;
                    i += 1;
                    continue;
                }
                let fbits = f64::from(nn.ilog2());
                let imax = ((0.5 * (fbits / 5.0).exp2()) as u64).max(20);
                ms[i] = Squfof {
                    valid: 1,
                    q0: 1,
                    b0: sq,
                    p: sq,
                    qn: rem as u64,
                    bn: (2 * sq) / rem as u64,
                    it: 0,
                    imax,
                };
            }
            if racing == 1 || ms[i].imax > rounds - done {
                ms[i].imax = rounds - done;
            }
            let f64v = ms[i].unit(nn);
            if f64v > 1 {
                let g = gcd(u128::from(f64v), u128::from(mult)) as u64;
                let reduced = f64v / g;
                if reduced > 1 {
                    let factor = gcd(u128::from(reduced), n);
                    if factor > 1 && factor < n {
                        return Some(factor);
                    }
                }
                ms[i].valid = 0;
            }
            if ms[i].valid == 0 {
                racing -= 1;
            }
            done += ms[i].imax;
            i += 1;
        }
    }
    None
}

/// C `pminus1_128`: Pollard p-1, stage 1 and stage 2, in Montgomery form.
#[allow(clippy::too_many_lines, clippy::explicit_counter_loop)] // Mirrors the C.
fn pminus1(n: u128, b1: u64, b2: u64) -> Option<u128> {
    if b1 < 7 {
        return None;
    }
    let ctx = Mont::new(n);
    let primes = primes_upto(b2.max(b1));
    let nm1 = |a: u128| {
        let an = ctx.exit(a);
        gcd(if an > 0 { an - 1 } else { n - 1 }, n)
    };
    let sqrt_b1 = b1.isqrt();
    let stage1_exp = |p: u64| {
        let mut k = p;
        if p <= sqrt_b1 {
            let km = b1 / p;
            while k <= km {
                k *= p;
            }
        }
        u128::from(k)
    };
    let stage1: Vec<u64> = primes
        .iter()
        .map(|&p| u64::from(p))
        .take_while(|&p| p <= b1)
        .collect();
    let mut a = ctx.enter(2);
    let mut save_a = a;
    let mut save_idx = 0usize; // Number of primes already folded into save_a.
    let mut j = 15u32;
    let mut backtrack = false;
    for (idx, &p) in stage1.iter().enumerate() {
        a = ctx.pow(a, stage1_exp(p));
        let check = j % 32 == 0;
        j += 1;
        if check {
            let g = nm1(a);
            if g == n {
                backtrack = true;
                break;
            }
            if g > 1 {
                return Some(g);
            }
            save_idx = idx + 1;
            save_a = a;
        }
    }
    if !backtrack {
        let g = nm1(a);
        if g == n {
            backtrack = true;
        } else if g > 1 {
            return Some(g);
        }
    }
    if backtrack {
        a = save_a;
        for &p in &stage1[save_idx..] {
            a = ctx.pow(a, stage1_exp(p));
            let g = nm1(a);
            if g == n {
                return None;
            }
            if g > 1 {
                return Some(g);
            }
        }
        return None;
    }

    if b2 <= b1 {
        return None;
    }
    // Stage 2: bm = a; bm^gap for the common prime gaps is cached lazily.
    let bm = a;
    let bm_sq = ctx.sqr(bm);
    let mut bm_pow = [0u128; 111];
    let mut bm_init = [false; 111];
    bm_pow[0] = bm_sq;
    bm_init[0] = true;
    for ii in 1..22 {
        bm_pow[ii] = ctx.mul(bm_pow[ii - 1], bm_sq);
        bm_init[ii] = true;
    }
    let stage2: Vec<u64> = primes
        .iter()
        .map(|&p| u64::from(p))
        .skip_while(|&p| p <= b1)
        .collect();
    let mut q = *stage2.first()?;
    a = ctx.pow(a, u128::from(q));
    let one = ctx.enter(1);
    let mut b = one;
    let mut j = 31u32;
    // The C loop starts at q itself, with a zero gap and so a unit step.
    for &p in &stage2 {
        let gap = p - q;
        let step = match usize::try_from(gap / 2).unwrap_or(usize::MAX) {
            0 => one,
            half @ 1..=111 => {
                if !bm_init[half - 1] {
                    bm_pow[half - 1] = ctx.pow(bm, u128::from(gap));
                    bm_init[half - 1] = true;
                }
                bm_pow[half - 1]
            }
            _ => ctx.pow(bm, u128::from(gap)),
        };
        a = ctx.mul(a, step);
        b = ctx.mul(b, sub_mod(a, one, n));
        let check = j % 64 == 0;
        j += 1;
        if check {
            let g = gcd(ctx.exit(b), n);
            if g > 1 && g < n {
                return Some(g);
            }
            if g == n {
                break;
            }
        }
        q = p;
    }
    let g = gcd(ctx.exit(b), n);
    (g > 1 && g < n).then_some(g)
}

/// Projective `(X:Z)` point on a Montgomery curve, in Montgomery form.
#[derive(Clone, Copy)]
struct Point {
    x: u128,
    z: u128,
}

/// `2P` on `By^2 = x^3 + Ax^2 + x` with `a24 = (A + 2) / 4`.
fn ecm_double(p: Point, a24: u128, ctx: &Mont) -> Point {
    let n = ctx.n;
    let u = ctx.sqr(sub_mod(p.x, p.z, n));
    let v = ctx.sqr(add_mod(p.x, p.z, n));
    let w = sub_mod(v, u, n);
    Point {
        x: ctx.mul(u, v),
        z: ctx.mul(w, add_mod(u, ctx.mul(a24, w), n)),
    }
}

/// Differential addition `P + Q` given `pm = P - Q`.
fn ecm_dadd(p: Point, q: Point, pm: Point, ctx: &Mont) -> Point {
    let n = ctx.n;
    let u = ctx.mul(sub_mod(p.x, p.z, n), add_mod(q.x, q.z, n));
    let v = ctx.mul(add_mod(p.x, p.z, n), sub_mod(q.x, q.z, n));
    let s = add_mod(u, v, n);
    let d = sub_mod(u, v, n);
    Point {
        x: ctx.mul(ctx.sqr(s), pm.z),
        z: ctx.mul(ctx.sqr(d), pm.x),
    }
}

/// Montgomery ladder `k * P`.
fn ecm_mul(p: Point, k: u64, a24: u128, ctx: &Mont) -> Point {
    if k == 1 {
        return p;
    }
    let mut r0 = p;
    let mut r1 = ecm_double(r0, a24, ctx);
    if k == 2 {
        return r1;
    }
    for bit in (0..k.ilog2()).rev() {
        if (k >> bit) & 1 == 1 {
            r0 = ecm_dadd(r0, r1, p, ctx);
            r1 = ecm_double(r1, a24, ctx);
        } else {
            r1 = ecm_dadd(r0, r1, p, ctx);
            r0 = ecm_double(r0, a24, ctx);
        }
    }
    r0
}

/// Fixed sigma values for Suyama's parameterization (C `ecm_sigmas`).
const ECM_SIGMAS: [u16; 400] = [
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

/// Batch conversion to affine x, still in Montgomery form
/// (C `ecm_batch_normalize_x128`).  `Err(f)` carries a factor, or 0 to
/// abandon the curve.
fn ecm_normalize(points: &[Point], ctx: &Mont) -> Result<Vec<u128>, u128> {
    let n = ctx.n;
    let one = ctx.enter(1);
    let mut acc = one;
    let prefix: Vec<u128> = points
        .iter()
        .map(|pt| {
            acc = ctx.mul(acc, pt.z);
            acc
        })
        .collect();
    let g = gcd(acc, n);
    if g > 1 {
        let mut found = if g < n { g } else { 0 };
        if g == n {
            if let Some(h) = points
                .iter()
                .map(|pt| gcd(pt.z, n))
                .find(|&h| h > 1 && h < n)
            {
                found = h;
            }
        }
        return Err(found);
    }
    let inv = mod_inv(ctx.exit(acc), n);
    if inv == 0 {
        return Err(0);
    }
    let mut inv = ctx.enter(inv);
    let mut xs = vec![0; points.len()];
    for j in (0..points.len()).rev() {
        let prev = if j == 0 { one } else { prefix[j - 1] };
        let zinv = ctx.mul(inv, prev);
        inv = ctx.mul(inv, points[j].z);
        xs[j] = ctx.mul(points[j].x, zinv);
    }
    Ok(xs)
}

/// Brent-Suyama ECM stage 2 (C `tinyecm128_stage2`); `is_prime` is a flag
/// table that covers `0..=b2`.
#[allow(clippy::comparison_chain, clippy::too_many_lines)] // Mirrors the C.
fn ecm_stage2(q: Point, a24: u128, ctx: &Mont, b1: u64, b2: u64, flags: &[bool]) -> u128 {
    let n = ctx.n;
    if b2 <= b1 || b2 < 2 {
        return 0;
    }
    let mut d = (b2 >> 1).isqrt();
    if d & 1 == 1 {
        d += 1;
    }
    let two_d = 2 * d;
    let mend = b2 + d;
    let nwindows = 1 + (mend - 2) / two_d;
    let size = |v: u64| usize::try_from(v).expect("ECM window fits memory");
    let (d_us, two_d_us) = (size(d), size(two_d));
    let mut nq = vec![Point { x: 0, z: 0 }; two_d_us + 1];
    nq[1] = q;
    for i in 2..=two_d_us {
        nq[i] = if i & 1 == 1 {
            ecm_dadd(nq[i / 2], nq[i / 2 + 1], q, ctx)
        } else {
            ecm_double(nq[i / 2], a24, ctx)
        };
    }
    let mut nqx = vec![0u128; d_us + 1];
    match ecm_normalize(&nq[1..=d_us], ctx) {
        Ok(xs) => nqx[1..].copy_from_slice(&xs),
        Err(f) => return f,
    }
    let mut s = vec![q];
    let mut xm = nq[two_d_us - 1];
    for w in 1..size(nwindows) {
        let old = s[w - 1];
        s.push(ecm_dadd(nq[two_d_us], s[w - 1], xm, ctx));
        xm = old;
    }
    let sx = match ecm_normalize(&s, ctx) {
        Ok(xs) => xs,
        Err(f) => return f,
    };
    let mut gprod = ctx.enter(1);
    let mut m = 1u64;
    for (w, &sxw) in sx.iter().enumerate() {
        let hi = if m > b2 || b2 - m < d { b2 } else { m + d };
        let mut lo = m.saturating_sub(d);
        if hi > b1 {
            if lo <= b1 {
                lo = b1 + 1;
            }
            lo = lo.max(2);
            if lo <= hi {
                for p in lo..=hi {
                    if !flags[size(p)] {
                        continue;
                    }
                    let idx = if p < m {
                        m - p
                    } else if p > m {
                        if m <= u64::MAX / 2 {
                            let mm = 2 * m;
                            if p <= mm {
                                let mirror = mm - p;
                                if mirror > b1 && mirror >= lo && flags[size(mirror)] {
                                    continue;
                                }
                            }
                        }
                        p - m
                    } else {
                        continue;
                    };
                    if idx <= d {
                        gprod = ctx.mul(gprod, sub_mod(sxw, nqx[size(idx)], n));
                    }
                }
                let f = gcd(ctx.exit(gprod), n);
                if f > 1 {
                    return if f < n { f } else { 0 };
                }
            }
        }
        if w + 1 < sx.len() {
            m += two_d;
        }
    }
    let f = gcd(ctx.exit(gprod), n);
    if f > 1 && f < n { f } else { 0 }
}

/// C `tinyecm128`: a nontrivial factor of `n` from up to `ncurves` curves.
#[allow(clippy::cast_possible_truncation)] // The sieve limit fits memory.
fn tinyecm(n: u128, b1: u64, b2: u64, ncurves: usize, sigma_offset: usize) -> Option<u128> {
    let ctx = Mont::new(n);
    let sqrt_b1 = b1.isqrt();
    let flags = prime_flags(b2.max(b1));
    let primes: Vec<u64> = (2..=b1).filter(|&p| flags[p as usize]).collect();
    for &sigma in ECM_SIGMAS.iter().skip(sigma_offset).take(ncurves) {
        let sigma = u64::from(sigma);
        let u = sigma * sigma - 5;
        let v = 4 * sigma;
        let umv = u - v;
        let t3uv = 3 * u + v;
        let u3 = u128::from(u) * u128::from(u) * u128::from(u);
        let v3 = u128::from(v) * u128::from(v) * u128::from(v);
        let umv3 = u128::from(umv) * u128::from(umv) * u128::from(umv);
        let abs_num_r = (umv3 * u128::from(t3uv)) % n;
        let num = if abs_num_r == 0 { 0 } else { n - abs_num_r };
        let den_r = (16 * u3 * u128::from(v)) % n;
        let den_inv = mod_inv(den_r, n);
        if den_inv == 0 {
            let g = gcd(den_r, n);
            if g > 1 && g < n {
                return Some(g);
            }
            continue;
        }
        let a24 = ctx.mul(ctx.enter(num), ctx.enter(den_inv));
        let mut pt = Point {
            x: ctx.enter(u3),
            z: ctx.enter(v3),
        };
        let mut abandoned = false;
        for (j, &p) in primes.iter().enumerate() {
            let mut k = p;
            if p <= sqrt_b1 {
                let pm = b1 / p;
                while k <= pm {
                    k *= p;
                }
            }
            pt = ecm_mul(pt, k, a24, &ctx);
            if j % 64 == 0 {
                let g = gcd(pt.z, n);
                if g > 1 && g < n {
                    return Some(g);
                }
                if g == n {
                    abandoned = true;
                    break;
                }
            }
        }
        if abandoned {
            continue;
        }
        let g = gcd(pt.z, n);
        if g > 1 && g < n {
            return Some(g);
        }
        if b2 > b1 {
            let g = ecm_stage2(pt, a24, &ctx, b1, b2, &flags);
            if g > 1 && g < n {
                return Some(g);
            }
        }
    }
    None
}

/// Factors `n` on the 128-bit path of `factor`, in ascending order.
///
/// Port of MPU 0.75 `factor128.c::factorintp128`: trial division by the
/// primes to 2011, then a stack of cofactors. A cofactor that fits in 64
/// bits goes to [`factor`](fn@crate::factor); a BPSW probable prime is kept; any
/// other cofactor is split by Pollard p-1 (B1 20000, B2 200000), then SQUFOF
/// (below 79 bits), then tiny ECM at the C's six `B1`/`B2`/curve tiers.
/// Zero returns `[0]` and one returns `[]`, as `factor` does.
///
/// # Errors
/// MPU dies with `internal: factorintp128 failed to factor` when every
/// stage fails. This returns the unsplit cofactor as the error instead.
///
/// ```
/// use math_prime_util::factor_u128;
/// assert_eq!(factor_u128(360), Ok(vec![2, 2, 2, 3, 3, 5]));
/// let p = 18_446_744_073_709_551_557u128; // the largest prime below 2^64
/// assert_eq!(factor_u128(p * p), Ok(vec![p, p]));
/// ```
pub fn factor_u128(n: u128) -> Result<Vec<u128>, u128> {
    if n < 4 {
        return Ok(match n {
            1 => Vec::new(),
            _ => vec![n],
        });
    }
    let mut n = n;
    let mut factors = Vec::new();
    for &p in small_primes() {
        let p = u128::from(p);
        if p * p > n {
            break;
        }
        while n % p == 0 {
            factors.push(p);
            n /= p;
        }
    }
    if n == 1 {
        return Ok(factors);
    }
    let mut stack = vec![n];
    while let Some(t) = stack.pop() {
        if let Ok(small) = u64::try_from(t) {
            factors.extend(crate::factor(small).into_iter().map(u128::from));
            continue;
        }
        if is_bpsw(t) {
            factors.push(t);
            continue;
        }
        let f = split(t).ok_or(t)?;
        stack.push(f);
        stack.push(t / f);
    }
    factors.sort_unstable();
    Ok(factors)
}

/// The splitter chain of `factorintp128`.
fn split(t: u128) -> Option<u128> {
    if let Some(f) = pminus1(t, 20_000, 200_000) {
        return Some(f);
    }
    if t >> 79 == 0 {
        let bits = f64::from(t.ilog2());
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // C casts.
        let rounds = ((10.0 * (bits / 5.0).exp2()) as u64).clamp(300_000, 5_000_000);
        if let Some(f) = squfof(t, rounds) {
            return Some(f);
        }
    }
    let mut curves = 0;
    for (b1k, b2mult, count) in [
        (2, 20, 40),
        (10, 20, 40),
        (40, 20, 40),
        (80, 20, 40),
        (100, 40, 120),
        (800, 40, 80),
    ] {
        let b1 = b1k * 1000;
        if let Some(f) = tinyecm(t, b1, b1 * b2mult, count, curves) {
            return Some(f);
        }
        curves += count;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const BIG: u128 = u128::MAX - 158; // The largest prime below 2^128.
    const SQ: u128 = 18_446_744_073_709_551_557; // The largest prime below 2^64.
    const P64: u128 = 18_446_744_073_709_551_629; // A prime just above 2^64.

    #[test]
    fn modular_helpers() {
        assert_eq!(add_mod(BIG - 1, BIG - 1, BIG), BIG - 2);
        assert_eq!(add_mod(3, 4, 11), 7);
        assert_eq!(sub_mod(0, 1, BIG), BIG - 1);
        assert_eq!(sub_mod(5, 2, 11), 3);
        assert_eq!(half_mod(BIG - 2, BIG), BIG - 1);
        assert_eq!(half_mod(4, BIG), 2);
        assert_eq!(mul_mod(BIG - 1, BIG - 1, BIG), 1);
        assert_eq!(mul_mod(3, 5, 7), 1);
        assert_eq!(mul_mod(1 << 100, 1 << 100, BIG), pow2_mod(200, BIG));
        assert_eq!(pow2_mod(3, 5), 3);
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(1 << 100, 1 << 70), 1 << 70);
        assert_eq!(mod_inv(0, 7), 0);
        assert_eq!(mod_inv(6, 9), 0);
        assert_eq!(mul_mod(mod_inv(12_345, BIG), 12_345, BIG), 1);
        assert_eq!(mul_mod(mod_inv(BIG - 5, BIG), BIG - 5, BIG), 1);
        assert_eq!(mul_mod(mod_inv(1 << 90, BIG), 1 << 90, BIG), 1);
        assert_eq!(mul_mod(mod_inv(7, 15), 7, 15), 1);
    }

    #[test]
    fn full_product() {
        assert_eq!(mul256(3, 5), (15, 0));
        assert_eq!(mul256(u128::MAX, u128::MAX), (1, u128::MAX - 1));
        assert_eq!(mul256(1 << 127, 2), (0, 1));
        assert_eq!(mul256(u128::MAX, 2), (u128::MAX - 1, 1));
    }

    #[test]
    fn montgomery_matches_plain_arithmetic() {
        for n in [BIG, (1 << 127) + 1, 10_007 * 10_009, P64 * 3, 3] {
            let ctx = Mont::new(n);
            for (a, b) in [(2, 3), (n - 1, n - 1), (n / 2, n / 3), (1, n - 1), (0, 5)] {
                let (a, b) = (a % n, b % n);
                let got = ctx.exit(ctx.mul(ctx.enter(a), ctx.enter(b)));
                assert_eq!(got, mul_mod(a, b, n), "{a} * {b} mod {n}");
                assert_eq!(ctx.exit(ctx.sqr(ctx.enter(a))), mul_mod(a, a, n));
            }
            assert_eq!(ctx.exit(ctx.pow(ctx.enter(2), 10)), 1024 % n);
            assert_eq!(ctx.exit(ctx.pow(ctx.enter(5), 0)), 1);
        }
    }

    #[test]
    fn jacobi_and_squares() {
        assert_eq!(jacobi(0, 3), 0);
        assert_eq!(jacobi(3, 7), -1);
        assert_eq!(jacobi(2, 7), 1);
        assert_eq!(jacobi(2, 5), -1);
        assert_eq!(jacobi(6, 9), 0);
        assert!(is_square(0));
        assert!(is_square(u128::from(u64::MAX).pow(2)));
        assert!(!is_square(u128::MAX));
    }

    #[test]
    fn primality_pieces() {
        assert!(!miller_rabin(2047, 3) && miller_rabin(10_007, 3));
        assert!(!miller_rabin(2047 * 3, 2));
        assert!(miller_rabin(2047, 2)); // 2047 is a base-2 strong pseudoprime.
        assert!(!extra_strong_lucas(2047));
        assert!(miller_rabin(BIG, 2) && extra_strong_lucas(BIG));
        assert!(!extra_strong_lucas(SQ * SQ)); // A square: no parameter found.
        assert_eq!(extra_strong_parameter(SQ * SQ), None);
        assert_eq!(extra_strong_parameter(5 * P64), None); // gcd(5, n) is nontrivial.
        assert_eq!(extra_strong_parameter(5), Some(3 + 1)); // D = 5 shares all of n.
        assert_eq!(
            extra_strong_parameter(P64),
            Some(extra_strong_parameter(P64).unwrap())
        );
        for n in [10_007, 10_009, 65_537, 104_729, (1 << 89) - 1] {
            assert!(extra_strong_lucas(n), "{n}");
        }
        for n in [2u128, 3, 5] {
            assert!(is_bpsw(n));
        }
        assert!(!is_bpsw(4) && !is_bpsw(1 << 100) && !is_bpsw(u128::MAX));
        assert!(is_bpsw(P64) && is_bpsw(BIG));
        assert!(!is_bpsw(P64 * 4_294_967_311));
    }

    #[test]
    fn is_prime_u128_cases() {
        assert_eq!(is_prime_u128(97), 2);
        assert_eq!(is_prime_u128(1 << 100), 0);
        assert_eq!(is_prime_u128((1 << 127) - 1), 1);
        assert_eq!(is_prime_u128(BIG), 1);
        assert_eq!(is_prime_u128(u128::MAX), 0);
        assert_eq!(is_prime_u128(P64 * 3), 0);
        assert_eq!(is_prime_u128(SQ * SQ), 0);
        assert_eq!(is_prime_u128(P64 * 4_294_967_311), 0);
        assert_eq!(is_prime_u128(P64), 1);
    }

    #[test]
    fn squfof_splits_balanced_semiprimes() {
        let n = 4_293_801_221u128 * 8_578_073_143;
        let f = squfof(n, 5_000_000).unwrap();
        assert!(f > 1 && f < n && n % f == 0);
        assert_eq!(squfof(n, 0), None);
        assert_eq!(squfof(n, 3), None);
        // Too large for any multiplier.
        assert_eq!(squfof(BIG, 1_000), None);
        // n * 55440 is a perfect square: the gcd shortcut.
        let c = 1u128 << 50;
        let n = 385 * c * c;
        let f = squfof(n, 1_000_000).unwrap();
        assert!(f > 1 && f < n && n % f == 0);
    }

    #[test]
    fn pminus1_stages() {
        let n = 4_284_736_327u128 * 8_589_539_549;
        assert_eq!(pminus1(n, 6, 100), None);
        let f = pminus1(n, 20_000, 200_000).unwrap();
        assert!(f > 1 && f < n && n % f == 0);
        assert_eq!(pminus1(n, 100, 100), None); // No stage 2.
        assert_eq!(pminus1(n, 100, 50), None);
        // Prime p with p - 1 = 2 * 3 * 5 * 7 * 11 * 13 * 17 * ... has a smooth order.
        for (a, b) in [(1009u128, 1013u128), (7, 11), (101, 103), (10_007, 10_009)] {
            let n = a * b;
            for b1 in [7, 8, 20, 30, 100, 1000] {
                if let Some(f) = pminus1(n, b1, 5 * b1) {
                    assert!(f > 1 && f < n && n % f == 0);
                }
            }
        }
    }

    #[test]
    fn ecm_finds_and_fails() {
        let n = 17_380_103_011u128 * 34_258_953_431;
        let f = tinyecm(n, 2_000, 40_000, 40, 0).unwrap();
        assert!(f > 1 && f < n && n % f == 0);
        assert_eq!(tinyecm(n, 11, 11, 1, 0), None);
        assert_eq!(tinyecm(n, 11, 12, 1, 0), None);
        assert_eq!(tinyecm(n, 2_000, 40_000, 0, 0), None);
        // Sigma 11 gives u = 116 and v = 44, so 29 and 11 show up in the denominator.
        let f = tinyecm(29 * 100_003, 100, 1_000, 1, 0).unwrap();
        assert!(f == 29 || f == 100_003);
        assert_eq!(tinyecm(11 * 29, 100, 1_000, 1, 0), None);
        for m in [BIG, SQ * SQ, 3 * 5 * 7 * 11 * 13, 1_000_003 * 999_983] {
            if let Some(f) = tinyecm(m | 1, 300, 6_000, 8, 0) {
                assert!(f > 1 && f < (m | 1) && (m | 1) % f == 0);
            }
        }
    }

    #[test]
    fn factor_u128_cases() {
        assert_eq!(factor_u128(0), Ok(vec![0]));
        assert_eq!(factor_u128(1), Ok(vec![]));
        assert_eq!(factor_u128(2), Ok(vec![2]));
        assert_eq!(factor_u128(3), Ok(vec![3]));
        assert_eq!(factor_u128(12), Ok(vec![2, 2, 3]));
        assert_eq!(factor_u128(2011), Ok(vec![2011]));
        assert_eq!(factor_u128(P64), Ok(vec![P64]));
        assert_eq!(factor_u128(BIG), Ok(vec![BIG]));
        assert_eq!(factor_u128(SQ * SQ), Ok(vec![SQ, SQ]));
        assert_eq!(factor_u128(P64 * 6), Ok(vec![2, 3, P64]));
        assert_eq!(factor_u128(1 << 127), Ok(vec![2; 127]));
        let q = 4_294_967_311u128;
        assert_eq!(factor_u128(P64 * q), Ok(vec![q, P64]));
        assert_eq!(
            factor_u128(u128::MAX),
            Ok(vec![
                3,
                5,
                17,
                257,
                641,
                65_537,
                274_177,
                6_700_417,
                67_280_421_310_721
            ])
        );
    }

    #[test]
    fn small_composites_exercise_every_exit() {
        for n in (15u128..6_000).step_by(2) {
            if is_prime_u128(n) != 0 {
                continue;
            }
            for b1 in [7, 10, 40, 100] {
                if let Some(f) = pminus1(n, b1, 20 * b1) {
                    assert!(f > 1 && f < n && n % f == 0, "pminus1 {n}");
                }
            }
            for offset in [0, 3] {
                if let Some(f) = tinyecm(n, 60, 1_200, 3, offset) {
                    assert!(f > 1 && f < n && n % f == 0, "ecm {n}");
                }
            }
            if let Some(f) = squfof(n, 400) {
                assert!(f > 1 && f < n && n % f == 0, "squfof {n}");
            }
        }
    }

    #[test]
    fn ecm_helpers() {
        let n = 10_007u128 * 10_009;
        let ctx = Mont::new(n);
        let a24 = ctx.enter(12_345);
        let p = Point {
            x: ctx.enter(3),
            z: ctx.enter(7),
        };
        assert_eq!(ecm_mul(p, 1, a24, &ctx).x, p.x);
        let two = ecm_double(p, a24, &ctx);
        assert_eq!(ecm_mul(p, 2, a24, &ctx).z, two.z);
        let three = ecm_dadd(two, p, p, &ctx);
        assert_eq!(ecm_mul(p, 3, a24, &ctx).x, three.x);
        assert_eq!(ecm_stage2(p, a24, &ctx, 100, 100, &[]), 0);
        // A point whose Z shares a factor with n exposes it; two such points
        // make the product zero and are checked one by one.
        let z = |v| Point {
            x: ctx.enter(1),
            z: v,
        };
        assert_eq!(ecm_normalize(&[z(10_007)], &ctx), Err(10_007));
        assert_eq!(ecm_normalize(&[z(10_007), z(10_009)], &ctx), Err(10_007));
        assert_eq!(ecm_normalize(&[z(0)], &ctx), Err(0));
        assert!(ecm_normalize(&[z(ctx.enter(5)), z(ctx.enter(6))], &ctx).is_ok());
    }
}

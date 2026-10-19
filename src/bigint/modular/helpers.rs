//! Additional modular helpers from MPU 0.75.
use super::{
    BigInt, BigUint, add, allsqrtmod, big, chinese, divide, factor_exp, gcd, is_one, is_prime,
    is_square, is_zero, kronecker, lucas_sequence, mul, one, pow, rootmod, roots_pk, sqrtint, sub,
    two, zero,
};

/// Return `(a - b) mod n`. Return `None` for modulus zero.
/// Port of `reference/mpu/mulmod.h::submod`.
/// ```
/// use math_prime_util::bigint::modular::submod;
/// assert_eq!(submod(&2u8.into(), &3u8.into(), &7u8.into()), Some(6u8.into()));
/// ```
pub fn submod(a: &BigUint, b: &BigUint, n: &BigUint) -> Option<BigUint> {
    (!is_zero(n)).then(|| sub(a, b, n))
}

/// Return `-a mod n`. Return `None` for modulus zero.
/// Port of `reference/mpu/mulmod.h::negmod`.
/// ```
/// use math_prime_util::bigint::modular::negmod;
/// assert_eq!(negmod(&2u8.into(), &7u8.into()), Some(5u8.into()));
/// ```
pub fn negmod(a: &BigUint, n: &BigUint) -> Option<BigUint> {
    submod(&zero(), a, n)
}

/// Return `(a*b + c) mod n`. Return `None` for modulus zero.
/// Port of `reference/mpu/mulmod.h::muladdmod`.
/// ```
/// use math_prime_util::bigint::modular::muladdmod;
/// assert_eq!(muladdmod(&2u8.into(), &3u8.into(), &4u8.into(), &7u8.into()), Some(3u8.into()));
/// ```
pub fn muladdmod(a: &BigUint, b: &BigUint, c: &BigUint, n: &BigUint) -> Option<BigUint> {
    (!is_zero(n)).then(|| (a * b + c) % n)
}

/// Return `(a*b - c) mod n`. Return `None` for modulus zero.
/// Port of `reference/mpu/mulmod.h::mulsubmod`.
/// ```
/// use math_prime_util::bigint::modular::mulsubmod;
/// assert_eq!(mulsubmod(&2u8.into(), &3u8.into(), &4u8.into(), &7u8.into()), Some(2u8.into()));
/// ```
pub fn mulsubmod(a: &BigUint, b: &BigUint, c: &BigUint, n: &BigUint) -> Option<BigUint> {
    (!is_zero(n)).then(|| sub(&(a * b), c, n))
}

/// Return the least CRT solution and the combined modulus.
/// Return `None` for zero moduli or inconsistent congruences.
/// Port of `reference/mpu/util.c::chinese` and `_simple_chinese`.
/// ```
/// use math_prime_util::bigint::modular::chinese2;
/// assert_eq!(chinese2(&[(2u8.into(), 3u8.into()), (3u8.into(), 5u8.into())]), Some((8u8.into(), 15u8.into())));
/// ```
pub fn chinese2(pairs: &[(BigUint, BigUint)]) -> Option<(BigUint, BigUint)> {
    let result = chinese(pairs)?;
    let mut modulus = if pairs.is_empty() { zero() } else { one() };
    for (_, n) in pairs {
        modulus = &modulus / gcd(&modulus, n) * n;
    }
    Some((result, modulus))
}

/// Return the Lucas values `(U_k, V_k)` modulo `n`.
/// Port of `reference/mpu/lucas_seq.c::lucasuvmod`.
/// # Panics
/// Panics if `n` is zero.
/// ```
/// use math_prime_util::bigint::modular::lucasuvmod;
/// assert_eq!(lucasuvmod(&1.into(), &(-1).into(), &5u8.into(), &100u8.into()), (5u8.into(), 11u8.into()));
/// ```
pub fn lucasuvmod(p: &BigInt, q: &BigInt, k: &BigUint, n: &BigUint) -> (BigUint, BigUint) {
    let (u, v, _) = lucas_sequence(n, p, q, k);
    (u, v)
}

/// Test whether `a` is a square modulo `n`. Zero moduli return false.
/// Port of `reference/mpu/util.c::is_qr`.
/// ```
/// use math_prime_util::bigint::modular::is_qr;
/// assert!(is_qr(&2u8.into(), &7u8.into()));
/// ```
pub fn is_qr(a: &BigUint, n: &BigUint) -> bool {
    if is_zero(n) {
        return false;
    }
    if *n <= two() || a % n <= one() {
        return true;
    }
    factor_exp(n)
        .iter()
        .all(|(p, e)| super::sqrt_prime_power(&(a % n), p, *e).is_some())
}

/// Return the first prime quadratic nonresidue. Return `None` for zero.
/// Port of `reference/mpu/util.c::qnr`.
/// ```
/// use math_prime_util::bigint::modular::qnr;
/// assert_eq!(qnr(&7u8.into()), Some(3u8.into()));
/// ```
pub fn qnr(n: &BigUint) -> Option<BigUint> {
    if is_zero(n) {
        return None;
    }
    if *n <= two() {
        return Some(n.clone());
    }
    let symbol =
        |a: &BigUint, p: &BigUint| kronecker(&BigInt::from(a.clone()), &BigInt::from(p.clone()));
    if symbol(&two(), n) == -1 {
        return Some(two());
    }
    let mut n = n.clone();
    let mut e = 0;
    while is_zero(&(&n % 2u8)) {
        n >>= 1usize;
        e += 1;
    }
    if e >= 2 || is_one(&n) {
        return Some(two());
    }
    let factors = factor_exp(&n);
    let mut a = two();
    while a < n {
        if factors.iter().any(|(p, _)| a < *p && symbol(&a, p) == -1) {
            return Some(a);
        }
        a = super::super::primality::next_prime(&a);
    }
    None
}

/// Solve `x*x + d*y*y = n`. Return `None` if there is no solution.
/// Port of `reference/mpu/util.c::cornacchia` and `corn_one`.
/// Prime moduli use Euclidean descent. Composite moduli use MPU's scan.
/// The scan can take impractical time for large composite inputs.
/// ```
/// use math_prime_util::bigint::modular::cornacchia;
/// assert_eq!(cornacchia(&1u8.into(), &5u8.into()), Some((2u8.into(), 1u8.into())));
/// ```
pub fn cornacchia(d: &BigUint, n: &BigUint) -> Option<(BigUint, BigUint)> {
    if is_zero(n) {
        return Some((zero(), zero()));
    }
    if is_zero(d) {
        return is_square(n).then(|| (sqrtint(n), zero()));
    }
    if is_prime(n) {
        if d == n {
            return Some((zero(), one()));
        }
        let r = super::sqrtmod(&sub(&zero(), d, n), n)?;
        let (mut a, mut b) = (n.clone(), std::cmp::max(r.clone(), n - r));
        let limit = sqrtint(n);
        while b > limit {
            (a, b) = (b.clone(), a % &b);
        }
        let rest = n - &b * &b;
        if is_zero(&(&rest % d)) && is_square(&(&rest / d)) {
            return Some((b, sqrtint(&(&rest / d))));
        }
        return None;
    }
    let limit = sqrtint(&(n / d));
    let mut y = zero();
    while y <= limit {
        let rest = n - d * &y * &y;
        if is_square(&rest) {
            return Some((sqrtint(&rest), y));
        }
        y += 1u8;
    }
    None
}

fn fibonacci(k: &BigUint, n: &BigUint) -> BigUint {
    lucasuvmod(&1.into(), &(-1).into(), k, n).0
}

/// Return the Fibonacci period modulo `n`.
/// Port of `reference/mpu/util.c::_pisano_prime_power` and `pisano_period`.
/// The modulus and candidate periods must be factorable.
/// ```
/// use math_prime_util::bigint::modular::pisano_period;
/// assert_eq!(pisano_period(&10u8.into()), 60u8.into());
/// ```
pub fn pisano_period(n: &BigUint) -> BigUint {
    if *n <= one() {
        return n.clone();
    }
    let mut k = one();
    for (p, e) in factor_exp(n) {
        let base = if p == two() {
            big(3)
        } else if p == big(3) {
            big(8)
        } else if p == big(5) {
            big(20)
        } else if p == big(7) {
            big(16)
        } else if p < big(300) {
            let (mut a, mut b, mut j) = (one(), one(), one());
            while !is_zero(&a) || !is_one(&b) {
                (a, b) = (b.clone(), add(&a, &b, &p));
                j += 1u8;
            }
            j
        } else {
            let mut j = if &p % 5u8 == one() || &p % 5u8 == big(4) {
                &p - 1u8
            } else {
                &p + 1u8
            };
            for (q, e) in factor_exp(&j) {
                for _ in 0..e {
                    if !is_zero(&fibonacci(&(&j / &q), &p)) {
                        break;
                    }
                    j /= &q;
                }
            }
            j
        };
        let part = base * p.pow(e - 1);
        k = &k / gcd(&k, &part) * part;
    }
    let mut r = k.clone();
    while fibonacci(&(&r - 1u8), n) != one() {
        r += &k;
    }
    r
}

fn bounded_count(n: &BigUint) -> u64 {
    assert!(*n <= big(600_000_000), "Maximum returned roots exceeded");
    n.iter_u64_digits().next().unwrap_or(0)
}

fn prime_roots(a: &BigUint, k: &BigUint, p: &BigUint) -> Vec<BigUint> {
    let a = a % p;
    if *p == two() || is_zero(&a) {
        return vec![a];
    }
    let Some(r) = rootmod(&a, k, p) else {
        return vec![];
    };
    let g = gcd(k, &(p - 1u8));
    if is_one(&g) {
        return vec![r];
    }
    let count = bounded_count(&g);
    let mut candidate = two();
    let z = loop {
        let z = pow(&candidate, &((p - 1u8) / &g), p);
        if !is_one(&z) {
            break z;
        }
        candidate += 1u8;
    };
    let mut roots = Vec::new();
    let mut next = r;
    for _ in 0..count {
        roots.push(next.clone());
        next = mul(&next, &z, p);
    }
    roots
}

// rootmod.c::_allrootmod_prime_power, with k prime.
fn prime_power(a: &BigUint, k: &BigUint, p: &BigUint, e: u32) -> Vec<BigUint> {
    if *k == two() {
        return roots_pk(a, p, e);
    }
    if e == 1 {
        return prime_roots(a, k, p);
    }
    let n = p.pow(e);
    if is_zero(&(a % &n)) {
        let t = (big(u64::from(e - 1)) / k + 1u8)
            .iter_u32_digits()
            .next()
            .unwrap_or(0);
        return (0..bounded_count(&p.pow(e - t)))
            .map(|i| big(i) * p.pow(t))
            .collect();
    }
    if *k < big(u64::from(e)) {
        let k32 = k.iter_u32_digits().next().unwrap_or(0);
        let pk = p.pow(k32);
        if is_zero(&(a % &pk)) {
            let roots = prime_power(&(a / pk), k, p, e - k32);
            let copies = bounded_count(&p.pow(k32 - 1));
            bounded_count(&(big(roots.len() as u64) * copies));
            return roots
                .iter()
                .flat_map(|r| (0..copies).map(move |j| r * p + big(j) * p.pow(e - k32 + 1)))
                .collect();
        }
    }
    if is_zero(&(a % p)) {
        return vec![];
    }
    let np = if k == p { &n * p } else { n.clone() };
    let ered = e.div_ceil(2) + u32::from(*p == two() && e >= 5);
    let mut result = Vec::new();
    for s in prime_power(a, k, p, ered) {
        let t = pow(&s, &(k - 1u8), &np);
        let r = add(
            &s,
            &divide(&sub(a, &mul(&t, &s, &np), &np), &mul(k, &t, &np), &n),
            &n,
        );
        if k != p {
            result.push(r);
        } else if pow(&r, k, &n) == a % &n {
            let count = bounded_count(k);
            bounded_count(&(big(result.len() as u64) + k));
            for i in 0..count {
                result.push(mul(&r, &add(&mul(&big(i), &(&n / p), &n), &one(), &n), &n));
            }
        }
    }
    result.sort_unstable();
    result.dedup();
    result
}

/// Return all `k`th roots modulo `n` in ascending order.
/// Port of `reference/mpu/rootmod.c::allrootmod` and `_allrootmod_prime_power`.
/// The modulus and exponent must be factorable.
/// # Panics
/// Panics if the result exceeds MPU's limit of 600 million roots.
/// ```
/// use math_prime_util::bigint::modular::allrootmod;
/// assert_eq!(allrootmod(&13u8.into(), &6u8.into(), &107u8.into()), vec![24u8.into(), 83u8.into()]);
/// ```
pub fn allrootmod(a: &BigUint, k: &BigUint, n: &BigUint) -> Vec<BigUint> {
    if is_zero(n) {
        return vec![];
    }
    let a = a % n;
    if is_one(n) {
        return vec![zero()];
    }
    if is_zero(k) {
        return if is_one(&a) {
            (0..bounded_count(n)).map(big).collect()
        } else {
            vec![]
        };
    }
    if *n == two() || is_one(k) {
        return vec![a];
    }
    if *k == two() {
        return allsqrtmod(&a, n);
    }
    let factors = factor_exp(n);
    let mut roots = vec![a];
    for prime in factor_exp(k)
        .into_iter()
        .flat_map(|(p, e)| std::iter::repeat_n(p, e as usize))
    {
        let mut next = Vec::new();
        for r in roots {
            let (mut rs, mut modulus) = (vec![zero()], one());
            for (p, e) in &factors {
                let pe = p.pow(*e);
                let more = prime_power(&r, &prime, p, *e);
                bounded_count(&(big(rs.len() as u64) * big(more.len() as u64)));
                rs = super::combine(&rs, &modulus, &more, &pe);
                modulus *= pe;
            }
            bounded_count(&big((next.len() + rs.len()) as u64));
            next.extend(rs);
        }
        roots = next;
        if roots.is_empty() {
            break;
        }
    }
    roots.sort_unstable();
    roots.dedup();
    roots
}

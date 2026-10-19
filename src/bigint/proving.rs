//! Primality proofs and certificates on arbitrary precision values: the
//! functions of [`crate::proving`] with the same names for [`BigUint`], the
//! MPU certificate text as a [`Certificate`] value, and its verifier.
//!
//! Port of MPU 0.75 `PP.pm::is_provable_prime_with_cert` and
//! `PrimalityProving.pm`: the BLS75 theorem 5 proof (`Type BLS5`) with a
//! partial factorization of `n - 1`, and `verify_cert` with every proof
//! type MPU reads (`Small`, `BLS3`, `BLS5`, `BLS15`, `Pocklington`, `Lucas`,
//! `ECPP`, `ECPP3`, `ECPP4`). Pure Perl MPU gives up (`1`) when `n - 1`
//! does not factor far enough; this module then runs an Atkin–Morain ECPP
//! downrun (`Type ECPP`) with the Hilbert class polynomials of the 250
//! discriminants of class number at most six and `|D|` at most 1500
//! (`tools/class_polynomials.py`), so proofs to 200 digits complete. Below 2^64
//! every function hands over to the native path and the certificate is
//! `Type Small`, as in MPU.
//!
//! The certificate text equals MPU's, so `Math::Prime::Util::verify_prime`
//! accepts ours and [`verify_prime`] accepts MPU's. With the cargo feature
//! `serde` the [`Certificate`] and [`Proof`] types derive `Serialize` and
//! `Deserialize`; every integer is a decimal string there.
//!
//! ```
//! use math_prime_util::bigint::proving::{is_provable_prime, prime_certificate, verify_prime};
//! use num_bigint::BigUint;
//! let p: BigUint = "1000000000000000000000000000057".parse().unwrap();
//! assert_eq!(is_provable_prime(&p), 2);
//! let cert = prime_certificate(&p);
//! assert!(cert.starts_with("[MPU - Primality Certificate]"));
//! assert_eq!(verify_prime(&cert), Ok(true));
//! ```

use super::Int;
use super::factor::factor_within;
use super::arith::{gcd, invmod, is_power_exp, is_square, jacobi, kronecker, rootint, sqrtint};
use super::primality::{is_prob_prime, lucas_sequence};
use num_bigint::{BigInt, BigUint, Sign};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::str::FromStr;
use std::time::Duration;

fn zero() -> BigUint {
    BigUint::from(0_u8)
}

fn one() -> BigUint {
    BigUint::from(1_u8)
}

fn two() -> BigUint {
    BigUint::from(2_u8)
}

/// `2^64 - 1`, the largest value MPU accepts in a `Type Small` proof.
fn smallval() -> BigUint {
    BigUint::from(u64::MAX)
}

/// A decimal constant of the class polynomial table.
fn big(text: &str) -> BigInt {
    BigInt::parse_bytes(text.as_bytes(), 10).unwrap_or_default()
}

/// `a mod n` in `0..n` for a signed `a`; `n` is nonzero.
fn reduce(a: &BigInt, n: &BigUint) -> BigUint {
    let r = a.magnitude() % n;
    if a.sign() == Sign::Minus && r != zero() {
        n - r
    } else {
        r
    }
}

/// `a - b mod n` for `a` and `b` below `n`.
fn submod(a: &BigUint, b: &BigUint, n: &BigUint) -> BigUint {
    if a >= b { a - b } else { n - (b - a) }
}

/// `a^k mod n` for `n > 1`.
fn powmod(a: &BigUint, k: &BigUint, n: &BigUint) -> BigUint {
    a.modpow(k, n)
}

/// True when `n` is at most 2^64 and prime (the native test is a proof).
fn native_prime(n: &BigUint) -> bool {
    n.to_u64().is_some_and(|v| crate::is_prime(v) != 0)
}

/// The primes to `limit`, for trial division.
fn small_primes(limit: u64) -> Vec<u64> {
    crate::primes(2, limit)
}

/// A square root of `a` mod the odd prime `p` (Tonelli–Shanks), or `None`
/// when `a` is a non-residue or `p` is not prime after all.
fn sqrtmod_prime(a: &BigUint, p: &BigUint) -> Option<BigUint> {
    if !p.bit(0) || *p < BigUint::from(3_u8) {
        return None;
    }
    let a = a % p;
    if a == zero() {
        return Some(zero());
    }
    let pm1 = p - 1_u8;
    let root = if (p % 4_u8) == BigUint::from(3_u8) {
        powmod(&a, &((p + 1_u8) >> 2), p)
    } else {
        let s = pm1.trailing_zeros().unwrap_or(0);
        let q = &pm1 >> s;
        let mut z = two();
        while jacobi(&BigInt::from(z.clone()), p) != -1 {
            z += 1_u8;
            if z > BigUint::from(1000_u16) {
                return None;
            }
        }
        let mut m = s;
        let mut c = powmod(&z, &q, p);
        let mut t = powmod(&a, &q, p);
        let mut r = powmod(&a, &((&q + 1_u8) >> 1), p);
        while t != one() {
            let mut i = 0;
            let mut t2 = t.clone();
            while t2 != one() {
                t2 = (&t2 * &t2) % p;
                i += 1;
                if i >= m {
                    return None;
                }
            }
            let mut b = c;
            for _ in 0..(m - i - 1) {
                b = (&b * &b) % p;
            }
            m = i;
            c = (&b * &b) % p;
            t = (&t * &c) % p;
            r = (&r * &b) % p;
        }
        r
    };
    ((&root * &root) % p == a).then_some(root)
}

// ---------------------------------------------------------------------------
// Elliptic curves in affine coordinates mod n (ECAffinePoint.pm).

/// A point of `y^2 = x^3 + a x + b` mod `n`; `None` is the point at infinity.
type Point = Option<(BigUint, BigUint)>;

/// The curve `y^2 = x^3 + a x + b` mod `n`; `b` plays no part in the group law.
struct Curve<'a> {
    a: BigUint,
    n: &'a BigUint,
}

impl Curve<'_> {
    /// Sum of two points, or `None` when an inversion fails: that only
    /// happens when `n` is composite, and the verifier then rejects.
    fn add(&self, p: &Point, q: &Point) -> Option<Point> {
        let (Some((x1, y1)), Some((x2, y2))) = (p, q) else {
            return Some(p.clone().or_else(|| q.clone()));
        };
        let n = self.n;
        if x1 == x2 {
            if (y1 + y2) % n == zero() {
                return Some(None);
            }
            return self.double(p);
        }
        let slope = (submod(y2, y1, n) * invmod(&submod(x2, x1, n), n)?) % n;
        let x3 = submod(&submod(&((&slope * &slope) % n), x1, n), x2, n);
        let y3 = submod(&((&slope * submod(x1, &x3, n)) % n), y1, n);
        Some(Some((x3, y3)))
    }

    /// Twice a point, with the same failure rule as [`Curve::add`].
    fn double(&self, p: &Point) -> Option<Point> {
        let Some((x, y)) = p else {
            return Some(None);
        };
        let n = self.n;
        if *y == zero() {
            return Some(None);
        }
        let slope = (((x * x * 3_u8) + &self.a) % n * invmod(&((y << 1_u8) % n), n)?) % n;
        let x3 = submod(&submod(&((&slope * &slope) % n), x, n), x, n);
        let y3 = submod(&((&slope * submod(x, &x3, n)) % n), y, n);
        Some(Some((x3, y3)))
    }

    /// `k` times a point by left-to-right double and add.
    fn mul(&self, p: &Point, k: &BigUint) -> Option<Point> {
        let mut result: Point = None;
        for i in (0..k.bits()).rev() {
            result = self.double(&result)?;
            if k.bit(i) {
                result = self.add(&result, p)?;
            }
        }
        Some(result)
    }
}

// ---------------------------------------------------------------------------
// Certificates.

#[cfg(feature = "serde")]
mod decimal {
    //! Integers as decimal strings in the serde form of a certificate.
    use num_bigint::{BigInt, BigUint};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub mod unsigned {
        use super::{BigUint, Deserialize, Deserializer, Serialize, Serializer};

        pub fn serialize<S: Serializer>(v: &BigUint, s: S) -> Result<S::Ok, S::Error> {
            v.to_string().serialize(s)
        }

        pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<BigUint, D::Error> {
            let text = String::deserialize(d)?;
            text.parse().map_err(serde::de::Error::custom)
        }
    }

    pub mod signed {
        use super::{BigInt, Deserialize, Deserializer, Serialize, Serializer};

        pub fn serialize<S: Serializer>(v: &BigInt, s: S) -> Result<S::Ok, S::Error> {
            v.to_string().serialize(s)
        }

        pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<BigInt, D::Error> {
            let text = String::deserialize(d)?;
            text.parse().map_err(serde::de::Error::custom)
        }
    }

    pub mod unsigned_list {
        use super::{BigUint, Deserialize, Deserializer, Serialize, Serializer};

        pub fn serialize<S: Serializer>(v: &[BigUint], s: S) -> Result<S::Ok, S::Error> {
            v.iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .serialize(s)
        }

        pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<BigUint>, D::Error> {
            let texts = Vec::<String>::deserialize(d)?;
            texts
                .iter()
                .map(|t| t.parse().map_err(serde::de::Error::custom))
                .collect()
        }
    }
}

/// One proof block of an MPU certificate: `N` is prime if the listed `Q`
/// values are prime. The field names are the keys of the text format.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum Proof {
    /// `Type Small`: `N` is at most 2^64 and passes BPSW.
    Small {
        /// The prime.
        #[cfg_attr(feature = "serde", serde(with = "decimal::unsigned"))]
        n: BigUint,
    },
    /// `Type BLS3`: BLS75 theorem 3, an odd prime `Q` dividing `N - 1` with
    /// `2Q + 1 > sqrt(N)` and a witness `A`.
    Bls3 {
        /// The prime.
        #[cfg_attr(feature = "serde", serde(with = "decimal::unsigned"))]
        n: BigUint,
        /// The large prime factor of `N - 1`.
        #[cfg_attr(feature = "serde", serde(with = "decimal::unsigned"))]
        q: BigUint,
        /// The witness.
        #[cfg_attr(feature = "serde", serde(with = "decimal::unsigned"))]
        a: BigUint,
    },
    /// `Type Pocklington`: `N = M Q + 1` with `M` even and below the prime
    /// `Q`, and a witness `A`.
    Pocklington {
        /// The prime.
        #[cfg_attr(feature = "serde", serde(with = "decimal::unsigned"))]
        n: BigUint,
        /// The large prime factor of `N - 1`.
        #[cfg_attr(feature = "serde", serde(with = "decimal::unsigned"))]
        q: BigUint,
        /// The witness.
        #[cfg_attr(feature = "serde", serde(with = "decimal::unsigned"))]
        a: BigUint,
    },
    /// `Type BLS15`: BLS75 theorem 15, an odd prime `Q` dividing `N + 1`
    /// with `2Q - 1 > sqrt(N)` and Lucas parameters `LP`, `LQ`.
    Bls15 {
        /// The prime.
        #[cfg_attr(feature = "serde", serde(with = "decimal::unsigned"))]
        n: BigUint,
        /// The large prime factor of `N + 1`.
        #[cfg_attr(feature = "serde", serde(with = "decimal::unsigned"))]
        q: BigUint,
        /// The Lucas parameter `P`.
        #[cfg_attr(feature = "serde", serde(with = "decimal::signed"))]
        lp: BigInt,
        /// The Lucas parameter `Q`.
        #[cfg_attr(feature = "serde", serde(with = "decimal::signed"))]
        lq: BigInt,
    },
    /// `Type BLS5`: BLS75 theorem 5, a partial factorization of `N - 1`.
    Bls5 {
        /// The prime.
        #[cfg_attr(feature = "serde", serde(with = "decimal::unsigned"))]
        n: BigUint,
        /// The prime factors `Q[1]`, `Q[2]`, ... of `N - 1`; the factor 2
        /// (`Q[0]`) is implicit.
        #[cfg_attr(feature = "serde", serde(with = "decimal::unsigned_list"))]
        q: Vec<BigUint>,
        /// One witness per factor, `A[0]` for the factor 2 first. A missing
        /// entry means 2, which the text omits.
        #[cfg_attr(feature = "serde", serde(with = "decimal::unsigned_list"))]
        a: Vec<BigUint>,
    },
    /// `Type Lucas`: `N - 1` completely factored and a primitive root `A`.
    Lucas {
        /// The prime.
        #[cfg_attr(feature = "serde", serde(with = "decimal::unsigned"))]
        n: BigUint,
        /// The distinct prime factors `Q[1]`, `Q[2]`, ... of `N - 1`.
        #[cfg_attr(feature = "serde", serde(with = "decimal::unsigned_list"))]
        q: Vec<BigUint>,
        /// The primitive root.
        #[cfg_attr(feature = "serde", serde(with = "decimal::unsigned"))]
        a: BigUint,
    },
    /// `Type ECPP`: the point `(X, Y)` of `y^2 = x^3 + A x + B` mod `N` has
    /// order a multiple of the prime `Q`, and `M = k Q` is the curve order.
    Ecpp {
        /// The prime.
        #[cfg_attr(feature = "serde", serde(with = "decimal::unsigned"))]
        n: BigUint,
        /// The curve coefficient `A`; a negative value is reduced mod `N`.
        #[cfg_attr(feature = "serde", serde(with = "decimal::signed"))]
        a: BigInt,
        /// The curve coefficient `B`; a negative value is reduced mod `N`.
        #[cfg_attr(feature = "serde", serde(with = "decimal::signed"))]
        b: BigInt,
        /// The curve order.
        #[cfg_attr(feature = "serde", serde(with = "decimal::unsigned"))]
        m: BigUint,
        /// The prime factor of `M` above `(N^(1/4) + 1)^2`.
        #[cfg_attr(feature = "serde", serde(with = "decimal::unsigned"))]
        q: BigUint,
        /// The point's `x`.
        #[cfg_attr(feature = "serde", serde(with = "decimal::unsigned"))]
        x: BigUint,
        /// The point's `y`.
        #[cfg_attr(feature = "serde", serde(with = "decimal::unsigned"))]
        y: BigUint,
    },
    /// `Type ECPP3` (Primo type 3): the curve `y^2 = x^3 + A x + B` scaled
    /// by `L = T^3 + A T + B`, with order `R S` and prime `R`.
    Ecpp3 {
        /// The prime.
        #[cfg_attr(feature = "serde", serde(with = "decimal::unsigned"))]
        n: BigUint,
        /// The cofactor `S`.
        #[cfg_attr(feature = "serde", serde(with = "decimal::unsigned"))]
        s: BigUint,
        /// The prime `R`.
        #[cfg_attr(feature = "serde", serde(with = "decimal::unsigned"))]
        r: BigUint,
        /// The curve coefficient `A`, at most `N/2` in size.
        #[cfg_attr(feature = "serde", serde(with = "decimal::signed"))]
        a: BigInt,
        /// The curve coefficient `B`, at most `N/2` in size.
        #[cfg_attr(feature = "serde", serde(with = "decimal::signed"))]
        b: BigInt,
        /// The point parameter `T`.
        #[cfg_attr(feature = "serde", serde(with = "decimal::unsigned"))]
        t: BigUint,
    },
    /// `Type ECPP4` (Primo type 4): the curve of `j`-invariant `J`, with
    /// `A = 3 J (1728 - J)` and `B = 2 J (1728 - J)^2`, scaled as in `ECPP3`.
    Ecpp4 {
        /// The prime.
        #[cfg_attr(feature = "serde", serde(with = "decimal::unsigned"))]
        n: BigUint,
        /// The cofactor `S`.
        #[cfg_attr(feature = "serde", serde(with = "decimal::unsigned"))]
        s: BigUint,
        /// The prime `R`.
        #[cfg_attr(feature = "serde", serde(with = "decimal::unsigned"))]
        r: BigUint,
        /// The `j`-invariant, at most `N/2` in size.
        #[cfg_attr(feature = "serde", serde(with = "decimal::signed"))]
        j: BigInt,
        /// The point parameter `T`.
        #[cfg_attr(feature = "serde", serde(with = "decimal::unsigned"))]
        t: BigUint,
    },
}

/// An MPU primality certificate: the target `n` and its proof blocks.
///
/// `Display` writes the MPU text format and `FromStr` reads it, so a
/// certificate round-trips through `Math::Prime::Util`.
///
/// ```
/// use math_prime_util::bigint::proving::{Certificate, Proof};
/// use num_bigint::BigUint;
/// let text = "[MPU - Primality Certificate]\nVersion 1.0\n\nProof for:\nN 101\n\nType Small\nN 101\n";
/// let cert: Certificate = text.parse().unwrap();
/// assert_eq!(cert.n, BigUint::from(101_u8));
/// assert_eq!(cert.proofs, vec![Proof::Small { n: BigUint::from(101_u8) }]);
/// assert_eq!(cert.to_string(), text);
/// assert_eq!(cert.verify(), Ok(true));
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Certificate {
    /// The number the certificate proves prime (the `Proof for:` value).
    #[cfg_attr(feature = "serde", serde(with = "decimal::unsigned"))]
    pub n: BigUint,
    /// The proof blocks, in text order.
    pub proofs: Vec<Proof>,
}

impl fmt::Display for Proof {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Small { n } => write!(f, "Type Small\nN {n}\n"),
            Self::Bls3 { n, q, a } => write!(f, "Type BLS3\nN {n}\nQ {q}\nA {a}\n"),
            Self::Pocklington { n, q, a } => {
                write!(f, "Type Pocklington\nN {n}\nQ {q}\nA {a}\n")
            }
            Self::Bls15 { n, q, lp, lq } => {
                write!(f, "Type BLS15\nN {n}\nQ {q}\nLP {lp}\nLQ {lq}\n")
            }
            Self::Bls5 { n, q, a } => {
                write!(f, "Type BLS5\nN {n}\n")?;
                for (i, q) in q.iter().enumerate() {
                    writeln!(f, "Q[{}] {q}", i + 1)?;
                }
                for (i, a) in a.iter().enumerate() {
                    if *a != two() {
                        writeln!(f, "A[{i}] {a}")?;
                    }
                }
                writeln!(f, "----")
            }
            Self::Lucas { n, q, a } => {
                write!(f, "Type Lucas\nN {n}\n")?;
                for (i, q) in q.iter().enumerate() {
                    writeln!(f, "Q[{}] {q}", i + 1)?;
                }
                writeln!(f, "A {a}")
            }
            Self::Ecpp {
                n,
                a,
                b,
                m,
                q,
                x,
                y,
            } => write!(
                f,
                "Type ECPP\nN {n}\nA {a}\nB {b}\nM {m}\nQ {q}\nX {x}\nY {y}\n"
            ),
            Self::Ecpp3 { n, s, r, a, b, t } => {
                write!(f, "Type ECPP3\nN {n}\nS {s}\nR {r}\nA {a}\nB {b}\nT {t}\n")
            }
            Self::Ecpp4 { n, s, r, j, t } => {
                write!(f, "Type ECPP4\nN {n}\nS {s}\nR {r}\nJ {j}\nT {t}\n")
            }
        }
    }
}

impl fmt::Display for Certificate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[MPU - Primality Certificate]\nVersion 1.0\n\nProof for:\nN {}\n",
            self.n
        )?;
        for proof in &self.proofs {
            write!(f, "\n{proof}")?;
        }
        Ok(())
    }
}

// --- parsing (PrimalityProving.pm::verify_cert and its _prove_* readers)

/// A line of certificate text with its blank and comment lines dropped.
struct Lines<'a> {
    lines: std::iter::Peekable<std::str::Lines<'a>>,
}

impl<'a> Lines<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            lines: text.lines().peekable(),
        }
    }

    fn next(&mut self) -> Option<&'a str> {
        loop {
            let line = self.lines.next()?;
            let trimmed = line.trim_end();
            if !trimmed.trim_start().is_empty() && !trimmed.trim_start().starts_with('#') {
                return Some(trimmed);
            }
        }
    }
}

/// `KEY value` with an optional minus sign, as `_read_vars` reads it: the
/// key is the first word and the value the digits that follow it.
fn parse_field(line: &str) -> Option<(String, BigInt)> {
    let mut words = line.split_whitespace();
    let key = words.next()?.to_ascii_uppercase();
    let value = words.next()?;
    let (negative, rest) = match value.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, value),
    };
    let digits: String = rest.chars().take_while(char::is_ascii_digit).collect();
    if digits.is_empty() {
        return None;
    }
    let value = big(&digits);
    Some((key, if negative { -value } else { value }))
}

/// `Q[i]` or `A[i]` with its index.
fn indexed(key: &str, name: &str) -> Option<usize> {
    key.strip_prefix(name)?
        .strip_prefix('[')?
        .strip_suffix(']')?
        .parse()
        .ok()
}

/// The values of `keys` for a proof type, in any order, one per line.
fn read_vars(lines: &mut Lines, kind: &str, keys: &[&str]) -> Result<Vec<BigInt>, String> {
    let mut found: BTreeMap<String, BigInt> = BTreeMap::new();
    while found.len() < keys.len() {
        let line = lines
            .next()
            .ok_or_else(|| format!("end of file during type {kind}"))?;
        if line.starts_with("Type ") {
            return Err(format!("Still missing values in type {kind}"));
        }
        let (key, value) = parse_field(line).ok_or_else(|| format!("Unrecognized line: {line}"))?;
        if !keys.contains(&key.as_str()) || found.contains_key(&key) {
            return Err(format!(
                "Type {kind}: repeated or inappropriate var: {line}"
            ));
        }
        found.insert(key, value);
    }
    Ok(keys.iter().filter_map(|k| found.remove(*k)).collect())
}

/// A non-negative value, or the error MPU raises for a negative one.
fn unsigned(value: &BigInt, what: &str) -> Result<BigUint, String> {
    value
        .to_biguint()
        .ok_or_else(|| format!("{what} must not be negative"))
}

/// The values of a `BLS5` or `Lucas` block: `N`, the `Q[i]`, the `A[i]`
/// with their indices, and the `A` of a `Lucas` block.
type Indexed = (
    BigUint,
    Vec<BigUint>,
    Vec<(usize, BigUint)>,
    Option<BigUint>,
);

/// `N` and the indexed `Q[i]` lines of a `BLS5` or `Lucas` block. For
/// `BLS5` the block ends at a line starting with `-` and `A[i]` lines give
/// the witnesses; for `Lucas` the `A` line ends the block.
fn read_indexed(lines: &mut Lines, kind: &str) -> Result<Indexed, String> {
    let mut n = None;
    let mut q = Vec::new();
    let mut a = Vec::new();
    loop {
        let line = lines
            .next()
            .ok_or_else(|| format!("end of file during type {kind}"))?;
        if kind == "BLS5" && line.starts_with('-') {
            break;
        }
        let mut words = line.split_whitespace();
        let (Some(key), Some(value)) = (words.next(), words.next()) else {
            return Err(format!("Unrecognized line: {line}"));
        };
        let digits: String = value.chars().take_while(char::is_ascii_digit).collect();
        if digits.is_empty() {
            return Err(format!("Unrecognized line: {line}"));
        }
        let value = BigUint::from_str(&digits).map_err(|e| e.to_string())?;
        if key == "N" {
            if n.is_some() {
                return Err(format!("{kind}: N redefined"));
            }
            n = Some(value);
        } else if let Some(i) = indexed(key, "Q") {
            if i != q.len() + 1 {
                return Err(format!("{kind}: Invalid index: {i}"));
            }
            q.push(value);
        } else if kind == "BLS5" && key.starts_with("A[") {
            let i = indexed(key, "A").ok_or_else(|| format!("Unrecognized line: {line}"))?;
            if i > q.len() {
                return Err(format!("BLS5: Invalid index: A[{i}]"));
            }
            a.push((i, value));
        } else if kind == "Lucas" && key == "A" {
            return Ok((n.ok_or("Lucas: N missing")?, q, a, Some(value)));
        } else {
            return Err(format!("Unrecognized line: {line}"));
        }
    }
    Ok((n.ok_or("BLS5: N missing")?, q, a, None))
}

fn read_proof(lines: &mut Lines, kind: &str) -> Result<Proof, String> {
    Ok(match kind {
        "SMALL" => {
            let v = read_vars(lines, "Small", &["N"])?;
            Proof::Small {
                n: unsigned(&v[0], "N")?,
            }
        }
        "BLS3" | "POCKLINGTON" => {
            let v = read_vars(lines, kind, &["N", "Q", "A"])?;
            let (n, q, a) = (
                unsigned(&v[0], "N")?,
                unsigned(&v[1], "Q")?,
                unsigned(&v[2], "A")?,
            );
            if kind == "BLS3" {
                Proof::Bls3 { n, q, a }
            } else {
                Proof::Pocklington { n, q, a }
            }
        }
        "BLS15" => {
            let v = read_vars(lines, kind, &["N", "Q", "LP", "LQ"])?;
            Proof::Bls15 {
                n: unsigned(&v[0], "N")?,
                q: unsigned(&v[1], "Q")?,
                lp: v[2].clone(),
                lq: v[3].clone(),
            }
        }
        "BLS5" => {
            let (n, q, witnesses, _) = read_indexed(lines, "BLS5")?;
            let mut a = vec![two(); q.len() + 1];
            for (i, value) in witnesses {
                a[i] = value;
            }
            Proof::Bls5 { n, q, a }
        }
        "LUCAS" => {
            let (n, q, _, a) = read_indexed(lines, "Lucas")?;
            Proof::Lucas {
                n,
                q,
                a: a.ok_or("Lucas: A missing")?,
            }
        }
        "ECPP" => {
            let v = read_vars(lines, kind, &["N", "A", "B", "M", "Q", "X", "Y"])?;
            Proof::Ecpp {
                n: unsigned(&v[0], "N")?,
                a: v[1].clone(),
                b: v[2].clone(),
                m: unsigned(&v[3], "M")?,
                q: unsigned(&v[4], "Q")?,
                x: unsigned(&v[5], "X")?,
                y: unsigned(&v[6], "Y")?,
            }
        }
        "ECPP3" => {
            let v = read_vars(lines, kind, &["N", "S", "R", "A", "B", "T"])?;
            Proof::Ecpp3 {
                n: unsigned(&v[0], "N")?,
                s: unsigned(&v[1], "S")?,
                r: unsigned(&v[2], "R")?,
                a: v[3].clone(),
                b: v[4].clone(),
                t: unsigned(&v[5], "T")?,
            }
        }
        "ECPP4" => {
            let v = read_vars(lines, kind, &["N", "S", "R", "J", "T"])?;
            Proof::Ecpp4 {
                n: unsigned(&v[0], "N")?,
                s: unsigned(&v[1], "S")?,
                r: unsigned(&v[2], "R")?,
                j: v[3].clone(),
                t: unsigned(&v[4], "T")?,
            }
        }
        _ => return Err(format!("Unknown type: {kind}")),
    })
}

impl FromStr for Certificate {
    type Err = String;

    /// Reads the MPU text format as `PrimalityProving.pm::verify_cert` does:
    /// blank and `#` lines are skipped, lines other than the header,
    /// `Proof for:`, `Base` and `Type` are ignored, and a block's values
    /// follow its `Type` line.
    fn from_str(text: &str) -> Result<Self, String> {
        let mut lines = Lines::new(text);
        let mut n: Option<BigUint> = None;
        let mut proofs = Vec::new();
        while let Some(line) = lines.next() {
            if let Some(rest) = line.strip_prefix('[') {
                if let Some(kind) = rest.strip_suffix(" - Primality Certificate]") {
                    if kind != "MPU" {
                        return Err(format!("Unknown certificate type: {kind}"));
                    }
                }
            } else if line.starts_with("Proof for:") {
                if n.is_some() {
                    return Err("Certificate with multiple N values".into());
                }
                let v = read_vars(&mut lines, "Proof for", &["N"])?;
                n = Some(unsigned(&v[0], "N")?);
            } else if let Some(base) = line.strip_prefix("Base ") {
                if base.trim() != "10" {
                    return Err("Only base 10 supported, sorry".into());
                }
            } else if let Some(kind) = line.strip_prefix("Type ") {
                if n.is_none() {
                    return Err("Starting type without telling me the N value!".into());
                }
                proofs.push(read_proof(&mut lines, &kind.trim().to_ascii_uppercase())?);
            }
        }
        Ok(Self {
            n: n.ok_or("No N")?,
            proofs,
        })
    }
}

// --- verification (PrimalityProving.pm::_verify_*)

/// `V_k(P, Q) mod n`.
fn lucas_v(p: &BigInt, q: &BigInt, k: &BigUint, n: &BigUint) -> BigUint {
    lucas_sequence(n, p, q, k).1
}

fn check_bls3(n: &BigUint, q: &BigUint, a: &BigUint) -> bool {
    if !q.bit(0) || *q <= two() || *n < two() {
        return false;
    }
    let nm1 = n - 1_u8;
    let (m, rem) = (&nm1 / q, &nm1 % q);
    rem == zero()
        && m > zero()
        && (q << 1_u8) + 1_u8 > sqrtint(n)
        && powmod(a, &(&nm1 >> 1), n) == nm1
        && powmod(a, &(m >> 1), n) != nm1
}

fn check_pocklington(n: &BigUint, q: &BigUint, a: &BigUint) -> bool {
    if *n < two() || *q == zero() {
        return false;
    }
    let nm1 = n - 1_u8;
    let (m, rem) = (&nm1 / q, &nm1 % q);
    rem == zero()
        && m.is_even()
        && m > zero()
        && m < *q
        && *a > one()
        && powmod(a, &nm1, n) == one()
        && gcd(&((powmod(a, &m, n) + &nm1) % n), n) == one()
}

fn check_bls15(n: &BigUint, q: &BigUint, lp: &BigInt, lq: &BigInt) -> bool {
    if !q.bit(0) || *q <= two() || !n.bit(0) {
        return false;
    }
    let np1 = n + 1_u8;
    let (m, rem) = (&np1 / q, &np1 % q);
    if rem != zero() || m == zero() || (q << 1_u8) <= sqrtint(n) + 1_u8 {
        return false;
    }
    let d = lp * lp - BigInt::from(4) * lq;
    d != BigInt::from(0)
        && jacobi(&d, n) == -1
        && lucas_v(lp, lq, &(m >> 1), n) != zero()
        && lucas_v(lp, lq, &(np1 >> 1), n) == zero()
}

/// The BLS75 theorem 5 bound `(F + 1)(2F^2 + (r - 1)F + 1)` and the square
/// test on `r^2 - 8s`, shared by the verifier and the prover.
fn bls5_bound(n: &BigUint, f: &BigUint, r_part: &BigUint) -> Option<bool> {
    if !f.is_even() || gcd(f, r_part) != one() {
        return None;
    }
    let two_f = f << 1_u8;
    let (s, r) = (r_part / &two_f, r_part % &two_f);
    let (fi, ri) = (BigInt::from(f.clone()), BigInt::from(r.clone()));
    let bound = (&fi + 1) * (&fi * &fi * 2 + (ri - 1) * &fi + 1);
    if BigInt::from(n.clone()) >= bound {
        return None;
    }
    let rtest: BigInt = BigInt::from(&r * &r) - BigInt::from(s.clone()) * 8;
    Some(s == zero() || !rtest.to_biguint().is_some_and(|v| is_square(&v)))
}

fn check_bls5(n: &BigUint, qs: &[BigUint], a: &[BigUint]) -> bool {
    if *n < two() {
        return false;
    }
    let nm1 = n - 1_u8;
    let mut f = one();
    let mut r = nm1.clone();
    let factors: Vec<BigUint> = std::iter::once(two()).chain(qs.iter().cloned()).collect();
    for (i, q) in factors.iter().enumerate() {
        let a = a.get(i).cloned().unwrap_or_else(two);
        if *q <= one() || *q >= nm1 || a <= one() || a >= *n || &nm1 % q != zero() {
            return false;
        }
        while &r % q == zero() {
            f *= q;
            r /= q;
        }
    }
    if bls5_bound(n, &f, &r) != Some(true) {
        return false;
    }
    factors.iter().enumerate().all(|(i, q)| {
        let a = a.get(i).cloned().unwrap_or_else(two);
        powmod(&a, &nm1, n) == one() && gcd(&((powmod(&a, &(&nm1 / q), n) + &nm1) % n), n) == one()
    })
}

fn check_lucas(n: &BigUint, qs: &[BigUint], a: &BigUint) -> bool {
    if *a <= one() || a >= n {
        return false;
    }
    let nm1 = n - 1_u8;
    if powmod(a, &nm1, n) != one() {
        return false;
    }
    let mut f = one();
    let mut r = nm1.clone();
    for q in qs {
        if *q <= one() || q >= &nm1 || &nm1 % q != zero() || powmod(a, &(&nm1 / q), n) == one() {
            return false;
        }
        while &r % q == zero() {
            f *= q;
            r /= q;
        }
    }
    r == one() && f == nm1
}

/// `_verify_ecpp`: the curve conditions and the point order test.
fn check_ecpp(
    n: &BigUint,
    a: &BigInt,
    b: &BigInt,
    m: &BigUint,
    q: &BigUint,
    x: &BigUint,
    y: &BigUint,
) -> bool {
    if *n == zero() || gcd(n, &BigUint::from(6_u8)) != one() {
        return false;
    }
    let (a, b) = (reduce(a, n), reduce(b, n));
    let disc = (&a * &a * &a * 4_u8 + &b * &b * 27_u8) % n;
    if gcd(&disc, n) != one() || (y * y) % n != (x * x * x + &a * x + &b) % n {
        return false;
    }
    let root4n = sqrtint(&(n << 2_u8));
    let np1 = n + 1_u8;
    if m + &root4n < np1 || *m > &np1 + &root4n {
        return false;
    }
    let qmin = rootint(n, 4) + 1_u8;
    if *q <= &qmin * &qmin || q >= n || m == q || m % q != zero() {
        return false;
    }
    let curve = Curve { a, n };
    let point: Point = Some((x % n, y % n));
    let Some(Some(u)) = curve.mul(&point, &(m / q)) else {
        return false;
    };
    curve.mul(&Some(u), q) == Some(None)
}

/// `_verify_ecpp3` and `_verify_ecpp4`: the Primo forms become an `ECPP` block.
fn check_primo(n: &BigUint, s: &BigUint, r: &BigUint, a: &BigInt, b: &BigInt, t: &BigUint) -> bool {
    if *n == zero() || t >= n {
        return false;
    }
    let ti = BigInt::from(t.clone());
    let l = reduce(&(&ti * &ti * &ti + a * &ti + b), n);
    let li = BigInt::from(l.clone());
    let (a2, b2) = (
        reduce(&(a * &li * &li), n),
        reduce(&(b * &li * &li * &li), n),
    );
    check_ecpp(
        n,
        &BigInt::from(a2),
        &BigInt::from(b2),
        &(r * s),
        r,
        &((t * &l) % n),
        &((&l * &l) % n),
    )
}

impl Proof {
    /// The number this block proves and the primes it rests on, or `None`
    /// when a condition of the theorem fails.
    fn check(&self) -> Option<(BigUint, Vec<BigUint>)> {
        let (n, qs, ok) = match self {
            Self::Small { n } => (n, vec![], *n <= smallval() && native_prime(n)),
            Self::Bls3 { n, q, a } => (n, vec![q.clone()], check_bls3(n, q, a)),
            Self::Pocklington { n, q, a } => (n, vec![q.clone()], check_pocklington(n, q, a)),
            Self::Bls15 { n, q, lp, lq } => (n, vec![q.clone()], check_bls15(n, q, lp, lq)),
            Self::Bls5 { n, q, a } => {
                let mut all = vec![two()];
                all.extend(q.iter().cloned());
                (n, all, check_bls5(n, q, a))
            }
            Self::Lucas { n, q, a } => (n, q.clone(), check_lucas(n, q, a)),
            Self::Ecpp {
                n,
                a,
                b,
                m,
                q,
                x,
                y,
            } => (n, vec![q.clone()], check_ecpp(n, a, b, m, q, x, y)),
            Self::Ecpp3 { n, s, r, a, b, t } => (
                n,
                vec![r.clone()],
                a.magnitude() <= &(n >> 1_u8)
                    && b.magnitude() <= &(n >> 1_u8)
                    && check_primo(n, s, r, a, b, t),
            ),
            Self::Ecpp4 { n, s, r, j, t } => {
                let k = BigInt::from(1728) - j;
                let (a, b) = (j * &k * 3, j * &k * &k * 2);
                (
                    n,
                    vec![r.clone()],
                    j.magnitude() <= &(n >> 1_u8) && check_primo(n, s, r, &a, &b, t),
                )
            }
        };
        ok.then(|| (n.clone(), qs))
    }
}

impl Certificate {
    /// Parses the MPU text format; the same as `text.parse()`.
    ///
    /// # Errors
    /// Returns the message MPU carps for malformed text.
    ///
    /// ```
    /// use math_prime_util::bigint::proving::Certificate;
    /// assert!(Certificate::parse("[MPU - Primality Certificate]\nType Small\nN 7\n").is_err());
    /// ```
    pub fn parse(text: &str) -> Result<Self, String> {
        text.parse()
    }

    /// Checks every block and the chain from `n` down to primes at most
    /// 2^64, as `PrimalityProving.pm::verify_cert` does.
    ///
    /// # Errors
    /// Returns an error when a number in the chain above 2^64 has no block.
    /// `Ok(false)` means a block fails its theorem's conditions.
    ///
    /// ```
    /// use math_prime_util::bigint::proving::{Certificate, Proof};
    /// use num_bigint::BigUint;
    /// let n = BigUint::from(23_u8);
    /// let cert = Certificate { n: n.clone(), proofs: vec![Proof::Bls3 { n, q: BigUint::from(11_u8), a: BigUint::from(5_u8) }] };
    /// assert_eq!(cert.verify(), Ok(true));
    /// ```
    pub fn verify(&self) -> Result<bool, String> {
        if is_prob_prime(&self.n) == 0 {
            return Ok(false);
        }
        let mut parts: BTreeMap<BigUint, Vec<BigUint>> = BTreeMap::new();
        for proof in &self.proofs {
            let Some((n, qs)) = proof.check() else {
                return Ok(false);
            };
            parts.insert(n, qs);
        }
        let mut queue = vec![self.n.clone()];
        let mut seen = BTreeSet::new();
        while let Some(q) = queue.pop() {
            if !seen.insert(q.clone()) {
                continue;
            }
            match parts.get(&q) {
                Some(qs) => queue.extend(qs.iter().cloned()),
                None if q > smallval() => return Err(format!("q value {q} has no proof")),
                None if !native_prime(&q) => return Ok(false),
                None => {}
            }
        }
        Ok(true)
    }

    /// Proves `n` prime and returns its certificate, or `None` when `n` is
    /// composite or the proof methods fail. See [`is_provable_prime_with_cert`].
    ///
    /// ```
    /// use math_prime_util::bigint::proving::Certificate;
    /// use num_bigint::BigUint;
    /// let p = (BigUint::from(1_u8) << 89_usize) - 1_u8;
    /// let cert = Certificate::prove(&p).unwrap();
    /// assert_eq!(cert.verify(), Ok(true));
    /// assert!(Certificate::prove(&(&p * 3_u8)).is_none());
    /// ```
    pub fn prove(n: &BigUint) -> Option<Self> {
        prove(n).ok().map(|proofs| Self {
            n: n.clone(),
            proofs,
        })
    }
}

// ---------------------------------------------------------------------------
// Proving.

/// Result of a proof attempt: the blocks, or MPU's status for no proof
/// (0 composite, 1 could not prove).
type Attempt = Result<Vec<Proof>, u8>;

/// `PrimalityProving.pm::primality_proof_bls75` for an odd probable prime
/// `n` above 2^64.
fn bls75(n: &BigUint) -> Attempt {
    let nm1 = n - 1_u8;
    let mut a_part = one();
    let mut b_part = nm1.clone();
    let mut factors = vec![two()];
    while b_part.is_even() {
        b_part >>= 1_u8;
        a_part <<= 1_u8;
    }
    for p in small_primes(5000).into_iter().skip(1) {
        if &b_part % p == zero() {
            factors.push(BigUint::from(p));
            while &b_part % p == zero() {
                b_part /= p;
                a_part *= p;
            }
        }
    }
    let mut stack = Vec::new();
    if b_part == one() {
    } else if is_prob_prime(&b_part) != 0 {
        factors.push(b_part.clone());
        a_part *= &b_part;
        b_part = one();
    } else {
        stack.push(b_part.clone());
    }
    while bls5_bound(n, &a_part, &b_part).is_none() {
        let Some(m) = stack.pop() else { break };
        // Try the same staged factoring methods as MPU's BLS75 proof.
        let factored = factor_within(&m, Duration::from_secs(1));
        for f in factored.primes {
            if !factors.contains(&f) {
                factors.push(f.clone());
            }
            while &b_part % &f == zero() {
                b_part /= &f;
                a_part *= &f;
            }
        }
    }
    factors.sort_unstable();
    factors.dedup();
    match bls5_bound(n, &a_part, &b_part) {
        None => return Err(1),
        Some(false) => return Err(0),
        Some(true) => {}
    }
    let mut witnesses = Vec::new();
    let mut proofs = Vec::new();
    let mut extra = Vec::new();
    for f in &factors {
        let exponent = &nm1 / f;
        let a = (2_u32..=10000)
            .map(BigUint::from)
            .find(|a| {
                powmod(a, &nm1, n) == one()
                    && gcd(&((powmod(a, &exponent, n) + &nm1) % n), n) == one()
            })
            .ok_or(0_u8)?;
        witnesses.push(a);
        if *f > smallval() {
            extra.extend(prove(f).map_err(|_| 1_u8)?);
        }
    }
    proofs.push(Proof::Bls5 {
        n: n.clone(),
        q: factors[1..].to_vec(),
        a: witnesses,
    });
    proofs.extend(extra);
    Ok(proofs)
}

mod class_poly_data;
use class_poly_data::CLASS_POLYNOMIALS;

/// Cornacchia for `4n = t^2 + |d| v^2` with the prime `n`; `d < 0`.
fn cornacchia(d: i64, n: &BigUint) -> Option<(BigUint, BigUint)> {
    let abs_d = BigUint::from(d.unsigned_abs());
    let four_n = n << 2_u8;
    let mut x0 = sqrtmod_prime(&reduce(&BigInt::from(d), n), n)?;
    if (&x0 + &abs_d).bit(0) {
        x0 = n - x0;
    }
    let (mut a, mut b) = (n << 1_u8, x0);
    let limit = sqrtint(&four_n);
    while b > limit {
        let r = &a % &b;
        a = std::mem::replace(&mut b, r);
    }
    let t = b;
    let rem = &four_n - &t * &t;
    if &rem % &abs_d != zero() {
        return None;
    }
    let v2 = rem / abs_d;
    let v = sqrtint(&v2);
    (&v * &v == v2).then_some((t, v))
}

/// The curve orders a discriminant allows: `n + 1 ± t`, plus the quartic
/// twists for `D = -4` and the sextic twists for `D = -3`.
fn curve_orders(d: i64, n: &BigUint, t: &BigUint, v: &BigUint) -> Vec<BigUint> {
    let np1 = BigInt::from(n + 1_u8);
    let (ti, vi) = (BigInt::from(t.clone()), BigInt::from(v.clone()));
    let mut deltas = vec![ti.clone()];
    match d {
        -4 => deltas.push(&vi * 2),
        -3 => {
            deltas.push((&ti + &vi * 3) / 2);
            deltas.push((&ti - &vi * 3) / 2);
        }
        _ => {}
    }
    deltas
        .iter()
        .flat_map(|delta| [&np1 + delta, &np1 - delta])
        .filter_map(|m| m.to_biguint())
        .collect()
}

/// Polynomials mod `n` with coefficients low to high, for the roots of a
/// class polynomial of degree three (Cantor–Zassenhaus).
struct PolyMod<'a> {
    n: &'a BigUint,
}

impl PolyMod<'_> {
    fn trim(mut p: Vec<BigUint>) -> Vec<BigUint> {
        while p.last().is_some_and(|c| *c == zero()) {
            p.pop();
        }
        p
    }

    /// `a mod f` for monic `f`.
    fn rem(&self, a: &[BigUint], f: &[BigUint]) -> Vec<BigUint> {
        let n = self.n;
        let mut a = a.to_vec();
        let deg_f = f.len() - 1;
        while a.len() > deg_f {
            let lead = a.pop().unwrap_or_else(zero);
            if lead == zero() {
                continue;
            }
            let shift = a.len() - deg_f;
            for (i, c) in f[..deg_f].iter().enumerate() {
                a[shift + i] = submod(&a[shift + i], &((&lead * c) % n), n);
            }
        }
        Self::trim(a)
    }

    fn mul(&self, a: &[BigUint], b: &[BigUint], f: &[BigUint]) -> Vec<BigUint> {
        if a.is_empty() || b.is_empty() {
            return vec![];
        }
        let mut out = vec![zero(); a.len() + b.len() - 1];
        for (i, x) in a.iter().enumerate() {
            for (j, y) in b.iter().enumerate() {
                out[i + j] = (&out[i + j] + x * y) % self.n;
            }
        }
        self.rem(&out, f)
    }

    /// `base^e mod f`.
    fn pow(&self, base: &[BigUint], e: &BigUint, f: &[BigUint]) -> Vec<BigUint> {
        let mut result = vec![one()];
        for i in (0..e.bits()).rev() {
            result = self.mul(&result, &result, f);
            if e.bit(i) {
                result = self.mul(&result, base, f);
            }
        }
        result
    }

    /// Monic greatest common divisor.
    fn gcd(&self, a: &[BigUint], b: &[BigUint]) -> Option<Vec<BigUint>> {
        let (mut a, mut b) = (Self::trim(a.to_vec()), Self::trim(b.to_vec()));
        while !b.is_empty() {
            let b_monic = self.monic(&b)?;
            let r = self.rem(&a, &b_monic);
            a = b_monic;
            b = r;
        }
        self.monic(&a)
    }

    fn monic(&self, a: &[BigUint]) -> Option<Vec<BigUint>> {
        let lead = a.last()?;
        let inv = invmod(lead, self.n)?;
        Some(a.iter().map(|c| (c * &inv) % self.n).collect())
    }

    /// The roots of the monic polynomial `f`, which splits into distinct
    /// linear factors mod the prime `n`.
    fn roots(&self, f: &[BigUint], depth: u32) -> Option<Vec<BigUint>> {
        let n = self.n;
        match f.len() {
            0 | 1 => return Some(vec![]),
            2 => return Some(vec![submod(&zero(), &f[0], n)]),
            _ => {}
        }
        if depth > 40 {
            return None;
        }
        // The product of the distinct linear factors: gcd(x^n - x, f).
        let x = vec![zero(), one()];
        let xn = self.pow(&x, n, f);
        let xn_minus_x = Self::trim(
            (0..f.len() - 1)
                .map(|i| {
                    let c = xn.get(i).cloned().unwrap_or_else(zero);
                    if i == 1 { submod(&c, &one(), n) } else { c }
                })
                .collect(),
        );
        let g = self.gcd(f, &xn_minus_x)?;
        if g.len() < f.len() {
            return self.roots(&g, depth + 1);
        }
        // Split with gcd((x + c)^((n-1)/2) - 1, f) for a shifting c.
        let shift = BigUint::from(depth + 1);
        let base = vec![shift, one()];
        let mut h = self.pow(&base, &((n - 1_u8) >> 1), f);
        h.resize(f.len() - 1, zero());
        h[0] = submod(&h[0], &one(), n);
        let h = Self::trim(h);
        let g = self.gcd(f, &h)?;
        if g.len() <= 1 || g.len() >= f.len() {
            return self.roots(f, depth + 1);
        }
        let mut roots = self.roots(&g, depth + 1)?;
        // The other factor: f / g.
        let other = self.div(f, &g)?;
        roots.extend(self.roots(&other, depth + 1)?);
        Some(roots)
    }

    /// `f / g` for monic `g` dividing monic `f`.
    fn div(&self, f: &[BigUint], g: &[BigUint]) -> Option<Vec<BigUint>> {
        let n = self.n;
        let mut rem = f.to_vec();
        let deg_g = g.len() - 1;
        let mut quotient = vec![zero(); f.len() - deg_g];
        while rem.len() > deg_g {
            let lead = rem.pop()?;
            let shift = rem.len() - deg_g;
            quotient[shift].clone_from(&lead);
            for (i, c) in g[..deg_g].iter().enumerate() {
                rem[shift + i] = submod(&rem[shift + i], &((&lead * c) % n), n);
            }
        }
        Some(quotient)
    }
}

/// The `j`-invariants of discriminant `d` mod the prime `n`.
fn class_roots(coefficients: &[&str], n: &BigUint) -> Vec<BigUint> {
    let mut f: Vec<BigUint> = coefficients.iter().map(|c| reduce(&big(c), n)).collect();
    f.push(one());
    PolyMod { n }.roots(&f, 0).unwrap_or_default()
}

/// The curves `(a, b)` with `j`-invariant `j` and their quadratic twist.
fn curves_for_j(j: &BigUint, n: &BigUint) -> Vec<(BigUint, BigUint)> {
    let k = submod(&BigUint::from(1728_u16), &(j % n), n);
    let a = (j * &k * 3_u8) % n;
    let b = (j * &k * &k * 2_u8) % n;
    let mut c = two();
    while jacobi(&BigInt::from(c.clone()), n) != -1 {
        c += 1_u8;
    }
    let twist = ((&a * &c * &c) % n, (&b * &c * &c * &c) % n);
    vec![(a, b), twist]
}

/// One ECPP candidate: the discriminant, the curve order `m` and its prime
/// factor `q` to prove next.
struct Candidate {
    d: i64,
    m: BigUint,
    q: BigUint,
}

/// The prime `q` of a curve order `m = k q`: the cofactor after the primes
/// of `primes` are divided out, split further by the base factoring stages
/// when `split` is set.
fn order_prime(m: &BigUint, primes: &[u64], split: bool) -> Option<BigUint> {
    let mut q = m.clone();
    for &p in primes {
        while &q % p == zero() {
            q /= p;
        }
    }
    if q == one() {
        return None;
    }
    if is_prob_prime(&q) != 0 {
        return Some(q);
    }
    if !split {
        return None;
    }
    factor_within(&q, Duration::from_secs(1)).primes.into_iter().max()
}

/// The `(D, m, q)` at `n` with `q` a probable prime above `(n^(1/4) + 1)^2`,
/// smallest `q` first. The discriminants are taken in table order until a
/// dozen candidates exist; with none, a second pass divides the orders by
/// the primes to a million and runs the factoring stages on the rest.
fn ecpp_candidates(n: &BigUint, primes: &[u64]) -> Vec<Candidate> {
    let qmin = rootint(n, 4) + 1_u8;
    let qmin = &qmin * &qmin;
    let mut out = Vec::new();
    let mut orders = Vec::new();
    for &(d, _) in CLASS_POLYNOMIALS {
        if out.len() >= 12 {
            break;
        }
        if kronecker(&BigInt::from(d), &BigInt::from(n.clone())) != 1 {
            continue;
        }
        let Some((t, v)) = cornacchia(d, n) else {
            continue;
        };
        for m in curve_orders(d, n, &t, &v) {
            if let Some(q) = order_prime(&m, primes, false) {
                if q > qmin && q < *n && q != m {
                    out.push(Candidate { d, m: m.clone(), q });
                }
            }
            orders.push((d, m));
        }
    }
    if out.is_empty() {
        let more = small_primes(1_000_000);
        for (d, m) in orders {
            if let Some(q) = order_prime(&m, &more, true) {
                if q > qmin && q < *n && q != m {
                    out.push(Candidate { d, m, q });
                }
            }
        }
    }
    out.sort_by_key(|c| c.q.bits());
    out
}

/// A curve of discriminant `d` with order `m` and a point of order a
/// multiple of `q` on it, as an `ECPP` block.
fn ecpp_block(n: &BigUint, cand: &Candidate) -> Option<Proof> {
    let curves: Vec<(BigUint, BigUint)> = match cand.d {
        -3 => (1_u32..=40).map(|b| (zero(), BigUint::from(b))).collect(),
        -4 => (1_u32..=40).map(|a| (BigUint::from(a), zero())).collect(),
        d => CLASS_POLYNOMIALS
            .iter()
            .find(|(dd, _)| *dd == d)
            .map(|(_, c)| class_roots(c, n))
            .unwrap_or_default()
            .iter()
            .flat_map(|j| curves_for_j(j, n))
            .collect(),
    };
    let k = &cand.m / &cand.q;
    for (a, b) in curves {
        let disc = (&a * &a * &a * 4_u8 + &b * &b * 27_u8) % n;
        if gcd(&disc, n) != one() {
            continue;
        }
        let curve = Curve { a: a.clone(), n };
        let mut tries = 0;
        let mut x = zero();
        while tries < 4 {
            let rhs = (&x * &x * &x + &a * &x + &b) % n;
            let Some(y) = sqrtmod_prime(&rhs, n) else {
                x += 1_u8;
                continue;
            };
            tries += 1;
            let point: Point = Some((x.clone(), y.clone()));
            match curve.mul(&point, &k) {
                Some(Some(u)) => {
                    if curve.mul(&Some(u), &cand.q) == Some(None) {
                        return Some(Proof::Ecpp {
                            n: n.clone(),
                            a: BigInt::from(a),
                            b: BigInt::from(b),
                            m: cand.m.clone(),
                            q: cand.q.clone(),
                            x,
                            y,
                        });
                    }
                    // The order of this twist is not m.
                    break;
                }
                Some(None) => x += 1_u8,
                None => return None,
            }
        }
    }
    None
}

/// The ECPP downrun from the probable prime `n`: each step proves `n`
/// prime if `q` is, until `q` has a BLS75 proof or is below 2^64.
/// `budget` bounds the blocks tried over all backtracking.
fn ecpp(n: &BigUint, primes: &[u64], budget: &mut u32) -> Attempt {
    if *n <= smallval() {
        return if native_prime(n) {
            Ok(vec![Proof::Small { n: n.clone() }])
        } else {
            Err(0)
        };
    }
    if let Ok(proofs) = bls75(n) {
        return Ok(proofs);
    }
    for cand in ecpp_candidates(n, primes) {
        if *budget == 0 {
            return Err(1);
        }
        *budget -= 1;
        let Some(block) = ecpp_block(n, &cand) else {
            continue;
        };
        if let Ok(rest) = ecpp(&cand.q, primes, budget) {
            let mut proofs = vec![block];
            proofs.extend(rest);
            return Ok(proofs);
        }
    }
    Err(1)
}

/// The proof blocks for `n`, or MPU's status for none.
fn prove(n: &BigUint) -> Attempt {
    if *n <= smallval() {
        return if native_prime(n) {
            Ok(vec![Proof::Small { n: n.clone() }])
        } else {
            Err(0)
        };
    }
    if is_prob_prime(n) == 0 {
        return Err(0);
    }
    match bls75(n) {
        Ok(proofs) => Ok(proofs),
        Err(0) => Err(0),
        Err(_) => {
            let primes = small_primes(10_000);
            let mut budget = 4000;
            ecpp(n, &primes, &mut budget)
        }
    }
}

/// Proves primality: 2 for a proven prime, 0 for a composite, 1 when no
/// proof was found.
///
/// Port of MPU 0.75 `PP.pm::is_provable_prime`: the native test below 2^64,
/// then BPSW and the BLS75 `n - 1` proof, then the ECPP downrun that
/// `Math::Prime::Util::GMP` provides to MPU.
///
/// ```
/// use math_prime_util::bigint::proving::is_provable_prime;
/// use num_bigint::BigUint;
/// let m89 = (BigUint::from(1_u8) << 89_usize) - 1_u8;
/// assert_eq!(is_provable_prime(&m89), 2);
/// assert_eq!(is_provable_prime(&(&m89 * &m89)), 0);
/// ```
pub fn is_provable_prime(n: &BigUint) -> u8 {
    is_provable_prime_with_cert(n).0
}

/// Proves primality and returns the status with the certificate text, or
/// an empty string when there is no proof.
///
/// Port of MPU 0.75 `PP.pm::is_provable_prime_with_cert`. The text is the
/// MPU format of [`Certificate`]; below 2^64 it is the native
/// [`crate::is_provable_prime_with_cert`] text.
///
/// ```
/// use math_prime_util::bigint::proving::{is_provable_prime_with_cert, verify_prime};
/// use num_bigint::BigUint;
/// let (status, cert) = is_provable_prime_with_cert(&((BigUint::from(1_u8) << 89_usize) - 1_u8));
/// assert_eq!(status, 2);
/// assert_eq!(verify_prime(&cert), Ok(true));
/// assert_eq!(is_provable_prime_with_cert(&BigUint::from(9_u8)), (0, String::new()));
/// ```
pub fn is_provable_prime_with_cert(n: &BigUint) -> (u8, String) {
    if let Some(small) = n.to_u64() {
        return crate::is_provable_prime_with_cert(small);
    }
    match prove(n) {
        Ok(proofs) => (
            2,
            Certificate {
                n: n.clone(),
                proofs,
            }
            .to_string(),
        ),
        Err(status) => (status, String::new()),
    }
}

/// The certificate text for `n`, or an empty string when `n` is composite
/// or no proof was found.
///
/// Port of MPU 0.75 `PP.pm::prime_certificate`.
///
/// ```
/// use math_prime_util::bigint::proving::prime_certificate;
/// use num_bigint::BigUint;
/// assert!(prime_certificate(&BigUint::from(u64::MAX)).is_empty());
/// assert!(prime_certificate(&(BigUint::from(u64::MAX) + 14_u8)).contains("Type BLS5"));
/// ```
pub fn prime_certificate(n: &BigUint) -> String {
    is_provable_prime_with_cert(n).1
}

/// Verifies an MPU certificate of any size.
///
/// Port of MPU 0.75 `PrimalityProving.pm::verify_cert` through
/// [`Certificate`]. Empty text gives `Ok(false)`.
///
/// # Errors
/// Returns the message MPU carps for malformed text or an incomplete chain;
/// `Ok(false)` when a block fails its conditions.
///
/// ```
/// use math_prime_util::bigint::proving::{prime_certificate, verify_prime};
/// use num_bigint::BigUint;
/// let cert = prime_certificate(&((BigUint::from(1_u8) << 89_usize) - 1_u8));
/// assert_eq!(verify_prime(&cert), Ok(true));
/// // 7 does not divide 2^89 - 2, so a block that claims it is rejected.
/// let q1 = cert.lines().find(|l| l.starts_with("Q[1] ")).unwrap_or("Q[1] ");
/// assert_eq!(verify_prime(&cert.replacen(q1, "Q[1] 7", 1)), Ok(false));
/// assert!(verify_prime("[MPU - Primality Certificate]\nType Small\nN 7\n").is_err());
/// ```
pub fn verify_prime(certificate: &str) -> Result<bool, String> {
    if certificate.trim().is_empty() {
        return Ok(false);
    }
    Certificate::parse(certificate)?.verify()
}

// ---------------------------------------------------------------------------
// AKS (PP.pm::is_aks_prime, Bernstein's 2003 variant).

#[allow(clippy::cast_precision_loss)]
fn log_gamma(x: f64) -> f64 {
    let lanczos = [
        0.999_999_999_999_809_9,
        676.520_368_121_885_1,
        -1_259.139_216_722_402_8,
        771.323_428_777_653_1,
        -176.615_029_162_140_6,
        12.507_343_278_686_905,
        -0.138_571_095_265_720_12,
        9.984_369_578_019_572e-6,
        1.505_632_735_149_311_6e-7,
    ];
    let base = x + 7.5;
    let sum = (1..=8)
        .rev()
        .fold(0.0, |s, i| s + lanczos[i] / (x + i as f64))
        + lanczos[0];
    0.918_938_533_204_672_8 + (sum / x).ln() + ((x + 0.5) * base.ln() - base)
}

#[allow(clippy::cast_precision_loss)]
fn log_binomial(n: i64, k: i64) -> f64 {
    if n < k {
        return -1e300;
    }
    log_gamma((n + 1) as f64) - log_gamma((k + 1) as f64) - log_gamma((n - k + 1) as f64)
}

/// `_bern41_acceptable` with `log_n = ln(n)`.
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
fn bern41_acceptable(log_n: f64, r: u64, s: u64) -> bool {
    let (r, s) = (
        i64::try_from(r).unwrap_or(i64::MAX),
        i64::try_from(s).unwrap_or(i64::MAX),
    );
    let scmp = (((r - 1) as f64 / 3.0).sqrt() + 0.99999).floor() * log_n;
    let d = ((0.5 * (r - 1) as f64) as i64).min(r - 2);
    let i = ((0.475 * (r - 1) as f64) as i64).min(d);
    let j = ((0.475 * (r - 1) as f64) as i64).min(r - 2 - d);
    let total = log_binomial(2 * s, i)
        + log_binomial(d, i)
        + log_binomial(2 * s - i, j)
        + log_binomial(r - 2 - d, j);
    total >= scmp + 1e-7
}

/// `ln(n)` from the top bits.
#[allow(clippy::cast_precision_loss)]
fn ln_big(n: &BigUint) -> f64 {
    let shift = n.bits().saturating_sub(53);
    let top = (n >> shift).to_u64().unwrap_or(1) as f64;
    top.ln() + shift as f64 * std::f64::consts::LN_2
}

/// `(px * py) mod (x^r - 1, n)`.
fn poly_mod_mul(px: &[BigUint], py: &[BigUint], r: usize, n: &BigUint) -> Vec<BigUint> {
    let mut res = vec![zero(); r];
    for (ix, x) in px.iter().enumerate() {
        if *x == zero() {
            continue;
        }
        for (iy, y) in py.iter().enumerate() {
            if *y == zero() {
                continue;
            }
            let k = (ix + iy) % r;
            res[k] = (&res[k] + x * y) % n;
        }
    }
    while res.len() > 1 && res.last().is_some_and(|c| *c == zero()) {
        res.pop();
    }
    res
}

/// `(x + a)^n = x^(n mod r) + a` in `(Z/nZ)[x] / (x^r - 1)`.
fn test_anr(a: u64, n: &BigUint, r: usize) -> bool {
    let mut pn = vec![BigUint::from(a) % n, one()];
    let mut res = vec![one()];
    for i in 0..n.bits() {
        if n.bit(i) {
            res = poly_mod_mul(&res, &pn, r, n);
        }
        if i + 1 < n.bits() {
            pn = poly_mod_mul(&pn, &pn, r, n);
        }
    }
    res.resize(r, zero());
    let nr = usize::try_from((n % r).to_u64().unwrap_or(0)).unwrap_or(0);
    res[nr] = submod(&res[nr], &one(), n);
    res[0] = submod(&res[0], &(BigUint::from(a) % n), n);
    res.iter().all(|c| *c == zero())
}

/// Tests primality with the AKS algorithm, Bernstein's 2003 variant.
///
/// Port of MPU 0.75 `PP.pm::is_aks_prime`; below 2^64 the native
/// [`crate::is_aks_prime`] answers. The polynomial step is slow: a 65-bit
/// prime takes seconds, and the time grows as the sixth power of the size.
///
/// ```
/// use math_prime_util::bigint::proving::is_aks_prime;
/// use num_bigint::BigUint;
/// assert!(is_aks_prime(&BigUint::from(101_u8)));
/// assert!(!is_aks_prime(&(BigUint::from(u64::MAX) + 2_u8)));
/// ```
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss
)]
pub fn is_aks_prime(n: &BigUint) -> bool {
    if let Some(small) = n.to_u64() {
        return crate::is_aks_prime(small);
    }
    if is_power_exp(n) > 1 || [2_u8, 3, 5, 7, 11].iter().any(|p| n % *p == zero()) {
        return false;
    }
    let log_n = ln_big(n);
    let log2n = log_n / std::f64::consts::LN_2 + 0.0001;
    let r0 = 0.010 * log2n * log2n;
    let rmult = 6;
    let mut r = crate::next_prime(if r0 < 2.0 { 2 } else { r0 as u64 }).unwrap_or(u64::MAX);
    while crate::is_primitive_root((n % r).to_u64().unwrap_or(0), r) != Some(true)
        || !bern41_acceptable(log_n, r, rmult * (r - 1))
    {
        r = crate::next_prime(r).unwrap_or(u64::MAX);
    }
    let (mut lo, mut hi) = ((0.475 * (r - 1) as f64) as u64, rmult * (r - 1));
    lo = lo.max(1);
    while lo < hi {
        let s = lo + ((hi - lo) >> 1);
        if bern41_acceptable(log_n, r, s) {
            hi = s;
        } else {
            lo = s + 1;
        }
    }
    let s = hi + 1;
    let slim = s * (s - 1);
    if small_primes(slim).iter().any(|p| n % *p == zero()) {
        return false;
    }
    let nm1 = n - 1_u8;
    if (2..=s).any(|a| powmod(&BigUint::from(a), &nm1, n) != one()) {
        return false;
    }
    (2..=s).all(|a| test_anr(a, n, r as usize))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::too_many_lines)]
    use super::*;

    fn parse(text: &str) -> BigUint {
        text.parse().unwrap_or_default()
    }

    #[test]
    fn square_roots_and_cornacchia() {
        let p = parse("1000000000000000000000000000057");
        for a in [2_u32, 3, 5, 7, 11, 13, 17, 19, 23] {
            let a = BigUint::from(a);
            let residue = jacobi(&BigInt::from(a.clone()), &p) == 1;
            let root = sqrtmod_prime(&a, &p);
            assert_eq!(root.is_some(), residue);
            if let Some(r) = root {
                assert_eq!((&r * &r) % &p, a);
            }
        }
        assert_eq!(sqrtmod_prime(&zero(), &p), Some(zero()));
        // p = 1 mod 4 takes the Tonelli–Shanks path: 2 is a non-residue there, 3 a residue.
        let p = parse("18446744073709551629");
        assert_eq!(sqrtmod_prime(&two(), &p), None);
        let a = (3_u8..=255)
            .map(BigUint::from)
            .find(|a| jacobi(&BigInt::from(a.clone()), &p) == 1);
        let a = a.unwrap_or_default();
        let root = sqrtmod_prime(&a, &p).unwrap_or_default();
        assert_eq!((&root * &root) % &p, a);
        assert_eq!(sqrtmod_prime(&two(), &BigUint::from(4_u8)), None);
        // A composite modulus gives no root or a wrong one, never a panic.
        let c = parse("18446744073709551617");
        assert!(sqrtmod_prime(&BigUint::from(3_u8), &c).is_none());
        for &(d, _) in CLASS_POLYNOMIALS {
            if kronecker(&BigInt::from(d), &BigInt::from(p.clone())) != 1 {
                continue;
            }
            if let Some((t, v)) = cornacchia(d, &p) {
                assert_eq!(&t * &t + &v * &v * d.unsigned_abs(), &p << 2_u8, "{d}");
            }
        }
    }

    /// Every table entry gives a curve whose order is one of the candidates.
    #[test]
    fn class_polynomials_give_curves() {
        let mut p = parse("18446744073709551629");
        let mut checked = 0;
        for &(d, coefficients) in CLASS_POLYNOMIALS {
            // A prime that splits for this discriminant.
            let mut tries = 0;
            loop {
                p = super::super::primality::next_prime(&p);
                tries += 1;
                assert!(tries < 2000, "no split prime for {d}");
                if kronecker(&BigInt::from(d), &BigInt::from(p.clone())) == 1
                    && cornacchia(d, &p).is_some()
                {
                    break;
                }
            }
            let (t, v) = cornacchia(d, &p).unwrap_or_default();
            let roots = if d == -3 || d == -4 {
                vec![]
            } else {
                let roots = class_roots(coefficients, &p);
                assert_eq!(roots.len(), coefficients.len(), "roots of {d}");
                roots
            };
            let curves: Vec<(BigUint, BigUint)> = match d {
                -3 => vec![(zero(), one())],
                -4 => vec![(one(), zero())],
                _ => curves_for_j(&roots[0], &p),
            };
            let orders = curve_orders(d, &p, &t, &v);
            let mut found = false;
            for (a, b) in curves {
                let curve = Curve {
                    a: a.clone(),
                    n: &p,
                };
                let mut x = one();
                let y = loop {
                    let rhs = (&x * &x * &x + &a * &x + &b) % &p;
                    if let Some(y) = sqrtmod_prime(&rhs, &p) {
                        break y;
                    }
                    x += 1_u8;
                };
                let point: Point = Some((x, y));
                found |= orders.iter().any(|m| curve.mul(&point, m) == Some(None));
            }
            assert!(found, "no curve of order m for D = {d}");
            checked += 1;
        }
        assert_eq!(checked, CLASS_POLYNOMIALS.len());
    }

    #[test]
    fn curve_arithmetic() {
        let n = BigUint::from(101_u8);
        let curve = Curve {
            a: BigUint::from(3_u8),
            n: &n,
        };
        let p: Point = Some((BigUint::from(1_u8), BigUint::from(2_u8)));
        assert_eq!(curve.add(&None, &p), Some(p.clone()));
        assert_eq!(curve.add(&p, &None), Some(p.clone()));
        let neg: Point = Some((BigUint::from(1_u8), BigUint::from(99_u8)));
        assert_eq!(curve.add(&p, &neg), Some(None));
        assert_eq!(curve.add(&p, &p), curve.double(&p));
        assert_eq!(curve.double(&None), Some(None));
        let two_torsion: Point = Some((BigUint::from(5_u8), zero()));
        assert_eq!(curve.double(&two_torsion), Some(None));
        assert_eq!(curve.mul(&p, &zero()), Some(None));
        assert_eq!(curve.mul(&p, &one()), Some(p.clone()));
        let triple = curve.add(&curve.double(&p).unwrap_or_default(), &p);
        assert_eq!(curve.mul(&p, &BigUint::from(3_u8)), triple);
        // A composite modulus can make an inversion fail.
        let n = BigUint::from(15_u8);
        let curve = Curve { a: one(), n: &n };
        let p: Point = Some((BigUint::from(4_u8), BigUint::from(1_u8)));
        let q: Point = Some((BigUint::from(9_u8), BigUint::from(2_u8)));
        assert_eq!(curve.add(&p, &q), None);
        let p: Point = Some((BigUint::from(1_u8), BigUint::from(5_u8)));
        assert_eq!(curve.double(&p), None);
        assert_eq!(curve.mul(&p, &two()), None);
    }

    #[test]
    fn polynomial_helpers() {
        let n = BigUint::from(101_u8);
        let pm = PolyMod { n: &n };
        // (x - 3)(x - 5)(x - 7) = x^3 - 15x^2 + 71x - 105.
        let f: Vec<BigUint> = [
            reduce(&BigInt::from(-105), &n),
            BigUint::from(71_u8),
            reduce(&BigInt::from(-15), &n),
            one(),
        ]
        .to_vec();
        let mut roots = pm.roots(&f, 0).unwrap_or_default();
        roots.sort();
        assert_eq!(
            roots,
            vec![
                BigUint::from(3_u8),
                BigUint::from(5_u8),
                BigUint::from(7_u8)
            ]
        );
        assert_eq!(pm.roots(&[one()], 0), Some(vec![]));
        assert_eq!(pm.roots(&[], 0), Some(vec![]));
        assert_eq!(PolyMod::trim(vec![one(), zero(), zero()]), vec![one()]);
        assert_eq!(pm.mul(&[], &[one()], &f), Vec::<BigUint>::new());
        assert_eq!(pm.gcd(&f, &[]), pm.monic(&f));
        assert!(pm.monic(&[]).is_none());
        assert!(pm.roots(&f, 41).is_none());
        assert_eq!(pm.rem(&[one(), zero(), zero(), zero()], &f), vec![one()]);
        // A repeated root: (x - 3)^2 (x - 5)(x - 7) still gives the distinct roots.
        let x5 = [zero(), zero(), zero(), zero(), zero(), one()];
        let g = pm.mul(&f, &[reduce(&BigInt::from(-3), &n), one()], &x5);
        let mut roots = pm.roots(&g, 0).unwrap_or_default();
        roots.sort();
        assert_eq!(
            roots,
            vec![
                BigUint::from(3_u8),
                BigUint::from(5_u8),
                BigUint::from(7_u8)
            ]
        );
        assert_eq!(
            poly_mod_mul(&[one(), one()], &[one(), zero(), one()], 3, &n),
            vec![two(), one(), one()]
        );
        let poly = poly_mod_mul(&[one(), one()], &[one(), one()], 2, &n);
        assert_eq!(poly, vec![two(), two()]);
        assert_eq!(poly_mod_mul(&[zero()], &[one()], 1, &n), vec![zero()]);
    }

    #[test]
    fn bounded_factoring_for_proofs() {
        let m521 = (one() << 521_usize) - 1_u8;
        let m607 = (one() << 607_usize) - 1_u8;
        let hard = &m521 * &m607;
        assert_eq!(factor_within(&hard, Duration::ZERO).unsplit, vec![hard.clone()]);
        let factored = factor_within(&(&hard * 6_u8), Duration::ZERO);
        assert_eq!(factored.primes, vec![two(), BigUint::from(3_u8)]);
        assert_eq!(factored.unsplit, vec![hard.clone()]);
        assert_eq!(factor_within(&(BigUint::from(360_u16) * &m521), Duration::from_secs(2)).primes.len(), 7);
        let one = factor_within(&one(), Duration::ZERO);
        assert!(one.primes.is_empty());
        assert!(one.unsplit.is_empty());
    }

    #[test]
    fn parser_errors_match_mpu() {
        let header = "[MPU - Primality Certificate]\nVersion 1.0\n\nProof for:\nN 101\n\n";
        for (text, message) in [
            (
                "[PRIMO - Primality Certificate]\n",
                "Unknown certificate type: PRIMO",
            ),
            (
                "[MPU - Primality Certificate]\nProof for:\nN 7\nProof for:\nN 7\n",
                "Certificate with multiple N values",
            ),
            (
                "[MPU - Primality Certificate]\nBase 16\n",
                "Only base 10 supported, sorry",
            ),
            (
                "[MPU - Primality Certificate]\nType Small\nN 7\n",
                "Starting type without telling me the N value!",
            ),
            ("[MPU - Primality Certificate]\n", "No N"),
            (
                "[MPU - Primality Certificate]\nProof for:\n",
                "end of file during type Proof for",
            ),
            (
                "[MPU - Primality Certificate]\nProof for:\nType Small\n",
                "Still missing values in type Proof for",
            ),
            (
                "[MPU - Primality Certificate]\nProof for:\nM 7\n",
                "Type Proof for: repeated or inappropriate var: M 7",
            ),
            (
                "[MPU - Primality Certificate]\nProof for:\nN\n",
                "Unrecognized line: N",
            ),
            (
                "[MPU - Primality Certificate]\nProof for:\nN -7\n",
                "N must not be negative",
            ),
        ] {
            assert_eq!(Certificate::parse(text), Err(message.to_string()), "{text}");
        }
        for (block, message) in [
            ("Type Nope\n", "Unknown type: NOPE"),
            ("Type BLS5\nN 101\n", "end of file during type BLS5"),
            ("Type BLS5\nN 101\nN 101\n----\n", "BLS5: N redefined"),
            ("Type BLS5\nN 101\nQ[2] 5\n----\n", "BLS5: Invalid index: 2"),
            (
                "Type BLS5\nN 101\nA[2] 5\n----\n",
                "BLS5: Invalid index: A[2]",
            ),
            (
                "Type BLS5\nN 101\nA[x] 5\n----\n",
                "Unrecognized line: A[x] 5",
            ),
            (
                "Type BLS5\nN 101\nQ[1] x\n----\n",
                "Unrecognized line: Q[1] x",
            ),
            ("Type BLS5\nN 101\nQ[1]\n----\n", "Unrecognized line: Q[1]"),
            ("Type BLS5\nQ[1] 5\n----\n", "BLS5: N missing"),
            ("Type BLS5\nN 101\nB 5\n----\n", "Unrecognized line: B 5"),
            (
                "Type Lucas\nN 101\nQ[1] 5\n",
                "end of file during type Lucas",
            ),
            ("Type Lucas\nQ[1] 5\nA 2\n", "Lucas: N missing"),
            (
                "Type Lucas\nN 101\nQ[1] 5\n----\n",
                "Unrecognized line: ----",
            ),
            ("Type ECPP\nN 101\nA 1\n", "end of file during type ECPP"),
            (
                "Type ECPP\nN 101\nA 1\nA 2\n",
                "Type ECPP: repeated or inappropriate var: A 2",
            ),
            (
                "Type ECPP\nN 101\nM -5\nA 1\nB 1\nQ 1\nX 1\nY 1\n",
                "M must not be negative",
            ),
            (
                "Type Small\nN 101\nType BLS3\nN 101\nQ 5\n",
                "end of file during type BLS3",
            ),
            (
                "Type BLS15\nN 101\nQ 5\nLP 1\nLQ x\n",
                "Unrecognized line: LQ x",
            ),
        ] {
            assert_eq!(
                Certificate::parse(&format!("{header}{block}")),
                Err(message.to_string()),
                "{block}"
            );
        }
        // Lines MPU ignores: anything else at the top level, trailing text on a value.
        let cert = Certificate::parse(&format!("{header}junk line\nType Small\nN 101 trailing\n"))
            .unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(
            cert.proofs,
            vec![Proof::Small {
                n: BigUint::from(101_u8)
            }]
        );
        assert_eq!(parse_field("N"), None);
        assert_eq!(parse_field("N -"), None);
        assert_eq!(parse_field("n -12x"), Some(("N".into(), BigInt::from(-12))));
        assert_eq!(indexed("Q[3]", "Q"), Some(3));
        assert_eq!(indexed("Q[3", "Q"), None);
    }

    #[test]
    fn verifier_conditions() {
        let n = BigUint::from(23_u8);
        let (q, a) = (BigUint::from(11_u8), BigUint::from(5_u8));
        assert!(check_bls3(&n, &q, &a));
        assert!(!check_bls3(&n, &BigUint::from(4_u8), &a));
        assert!(!check_bls3(&n, &BigUint::from(3_u8), &a));
        assert!(!check_bls3(&one(), &q, &a));
        assert!(!check_bls3(&n, &q, &one()));
        assert!(check_pocklington(&n, &q, &a));
        assert!(!check_pocklington(&n, &q, &one()));
        assert!(!check_pocklington(&n, &zero(), &a));
        assert!(!check_pocklington(&one(), &q, &a));
        assert!(!check_pocklington(
            &BigUint::from(25_u8),
            &BigUint::from(3_u8),
            &a
        ));
        // BLS15: 23 + 1 = 2 * 3 * 4; q = 3 is an odd prime with 2q - 1 > sqrt(23).
        assert!(check_bls15(
            &n,
            &BigUint::from(3_u8),
            &BigInt::from(1),
            &BigInt::from(-1)
        ));
        assert!(!check_bls15(
            &n,
            &BigUint::from(12_u8),
            &BigInt::from(1),
            &BigInt::from(-1)
        ));
        assert!(!check_bls15(
            &BigUint::from(22_u8),
            &q,
            &BigInt::from(1),
            &BigInt::from(-1)
        ));
        // 29 + 1 = 2 * 3 * 5: q = 5, D = P^2 - 4Q with (D/29) = -1; q = 3 has 2q - 1 = sqrt(29).
        let n29 = BigUint::from(29_u8);
        assert!(!check_bls15(
            &n29,
            &BigUint::from(3_u8),
            &BigInt::from(1),
            &BigInt::from(-1)
        ));
        assert!(!check_bls15(
            &n29,
            &BigUint::from(5_u8),
            &BigInt::from(2),
            &BigInt::from(1)
        ));
        let mut found = false;
        for p in 1..30 {
            for qq in -10..10_i64 {
                found |= check_bls15(
                    &n29,
                    &BigUint::from(5_u8),
                    &BigInt::from(p),
                    &BigInt::from(qq),
                );
            }
        }
        assert!(found);
        assert!(!check_bls15(
            &n29,
            &BigUint::from(7_u8),
            &BigInt::from(1),
            &BigInt::from(-1)
        ));
        // Lucas: 101 - 1 = 2^2 * 5^2.
        let n101 = BigUint::from(101_u8);
        let qs = [two(), BigUint::from(5_u8)];
        assert!(check_lucas(&n101, &qs, &two()));
        assert!(!check_lucas(&n101, &qs, &one()));
        assert!(!check_lucas(&n101, &qs, &BigUint::from(4_u8)));
        assert!(!check_lucas(&n101, &qs[..1], &two()));
        assert!(!check_lucas(&n101, &[BigUint::from(3_u8)], &two()));
        assert!(!check_lucas(&n101, &qs, &n101));
        assert!(!check_lucas(&n101, &[one()], &two()));
        // BLS5: 101 with Q[1] = 5.
        assert!(check_bls5(&n101, &[BigUint::from(5_u8)], &[two(), two()]));
        assert!(check_bls5(&n101, &[BigUint::from(5_u8)], &[]));
        assert!(!check_bls5(&n101, &[BigUint::from(3_u8)], &[]));
        assert!(!check_bls5(&n101, &[BigUint::from(5_u8)], &[one()]));
        // The factor 2 alone: F = 4, R = 25, and 101 < 5 * 33 with r^2 - 8s < 0.
        assert!(check_bls5(&n101, &[], &[]));
        assert!(!check_bls5(&one(), &[], &[]));
        assert!(!check_bls5(
            &BigUint::from(31_u8),
            &[BigUint::from(3_u8)],
            &[BigUint::from(3_u8), two()]
        ));
        assert!(check_bls5(
            &BigUint::from(31_u8),
            &[BigUint::from(3_u8), BigUint::from(5_u8)],
            &[
                BigUint::from(3_u8),
                BigUint::from(3_u8),
                BigUint::from(3_u8)
            ]
        ));
        assert_eq!(bls5_bound(&n101, &one(), &BigUint::from(100_u8)), None);
        // The factor 2 alone is not enough for a 65-bit prime.
        assert!(!check_bls5(&(smallval() + 14_u8), &[], &[]));
        // A Fermat failure in a Lucas block.
        assert!(!check_lucas(&BigUint::from(9_u8), &[two()], &two()));
        // (m/q)P is the point at infinity: (0, 1) has order 3 on y^2 = x^3 + 1.
        assert!(!check_ecpp(
            &BigUint::from(35_u8),
            &BigInt::from(0),
            &BigInt::from(1),
            &BigUint::from(39_u8),
            &BigUint::from(13_u8),
            &zero(),
            &one()
        ));
        assert_eq!(
            bls5_bound(&n101, &BigUint::from(4_u8), &BigUint::from(10_u8)),
            None
        );
        // ECPP conditions one by one.
        let ok = Proof::Ecpp {
            n: BigUint::from(1_000_003_u32),
            a: BigInt::from(0),
            b: BigInt::from(1),
            m: BigUint::from(1_000_004_u32),
            q: BigUint::from(250_001_u32),
            x: BigUint::from(0_u8),
            y: BigUint::from(1_u8),
        };
        // Whatever the point order, every arithmetic branch runs.
        let _ = ok.check();
        assert!(!check_ecpp(
            &BigUint::from(9_u8),
            &BigInt::from(1),
            &BigInt::from(1),
            &one(),
            &one(),
            &one(),
            &one()
        ));
        assert!(!check_ecpp(
            &zero(),
            &BigInt::from(1),
            &BigInt::from(1),
            &one(),
            &one(),
            &one(),
            &one()
        ));
        let n = BigUint::from(1_000_003_u32);
        assert!(!check_ecpp(
            &n,
            &BigInt::from(0),
            &BigInt::from(0),
            &n,
            &one(),
            &one(),
            &one()
        ));
        assert!(!check_ecpp(
            &n,
            &BigInt::from(0),
            &BigInt::from(1),
            &n,
            &one(),
            &one(),
            &one()
        ));
        assert!(!check_ecpp(
            &n,
            &BigInt::from(0),
            &BigInt::from(1),
            &one(),
            &one(),
            &zero(),
            &one()
        ));
        assert!(!check_ecpp(
            &n,
            &BigInt::from(0),
            &BigInt::from(1),
            &(&n * 2_u8),
            &one(),
            &zero(),
            &one()
        ));
        assert!(!check_ecpp(
            &n,
            &BigInt::from(0),
            &BigInt::from(1),
            &(&n + 1_u8),
            &one(),
            &zero(),
            &one()
        ));
        assert!(!check_ecpp(
            &n,
            &BigInt::from(0),
            &BigInt::from(1),
            &(&n + 1_u8),
            &n,
            &zero(),
            &one()
        ));
        assert!(!check_ecpp(
            &n,
            &BigInt::from(0),
            &BigInt::from(1),
            &(&n + 1_u8),
            &(&n + 1_u8),
            &zero(),
            &one()
        ));
        assert!(!check_ecpp(
            &n,
            &BigInt::from(0),
            &BigInt::from(1),
            &(&n + 1_u8),
            &BigUint::from(3001_u16),
            &zero(),
            &one()
        ));
        assert!(!check_primo(
            &zero(),
            &one(),
            &one(),
            &BigInt::from(1),
            &BigInt::from(1),
            &one()
        ));
        assert!(!check_primo(
            &n,
            &one(),
            &one(),
            &BigInt::from(1),
            &BigInt::from(1),
            &n
        ));
        assert!(
            Proof::Ecpp3 {
                n: n.clone(),
                s: one(),
                r: one(),
                a: BigInt::from(n.clone()),
                b: BigInt::from(1),
                t: one()
            }
            .check()
            .is_none()
        );
        assert!(
            Proof::Ecpp4 {
                n: n.clone(),
                s: one(),
                r: one(),
                j: BigInt::from(n.clone()),
                t: one()
            }
            .check()
            .is_none()
        );
        assert!(
            Proof::Ecpp4 {
                n: n.clone(),
                s: one(),
                r: one(),
                j: BigInt::from(5),
                t: one()
            }
            .check()
            .is_none()
        );
        assert!(
            Proof::Small {
                n: smallval() + 14_u8
            }
            .check()
            .is_none()
        );
        assert!(
            Proof::Small {
                n: BigUint::from(9_u8)
            }
            .check()
            .is_none()
        );
    }

    #[test]
    fn chain_rules() {
        let n = BigUint::from(101_u8);
        let small = Certificate {
            n: n.clone(),
            proofs: vec![Proof::Small { n: n.clone() }],
        };
        assert_eq!(small.verify(), Ok(true));
        // No proof block at all: a small prime still verifies, a composite target does not.
        assert_eq!(
            Certificate {
                n: n.clone(),
                proofs: vec![]
            }
            .verify(),
            Ok(true)
        );
        assert_eq!(
            Certificate {
                n: BigUint::from(9_u8),
                proofs: vec![]
            }
            .verify(),
            Ok(false)
        );
        let big = smallval() + 14_u8;
        assert_eq!(
            Certificate {
                n: big.clone(),
                proofs: vec![]
            }
            .verify(),
            Err(format!("q value {big} has no proof"))
        );
        // A block whose q is a small composite.
        let bad = Certificate {
            n: n.clone(),
            proofs: vec![
                Proof::Lucas {
                    n: n.clone(),
                    q: vec![two(), BigUint::from(5_u8)],
                    a: two(),
                },
                Proof::Bls3 {
                    n: BigUint::from(5_u8),
                    q: BigUint::from(4_u8),
                    a: two(),
                },
            ],
        };
        assert_eq!(bad.verify(), Ok(false));
        let bad = Certificate {
            n: n.clone(),
            proofs: vec![Proof::Small {
                n: BigUint::from(9_u8),
            }],
        };
        assert_eq!(bad.verify(), Ok(false));
        // The same n twice: the later block wins, as in MPU's hash.
        let twice = Certificate {
            n: n.clone(),
            proofs: vec![Proof::Small { n: n.clone() }, Proof::Small { n: n.clone() }],
        };
        assert_eq!(twice.verify(), Ok(true));
        assert_eq!(verify_prime("  \n"), Ok(false));
    }

    #[test]
    fn display_round_trip() {
        let n = smallval() + 14_u8;
        let proofs = vec![
            Proof::Bls5 {
                n: n.clone(),
                q: vec![BigUint::from(3_u8)],
                a: vec![two(), BigUint::from(5_u8)],
            },
            Proof::Lucas {
                n: n.clone(),
                q: vec![two()],
                a: two(),
            },
            Proof::Bls3 {
                n: n.clone(),
                q: two(),
                a: two(),
            },
            Proof::Pocklington {
                n: n.clone(),
                q: two(),
                a: two(),
            },
            Proof::Bls15 {
                n: n.clone(),
                q: two(),
                lp: BigInt::from(-1),
                lq: BigInt::from(2),
            },
            Proof::Ecpp {
                n: n.clone(),
                a: BigInt::from(-1),
                b: BigInt::from(2),
                m: one(),
                q: one(),
                x: one(),
                y: one(),
            },
            Proof::Ecpp3 {
                n: n.clone(),
                s: one(),
                r: one(),
                a: BigInt::from(-1),
                b: BigInt::from(2),
                t: one(),
            },
            Proof::Ecpp4 {
                n: n.clone(),
                s: one(),
                r: one(),
                j: BigInt::from(-1),
                t: one(),
            },
            Proof::Small { n: n.clone() },
        ];
        let cert = Certificate {
            n: n.clone(),
            proofs,
        };
        let text = cert.to_string();
        assert!(text.contains("Type BLS5\nN 18446744073709551629\nQ[1] 3\nA[1] 5\n----\n"));
        assert!(text.contains("Type ECPP4\nN 18446744073709551629\nS 1\nR 1\nJ -1\nT 1\n"));
        assert_eq!(Certificate::parse(&text), Ok(cert));
    }

    /// A JSON writer over `serde::Serializer` and a value tree over
    /// `serde::Deserializer`, enough to exercise the derived forms without a
    /// format crate.
    #[cfg(feature = "serde")]
    mod serde_support {
        use serde::de::{self, IntoDeserializer, Visitor};
        use serde::ser::{self, Impossible, Serialize};
        use std::fmt::{Display, Write};

        #[derive(Debug)]
        pub struct Error(String);
        impl Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.0)
            }
        }
        impl std::error::Error for Error {}
        impl ser::Error for Error {
            fn custom<T: Display>(m: T) -> Self {
                Self(m.to_string())
            }
        }

        pub struct Json(pub String);
        pub struct Group<'a> {
            out: &'a mut Json,
            first: bool,
            close: char,
        }
        impl Group<'_> {
            fn sep(&mut self) {
                if !self.first {
                    self.out.0.push(',');
                }
                self.first = false;
            }
        }
        impl ser::SerializeSeq for Group<'_> {
            type Ok = ();
            type Error = Error;
            fn serialize_element<T: Serialize + ?Sized>(&mut self, v: &T) -> Result<(), Error> {
                self.sep();
                v.serialize(&mut *self.out)
            }
            fn end(self) -> Result<(), Error> {
                self.out.0.push(self.close);
                Ok(())
            }
        }
        impl ser::SerializeStruct for Group<'_> {
            type Ok = ();
            type Error = Error;
            fn serialize_field<T: Serialize + ?Sized>(
                &mut self,
                key: &'static str,
                v: &T,
            ) -> Result<(), Error> {
                self.sep();
                let _ = write!(self.out.0, "{key:?}:");
                v.serialize(&mut *self.out)
            }
            fn end(self) -> Result<(), Error> {
                self.out.0.push(self.close);
                Ok(())
            }
        }
        macro_rules! unsupported {
            ($($name:ident($($arg:ident: $ty:ty),*) -> $ret:ty;)*) => {
                $(fn $name(self, $($arg: $ty),*) -> Result<$ret, Error> {
                    $(let _ = $arg;)*
                    Err(ser::Error::custom(stringify!($name)))
                })*
            };
        }
        impl<'a> ser::Serializer for &'a mut Json {
            type Ok = ();
            type Error = Error;
            type SerializeSeq = Group<'a>;
            type SerializeTuple = Impossible<(), Error>;
            type SerializeTupleStruct = Impossible<(), Error>;
            type SerializeTupleVariant = Impossible<(), Error>;
            type SerializeMap = Impossible<(), Error>;
            type SerializeStruct = Group<'a>;
            type SerializeStructVariant = Impossible<(), Error>;
            fn serialize_str(self, v: &str) -> Result<(), Error> {
                let _ = write!(self.0, "{v:?}");
                Ok(())
            }
            fn serialize_seq(self, _len: Option<usize>) -> Result<Group<'a>, Error> {
                self.0.push('[');
                Ok(Group {
                    out: self,
                    first: true,
                    close: ']',
                })
            }
            fn serialize_struct(
                self,
                _name: &'static str,
                _len: usize,
            ) -> Result<Group<'a>, Error> {
                self.0.push('{');
                Ok(Group {
                    out: self,
                    first: true,
                    close: '}',
                })
            }
            fn serialize_some<T: Serialize + ?Sized>(self, _v: &T) -> Result<(), Error> {
                Err(ser::Error::custom("some"))
            }
            fn serialize_newtype_struct<T: Serialize + ?Sized>(
                self,
                _n: &'static str,
                _v: &T,
            ) -> Result<(), Error> {
                Err(ser::Error::custom("newtype struct"))
            }
            fn serialize_newtype_variant<T: Serialize + ?Sized>(
                self,
                _n: &'static str,
                _i: u32,
                _v: &'static str,
                _x: &T,
            ) -> Result<(), Error> {
                Err(ser::Error::custom("newtype variant"))
            }
            unsupported! {
                serialize_bool(v: bool) -> (); serialize_i8(v: i8) -> (); serialize_i16(v: i16) -> ();
                serialize_i32(v: i32) -> (); serialize_i64(v: i64) -> (); serialize_u8(v: u8) -> ();
                serialize_u16(v: u16) -> (); serialize_u32(v: u32) -> (); serialize_u64(v: u64) -> ();
                serialize_f32(v: f32) -> (); serialize_f64(v: f64) -> (); serialize_char(v: char) -> ();
                serialize_bytes(v: &[u8]) -> (); serialize_none() -> (); serialize_unit() -> ();
                serialize_unit_struct(name: &'static str) -> ();
                serialize_unit_variant(name: &'static str, index: u32, variant: &'static str) -> ();
                serialize_tuple(len: usize) -> Self::SerializeTuple;
                serialize_tuple_struct(name: &'static str, len: usize) -> Self::SerializeTupleStruct;
                serialize_tuple_variant(name: &'static str, index: u32, variant: &'static str, len: usize) -> Self::SerializeTupleVariant;
                serialize_map(len: Option<usize>) -> Self::SerializeMap;
                serialize_struct_variant(name: &'static str, index: u32, variant: &'static str, len: usize) -> Self::SerializeStructVariant;
            }
        }

        /// A self-describing value tree.
        #[derive(Clone)]
        pub enum Val {
            S(&'static str),
            L(Vec<Val>),
            M(Vec<(&'static str, Val)>),
        }
        pub type DeError = de::value::Error;
        impl IntoDeserializer<'_, DeError> for Val {
            type Deserializer = Self;
            fn into_deserializer(self) -> Self {
                self
            }
        }
        impl<'de> de::Deserializer<'de> for Val {
            type Error = DeError;
            fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, DeError> {
                match self {
                    Self::S(s) => visitor.visit_str(s),
                    Self::L(items) => {
                        visitor.visit_seq(de::value::SeqDeserializer::new(items.into_iter()))
                    }
                    Self::M(pairs) => {
                        visitor.visit_map(de::value::MapDeserializer::new(pairs.into_iter()))
                    }
                }
            }
            serde::forward_to_deserialize_any! {
                bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes
                byte_buf option unit unit_struct newtype_struct seq tuple tuple_struct
                map struct enum identifier ignored_any
            }
        }
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_round_trip() {
        use serde::{Deserialize, Serialize};
        use serde_support::{Json, Val};
        let n = smallval() + 14_u8;
        let cert = Certificate {
            n: n.clone(),
            proofs: vec![
                Proof::Bls5 {
                    n: n.clone(),
                    q: vec![BigUint::from(3_u8)],
                    a: vec![two(), two()],
                },
                Proof::Ecpp {
                    n: n.clone(),
                    a: BigInt::from(-1),
                    b: BigInt::from(2),
                    m: one(),
                    q: one(),
                    x: one(),
                    y: one(),
                },
            ],
        };
        let mut json = Json(String::new());
        assert!(cert.serialize(&mut json).is_ok());
        assert_eq!(
            json.0,
            "{\"n\":\"18446744073709551629\",\"proofs\":[\
             {\"type\":\"Bls5\",\"n\":\"18446744073709551629\",\"q\":[\"3\"],\"a\":[\"2\",\"2\"]},\
             {\"type\":\"Ecpp\",\"n\":\"18446744073709551629\",\"a\":\"-1\",\"b\":\"2\",\"m\":\"1\",\"q\":\"1\",\"x\":\"1\",\"y\":\"1\"}]}"
        );
        let tree = Val::M(vec![
            ("n", Val::S("18446744073709551629")),
            (
                "proofs",
                Val::L(vec![
                    Val::M(vec![
                        ("type", Val::S("Bls5")),
                        ("n", Val::S("18446744073709551629")),
                        ("q", Val::L(vec![Val::S("3")])),
                        ("a", Val::L(vec![Val::S("2"), Val::S("2")])),
                    ]),
                    Val::M(vec![
                        ("type", Val::S("Ecpp")),
                        ("n", Val::S("18446744073709551629")),
                        ("a", Val::S("-1")),
                        ("b", Val::S("2")),
                        ("m", Val::S("1")),
                        ("q", Val::S("1")),
                        ("x", Val::S("1")),
                        ("y", Val::S("1")),
                    ]),
                ]),
            ),
        ]);
        assert_eq!(Certificate::deserialize(tree).ok(), Some(cert));
        for bad in [
            Val::M(vec![("n", Val::S("x")), ("proofs", Val::L(vec![]))]),
            Val::M(vec![
                ("n", Val::S("5")),
                (
                    "proofs",
                    Val::L(vec![Val::M(vec![
                        ("type", Val::S("Bls5")),
                        ("n", Val::S("5")),
                        ("q", Val::L(vec![Val::S("y")])),
                        ("a", Val::L(vec![])),
                    ])]),
                ),
            ]),
            Val::M(vec![
                ("n", Val::S("5")),
                (
                    "proofs",
                    Val::L(vec![Val::M(vec![
                        ("type", Val::S("Small")),
                        ("n", Val::S("-5")),
                    ])]),
                ),
            ]),
            Val::M(vec![
                ("n", Val::S("5")),
                (
                    "proofs",
                    Val::L(vec![Val::M(vec![
                        ("type", Val::S("Ecpp")),
                        ("n", Val::S("5")),
                        ("a", Val::S("-")),
                        ("b", Val::S("1")),
                        ("m", Val::S("1")),
                        ("q", Val::S("1")),
                        ("x", Val::S("1")),
                        ("y", Val::S("1")),
                    ])]),
                ),
            ]),
        ] {
            assert!(Certificate::deserialize(bad).is_err());
        }
    }

    #[test]
    fn proving_paths() {
        // Native range.
        assert_eq!(is_provable_prime(&BigUint::from(97_u8)), 2);
        assert_eq!(is_provable_prime(&BigUint::from(1_u8)), 0);
        assert_eq!(
            prime_certificate(&BigUint::from(97_u8)),
            crate::prime_certificate(97)
        );
        // Composite above 2^64.
        let c = (one() << 67_usize) - 1_u8;
        assert_eq!(is_provable_prime_with_cert(&c), (0, String::new()));
        assert!(Certificate::prove(&c).is_none());
        // 2^64 + 13 by BLS5.
        let p = smallval() + 14_u8;
        let (status, cert) = is_provable_prime_with_cert(&p);
        assert_eq!(status, 2);
        assert_eq!(verify_prime(&cert), Ok(true));
        // A prime whose n - 1 is 2 * (a hard semiprime): the n - 1 proof fails and ECPP runs.
        let m89 = (one() << 89_usize) - 1_u8;
        let m107 = (one() << 107_usize) - 1_u8;
        let mut n = &m89 * &m107 * 2_u8 + 1_u8;
        while is_prob_prime(&n) == 0 {
            n += &m89 * &m107 * 2_u8;
        }
        let mut budget = 0;
        assert_eq!(ecpp(&n, &small_primes(10_000), &mut budget), Err(1));
        let cert = Certificate::prove(&n).unwrap_or_else(|| panic!("no proof for {n}"));
        assert!(cert.proofs.iter().any(|p| matches!(p, Proof::Ecpp { .. })));
        assert_eq!(cert.verify(), Ok(true));
        assert_eq!(bls75(&n), Err(1));
        // ecpp on a small value and on a small composite.
        assert_eq!(
            ecpp(&BigUint::from(97_u8), &[], &mut budget),
            Ok(vec![Proof::Small {
                n: BigUint::from(97_u8)
            }])
        );
        assert_eq!(ecpp(&BigUint::from(95_u8), &[], &mut budget), Err(0));
        // A candidate with a wrong discriminant gives no block.
        let cand = Candidate {
            d: -5,
            m: n.clone(),
            q: n.clone(),
        };
        assert!(ecpp_block(&n, &cand).is_none());
        // A D = -4 candidate (curves y^2 = x^3 + a x) from a prime that is 1 mod 4.
        let mut p = smallval() + 14_u8;
        let primes = small_primes(10_000);
        let block = loop {
            let found = ecpp_candidates(&p, &primes)
                .into_iter()
                .find(|c| c.d == -4)
                .and_then(|c| ecpp_block(&p, &c));
            if let Some(block) = found {
                break block;
            }
            p = super::super::primality::next_prime(&p);
        };
        assert!(
            matches!(&block, Proof::Ecpp { a, b, .. } if *b == BigInt::from(0) && *a != BigInt::from(0))
        );
        assert!(block.check().is_some());
        // Order primes: a smooth order, a prime cofactor, a split cofactor.
        let primes = small_primes(100);
        assert_eq!(order_prime(&BigUint::from(96_u8), &primes, false), None);
        assert_eq!(
            order_prime(&(&m89 * 6_u8), &primes, false),
            Some(m89.clone())
        );
        assert_eq!(order_prime(&(&m89 * &m107), &primes, false), None);
        assert_eq!(
            order_prime(&(&m89 * 1009_u32 * 1013_u32), &primes, true),
            Some(m89.clone())
        );
    }

    #[test]
    fn aks_edges() {
        assert!(!is_aks_prime(&(smallval() + 2_u8)));
        assert!(!is_aks_prime(&(one() << 70_usize)));
        assert!(!is_aks_prime(&(smallval() * 3_u8)));
        // A composite that passes the trial division and fails Fermat: the
        // Mersenne number 2^67 - 1.
        assert!(!is_aks_prime(&((one() << 67_usize) - 1_u8)));
        // Trial division to s(s - 1) finds the factor 13.
        assert!(!is_aks_prime(&((smallval() + 14_u8) * 13_u8)));
        assert!(bern41_acceptable(45.0, 43, 10_000));
        assert!(!bern41_acceptable(45.0, 43, 1));
        assert!(log_binomial(3, 5) < -1e299);
        assert!((ln_big(&(one() << 100_usize)) - 100.0 * std::f64::consts::LN_2).abs() < 1e-9);
    }

    /// A 65-bit prime through the polynomial step: seconds, not minutes.
    #[test]
    fn aks_proves_a_prime_above_native_range() {
        assert!(is_aks_prime(&(smallval() + 14_u8)));
        assert!(!test_anr(2, &(smallval() + 2_u8), 5));
    }
}

//! MPU native certificate parser and n-1 proof checks.
use crate::{gcd, is_prime, modular::powmod_u};
use std::collections::BTreeMap;

fn value(fields: &BTreeMap<String, u64>, key: &str) -> Result<u64, String> {
    fields
        .get(key)
        .copied()
        .ok_or_else(|| format!("Missing {key}"))
}
fn coprime_power(a: u64, exponent: u64, n: u64) -> bool {
    let power = powmod_u(a, exponent, n);
    gcd(if power == 0 { n - 1 } else { power - 1 }, n) == 1
}
fn check(kind: &str, fields: &BTreeMap<String, u64>) -> Result<bool, String> {
    let n = value(fields, "N")?;
    if n < 2 {
        return Ok(false);
    }
    if kind == "SMALL" {
        return Ok(fields.len() == 1 && is_prime(n) != 0);
    }
    let nm1 = n - 1;
    if kind == "BLS3" || kind == "POCKLINGTON" {
        let q = value(fields, "Q")?;
        let a = value(fields, "A")?;
        if fields.len() != 3 || q < 2 || q >= n || nm1 % q != 0 || is_prime(q) == 0 {
            return Ok(false);
        }
        let m = nm1 / q;
        return Ok(if kind == "BLS3" {
            q > 2
                && q % 2 == 1
                && u128::from(q) * 2 + 1 > u128::from(crate::sqrtint(n))
                && powmod_u(a, nm1 / 2, n) == nm1
                && powmod_u(a, m / 2, n) != nm1
        } else {
            m % 2 == 0 && m < q && a > 1 && powmod_u(a, nm1, n) == 1 && coprime_power(a, m, n)
        });
    }
    let mut factors = if kind == "BLS5" { vec![2] } else { Vec::new() };
    let mut index = 1;
    while let Some(&q) = fields.get(&format!("Q[{index}]")) {
        factors.push(q);
        index += 1;
    }
    let mut remaining = nm1;
    for (i, &q) in factors.iter().enumerate() {
        if q < 2 || q >= nm1 || nm1 % q != 0 || is_prime(q) == 0 {
            return Ok(false);
        }
        let a = if kind == "BLS5" {
            fields.get(&format!("A[{i}]")).copied().unwrap_or(2)
        } else {
            value(fields, "A")?
        };
        if a < 2 || a >= n || powmod_u(a, nm1, n) != 1 {
            return Ok(false);
        }
        if kind == "BLS5" {
            if !coprime_power(a, nm1 / q, n) {
                return Ok(false);
            }
        } else if powmod_u(a, nm1 / q, n) == 1 {
            return Ok(false);
        }
        while remaining % q == 0 {
            remaining /= q;
        }
    }
    for key in fields.keys() {
        if key == "N" || (kind == "BLS15" && fields.len() == 4) || (kind == "LUCAS" && key == "A") {
            continue;
        }
        let valid = key
            .strip_prefix("Q[")
            .and_then(|v| v.strip_suffix(']'))
            .and_then(|v| v.parse::<usize>().ok())
            .is_some_and(|v| v > 0 && v < index)
            || (kind == "BLS5"
                && key
                    .strip_prefix("A[")
                    .and_then(|v| v.strip_suffix(']'))
                    .and_then(|v| v.parse::<usize>().ok())
                    .is_some_and(|v| v < factors.len()));
        if !valid {
            return Err(format!("Invalid field {key}"));
        }
    }
    if kind == "LUCAS" {
        return Ok(!factors.is_empty() && remaining == 1);
    }
    let f = u128::from(nm1 / remaining);
    let r_total = u128::from(remaining);
    if f % 2 != 0 || gcd(nm1 / remaining, remaining) != 1 {
        return Ok(false);
    }
    let s = r_total / (2 * f);
    let r = r_total % (2 * f);
    // Saturation preserves this comparison with a u64 n, even when F^3 exceeds u128.
    let inner = (2 * f)
        .saturating_mul(f)
        .saturating_add(r * f)
        .saturating_sub(f)
        .saturating_add(1);
    let bound = (f + 1).saturating_mul(inner);
    let discriminant = (r * r).checked_sub(8 * s);
    let square = discriminant.is_some_and(|d| {
        // r <= R and F >= 2, so the discriminant fits u64 when s != 0.
        let root = crate::sqrtint(u64::try_from(d).unwrap_or(u64::MAX));
        u128::from(root) * u128::from(root) == d
    });
    Ok(u128::from(n) < bound && (s == 0 || !square))
}

// Jacobi reduction from PrimalityProving.pm::_jacobi, for an odd native modulus.
fn jacobi(mut a: u64, mut n: u64) -> i32 {
    let mut sign = 1;
    while a != 0 {
        let twos = a.trailing_zeros();
        a >>= twos;
        if twos % 2 == 1 && matches!(n % 8, 3 | 5) {
            sign = -sign;
        }
        if a % 4 == 3 && n % 4 == 3 {
            sign = -sign;
        }
        (a, n) = (n % a, a);
    }
    if n == 1 { sign } else { 0 }
}

// Port of PrimalityProving.pm::_verify_bls15. Use residues to avoid D overflow.
fn check_bls15(fields: &BTreeMap<String, i128>) -> Result<bool, String> {
    let get = |key: &str| {
        fields
            .get(key)
            .copied()
            .ok_or_else(|| format!("Missing {key}"))
    };
    let n = u64::try_from(get("N")?).map_err(|_| "Invalid N")?;
    let q = u64::try_from(get("Q")?).map_err(|_| "Invalid Q")?;
    let lp = get("LP")?;
    let lq = get("LQ")?;
    if fields.len() != 4 || n < 3 || n % 2 == 0 || q <= 2 || q % 2 == 0 {
        return Ok(false);
    }
    let np1 = u128::from(n) + 1;
    if np1 % u128::from(q) != 0
        || u128::from(q) * 2 - 1 <= u128::from(crate::sqrtint(n))
        || is_prime(q) == 0
    {
        return Ok(false);
    }
    let modulus = i128::from(n);
    let p = u64::try_from(lp.rem_euclid(modulus)).map_err(|_| "Invalid LP")?;
    let r = u64::try_from(lq.rem_euclid(modulus)).map_err(|_| "Invalid LQ")?;
    let d = (u128::from(p) * u128::from(p) % u128::from(n) + u128::from(n)
        - 4 * u128::from(r) % u128::from(n))
        % u128::from(n);
    if jacobi(u64::try_from(d).map_err(|_| "Invalid discriminant")?, n) != -1 {
        return Ok(false);
    }
    // Center each residue so lucasvmod can use its signed native API.
    let centered = |v: u64| {
        i64::try_from(if v > i64::MAX as u64 {
            i128::from(v) - modulus
        } else {
            i128::from(v)
        })
        .map_err(|_| "Invalid Lucas parameter")
    };
    let p = centered(p)?;
    let r = centered(r)?;
    let half_m = u64::try_from(np1 / u128::from(q) / 2).map_err(|_| "Invalid M")?;
    let half_n = u64::try_from(np1 / 2).map_err(|_| "Invalid N")?;
    Ok(crate::lucasvmod(p, r, half_m, n) != 0 && crate::lucasvmod(p, r, half_n, n) == 0)
}

/// Verify an MPU certificate for a native integer.
///
/// Port of `reference/mpu/lib/Math/Prime/Util/PrimalityProving.pm`:
/// `verify_cert`, `_verify_small`, `_verify_bls5`, `_verify_bls3`,
/// `_verify_bls15`, `_verify_pock`, and `_verify_lucas`.
/// `reference/mpu/aks.c::is_aks_prime` is a separate primality test.
/// ECPP and integers above `u64::MAX` are not supported.
///
/// # Errors
/// Return an error for malformed input or an unsupported proof type.
/// Return `Ok(false)` when the proof fails its mathematical checks.
///
/// ```
/// let cert = math_prime_util::prime_certificate(97);
/// assert_eq!(math_prime_util::verify_prime(&cert), Ok(true));
/// assert_eq!(math_prime_util::verify_prime(""), Ok(false));
/// ```
pub fn verify_prime(certificate: &str) -> Result<bool, String> {
    if certificate.trim().is_empty() {
        return Ok(false);
    }
    let mut lines = certificate
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .peekable();
    if lines.next() != Some("[MPU - Primality Certificate]") {
        return Err("Missing MPU header".into());
    }
    let mut target = None;
    let mut seen_proof = false;
    while let Some(line) = lines.next() {
        if line.starts_with("Version ") || line == "Base 10" || line.starts_with('-') {
            continue;
        }
        if line == "Proof for:" {
            if target.is_some() {
                return Err("Repeated target".into());
            }
            target = Some(
                lines
                    .next()
                    .and_then(|l| {
                        let mut words = l.split_whitespace();
                        let key = words.next()?;
                        let n = words.next()?.parse::<u64>().ok()?;
                        (key.eq_ignore_ascii_case("N") && words.next().is_none()).then_some(n)
                    })
                    .ok_or("Invalid target")?,
            );
            continue;
        }
        let kind = line
            .strip_prefix("Type ")
            .ok_or("Unexpected certificate line")?
            .to_ascii_uppercase();
        if target.is_none() {
            return Err("Proof before target".into());
        }
        if !["SMALL", "BLS5", "BLS3", "BLS15", "POCKLINGTON", "LUCAS"].contains(&kind.as_str()) {
            return Err("Unsupported proof type".into());
        }
        let mut fields = BTreeMap::new();
        let mut factor_index = 0;
        while let Some(&next) = lines.peek() {
            if next.starts_with("Type ") || next.starts_with('-') {
                break;
            }
            let line = lines.next().ok_or("Missing field")?;
            let mut words = line.split_whitespace();
            let key = words.next().ok_or("Missing key")?.to_ascii_uppercase();
            let number = words
                .next()
                .and_then(|w| w.parse::<i128>().ok())
                .ok_or("Invalid integer")?;
            if number < -i128::from(u64::MAX)
                || number > i128::from(u64::MAX)
                || (number < 0 && !(kind == "BLS15" && (key == "LP" || key == "LQ")))
            {
                return Err("Integer outside native range".into());
            }
            if kind == "BLS5" || kind == "LUCAS" {
                if let Some(index) = key.strip_prefix("Q[").and_then(|s| s.strip_suffix(']')) {
                    factor_index += 1;
                    if index.parse::<usize>().ok() != Some(factor_index) {
                        return Err("Invalid Q index".into());
                    }
                }
                if let Some(index) = key.strip_prefix("A[").and_then(|s| s.strip_suffix(']')) {
                    if !index
                        .parse::<usize>()
                        .ok()
                        .is_some_and(|i| i <= factor_index)
                    {
                        return Err("Invalid A index".into());
                    }
                }
            }
            if words.next().is_some() || fields.insert(key.clone(), number).is_some() {
                return Err("Repeated or malformed field".into());
            }
            if kind == "SMALL"
                || ((kind == "BLS3" || kind == "POCKLINGTON") && fields.len() == 3)
                || (kind == "BLS15" && fields.len() == 4)
                || (kind == "LUCAS" && key == "A")
            {
                break;
            }
        }
        let valid = if kind == "BLS15" {
            check_bls15(&fields)?
        } else {
            let fields = fields
                .into_iter()
                .map(|(key, n)| {
                    u64::try_from(n)
                        .map(|n| (key, n))
                        .map_err(|_| "Invalid integer")
                })
                .collect::<Result<BTreeMap<_, _>, _>>()?;
            check(&kind, &fields)?
        };
        if !valid {
            return Ok(false);
        }
        seen_proof = true;
    }
    let n = target.ok_or("Missing target")?;
    Ok(seen_proof && is_prime(n) != 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fields(pairs: &[(&str, u64)]) -> BTreeMap<String, u64> {
        pairs.iter().map(|&(k, v)| (k.to_string(), v)).collect()
    }
    #[test]
    fn theorem_boundaries() {
        assert!(
            check(
                "BLS5",
                &fields(&[
                    ("N", 18_446_744_069_414_584_321),
                    ("Q[1]", 3),
                    ("Q[2]", 5),
                    ("Q[3]", 17),
                    ("Q[4]", 257),
                    ("Q[5]", 65537),
                    ("A[0]", 7),
                    ("A[1]", 7),
                    ("A[2]", 7),
                    ("A[3]", 7),
                    ("A[4]", 7),
                    ("A[5]", 7)
                ])
            )
            .is_ok()
        );
        for kind in ["BLS3", "POCKLINGTON"] {
            assert_eq!(
                check(kind, &fields(&[("N", 23), ("Q", 11), ("A", 5)])),
                Ok(true)
            );
            for pairs in [
                vec![("N", 0)],
                vec![("N", 23), ("Q", 0), ("A", 5)],
                vec![("N", 23), ("Q", 11), ("A", 1)],
                vec![("N", 23), ("Q", 4), ("A", 5)],
            ] {
                assert_eq!(check(kind, &fields(&pairs)), Ok(false));
            }
            assert!(check(kind, &fields(&[("N", 23)])).is_err());
        }
        assert_eq!(check("BLS5", &fields(&[("N", 3)])), Ok(false));
        assert_eq!(check("BLS5", &fields(&[("N", 101)])), Ok(true));
        assert_eq!(
            check(
                "BLS5",
                &fields(&[("N", 31), ("Q[1]", 3), ("A[0]", 3), ("A[1]", 3)])
            ),
            Ok(true)
        );
        assert_eq!(
            check("LUCAS", &fields(&[("N", 101), ("Q[1]", 2), ("A", 2)])),
            Ok(false)
        );
        assert_eq!(
            check("LUCAS", &fields(&[("N", 101), ("Q[1]", 2), ("A", 4)])),
            Ok(false)
        );
        assert!(coprime_power(0, 2, 7));
    }
    #[test]
    fn parser_boundaries() {
        let header = "[MPU - Primality Certificate]\n";
        for suffix in [
            "Proof for:\nN 101\nProof for:\nN 101",
            "Proof for:\nN x",
            "Type Small\nN 2",
            "Proof for:\nN 101\nType BLS5\nN 101\nQ[1] x",
            "Base 16",
            "Version 1.0",
            "Proof for:\nN 101\nType BLS5\nN 101 extra",
        ] {
            assert!(
                verify_prime(&format!("{header}{suffix}")).is_err(),
                "{suffix}"
            );
        }
        assert_eq!(
            verify_prime(&format!(
                "{header}# comment\nBase 10\nProof for:\nN 23\nType BLS3\nN 23\nQ 11\nA 5\n"
            )),
            Ok(true)
        );
        assert_eq!(
            verify_prime(&format!(
                "{header}Proof for:\nN 23\nType POCKLINGTON\nN 23\nQ 11\nA 5\n"
            )),
            Ok(true)
        );
    }
}

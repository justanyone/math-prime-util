//! MPU native certificate parser and n-1 proof checks.
use std::collections::BTreeMap;
use crate::{gcd, is_prime, modular::powmod_u};

fn value(fields: &BTreeMap<String, u64>, key: &str) -> Result<u64, String> {
    fields.get(key).copied().ok_or_else(|| format!("Missing {key}"))
}
fn coprime_power(a: u64, exponent: u64, n: u64) -> bool {
    let power = powmod_u(a, exponent, n);
    gcd(if power == 0 { n-1 } else { power-1 }, n) == 1
}
fn check(kind: &str, fields: &BTreeMap<String, u64>) -> Result<bool, String> {
    let n = value(fields, "N")?;
    if n < 2 { return Ok(false); }
    if kind == "SMALL" { return Ok(fields.len() == 1 && is_prime(n) != 0); }
    let nm1 = n-1;
    if kind == "BLS3" || kind == "POCKLINGTON" {
        let q = value(fields, "Q")?;
        let a = value(fields, "A")?;
        if fields.len() != 3 || q < 2 || q >= n || nm1 % q != 0 || is_prime(q) == 0 { return Ok(false); }
        let m = nm1 / q;
        return Ok(if kind == "BLS3" {
            q > 2 && q % 2 == 1 && u128::from(q)*2+1 > u128::from(crate::sqrtint(n))
                && powmod_u(a, nm1/2, n) == nm1 && powmod_u(a, m/2, n) != nm1
        } else {
            m % 2 == 0 && m < q && a > 1 && powmod_u(a, nm1, n) == 1 && coprime_power(a, m, n)
        });
    }
    let mut factors = if kind == "BLS5" { vec![2] } else { Vec::new() };
    let mut index = 1;
    while let Some(&q) = fields.get(&format!("Q[{index}]")) { factors.push(q); index += 1; }
    let mut remaining = nm1;
    for (i, &q) in factors.iter().enumerate() {
        if q < 2 || q >= nm1 || nm1 % q != 0 || is_prime(q) == 0 { return Ok(false); }
        let a = if kind == "BLS5" { fields.get(&format!("A[{i}]")).copied().unwrap_or(2) } else { value(fields, "A")? };
        if a < 2 || a >= n || powmod_u(a, nm1, n) != 1 { return Ok(false); }
        if kind == "BLS5" {
            if !coprime_power(a, nm1/q, n) { return Ok(false); }
        } else if powmod_u(a, nm1/q, n) == 1 { return Ok(false); }
        while remaining % q == 0 { remaining /= q; }
    }
    for key in fields.keys() {
        if key == "N" || (kind == "LUCAS" && key == "A") { continue; }
        let valid = key.strip_prefix("Q[").and_then(|v| v.strip_suffix(']')).and_then(|v| v.parse::<usize>().ok()).is_some_and(|v| v > 0 && v < index)
            || (kind == "BLS5" && key.strip_prefix("A[").and_then(|v| v.strip_suffix(']')).and_then(|v| v.parse::<usize>().ok()).is_some_and(|v| v < factors.len()));
        if !valid { return Err(format!("Invalid field {key}")); }
    }
    if kind == "LUCAS" { return Ok(!factors.is_empty() && remaining == 1); }
    let f = u128::from(nm1/remaining);
    let r_total = u128::from(remaining);
    if f % 2 != 0 || gcd(nm1/remaining, remaining) != 1 { return Ok(false); }
    let s = r_total / (2*f);
    let r = r_total % (2*f);
    // Saturation preserves this comparison with a u64 n, even when F^3 exceeds u128.
    let inner = (2*f*f).saturating_add(r*f).saturating_sub(f).saturating_add(1);
    let bound = (f+1).saturating_mul(inner);
    let discriminant = (r*r).checked_sub(8*s);
    let square = discriminant.is_some_and(|d| {
        // r <= R and F >= 2, so the discriminant fits u64 when s != 0.
        let root = crate::sqrtint(u64::try_from(d).unwrap_or(u64::MAX));
        u128::from(root)*u128::from(root) == d
    });
    Ok(u128::from(n) < bound && (s == 0 || !square))
}

/// Verify an MPU certificate for a native integer.
///
/// Port of `PrimalityProving.pm::verify_cert`, `_verify_small`, `_verify_bls5`,
/// `_verify_bls3`, `_verify_pock`, and `_verify_lucas`.
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
    if certificate.trim().is_empty() { return Ok(false); }
    let mut lines = certificate.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')).peekable();
    if lines.next() != Some("[MPU - Primality Certificate]") { return Err("Missing MPU header".into()); }
    let mut target = None;
    let mut seen_proof = false;
    while let Some(line) = lines.next() {
        if line.starts_with("Version ") || line == "Base 10" || line.starts_with('-') { continue; }
        if line == "Proof for:" {
            if target.is_some() { return Err("Repeated target".into()); }
            target = Some(lines.next().and_then(|l| l.strip_prefix("N ")).and_then(|n| n.trim().parse::<u64>().ok()).ok_or("Invalid target")?);
            continue;
        }
        let kind = line.strip_prefix("Type ").ok_or("Unexpected certificate line")?.to_ascii_uppercase();
        if target.is_none() { return Err("Proof before target".into()); }
        if !["SMALL", "BLS5", "BLS3", "POCKLINGTON", "LUCAS"].contains(&kind.as_str()) { return Err("Unsupported proof type".into()); }
        let mut fields = BTreeMap::new();
        while let Some(&next) = lines.peek() {
            if next.starts_with("Type ") || next.starts_with('-') { break; }
            let line = lines.next().ok_or("Missing field")?;
            let mut words = line.split_whitespace();
            let key = words.next().ok_or("Missing key")?;
            let number = words.next().and_then(|w| w.parse::<u64>().ok()).ok_or("Invalid integer")?;
            if words.next().is_some() || fields.insert(key.to_string(), number).is_some() { return Err("Repeated or malformed field".into()); }
            if kind == "SMALL" || ((kind == "BLS3" || kind == "POCKLINGTON") && fields.len() == 3) || (kind == "LUCAS" && key == "A") { break; }
        }
        if !check(&kind, &fields)? { return Ok(false); }
        seen_proof = true;
    }
    let n = target.ok_or("Missing target")?;
    Ok(seen_proof && is_prime(n) != 0)
}

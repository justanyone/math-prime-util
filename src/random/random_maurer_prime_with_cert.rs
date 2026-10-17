use super::{urandomb, urandomm};

// MPU random_prime.c: random_nbit_prime. Keep its small-prime tables and draws.
pub(super) fn nbit_prime(bits: u32) -> u64 {
    match bits {
        2 => return if urandomb(1) == 0 { 3 } else { 2 },
        3 => return if urandomb(1) == 0 { 7 } else { 5 },
        4 => return if urandomb(1) == 0 { 13 } else { 11 },
        5..=9 => {
            let (start, count) = [(7, 5), (12, 7), (19, 13), (32, 23), (55, 43)]
                [usize::try_from(bits - 5).unwrap()];
            let index = start + urandomm(count);
            let mut found = 0;
            for n in 2.. {
                if crate::is_prime(n) != 0 {
                    found += 1;
                    if found == index {
                        return n;
                    }
                }
            }
            unreachable!()
        }
        _ => {}
    }
    let base = (1_u64 << (bits - 1)) + 1;
    loop {
        let n = base + (urandomb(bits - 2) << 1);
        if crate::is_prob_prime(n) != 0 {
            return n;
        }
    }
}

pub(super) fn small_certificate(n: u64) -> String {
    format!("[MPU - Primality Certificate]\nVersion 1.0\n\nProof for:\nN {n}\n\nType Small\nN {n}\n")
}

/// Returns a native Maurer prime and its MPU certificate.
/// Ports `lib/Math/Prime/Util/RandomPrimes.pm`: `random_maurer_prime_with_cert`,
/// with `random_prime.c`: `random_nbit_prime` for the native base case.
/// Returns `None` above 64 bits until bigint support is available.
/// # Panics
/// Panics if `bits < 2`, or if an unseeded thread cannot read OS entropy.
/// ```
/// math_prime_util::srand(15);
/// let (p, cert) = math_prime_util::random_maurer_prime_with_cert(16).unwrap();
/// assert_eq!(math_prime_util::is_prime(p), 2);
/// assert!(cert.contains(&format!("N {p}\n")));
/// ```
pub fn random_maurer_prime_with_cert(bits: u32) -> Option<(u64, String)> {
    assert!(bits >= 2, "random_maurer_prime, bits must be >= 2");
    if bits > 64 {
        return None;
    }
    let n = nbit_prime(bits);
    Some((n, small_certificate(n)))
}

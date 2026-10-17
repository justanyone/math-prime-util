use super::random_maurer_prime_with_cert::small_certificate;

const K: [u32; 64] = [
    0x428a_2f98,
    0x7137_4491,
    0xb5c0_fbcf,
    0xe9b5_dba5,
    0x3956_c25b,
    0x59f1_11f1,
    0x923f_82a4,
    0xab1c_5ed5,
    0xd807_aa98,
    0x1283_5b01,
    0x2431_85be,
    0x550c_7dc3,
    0x72be_5d74,
    0x80de_b1fe,
    0x9bdc_06a7,
    0xc19b_f174,
    0xe49b_69c1,
    0xefbe_4786,
    0x0fc1_9dc6,
    0x240c_a1cc,
    0x2de9_2c6f,
    0x4a74_84aa,
    0x5cb0_a9dc,
    0x76f9_88da,
    0x983e_5152,
    0xa831_c66d,
    0xb003_27c8,
    0xbf59_7fc7,
    0xc6e0_0bf3,
    0xd5a7_9147,
    0x06ca_6351,
    0x1429_2967,
    0x27b7_0a85,
    0x2e1b_2138,
    0x4d2c_6dfc,
    0x5338_0d13,
    0x650a_7354,
    0x766a_0abb,
    0x81c2_c92e,
    0x9272_2c85,
    0xa2bf_e8a1,
    0xa81a_664b,
    0xc24b_8b70,
    0xc76c_51a3,
    0xd192_e819,
    0xd699_0624,
    0xf40e_3585,
    0x106a_a070,
    0x19a4_c116,
    0x1e37_6c08,
    0x2748_774c,
    0x34b0_bcb5,
    0x391c_0cb3,
    0x4ed8_aa4a,
    0x5b9c_ca4f,
    0x682e_6ff3,
    0x748f_82ee,
    0x78a5_636f,
    0x84c8_7814,
    0x8cc7_0208,
    0x90be_fffa,
    0xa450_6ceb,
    0xbef9_a3f7,
    0xc671_78f2,
];

// FIPS 180-4 SHA-256, used by MPU's Digest::SHA::sha256 calls.
fn sha256(input: &[u8]) -> [u8; 32] {
    let mut data = input.to_vec();
    data.push(0x80);
    while data.len() % 64 != 56 {
        data.push(0);
    }
    data.extend_from_slice(&(u64::try_from(input.len()).unwrap() * 8).to_be_bytes());
    let mut h = [
        0x6a09_e667_u32,
        0xbb67_ae85,
        0x3c6e_f372,
        0xa54f_f53a,
        0x510e_527f,
        0x9b05_688c,
        0x1f83_d9ab,
        0x5be0_cd19,
    ];
    for block in data.chunks_exact(64) {
        let mut w = [0_u32; 64];
        for (word, bytes) in w.iter_mut().zip(block.chunks_exact(4)) {
            *word = u32::from_be_bytes(bytes.try_into().unwrap());
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut v] = h;
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ (!e & g);
            let t1 = v
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let t2 = s0.wrapping_add((a & b) ^ (a & c) ^ (b & c));
            v = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }
        for (word, value) in h.iter_mut().zip([a, b, c, d, e, f, g, v]) {
            *word = word.wrapping_add(value);
        }
    }
    let mut result = [0; 32];
    for (bytes, word) in result.chunks_exact_mut(4).zip(h) {
        bytes.copy_from_slice(&word.to_be_bytes());
    }
    result
}

// RandomPrimes.pm: _seed_plus_one (big-endian seed).
fn increment(seed: &mut [u8; 64]) {
    for byte in seed.iter_mut().rev() {
        *byte = byte.wrapping_add(1);
        if *byte != 0 {
            break;
        }
    }
}

fn hash_next(seed: &mut [u8; 64]) -> [u8; 32] {
    let hash = sha256(seed);
    increment(seed);
    hash
}

fn residue(bytes: &[u8], modulus: u64) -> u64 {
    let mut r = 0_u128;
    for &byte in bytes {
        r = (r * 256 + u128::from(byte)) % u128::from(modulus);
    }
    u64::try_from(r).unwrap()
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

// RandomPrimes.pm: _ST_Random_prime. SHA-256 needs one iteration for u64.
fn generate(
    bits: u32,
    seed: &mut [u8; 64],
    limit: u32,
) -> Result<(u64, u32, String), &'static str> {
    let limit = limit.min(10001 + 16 * bits);
    let top = 1_u64 << (bits - 1);
    if bits <= 32 {
        for counter in 1..=limit {
            let first = hash_next(seed);
            let second = hash_next(seed);
            let low = u32::from_be_bytes(std::array::from_fn(|i| first[28 + i] ^ second[28 + i]));
            let n = top | (u64::from(low) & (top - 1)) | 1;
            if crate::is_prime(n) != 0 {
                return Ok((n, counter, small_certificate(n)));
            }
        }
        return Err("Shawe-Taylor random prime failure");
    }
    let (q, old_counter, _) = generate(bits.div_ceil(2) + 1, seed, limit)?;
    let twice_q = 2 * q;
    let x = top + residue(&hash_next(seed), top);
    let mut t = u128::from(x).div_ceil(u128::from(twice_q));
    for counter in 1..=limit {
        let mut candidate = t * u128::from(twice_q) + 1;
        if candidate > 2 * u128::from(top) {
            t = u128::from(top).div_ceil(u128::from(twice_q));
            candidate = t * u128::from(twice_q) + 1;
        }
        let n = u64::try_from(candidate).unwrap();
        // Match MPU's non-GMP filter. Advance the seed even for rejected n.
        if gcd(n, 111_546_435) == 1 && crate::primality::is_strong_pseudoprime(n, 3) {
            let a = residue(&hash_next(seed), n - 3) + 2;
            let z = crate::powmod(a, u64::try_from(2 * t).unwrap(), n).unwrap();
            if gcd(z.wrapping_sub(1), n) == 1 && crate::powmod(z, q, n) == Some(1) {
                let cert = format!(
                    "[MPU - Primality Certificate]\nVersion 1.0\n\nProof for:\nN {n}\n\nType Pocklington\nN {n}\nQ {q}\nA {a}\n"
                );
                return Ok((n, old_counter + counter, cert));
            }
        } else {
            increment(seed);
        }
        t += 1;
    }
    Err("Shawe-Taylor random prime failure")
}

/// Returns a Shawe–Taylor prime and an MPU certificate.
/// Ports `lib/Math/Prime/Util/RandomPrimes.pm`:
/// `random_shawe_taylor_prime_with_cert` and `_ST_Random_prime` (FIPS 186-4).
/// Returns `None` above 64 bits until bigint support is available.
/// # Errors
/// Returns an error if the reference search limit is reached.
/// # Panics
/// Panics if `bits < 2`, or if an unseeded thread cannot read OS entropy.
/// ```
/// math_prime_util::srand(15);
/// let (p, cert) = math_prime_util::random_shawe_taylor_prime_with_cert(40)
///     .unwrap().unwrap();
/// assert_eq!(math_prime_util::is_prime(p), 2);
/// assert!(cert.contains("Type Pocklington"));
/// ```
pub fn random_shawe_taylor_prime_with_cert(
    bits: u32,
) -> Result<Option<(u64, String)>, &'static str> {
    assert!(bits >= 2, "Shawe-Taylor random prime must have length >= 2");
    if bits > 64 {
        return Ok(None);
    }
    let mut seed = super::random_bytes(64).try_into().unwrap();
    generate(bits, &mut seed, 10001 + 16 * bits).map(|(p, _, cert)| Some((p, cert)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha_vectors_and_seed_carry() {
        let hex = |bytes: [u8; 32]| bytes.iter().map(|b| format!("{b:02x}")).collect::<String>();
        assert_eq!(
            hex(sha256(b"abc")),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        assert_eq!(
            hex(sha256(&[b'a'; 64])),
            "ffe054fe7ae0cb6dc65c3af9b61d5209f439851db43d0ba5997337df154668eb"
        );
        let mut seed = [255; 64];
        increment(&mut seed);
        assert_eq!(seed, [0; 64]);
        increment(&mut seed);
        assert_eq!(seed[63], 1);
        assert!(generate(16, &mut seed, 0).is_err());
        assert!(generate(64, &mut seed, 0).is_err());
    }
}

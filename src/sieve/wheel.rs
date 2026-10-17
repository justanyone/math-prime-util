//! Mod-30 wheel. Tables and steps follow `sieve.h` (`wheel30`, `masktab30`,
//! `wheeladvance30`, `wheelretreat30`, `nextwheel30`, `prevwheel30`).

/// `constants.h` `_MPU_FILL_EXTRA_N`.
pub(crate) const FILL_EXTRA_N: u64 = 128 * 30;

/// `constants.h` `MPU_MAX_PRIME` on a 64-bit build (`UINT64_MAX - 58`).
pub(crate) const MPU_MAX_PRIME: u64 = u64::MAX - 58;

const WHEEL_RES: [u8; 8] = [1, 7, 11, 13, 17, 19, 23, 29];

const NEXT_WHEEL: [u8; 30] = [
    1, 7, 7, 7, 7, 7, 7, 11, 11, 11, 11, 13, 13, 17, 17, 17, 17, 19, 19, 23, 23, 23, 23, 29, 29,
    29, 29, 29, 29, 1,
];

const PREV_WHEEL: [u8; 30] = [
    29, 29, 1, 1, 1, 1, 1, 1, 7, 7, 7, 7, 11, 11, 13, 13, 13, 13, 17, 17, 19, 19, 19, 19, 23, 23,
    23, 23, 23, 23,
];

const WHEEL_ADVANCE: [u8; 30] = [
    1, 6, 5, 4, 3, 2, 1, 4, 3, 2, 1, 2, 1, 4, 3, 2, 1, 2, 1, 4, 3, 2, 1, 6, 5, 4, 3, 2, 1, 2,
];

const WHEEL_RETREAT: [u8; 30] = [
    1, 2, 1, 2, 3, 4, 5, 6, 1, 2, 3, 4, 1, 2, 1, 2, 3, 4, 1, 2, 1, 2, 3, 4, 1, 2, 3, 4, 5, 6,
];

/// Round `n` up the way `cache.c` `_erase_and_fill_prime_cache` does.
pub(crate) fn pad_limit(n: u64) -> u64 {
    if n >= u64::MAX - FILL_EXTRA_N {
        return (u64::MAX / 30) * 30;
    }
    ((n + FILL_EXTRA_N) / 30) * 30
}

fn mask_of(residue: u64) -> u8 {
    match residue {
        1 => 1,
        7 => 2,
        11 => 4,
        13 => 8,
        17 => 16,
        19 => 32,
        23 => 64,
        29 => 128,
        _ => 0,
    }
}

fn clear_prev(residue: usize) -> u8 {
    let mut mask = 0u8;
    for (i, &r) in WHEEL_RES.iter().enumerate() {
        if usize::from(r) < residue {
            mask |= 1 << i;
        }
    }
    mask
}

fn first_zero(bits: u8) -> usize {
    for i in 0..8 {
        if bits & (1 << i) == 0 {
            return i;
        }
    }
    8
}

/// Sieve of the numbers strictly below `limit` (`sieve.c` `sieve_erat30`).
/// `limit` is a multiple of 30. A clear bit is prime.
pub(crate) fn build_sieve(limit: u64) -> Vec<u8> {
    if limit < 30 {
        return Vec::new();
    }
    let nbytes = usize::try_from(limit / 30).unwrap_or(0);
    let mut s = vec![0u8; nbytes];
    if nbytes > 0 {
        s[0] |= 1; // 1 is not prime (`sieve.h` bit 0).
    }
    let root = crate::int::sqrtint(limit - 1);
    for d in 0..nbytes {
        for b in 0..8 {
            if s[d] & (1 << b) != 0 {
                continue;
            }
            let p = d as u64 * 30 + u64::from(WHEEL_RES[b]);
            if p < 7 {
                continue;
            }
            if p > root {
                return s;
            }
            let mut m = p * p;
            while m < limit {
                let bit = mask_of(m % 30);
                if bit != 0 {
                    let md = (m / 30) as usize;
                    s[md] |= bit;
                }
                if limit - m < p {
                    break;
                }
                m += p;
            }
        }
    }
    s
}

/// `sieve.h` `next_prime_in_sieve`. `None` when the next prime is not below `lastp`.
pub(crate) fn next_in_sieve(sieve: &[u8], p: u64, lastp: u64) -> Option<u64> {
    if p < 7 {
        let n = if p < 2 {
            2
        } else if p < 3 {
            3
        } else if p < 5 {
            5
        } else {
            7
        };
        return (n < lastp).then_some(n);
    }
    let p1 = p + 1;
    if p1 >= lastp {
        return None;
    }
    let mut d = usize::try_from(p1 / 30).unwrap_or(usize::MAX);
    let residue = (p1 % 30) as usize;
    if d >= sieve.len() {
        return None;
    }
    let mut bits = sieve[d] | clear_prev(residue);
    loop {
        if bits != 0xFF {
            let z = first_zero(bits);
            if z >= 8 {
                return None;
            }
            let prime = d as u64 * 30 + u64::from(WHEEL_RES[z]);
            if prime >= lastp || prime <= p {
                return None;
            }
            return Some(prime);
        }
        d += 1;
        if d >= sieve.len() || d as u64 * 30 >= lastp {
            return None;
        }
        bits = sieve[d];
    }
}

/// `sieve.h` `prev_prime_in_sieve`.
pub(crate) fn prev_in_sieve(sieve: &[u8], p: u64) -> Option<u64> {
    if p <= 7 {
        return if p <= 2 {
            None
        } else if p <= 3 {
            Some(2)
        } else if p <= 5 {
            Some(3)
        } else {
            Some(5)
        };
    }
    let mut d = usize::try_from(p / 30).unwrap_or(usize::MAX);
    let mut m = (p % 30) as usize;
    if d >= sieve.len() {
        return None;
    }
    loop {
        let prev_m = usize::from(PREV_WHEEL[m]);
        if prev_m == 29 {
            if d == 0 {
                return None;
            }
            d -= 1;
        }
        m = prev_m;
        if sieve[d] & mask_of(u64::try_from(m).unwrap_or(0)) == 0 {
            return Some(d as u64 * 30 + u64::try_from(m).unwrap_or(0));
        }
    }
}

/// Strictly next prime, walking `wheeladvance30` (`util.c` `next_prime`).
pub(crate) fn wheel_next(mut n: u64) -> Option<u64> {
    if n >= MPU_MAX_PRIME {
        return None;
    }
    if n < 2 {
        return Some(2);
    }
    if n == 2 {
        return Some(3);
    }
    if n < 5 {
        return Some(5);
    }
    if n < 7 {
        return Some(7);
    }
    let mut m = (n % 30) as usize;
    loop {
        let step = u64::from(WHEEL_ADVANCE[m]);
        n = n.checked_add(step)?;
        if n > MPU_MAX_PRIME {
            return None;
        }
        m = usize::from(NEXT_WHEEL[m]);
        if is_prime(n) {
            return Some(n);
        }
    }
}

/// Strictly previous prime, walking `wheelretreat30` (`util.c` `prev_prime`).
pub(crate) fn wheel_prev(mut n: u64) -> Option<u64> {
    if n <= 2 {
        return None;
    }
    if n <= 3 {
        return Some(2);
    }
    if n <= 5 {
        return Some(3);
    }
    if n <= 7 {
        return Some(5);
    }
    let mut m = (n % 30) as usize;
    loop {
        let step = u64::from(WHEEL_RETREAT[m]);
        n -= step;
        m = usize::from(PREV_WHEEL[m]);
        if is_prime(n) {
            return Some(n);
        }
        if n < 7 {
            return wheel_prev_small(n);
        }
    }
}

fn wheel_prev_small(n: u64) -> Option<u64> {
    if n <= 2 {
        None
    } else if n <= 3 {
        Some(2)
    } else if n <= 5 {
        Some(3)
    } else {
        Some(5)
    }
}

/// Deterministic Miller-Rabin for every `u64`.
/// Bases proved for `n < 2^64` (Sinclair). `util.c` calls `is_prob_prime`;
/// on this width the two agree.
pub(crate) fn is_prime(n: u64) -> bool {
    if n <= 3 {
        return n >= 2;
    }
    if n % 2 == 0 || n % 3 == 0 || n % 5 == 0 || n % 7 == 0 {
        return false;
    }
    if n < 121 {
        return true;
    }
    miller_rabin(n)
}

const MR_BASES: [u64; 7] = [2, 325, 9375, 28_178, 450_775, 9_780_504, 1_795_265_022];

fn miller_rabin(n: u64) -> bool {
    let mut d = n - 1;
    let mut s = 0u32;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    for &base in &MR_BASES {
        let a = base % n;
        if a == 0 {
            continue;
        }
        let mut x = modpow(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        let mut witness = true;
        for _ in 1..s {
            x = modmul(x, x, n);
            if x == n - 1 {
                witness = false;
                break;
            }
        }
        if witness {
            return false;
        }
    }
    true
}

fn modmul(a: u64, b: u64, n: u64) -> u64 {
    u64::try_from((u128::from(a) * u128::from(b)) % u128::from(n)).unwrap_or(0)
}

fn modpow(mut a: u64, mut e: u64, n: u64) -> u64 {
    let mut r = 1u64;
    while e > 0 {
        if e & 1 == 1 {
            r = modmul(r, a, n);
        }
        a = modmul(a, a, n);
        e >>= 1;
    }
    r
}

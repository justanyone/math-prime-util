//! MPU's ChaCha20 random stream with one state per thread.
//!
//! Seeded calls match MPU 0.75 on a 64-bit host. Seeds are not shared between
//! threads. The first draw on a thread reads OS entropy. Reseed after a fork.

use std::cell::RefCell;
use std::io::{self, Read};

struct Stream {
    state: [u32; 16],
    buffer: [u8; 1024],
    position: usize,
}

fn pcg(state: &mut u32, increment: u32) -> u32 {
    let old = *state;
    *state = old.wrapping_mul(747_796_405).wrapping_add(increment);
    let word = ((old >> ((old >> 28) + 4)) ^ old).wrapping_mul(277_803_737);
    (word >> 22) ^ word
}

fn quarter(x: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize) {
    x[a] = x[a].wrapping_add(x[b]);
    x[d] = (x[d] ^ x[a]).rotate_left(16);
    x[c] = x[c].wrapping_add(x[d]);
    x[b] = (x[b] ^ x[c]).rotate_left(12);
    x[a] = x[a].wrapping_add(x[b]);
    x[d] = (x[d] ^ x[a]).rotate_left(8);
    x[c] = x[c].wrapping_add(x[d]);
    x[b] = (x[b] ^ x[c]).rotate_left(7);
}

impl Stream {
    // reference/mpu/csprng.c: csprng_seed and prng_new.
    fn seeded(data: &[u8]) -> Self {
        let mut seed = [0u8; 40];
        let len = data.len().min(40);
        seed[..len].copy_from_slice(&data[..len]);
        if len < 40 {
            let words: Vec<u32> = seed[..16]
                .chunks_exact(4)
                .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
                .collect();
            let mut state = 0;
            let increment = (words[1] << 1) | 1;
            pcg(&mut state, increment);
            state = state.wrapping_add(words[0]);
            pcg(&mut state, increment);
            state ^= words[2];
            pcg(&mut state, increment);
            state ^= words[3];
            pcg(&mut state, increment);
            for chunk in seed[4 * len.div_ceil(4)..].chunks_exact_mut(4) {
                chunk.copy_from_slice(&pcg(&mut state, increment).to_le_bytes());
            }
        }
        let mut state = [0u32; 16];
        state[..4].copy_from_slice(&[0x6170_7865, 0x3320_646e, 0x7962_2d32, 0x6b20_6574]);
        for (word, chunk) in state[4..12].iter_mut().zip(seed[..32].chunks_exact(4)) {
            *word = u32::from_le_bytes(chunk.try_into().unwrap());
        }
        state[14] = u32::from_le_bytes(seed[32..36].try_into().unwrap());
        state[15] = u32::from_le_bytes(seed[36..40].try_into().unwrap());
        Self {
            state,
            buffer: [0; 1024],
            position: 1024,
        }
    }

    // reference/mpu/chacha.c: chacha_core and _refill_buffer.
    fn refill(&mut self) {
        for block in self.buffer.chunks_exact_mut(64) {
            let mut x = self.state;
            for _ in 0..10 {
                for (a, b, c, d) in [
                    (0, 4, 8, 12),
                    (1, 5, 9, 13),
                    (2, 6, 10, 14),
                    (3, 7, 11, 15),
                    (0, 5, 10, 15),
                    (1, 6, 11, 12),
                    (2, 7, 8, 13),
                    (3, 4, 9, 14),
                ] {
                    quarter(&mut x, a, b, c, d);
                }
            }
            for ((out, word), original) in block.chunks_exact_mut(4).zip(x).zip(self.state) {
                out.copy_from_slice(&word.wrapping_add(original).to_le_bytes());
            }
            self.state[12] = self.state[12].wrapping_add(1);
            if self.state[12] == 0 {
                self.state[13] = self.state[13].wrapping_add(1);
            }
        }
        self.position = 0;
    }

    fn fill(&mut self, mut output: &mut [u8]) {
        while !output.is_empty() {
            if self.position == 1024 {
                self.refill();
            }
            let n = output.len().min(1024 - self.position);
            output[..n].copy_from_slice(&self.buffer[self.position..self.position + n]);
            self.buffer[self.position..self.position + n].fill(0);
            self.position += n;
            output = &mut output[n..];
        }
    }

    fn word32(&mut self) -> u32 {
        let mut bytes = [0; 4];
        self.fill(&mut bytes);
        u32::from_le_bytes(bytes)
    }

    fn word64(&mut self) -> u64 {
        (u64::from(self.word32()) << 32) | u64::from(self.word32())
    }

    fn bounded(&mut self, n: u64) -> u64 {
        if n <= 1 {
            return 0;
        }
        if let Ok(n) = u32::try_from(n) {
            let minimum = n.wrapping_neg() % n;
            loop {
                let r = self.word32();
                if r >= minimum {
                    return u64::from(r % n);
                }
            }
        }
        if n == 1 << 32 {
            return u64::from(self.word32());
        }
        let minimum = n.wrapping_neg() % n;
        loop {
            let r = self.word64();
            if r >= minimum {
                return r % n;
            }
        }
    }
}

thread_local! { static STREAM: RefCell<Option<Stream>> = const { RefCell::new(None) }; }

fn with_stream<T>(f: impl FnOnce(&mut Stream) -> T) -> T {
    STREAM.with(|cell| {
        let mut slot = cell.borrow_mut();
        let stream = slot.get_or_insert_with(|| {
            Stream::seeded(&entropy_bytes(40).expect("OS entropy is unavailable"))
        });
        f(stream)
    })
}

/// Seeds this thread's stream with bytes. MPU `csprng.c`: `csprng_seed`.
/// Seeds longer than 40 bytes use only the first 40 bytes.
/// ```
/// math_prime_util::csrand(b"repeatable seed");
/// ```
pub fn csrand(seed: &[u8]) {
    STREAM.with(|cell| *cell.borrow_mut() = Some(Stream::seeded(seed)));
}

/// Seeds this thread with an integer. MPU `csprng.c`: `csprng_srand`.
/// This seed has at most 64 bits of entropy.
/// ```
/// math_prime_util::srand(42);
/// assert_eq!(math_prime_util::irand(), 445_265_827);
/// ```
pub fn srand(seed: u64) {
    let bytes = seed.to_le_bytes();
    csrand(&bytes[..if u32::try_from(seed).is_ok() { 4 } else { 8 }]);
}

/// Returns a 32-bit integer. MPU `csprng.c`: `irand32`.
/// # Panics
/// Panics if the first draw cannot read OS entropy. Call [`csrand`] to seed.
/// ```
/// math_prime_util::srand(15);
/// assert_eq!(math_prime_util::irand32(), 140_602_708);
/// ```
pub fn irand32() -> u32 {
    with_stream(Stream::word32)
}

/// Returns MPU's native `irand` (32 random bits). MPU `csprng.c`: `irand32`.
/// # Panics
/// Panics if an unseeded thread cannot read OS entropy.
/// ```
/// math_prime_util::srand(15);
/// assert_eq!(math_prime_util::irand(), 140_602_708);
/// ```
pub fn irand() -> u64 {
    u64::from(irand32())
}

/// Returns 64 random bits. MPU `chacha.c`: `chacha_irand64`.
/// # Panics
/// Panics if an unseeded thread cannot read OS entropy.
/// ```
/// math_prime_util::srand(15);
/// assert_eq!(math_prime_util::irand64(), 603_884_034_911_023_882);
/// ```
pub fn irand64() -> u64 {
    with_stream(Stream::word64)
}

/// Returns a random float scaled by `limit`. Use 1.0 for the unit interval.
/// MPU `csprng.c`: `drand64`; `XS.xs`: `drand`.
/// # Panics
/// Panics if an unseeded thread cannot read OS entropy.
/// ```
/// math_prime_util::srand(15);
/// assert!((0.0..1.0).contains(&math_prime_util::drand(1.0)));
/// ```
#[allow(clippy::cast_precision_loss)] // Match C's NV conversion and rounding.
pub fn drand(limit: f64) -> f64 {
    with_stream(|stream| {
        loop {
            let r = stream.word64() as f64 * (1.0 / 18_446_744_073_709_551_616.0);
            if r < 1.0 {
                return r * if limit == 0.0 { 1.0 } else { limit };
            }
        }
    })
}

/// Returns a scaled random float. MPU `XS.xs`: `rand` (alias of `drand`).
/// # Panics
/// Panics if an unseeded thread cannot read OS entropy.
/// ```
/// math_prime_util::srand(42);
/// assert!(math_prime_util::rand(10.0) < 10.0);
/// ```
pub fn rand(limit: f64) -> f64 {
    drand(limit)
}

/// Returns stream bytes. MPU `csprng.c`: `csprng_rand_bytes`.
/// # Panics
/// Panics if `len` exceeds MPU's limit of 2,147,483,646 bytes.
/// Panics if an unseeded thread cannot read OS entropy.
/// ```
/// math_prime_util::srand(15);
/// assert_eq!(math_prime_util::random_bytes(4), [0x54, 0x6d, 0x61, 0x08]);
/// ```
pub fn random_bytes(len: usize) -> Vec<u8> {
    assert!(len <= 2_147_483_646, "byte count exceeds MPU limit");
    let mut bytes = vec![0; len];
    if len > 0 {
        with_stream(|stream| stream.fill(&mut bytes));
    }
    bytes
}

/// Reads OS entropy. MPU `entropy.c`: `get_entropy_bytes` (device path).
/// This uses `/dev/urandom`, then `/dev/random`. It does not use timer entropy.
/// # Errors
/// Returns an I/O error if neither device can supply the requested bytes.
/// Platforms without these devices return an error. No seed state is changed.
/// # Panics
/// Panics if `len` exceeds MPU's limit of 2,147,483,646 bytes.
/// ```
/// # fn main() -> std::io::Result<()> {
/// assert_eq!(math_prime_util::entropy_bytes(0)?, Vec::<u8>::new());
/// # Ok(()) }
/// ```
pub fn entropy_bytes(len: usize) -> io::Result<Vec<u8>> {
    assert!(len <= 2_147_483_646, "byte count exceeds MPU limit");
    let mut bytes = vec![0; len];
    if len == 0 {
        return Ok(bytes);
    }
    std::fs::File::open("/dev/urandom")
        .or_else(|_| std::fs::File::open("/dev/random"))?
        .read_exact(&mut bytes)?;
    Ok(bytes)
}

/// Returns an unsigned integer with at most `bits` bits.
/// MPU `csprng.c`: `urandomb`.
/// # Panics
/// Panics if `bits > 64`, or if an unseeded thread cannot read OS entropy.
/// ```
/// math_prime_util::srand(15);
/// assert_eq!(math_prime_util::urandomb(8), 8);
/// ```
pub fn urandomb(bits: u32) -> u64 {
    assert!(bits <= 64, "too many bits for u64");
    match bits {
        0 => 0,
        1..=32 => u64::from(irand32() >> (32 - bits)),
        _ => irand64() >> (64 - bits),
    }
}

/// Returns an integer in `0..modulus`. MPU `csprng.c`: `urandomm64`.
/// # Panics
/// Panics if `modulus == 0`, or if an unseeded thread cannot read OS entropy.
/// ```
/// math_prime_util::srand(15);
/// assert_eq!(math_prime_util::urandomm(10), 8);
/// ```
pub fn urandomm(modulus: u64) -> u64 {
    assert!(modulus > 0, "modulus must be positive");
    if modulus == 1 {
        return 0;
    }
    with_stream(|stream| stream.bounded(modulus))
}

/// Returns an integer in `low..=high`, or `None` if the range is empty.
/// MPU `XS.xs`: `urandomr`, with `csprng.c`: `urandomm64`.
/// # Panics
/// Panics if an unseeded thread cannot read OS entropy.
/// ```
/// math_prime_util::srand(15);
/// assert_eq!(math_prime_util::urandomr(3, 8), Some(7));
/// ```
pub fn urandomr(low: u64, high: u64) -> Option<u64> {
    if low > high {
        return None;
    }
    let width = high - low;
    Some(if width == u64::MAX {
        irand64()
    } else {
        low + urandomm(width + 1)
    })
}

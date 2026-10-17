//! Lucky-number sieves and bounds from MPU 0.75.
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

const SMALL_LUCKY: [u64; 48] = [
    1, 3, 7, 9, 13, 15, 21, 25, 31, 33, 37, 43, 49, 51, 63, 67, 69, 73, 75, 79, 87, 93, 99, 105,
    111, 115, 127, 129, 133, 135, 141, 151, 159, 163, 169, 171, 189, 193, 195, 201, 205, 211, 219,
    223, 231, 235, 237, 241,
];
const SMALL_LUCKY_COUNT: [u64; 48] = [
    0, 1, 1, 2, 2, 2, 2, 3, 3, 4, 4, 4, 4, 5, 5, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 8, 8, 8, 8, 8, 8, 9,
    9, 10, 10, 10, 10, 11, 11, 11, 11, 11, 11, 12, 12, 12, 12, 12,
];
const LMASK63: [u8; 65] = [
    1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 1, 1,
    0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1,
    1,
];
const LMASK5: [u32; 26] = [
    2334495963, 2261929142, 1169344621, 2204739155, 2727961910, 1639207725, 3513561243, 2430232978,
    1754683725, 3630970059, 3025873062, 1278646881, 3658323539, 3055177010, 1830209833, 3406669457,
    3054200212, 1837519692, 1531293898, 650340770, 757258597, 2606838995, 2530306226, 1169218145,
    3408442969, 11572,
];
const MAX_LUCKY: u64 = 18_446_744_073_709_551_517;

// MPU lucky_numbers.c: _small_lucky_sieve32. The list retains the C block-copy sieve.
fn small_sieve(n: u64) -> Vec<u64> {
    if n < 259 {
        return SMALL_LUCKY
            .iter()
            .copied()
            .take_while(|&v| v <= n)
            .collect();
    }
    let mut lucky = Vec::new();
    let mut c13 = 0;
    for i in (1..=n).step_by(2) {
        if i % 6 != 5 && LMASK63[(i % 63) as usize] != 0 {
            c13 += 1;
            if c13 == 13 {
                c13 = 0;
            } else {
                lucky.push(i);
            }
        }
    }
    let mut level = 5;
    while level < lucky.len() && lucky[level] - 1 < lucky.len() as u64 {
        let skip = lucky[level] as usize - 1;
        if 2 * (skip + 1) > lucky.len() {
            break;
        }
        let mut dest = skip;
        let mut source = skip + 1;
        while source < lucky.len() {
            let count = skip.min(lucky.len() - source);
            lucky.copy_within(source..source + count, dest);
            dest += count;
            source += skip + 1;
        }
        lucky.truncate(dest);
        level += 1;
    }
    if level < lucky.len() && lucky[level] - 1 < lucky.len() as u64 {
        let mut skip = lucky[level] as usize;
        let mut dest = skip - 1;
        while skip < lucky.len() {
            let count = ((lucky[level + 1] - lucky[level]) as usize).min(lucky.len() - skip);
            lucky.copy_within(skip..skip + count, dest);
            dest += count;
            skip += count + 1;
            level += 1;
        }
        lucky.truncate(dest);
    }
    lucky
}

// MPU ds_bitmask126.h: the same 32-bit words per 126 integers and rank tree.
struct BitSieve {
    words: Vec<u32>,
    tree: Vec<usize>,
    len: usize,
}
const OFFSETS: [u64; 32] = [
    1, 3, 7, 9, 13, 15, 21, 25, 31, 33, 37, 43, 45, 49, 51, 55, 63, 67, 69, 73, 75, 79, 85, 87, 93,
    97, 99, 105, 109, 111, 115, 117,
];
impl BitSieve {
    fn new(n: u64) -> Self {
        let count = usize::try_from(n / 126 + 1).expect("lucky sieve size exceeds address space");
        let mut words = vec![0u32; count];
        let end = if n <= 200_000_000 {
            6
        } else if n <= u64::from(u32::MAX) {
            7
        } else {
            20
        };
        let mut counters = [0u64; 48];
        for i in (1..=n).step_by(2) {
            let m = ((i >> 1) % 819) as usize;
            if LMASK5[m >> 5] & (1 << (m & 31)) == 0 {
                continue;
            }
            let mut keep = true;
            for level in 5..=end {
                counters[level] += 1;
                if counters[level] == SMALL_LUCKY[level] {
                    counters[level] = 0;
                    keep = false;
                    break;
                }
            }
            if keep {
                let bit = OFFSETS.binary_search(&(i % 126)).expect("presieve offset");
                words[(i / 126) as usize] |= 1 << bit;
            }
        }
        let base = count.next_power_of_two();
        let mut tree = vec![0; 2 * base];
        for (i, &word) in words.iter().enumerate() {
            tree[base + i] = word.count_ones() as usize;
        }
        for i in (1..base).rev() {
            tree[i] = tree[2 * i] + tree[2 * i + 1];
        }
        let mut sieve = Self {
            len: tree[1],
            words,
            tree,
        };
        let mut level = end + 1;
        while level < sieve.len {
            let skip = sieve.get(level).0 as usize - 1;
            if skip >= sieve.len {
                break;
            }
            let mut index = skip;
            while index < sieve.len {
                sieve.delete(index);
                index += skip;
            }
            level += 1;
        }
        sieve
    }
    fn get(&self, mut rank: usize) -> (u64, usize, u32) {
        let base = self.tree.len() / 2;
        let mut node = 1;
        while node < base {
            node *= 2;
            if rank >= self.tree[node] {
                rank -= self.tree[node];
                node += 1;
            }
        }
        let word_index = node - base;
        let mut word = self.words[word_index];
        for _ in 0..rank {
            word &= word - 1;
        }
        let bit = word.trailing_zeros();
        (
            word_index as u64 * 126 + OFFSETS[bit as usize],
            word_index,
            bit,
        )
    }
    fn delete(&mut self, rank: usize) {
        let (_, word, bit) = self.get(rank);
        self.words[word] &= !(1 << bit);
        let mut node = self.tree.len() / 2 + word;
        while node > 0 {
            self.tree[node] -= 1;
            node /= 2;
        }
        self.len -= 1;
    }
    fn values(&self) -> Vec<u64> {
        let mut values = Vec::with_capacity(self.len);
        for (i, &w) in self.words.iter().enumerate() {
            let mut word = w;
            while word != 0 {
                values.push(i as u64 * 126 + OFFSETS[word.trailing_zeros() as usize]);
                word &= word - 1;
            }
        }
        values
    }
}
fn sieve(n: u64) -> Vec<u64> {
    if n <= 240_000 {
        small_sieve(n)
    } else {
        BitSieve::new(n).values()
    }
}

/// Return the lucky numbers in the inclusive range.
/// Port of `lucky_numbers.c`: `lucky_sieve_range`.
/// ```
/// assert_eq!(math_prime_util::lucky_numbers(3, 10), vec![3, 7, 9]);
/// ```
pub fn lucky_numbers(lo: u64, hi: u64) -> Vec<u64> {
    if lo > hi {
        return Vec::new();
    }
    sieve(hi).into_iter().filter(|&n| n >= lo).collect()
}

/// Count lucky numbers at most `n`.
/// Port of `lucky_numbers.c`: `lucky_count` and `lucky_count_range`.
/// ```
/// assert_eq!(math_prime_util::lucky_count(10), 4);
/// ```
pub fn lucky_count(n: u64) -> u64 {
    if n < 48 {
        return SMALL_LUCKY_COUNT[n as usize];
    }
    let hi = if n == u64::MAX { n - 1 } else { n + (n & 1) };
    let mut count = hi / 2;
    for l in sieve(1 + lucky_count_upper(hi)).into_iter().skip(1) {
        if l > count {
            break;
        }
        count -= count / l;
    }
    count
}

/// Return the `n`th lucky number. Index zero has no value.
/// Port of `lucky_numbers.c`: `nth_lucky` (Wilson's backward sieve).
/// ```
/// assert_eq!(math_prime_util::nth_lucky(5), Some(13));
/// assert_eq!(math_prime_util::nth_lucky(0), None);
/// ```
pub fn nth_lucky(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    if n <= 48 {
        return Some(SMALL_LUCKY[(n - 1) as usize]);
    }
    let mut k = u128::from(n - 1);
    for l in sieve(n).into_iter().skip(1).rev() {
        k += k / u128::from(l - 1);
    }
    u64::try_from(2 * k + 1).ok()
}

/// Test whether `n` is lucky.
/// Port of `lucky_numbers.c`: `is_lucky` and `_test_lucky_to`.
/// ```
/// assert!(math_prime_util::is_lucky(13));
/// assert!(!math_prime_util::is_lucky(19));
/// ```
pub fn is_lucky(n: u64) -> bool {
    if n & 1 == 0 || n % 6 == 5 || LMASK63[(n % 63) as usize] == 0 || n > MAX_LUCKY {
        return false;
    }
    if n < 45 {
        return true;
    }
    let mut pos = (n + 1) >> 1;
    let mut index = 1;
    for &l in &SMALL_LUCKY[1..] {
        if pos < l {
            return true;
        }
        if pos % l == 0 {
            return false;
        }
        pos -= pos / l;
        index += 1;
    }
    let limit = 1 + lucky_count_upper(n);
    let (mut size, mut growth) = (600u64, 6u64);
    loop {
        let bound = if size < limit / 3 { size } else { limit };
        let values = sieve(bound);
        for &l in values.iter().skip(index) {
            index += 1;
            if pos < l {
                return true;
            }
            if pos % l == 0 {
                return false;
            }
            pos -= pos / l;
        }
        if bound == limit {
            return true;
        }
        size = size.saturating_mul(growth);
        growth += 1;
    }
}

fn simple_approx(n: u64) -> u64 {
    if n < 7 {
        return u64::from(n > 0) + u64::from(n > 2);
    }
    let x = n as f64;
    let logn = x.ln();
    let factor = if n <= 10_000 {
        1.03591
    } else if n <= 1_000_000 {
        0.99575
    } else if n <= 10_000_000 {
        1.03523 - logn / 305.0
    } else if n <= 100_000_000 {
        1.03432 - logn / 304.0
    } else if n <= 4_000_000_000 {
        1.03613 - logn / (100.0 * logn.ln())
    } else {
        1.03654 - logn / (100.0 * logn.ln())
    };
    (factor * x / logn) as u64
}
fn simple_lower(n: u64) -> u64 {
    if n <= 6 {
        return u64::from(n > 0) + u64::from(n > 2);
    }
    if n <= 9000 {
        (1.028 * n as f64 / (n as f64).ln() - 1.0) as u64
    } else {
        (0.99 * simple_approx(n) as f64) as u64
    }
}
fn simple_upper(n: u64) -> u64 {
    if n <= 6 {
        return u64::from(n > 0) + u64::from(n > 2);
    }
    let x = n as f64;
    let logn = x.ln();
    if n <= 7000 {
        return (5.0 + 1.039 * x / logn) as u64;
    }
    let a = if n < 10_017_000 {
        0.58003 - 3.00e-9 * (x - 7000.0)
    } else {
        0.55
    };
    (x / (1.065 * logn - a - 3.1 / logn - 2.85 / (logn * logn))) as u64
}

// MPU inverse_interpolate.c: find the first equality, or the predecessor of a gap.
fn inverse(mut lo: u64, mut hi: u64, n: u64, f: fn(u64) -> Option<u64>) -> u64 {
    if f(lo) == Some(n) {
        return lo;
    }
    while hi - lo > 1 {
        let mid = lo + (hi - lo) / 2;
        if f(mid).unwrap_or(0) < n {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    if f(hi).unwrap_or(0) > n { lo } else { hi }
}

/// Estimate the count of lucky numbers at most `n`.
/// Port of `lucky_numbers.c`: `lucky_count_approx`.
/// ```
/// assert_eq!(math_prime_util::lucky_count_approx(10), 4);
/// ```
pub fn lucky_count_approx(n: u64) -> u64 {
    if n < 48 {
        return SMALL_LUCKY_COUNT[n as usize];
    }
    inverse(simple_lower(n), simple_upper(n), n, nth_lucky_approx)
}
/// Return MPU's lower bound for the lucky count.
/// Port of `lucky_numbers.c`: `lucky_count_lower`.
/// ```
/// assert_eq!(math_prime_util::lucky_count_lower(10), 4);
/// ```
pub fn lucky_count_lower(n: u64) -> u64 {
    if n < 48 {
        return SMALL_LUCKY_COUNT[n as usize];
    }
    if n < 9000 {
        return simple_lower(n);
    }
    inverse(simple_lower(n), simple_upper(n), n, nth_lucky_upper)
}
/// Return MPU's upper bound for the lucky count.
/// Port of `lucky_numbers.c`: `lucky_count_upper`.
/// ```
/// assert_eq!(math_prime_util::lucky_count_upper(10), 4);
/// ```
pub fn lucky_count_upper(n: u64) -> u64 {
    if n < 48 {
        return SMALL_LUCKY_COUNT[n as usize];
    }
    if n < 40_000_000 || n > 18_428_297_000_000_000_000 {
        return simple_upper(n);
    }
    inverse(
        simple_lower(n),
        1 + (simple_upper(n) as f64 * 1.001) as u64,
        n,
        nth_lucky_lower,
    )
}

/// Estimate the `n`th lucky number. Index zero has no value.
/// Port of `lucky_numbers.c`: `nth_lucky_approx`, with the same fitted coefficients.
/// ```
/// assert_eq!(math_prime_util::nth_lucky_approx(5), Some(13));
/// ```
pub fn nth_lucky_approx(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    if n <= 48 {
        return Some(SMALL_LUCKY[(n - 1) as usize]);
    }
    let x = n as f64;
    let logn = x.ln();
    let loglogn = logn.ln();
    let square = loglogn * loglogn;
    let est = if n <= 65536 {
        let corr = if n >= 16384 {
            0.25427076035 + 0.00883698771 * (square - 5.16445809103)
        } else if n >= 2048 {
            0.24513311782 + 0.00880360023 * (square - 4.12651426090)
        } else if n >= 256 {
            0.25585213066 - 0.00898952075 * (square - 2.93412446098)
        } else {
            0.38691439589 - 0.12050840608 * (square - 1.84654667704)
        };
        x * (logn + 0.5 * loglogn + corr * square) + 0.5
    } else {
        let corr = if n >= 1_099_511_627_776 {
            -0.05012215934 - 0.00139445216 * (square - 11.03811938314)
        } else if n >= 68_719_476_736 {
            -0.04904974983 - 0.00155649126 * (square - 10.34912771904)
        } else if n >= 4_294_967_296 {
            -0.04770894029 - 0.00180229750 * (square - 9.60518309351)
        } else if n >= 67_108_864 {
            -0.04484819198 - 0.00229977135 * (square - 8.36125581665)
        } else if n >= 1_048_576 {
            -0.03971615189 - 0.00354309756 * (square - 6.91279440604)
        } else {
            -0.03240114452 - 0.00651036735 * (square - 5.78920076332)
        };
        x * (logn + (0.5 + corr) * square) + 0.5
    };
    Some(if est >= MAX_LUCKY as f64 {
        MAX_LUCKY
    } else {
        est as u64
    })
}

/// Return MPU's lower bound for the `n`th lucky number.
/// Index zero has no value. Port of `lucky_numbers.c`: `nth_lucky_lower`.
/// ```
/// assert_eq!(math_prime_util::nth_lucky_lower(5), Some(13));
/// ```
pub fn nth_lucky_lower(n: u64) -> Option<u64> {
    let approx = nth_lucky_approx(n)?;
    if n <= 48 {
        return Some(approx);
    }
    let corr = if n <= 122 {
        0.95
    } else if n <= 4096 {
        0.97
    } else if n <= 115_000 {
        0.998
    } else {
        0.999
    };
    Some((corr * approx as f64) as u64)
}

/// Return MPU's upper bound for the `n`th lucky number.
/// Index zero has no value. Port of `lucky_numbers.c`: `nth_lucky_upper`.
/// ```
/// assert_eq!(math_prime_util::nth_lucky_upper(5), Some(13));
/// ```
pub fn nth_lucky_upper(n: u64) -> Option<u64> {
    let approx = nth_lucky_approx(n)?;
    if n <= 48 {
        return Some(approx);
    }
    let corr = if n <= 1000 {
        1.01
    } else if n <= 8200 {
        1.005
    } else {
        1.001
    };
    let est = corr * approx as f64 + 0.5;
    Some(if est >= MAX_LUCKY as f64 {
        MAX_LUCKY
    } else {
        est as u64
    })
}

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
    2_334_495_963,
    2_261_929_142,
    1_169_344_621,
    2_204_739_155,
    2_727_961_910,
    1_639_207_725,
    3_513_561_243,
    2_430_232_978,
    1_754_683_725,
    3_630_970_059,
    3_025_873_062,
    1_278_646_881,
    3_658_323_539,
    3_055_177_010,
    1_830_209_833,
    3_406_669_457,
    3_054_200_212,
    1_837_519_692,
    1_531_293_898,
    650_340_770,
    757_258_597,
    2_606_838_995,
    2_530_306_226,
    1_169_218_145,
    3_408_442_969,
    11_572,
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
        } else if u32::try_from(n).is_ok() {
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
        1.035_91
    } else if n <= 1_000_000 {
        0.995_75
    } else if n <= 10_000_000 {
        1.035_23 - logn / 305.0
    } else if n <= 100_000_000 {
        1.034_32 - logn / 304.0
    } else if n <= 4_000_000_000 {
        1.036_13 - logn / (100.0 * logn.ln())
    } else {
        1.036_54 - logn / (100.0 * logn.ln())
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
        0.580_03 - 3.00e-9 * (x - 7000.0)
    } else {
        0.55
    };
    (x / (1.065 * logn - a - 3.1 / logn - 2.85 / (logn * logn))) as u64
}

// MPU inverse_interpolate.c: _linear_interpolate and _inverse_interpolate.
fn linear(lo: u64, hi: u64, rlo: u64, rhi: u64, n: u64) -> u64 {
    if n == rlo {
        return lo + 1;
    }
    if n == rhi {
        return hi - 1;
    }
    let span = hi - lo;
    let movement = ((n - rlo) as f64 * span as f64 / (rhi - rlo) as f64 + 0.5) as u64;
    lo + movement.clamp(1, span - 1)
}
fn inverse(mut lo: u64, mut hi: u64, n: u64, f: fn(u64) -> Option<u64>) -> u64 {
    let mut mid = lo + (hi - lo) / 2;
    let rmid = f(mid).unwrap_or(0);
    let (mut rlo, mut rhi);
    if rmid >= n {
        hi = mid;
        rhi = rmid;
        rlo = f(lo).unwrap_or(0);
        if rlo == n {
            return lo;
        }
    } else {
        lo = mid;
        rlo = rmid;
        rhi = f(hi).unwrap_or(0);
    }
    if hi - lo <= 1 {
        return if rlo == n || rhi > n { lo } else { hi };
    }
    mid = linear(lo, hi, rlo, rhi, n);
    let mut iteration = 1;
    while hi - lo > 1 && rhi > n {
        let rmid = f(mid).unwrap_or(0);
        if rmid >= n {
            hi = mid;
            rhi = rmid;
        } else {
            lo = mid;
            rlo = rmid;
        }
        if rhi == n || hi - lo <= 1 {
            break;
        }
        mid = linear(lo, hi, rlo, rhi, n);
        if iteration % 6 == 0 {
            let close = (0.003 * (hi - lo) as f64 + 1.0) as u64;
            if lo + close > mid {
                mid = lo + close;
            } else if hi - close < mid {
                mid = hi - close;
            }
        }
        mid = mid.clamp(lo + 1, hi - 1);
        iteration += 1;
    }
    if rlo == n {
        return lo;
    }
    if hi - lo <= 1 {
        return if rlo == n || rhi > n { lo } else { hi };
    }
    while hi - lo > 8 {
        let mut x1 = lo + (hi - lo) / 2;
        let mut rx1 = f(x1).unwrap_or(0);
        let d0 = (n - rlo) as f64;
        let d1 = rx1.abs_diff(n) as f64;
        let d2 = (rhi - n + 1) as f64;
        let sign = if rx1 >= n { 1.0 } else { -1.0 };
        let pos = sign * (x1 - lo) as f64 * d1 / (d1 * d1 + d0 * d2).sqrt();
        let movement = (pos + if pos >= 0.0 { 0.5 } else { -0.5 }) as i128;
        let candidate = i128::from(x1) - movement;
        if candidate <= i128::from(lo) || candidate >= i128::from(hi) || candidate == i128::from(x1)
        {
            if rx1 >= n {
                hi = x1;
                rhi = rx1;
            } else {
                lo = x1;
                rlo = rx1;
            }
        } else {
            let mut x3 = candidate as u64;
            let mut rx3 = f(x3).unwrap_or(0);
            if rx1 > rx3 {
                std::mem::swap(&mut x1, &mut x3);
                std::mem::swap(&mut rx1, &mut rx3);
            }
            if rx1 >= n {
                hi = x1;
                rhi = rx1;
            } else if rx3 >= n {
                lo = x1;
                rlo = rx1;
                hi = x3;
                rhi = rx3;
            } else {
                lo = x3;
                rlo = rx3;
            }
        }
    }
    while hi - lo > 1 {
        mid = lo + (hi - lo) / 2;
        if f(mid).unwrap_or(0) < n {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    hi
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
    if !(40_000_000..=18_428_297_000_000_000_000).contains(&n) {
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
    let est = if n <= 65_536 {
        let corr = if n >= 16_384 {
            0.254_270_760_35 + 0.008_836_987_71 * (square - 5.164_458_091_03)
        } else if n >= 2048 {
            0.245_133_117_82 + 0.008_803_600_23 * (square - 4.126_514_260_90)
        } else if n >= 256 {
            0.255_852_130_66 - 0.008_989_520_75 * (square - 2.934_124_460_98)
        } else {
            0.386_914_395_89 - 0.120_508_406_08 * (square - 1.846_546_677_04)
        };
        x * (logn + 0.5 * loglogn + corr * square) + 0.5
    } else {
        let corr = if n >= 1_099_511_627_776 {
            -0.050_122_159_34 - 0.001_394_452_16 * (square - 11.038_119_383_14)
        } else if n >= 68_719_476_736 {
            -0.049_049_749_83 - 0.001_556_491_26 * (square - 10.349_127_719_04)
        } else if n >= 4_294_967_296 {
            -0.047_708_940_29 - 0.001_802_297_50 * (square - 9.605_183_093_51)
        } else if n >= 67_108_864 {
            -0.044_848_191_98 - 0.002_299_771_35 * (square - 8.361_255_816_65)
        } else if n >= 1_048_576 {
            -0.039_716_151_89 - 0.003_543_097_56 * (square - 6.912_794_406_04)
        } else {
            -0.032_401_144_52 - 0.006_510_367_35 * (square - 5.789_200_763_32)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn presieve_and_rank_tree_match_small_sieve() {
        for n in [
            0, 1, 47, 241, 258, 259, 600, 4096, 240_000, 240_001, 300_000,
        ] {
            let bit = BitSieve::new(n);
            let values = bit.values();
            assert_eq!(values, small_sieve(n));
            for (index, &value) in values.iter().enumerate() {
                assert_eq!(bit.get(index).0, value);
            }
        }
    }

    #[test]
    fn estimator_endpoints() {
        for n in 0..=6 {
            let count = u64::from(n > 0) + u64::from(n > 2);
            assert_eq!(simple_approx(n), count);
            assert_eq!(simple_lower(n), count);
            assert_eq!(simple_upper(n), count);
        }
        for n in [
            7,
            7000,
            7001,
            9000,
            10_000,
            1_000_000,
            10_000_000,
            100_000_000,
            4_000_000_000,
            10_000_000_000,
        ] {
            assert!(simple_lower(n) <= simple_upper(n));
        }
        assert_eq!(nth_lucky_approx(u64::MAX), Some(MAX_LUCKY));
        assert_eq!(nth_lucky_upper(u64::MAX), Some(MAX_LUCKY));
        assert!(lucky_count_upper(u64::MAX) > 0);
        assert!(!is_lucky(MAX_LUCKY + 2));
    }

    #[test]
    fn interpolation_handles_equal_values_and_gaps() {
        assert_eq!(linear(10, 20, 100, 200, 100), 11);
        assert_eq!(linear(10, 20, 100, 200, 200), 19);
        assert_eq!(linear(10, 20, 100, 200, 150), 15);
        for n in 2..=2000 {
            let target = n / 3;
            assert_eq!(inverse(0, 1000, n, |x| Some(x * 3)), target);
            let first = n * 3;
            assert_eq!(inverse(0, 10000, n, |x| Some(x / 3)), first);
        }
    }

    // Reference: the textbook lucky sieve on odd numbers.
    fn naive_lucky(n: u64) -> Vec<u64> {
        let mut v: Vec<u64> = (1..=n).step_by(2).collect();
        let mut i = 1;
        while i < v.len() {
            let step = v[i] as usize;
            if step > v.len() {
                break;
            }
            let mut keep = Vec::with_capacity(v.len());
            for (j, &x) in v.iter().enumerate() {
                if (j + 1) % step != 0 {
                    keep.push(x);
                }
            }
            v = keep;
            i += 1;
        }
        v
    }

    fn list_1m() -> &'static [u64] {
        static LIST: std::sync::OnceLock<Vec<u64>> = std::sync::OnceLock::new();
        LIST.get_or_init(|| naive_lucky(1_000_000))
    }

    #[test]
    fn lucky_numbers_match_textbook_sieve() {
        assert_eq!(lucky_numbers(10, 3), Vec::<u64>::new());
        assert_eq!(lucky_numbers(0, 100), naive_lucky(100));
        assert_eq!(lucky_numbers(1, 5000), naive_lucky(5000));
        // Above 240000 the bit sieve path is used.
        let big = naive_lucky(260_000);
        assert_eq!(lucky_numbers(250_000, 260_000), {
            big.iter()
                .copied()
                .filter(|&x| x >= 250_000)
                .collect::<Vec<_>>()
        });
        assert_eq!(lucky_numbers(1, 260_000), big);
    }

    #[test]
    fn count_and_nth_agree_with_the_list() {
        let list = list_1m();
        for n in [
            0u64, 1, 2, 47, 48, 49, 100, 1000, 4096, 65_537, 299_999, 300_000,
        ] {
            let want = list.iter().filter(|&&x| x <= n).count() as u64;
            assert_eq!(lucky_count(n), want, "count {n}");
        }
        assert_eq!(nth_lucky(0), None);
        for k in [1usize, 5, 48, 49, 50, 500, 5000, 20_000, list.len()] {
            assert_eq!(nth_lucky(k as u64), Some(list[k - 1]), "nth {k}");
        }
    }

    #[test]
    fn is_lucky_matches_membership() {
        let list = list_1m();
        let set: std::collections::HashSet<u64> = list.iter().copied().collect();
        for n in 0..3000 {
            assert_eq!(is_lucky(n), set.contains(&n), "{n}");
        }
        for &n in list.iter().rev().take(60) {
            assert!(is_lucky(n), "{n}");
        }
        for n in (990_000..1_000_000).step_by(7) {
            assert_eq!(is_lucky(n), set.contains(&n), "{n}");
        }
    }

    #[test]
    fn bounds_bracket_the_exact_count_and_nth() {
        let list = list_1m();
        for n in [48u64, 100, 1000, 8999, 9000, 10_000, 100_000, 999_999] {
            let exact = list.iter().filter(|&&x| x <= n).count() as u64;
            let (lo, hi) = (lucky_count_lower(n), lucky_count_upper(n));
            assert!(lo <= exact && exact <= hi, "{n}: {lo} {exact} {hi}");
            let approx = lucky_count_approx(n);
            assert!(approx.abs_diff(exact) * 50 <= exact + 50, "{n}: {approx}");
        }
        for k in [49u64, 100, 122, 123, 1000, 4097, 8200, 8201, 50_000, 70_000] {
            let exact = list[(k - 1) as usize];
            assert_eq!(nth_lucky(k), Some(exact));
            let (lo, hi) = (nth_lucky_lower(k).unwrap(), nth_lucky_upper(k).unwrap());
            assert!(lo <= exact && exact <= hi, "{k}: {lo} {exact} {hi}");
            assert!(nth_lucky_approx(k).unwrap().abs_diff(exact) * 10 <= exact);
        }
        assert_eq!(nth_lucky_lower(0), None);
        assert_eq!(nth_lucky_upper(0), None);
        assert_eq!(nth_lucky_approx(0), None);
        assert_eq!(nth_lucky_lower(5), Some(13));
    }

    #[test]
    fn approximations_cover_every_correction_band() {
        let mut last = 0;
        for n in [
            49u64,
            255,
            256,
            2047,
            2048,
            16_383,
            16_384,
            65_536,
            65_537,
            1_048_575,
            1_048_576,
            67_108_864,
            4_294_967_296,
            68_719_476_736,
            1_099_511_627_776,
        ] {
            let v = nth_lucky_approx(n).unwrap();
            assert!(v > last, "{n}");
            assert!(nth_lucky_lower(n).unwrap() <= v && v <= nth_lucky_upper(n).unwrap() + 1);
            last = v;
        }
        // Large-n count bounds go through the interpolation search.
        for n in [40_000_000u64, 1_000_000_000, 1_000_000_000_000] {
            let (lo, ap, hi) = (
                lucky_count_lower(n),
                lucky_count_approx(n),
                lucky_count_upper(n),
            );
            assert!(lo <= ap && ap <= hi, "{n}: {lo} {ap} {hi}");
        }
        assert!(lucky_count_upper(39_999_999) > 0);
    }

    #[test]
    fn inverse_search_on_nonlinear_functions() {
        // The result brackets n: it is a crossing point of the monotone function.
        fn crossing(f: fn(u64) -> Option<u64>, r: u64, n: u64) -> bool {
            let v = |x: u64| f(x).unwrap();
            (v(r) <= n && v(r + 1) > n) || (v(r) >= n && (r == 0 || v(r - 1) < n))
        }
        let square: fn(u64) -> Option<u64> = |x| Some(x * x / 7);
        let root: fn(u64) -> Option<u64> = |x| Some(((x as f64).sqrt() * 5.0) as u64);
        let flat: fn(u64) -> Option<u64> = |x| Some(x / 50);
        for n in (1..400u64).chain([1000, 5000, 77_777]) {
            let r = inverse(0, 1_000_000, n, square);
            assert!(crossing(square, r, n), "square {n} {r}");
            let r = inverse(0, 1_000_000_000_000, n, root);
            assert!(crossing(root, r, n), "root {n} {r}");
            let r = inverse(0, 100_000_000, n, flat);
            assert!(crossing(flat, r, n), "flat {n} {r}");
        }
        // A function that always fails reads as zero.
        assert_eq!(inverse(0, 10, 3, |_| None), 10);
    }
}

#[cfg(test)]
mod estimate_mutation_tests {
    use crate as math_prime_util;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/mutants_estimates.rs"
    ));
    private_estimate_tests!(lucky);
}

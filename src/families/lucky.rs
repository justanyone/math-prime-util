//! Lucky-number sieves and bounds from MPU 0.75.
#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]

const SMALL_LUCKY: [u64; 48] = [1,3,7,9,13,15,21,25,31,33,37,43,49,51,63,67,69,73,75,79,87,93,99,105,111,115,127,129,133,135,141,151,159,163,169,171,189,193,195,201,205,211,219,223,231,235,237,241];
const SMALL_LUCKY_COUNT: [u64; 48] = [0,1,1,2,2,2,2,3,3,4,4,4,4,5,5,6,6,6,6,6,6,7,7,7,7,8,8,8,8,8,8,9,9,10,10,10,10,11,11,11,11,11,11,12,12,12,12,12];
const LMASK63: [u8; 65] = [1,1,0,1,1,0,1,1,0,1,1,0,1,1,0,1,1,0,0,0,0,1,1,0,1,1,0,0,0,0,1,1,0,1,1,0,1,1,0,0,0,0,1,1,0,1,1,0,1,1,0,1,1,0,1,1,0,0,0,0,0,0,0,1,1];
const LMASK5: [u32; 26] = [2334495963,2261929142,1169344621,2204739155,2727961910,1639207725,3513561243,2430232978,1754683725,3630970059,3025873062,1278646881,3658323539,3055177010,1830209833,3406669457,3054200212,1837519692,1531293898,650340770,757258597,2606838995,2530306226,1169218145,3408442969,11572];
const MAX_LUCKY: u64 = 18_446_744_073_709_551_517;

// MPU lucky_numbers.c: _small_lucky_sieve32. The list retains the C block-copy sieve.
fn small_sieve(n: u64) -> Vec<u64> {
    if n < 259 {
        return SMALL_LUCKY.iter().copied().take_while(|&v| v <= n).collect();
    }
    let mut lucky = Vec::new();
    let mut c13 = 0;
    for i in (1..=n).step_by(2) {
        if i % 6 != 5 && LMASK63[(i % 63) as usize] != 0 {
            c13 += 1;
            if c13 == 13 { c13 = 0; } else { lucky.push(i); }
        }
    }
    let mut level = 5;
    while level < lucky.len() && lucky[level] - 1 < lucky.len() as u64 {
        let skip = lucky[level] as usize - 1;
        if 2 * (skip + 1) > lucky.len() { break; }
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
const OFFSETS: [u64; 32] = [1,3,7,9,13,15,19,21,25,27,31,33,37,39,43,45,49,51,55,57,61,63,67,69,73,75,79,81,85,87,91,93];

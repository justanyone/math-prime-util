use std::sync::Mutex;

static CACHE: Mutex<Vec<u8>> = Mutex::new(Vec::new());

/// Returns the least number of ones needed to express `n` with `+` and `*`.
///
/// Zero returns zero and clears the cache. Uses MPU 0.75 `intcomplexity.c`,
/// `integer_complexity`, `expand_ic`, and `_calcg`, including cache growth.
///
/// # Panics
///
/// Panics if `n` is `u64::MAX`, or the cache size exceeds the address space.
///
/// ```
/// assert_eq!(math_prime_util::integer_complexity(12), 7);
/// ```
pub fn integer_complexity(n: u64) -> u64 {
    assert!(n != u64::MAX, "integer_complexity: input too large");
    let index = usize::try_from(n).expect("integer_complexity: input too large");
    // Recovering the guard also permits a reset after an allocation panic.
    let mut cache = CACHE
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    if n == 0 {
        *cache = Vec::new();
        return 0;
    }
    if index >= cache.len() {
        expand(&mut cache, index);
    }
    u64::from(cache[index])
}

fn calc_g(mut n: usize) -> u64 {
    if n >= 122 {
        return u64::MAX;
    }
    let mut result = 1;
    while n >= 5 || n == 3 {
        result *= 3;
        n -= 3;
    }
    result << (n / 2)
}

fn expand(cache: &mut Vec<u8>, requested: usize) {
    let to = requested.saturating_add(32767).min(usize::MAX - 1);
    let first = if cache.len() < 2 { 2 } else { cache.len() };
    cache.resize(to + 1, 127);
    if first == 2 {
        cache[0] = 0;
        cache[1] = 1;
    }
    let g: [u64; 128] = std::array::from_fn(calc_g);
    for i in 2..first.min(to / 2 + 1) {
        for j in 2..=i.min(to / i) {
            let product = i * j;
            if product >= first {
                cache[product] = cache[product].min(cache[i] + cache[j]);
            }
        }
    }
    for i in first..=to {
        let t = usize::from(cache[i - 1]);
        let mut k = 1;
        cache[i] = cache[i].min(cache[i - 1] + 1);
        while k < t / 2 && u128::from(g[k + 1]) + u128::from(g[t - k - 1]) >= i as u128 {
            k += 1;
        }
        for j in 6..=usize::try_from(g[k]).expect("addition bound fits the cache") {
            cache[i] = cache[i].min(cache[j] + cache[i - j]);
        }
        if i <= to / 2 {
            for j in 2..=i.min(to / i) {
                let product = i * j;
                cache[product] = cache[product].min(cache[i] + cache[j]);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{calc_g, expand, integer_complexity};

    #[test]
    fn bounds_and_growth() {
        assert_eq!(calc_g(0), 1);
        assert_eq!(calc_g(3), 3);
        assert_eq!(calc_g(121), 16_210_220_612_075_905_068);
        assert_eq!(calc_g(122), u64::MAX);
        let mut cache = Vec::new();
        expand(&mut cache, 1);
        assert_eq!(cache[12], 7);
        let old_len = cache.len();
        expand(&mut cache, old_len);
        assert_eq!(cache[old_len], 30);
    }

    #[test]
    fn repeated_calls_and_concurrent_resets() {
        let threads: Vec<_> = (0..4)
            .map(|_| {
                std::thread::spawn(|| {
                    assert_eq!(integer_complexity(0), 0);
                    for _ in 0..2 {
                        assert_eq!(integer_complexity(12), 7);
                        assert_eq!(integer_complexity(27), 9);
                    }
                })
            })
            .collect();
        for thread in threads {
            thread.join().unwrap();
        }
    }

    #[test]
    #[should_panic(expected = "input too large")]
    fn invalid_input() {
        integer_complexity(u64::MAX);
    }
}

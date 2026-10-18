use std::sync::Mutex;

static CACHE: Mutex<Vec<u8>> = Mutex::new(Vec::new());

/// Returns the least number of ones needed to express `n` with `+` and `*`.
///
/// Zero returns `None` and clears the cache. Uses MPU 0.75 `intcomplexity.c`,
/// `integer_complexity`, `expand_ic`, and `_calcg`, including cache growth.
///
/// # Panics
///
/// Panics if `n` exceeds `i64::MAX`, or the cache size exceeds the address space.
///
/// ```
/// assert_eq!(math_prime_util::integer_complexity(12), Some(7));
/// ```
pub fn integer_complexity(n: u64) -> Option<u64> {
    assert!(
        i64::try_from(n).is_ok(),
        "integer_complexity: input too large"
    );
    let index = usize::try_from(n).expect("integer_complexity: input too large");
    // Recovering the guard also permits a reset after an allocation panic.
    let mut cache = CACHE
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    if n == 0 {
        *cache = Vec::new();
        return None;
    }
    if index >= cache.len() {
        expand(&mut cache, index);
    }
    Some(u64::from(cache[index]))
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
        assert_eq!(cache[59_049], 30);
    }

    /// Complexity by the textbook recurrence: sums and products of smaller values.
    fn reference(limit: usize) -> Vec<u8> {
        let mut c = vec![0_u8; limit + 1];
        for n in 1..=limit {
            let mut best = if n == 1 { 1 } else { u8::MAX };
            for a in 1..=n / 2 {
                best = best.min(c[a] + c[n - a]);
            }
            let mut d = 2;
            while d * d <= n {
                if n % d == 0 {
                    best = best.min(c[d] + c[n / d]);
                }
                d += 1;
            }
            c[n] = best;
        }
        c
    }

    #[test]
    fn matches_the_recurrence() {
        let want = reference(6_000);
        let mut cache = Vec::new();
        expand(&mut cache, 1);
        assert_eq!(cache[1..=6_000], want[1..=6_000]);
        for n in [1_u64, 2, 3, 11, 12, 27, 239, 1_000, 5_999] {
            assert_eq!(
                integer_complexity(n),
                Some(u64::from(want[usize::try_from(n).unwrap()]))
            );
        }
    }

    #[test]
    fn growing_in_steps_equals_one_big_step() {
        // The second call starts above half of its end, the case where the
        // product pass over the old part of the table must reach `to / 2`.
        let mut grown = Vec::new();
        expand(&mut grown, 1);
        assert_eq!(grown.len(), 32_769);
        expand(&mut grown, 32_769);
        assert_eq!(grown.len(), 65_537);
        let mut once = Vec::new();
        expand(&mut once, 32_769);
        assert_eq!(once.len(), 65_537);
        assert_eq!(grown, once);
        // 2^16 = 2 * 2^15 is reached only through a product with i = to / 2.
        assert_eq!(grown[65_536], 32);
    }

    #[test]
    #[should_panic(expected = "capacity overflow")]
    fn expand_to_the_address_space_limit() {
        // The clamp keeps `to + 1` from wrapping, so the failure is the allocation.
        let mut cache = Vec::new();
        expand(&mut cache, usize::MAX);
    }

    #[test]
    fn repeated_calls_and_concurrent_resets() {
        let threads: Vec<_> = (0..4)
            .map(|_| {
                std::thread::spawn(|| {
                    assert_eq!(integer_complexity(0), None);
                    for _ in 0..2 {
                        assert_eq!(integer_complexity(12), Some(7));
                        assert_eq!(integer_complexity(27), Some(9));
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

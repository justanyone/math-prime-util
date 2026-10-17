use super::urandomm;

/// Returns `k` random items from `data`.
///
/// Port of `reference/mpu/XS.xs`, `vecsample`.
/// The last item moves to the front. Then `k` Fisher-Yates swaps run.
/// `k == 0` or an empty list returns an empty vector.
/// A larger `k` returns every item once.
///
/// # Panics
/// Panics if the first OS entropy request fails.
///
/// ```
/// math_prime_util::srand(15);
/// assert_eq!(
///     math_prime_util::vecsample(3, &[1, 2, 3, 4, 5, 6, 7, 8, 9]),
///     vec![1, 3, 5]
/// );
/// ```
pub fn vecsample<T: Clone>(k: u64, data: &[T]) -> Vec<T> {
    let n = data.len();
    if n == 0 || k == 0 {
        return Vec::new();
    }
    let limit = u64::try_from(n).expect("list length fits in u64");
    let k = usize::try_from(k.min(limit)).expect("sample size fits");
    // Same rotation as XS: position 0 holds the last item.
    let mut index = Vec::with_capacity(n);
    index.push(n - 1);
    index.extend(0..n - 1);
    let mut out = Vec::with_capacity(k);
    for i in 0..k {
        let span = u64::try_from(n - i).expect("span fits in u64");
        let j = usize::try_from(urandomm(span)).expect("index fits");
        index.swap(i, i + j);
        out.push(data[index[i]].clone());
    }
    out
}

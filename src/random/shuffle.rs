use super::urandomm;

/// Shuffles a slice in place with the forward Fisher-Yates method.
///
/// Port of `reference/mpu/XS.xs`: `shuffle`.
///
/// # Panics
/// Panics if an unseeded thread cannot read OS entropy.
///
/// ```
/// math_prime_util::srand(15);
/// let mut items = [1, 2, 3, 4, 5];
/// math_prime_util::shuffle(&mut items);
/// assert_eq!(items, [4, 1, 2, 5, 3]);
/// ```
pub fn shuffle<T>(items: &mut [T]) {
    for i in 0..items.len().saturating_sub(1) {
        let j = usize::try_from(urandomm((items.len() - i) as u64)).unwrap();
        items.swap(i, i + j);
    }
}

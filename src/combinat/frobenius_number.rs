/// Return the largest integer outside the nonnegative span of `values`.
/// Return `Some(-1)` when the set contains 1. Return `None` on overflow.
///
/// Port of `util.c`, `frobenius_number`, from MPU 0.75.
/// Use the Round Robin algorithm of Böcker and Lipták.
///
/// # Panics
/// Panics if a value is zero, the set is not coprime, or a residue
/// table cannot be allocated.
///
/// ```
/// use math_prime_util::frobenius_number;
/// assert_eq!(frobenius_number(&[6, 9, 20]), Some(43));
/// assert_eq!(frobenius_number(&[1, 5]), Some(-1));
/// ```
pub fn frobenius_number(values: &[u64]) -> Option<i128> {
    assert!(!values.contains(&0), "Frobenius values must be positive");
    let mut basis = values.to_vec();
    basis.sort_unstable();
    if basis.first() == Some(&1) {
        return Some(-1);
    }
    if basis.len() <= 1 {
        return None;
    }
    assert_eq!(
        basis.iter().copied().reduce(crate::gcd),
        Some(1),
        "Frobenius number set must be coprime"
    );
    let product = u128::from(basis[0]) * u128::from(basis[1]);
    if basis.len() == 2 {
        let result = product - u128::from(basis[0]) - u128::from(basis[1]);
        return u64::try_from(result).ok().map(i128::from);
    }
    let modulus = usize::try_from(basis[0]).expect("Frobenius residue table is too large");
    let mut distances = Vec::new();
    distances
        .try_reserve_exact(modulus)
        .expect("Frobenius residue table cannot be allocated");
    distances.resize(modulus, u128::MAX);
    distances[0] = 0;
    for &coin in &basis[1..] {
        let residue = usize::try_from(coin % basis[0]).ok()?;
        if distances[residue] != u128::MAX && distances[residue] <= u128::from(coin) {
            continue;
        }
        let divisor = usize::try_from(crate::gcd(basis[0], coin)).ok()?;
        for r in 0..divisor {
            let mut distance = if r == 0 {
                0
            } else {
                (r..modulus)
                    .step_by(divisor)
                    .map(|q| distances[q])
                    .min()
                    .unwrap_or(u128::MAX)
            };
            if distance == u128::MAX {
                continue;
            }
            for _ in 1..modulus / divisor {
                distance = distance.checked_add(u128::from(coin))?;
                let p = usize::try_from(distance % u128::from(basis[0])).ok()?;
                distances[p] = distances[p].min(distance);
                distance = distances[p];
            }
        }
    }
    let maximum = distances.into_iter().max()?;
    if maximum == u128::MAX {
        None
    } else {
        u64::try_from(maximum - u128::from(basis[0]))
            .ok()
            .map(i128::from)
    }
}

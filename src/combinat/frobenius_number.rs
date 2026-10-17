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
    let product = basis[0].checked_mul(basis[1])?;
    if basis.len() == 2 {
        return Some(i128::from(product - basis[0] - basis[1]));
    }
    let modulus = usize::try_from(basis[0]).expect("Frobenius residue table is too large");
    let mut distances = Vec::new();
    distances
        .try_reserve_exact(modulus)
        .expect("Frobenius residue table cannot be allocated");
    distances.resize(modulus, u64::MAX);
    distances[0] = 0;
    for &coin in &basis[1..] {
        let residue = usize::try_from(coin % basis[0]).ok()?;
        if distances[residue] != u64::MAX && distances[residue] <= coin {
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
                    .unwrap_or(u64::MAX)
            };
            if distance == u64::MAX {
                continue;
            }
            for _ in 1..modulus / divisor {
                distance = distance.checked_add(coin)?;
                let p = usize::try_from(distance % basis[0]).ok()?;
                distances[p] = distances[p].min(distance);
                distance = distances[p];
            }
        }
    }
    let maximum = distances.into_iter().max()?;
    if maximum == u64::MAX {
        None
    } else {
        Some(i128::from(maximum - basis[0]))
    }
}

/// Returns twelve times the Hurwitz class number of `n`.
///
/// Returns `Some(-1)` at zero. Returns `None` beyond the C native bound
/// (`i64::MAX`) or if the result exceeds `u64`.
/// Port of `hclassno` and `_count_class_div` in MPU 0.75 `util.c`.
///
/// ```
/// assert_eq!(math_prime_util::hclassno(0), Some(-1));
/// assert_eq!(math_prime_util::hclassno(3), Some(4));
/// assert_eq!(math_prime_util::hclassno(4), Some(6));
/// ```
pub fn hclassno(n: u64) -> Option<i128> {
    if n == 0 {
        return Some(-1);
    }
    if n > (1_u64 << 63) - 1 {
        return None;
    }
    if matches!(n % 4, 1 | 2) {
        return Some(0);
    }
    if n == 3 {
        return Some(4);
    }
    let mut b = n & 1;
    let mut b2 = (n + 1) >> 2;
    let square = crate::is_square(b2);
    let mut h = divisors(b2).len() as u64 >> 1;
    if b == 1 {
        h = 1 + u64::from(square) + ((h - 1) << 1);
    }
    b += 2;
    loop {
        b2 = (n + b * b) >> 2;
        if 3 * b2 >= n {
            break;
        }
        h = h.checked_add(
            u64::from(b2 % b == 0)
                + u64::from(crate::is_square(b2))
                + 2 * count_class_div(b + 1, b2),
        )?;
        b += 2;
    }
    let correction = if b2 * 3 == n {
        4
    } else if square && n & 1 == 0 {
        6
    } else {
        0
    };
    h.checked_mul(12)?.checked_add(correction).map(i128::from)
}

fn count_class_div(start: u64, n: u64) -> u64 {
    let root = crate::sqrtint(n);
    let limit = root - u64::from(root * root == n);
    if start > limit {
        return 0;
    }
    if limit - start < 70 {
        (start..=limit)
            .filter(|d| n % d == 0)
            .count()
            .try_into()
            .expect("at most 70 divisors")
    } else {
        divisors(n)
            .into_iter()
            .filter(|&d| d >= start && d <= limit)
            .count()
            .try_into()
            .expect("divisor count fits u64")
    }
}

// Trial factorization supplies the divisor list used by the C algorithm.
fn divisors(mut n: u64) -> Vec<u64> {
    let mut result = vec![1];
    let mut p = 2;
    while p <= n / p {
        if n % p == 0 {
            let len = result.len();
            let mut power = 1;
            while n % p == 0 {
                n /= p;
                power *= p;
                for i in 0..len {
                    result.push(result[i] * power);
                }
            }
        }
        p += if p == 2 { 1 } else { 2 };
    }
    if n > 1 {
        for i in 0..result.len() {
            result.push(result[i] * n);
        }
    }
    result
}

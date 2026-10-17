/// Returns the reduced Bernoulli fraction, with `B_1 = +1/2`.
///
/// Returns `None` when the fraction exceeds `(i128, u128)`.
/// Port of MPU 0.75 `util.c`, `bernfrac`, including its table. Larger values
/// use `lib/Math/Prime/Util/PP.pm`, `_bernoulli_seidel`, the Seidel method
/// proposed in the C routine. Three limbs retain the exact intermediate sums.
///
/// ```
/// assert_eq!(math_prime_util::bernfrac(12), Some((-691, 2730)));
/// assert_eq!(math_prime_util::bernfrac(3), Some((0, 1)));
/// ```
pub fn bernfrac(n: u64) -> Option<(i128, u128)> {
    const SMALL: [(i128, u128); 14] = [
        (1, 1), (1, 6), (-1, 30), (1, 42), (-1, 30), (5, 66),
        (-691, 2730), (7, 6), (-3617, 510), (43867, 798),
        (-174611, 330), (854513, 138), (-236364091, 2730), (8553103, 6),
    ];
    if n == 1 {
        return Some((1, 2));
    }
    if n % 2 == 1 {
        return Some((0, 1));
    }
    // B_60 and all later even Bernoulli numerators exceed i128.
    if n >= 60 {
        return None;
    }
    let n = usize::try_from(n).ok()?;
    if let Some(&value) = SMALL.get(n / 2) {
        return Some(value);
    }
    let mut d = [[0_u64; 3]; 31];
    d[1][0] = 1;
    let (mut h, mut w) = (1, 1);
    for _ in 0..n {
        w ^= 1;
        if w != 0 {
            for j in 1..h {
                d[j] = add(d[j], d[j - 1]);
            }
        } else {
            w = h;
            h += 1;
            while w > 1 {
                w -= 1;
                d[w] = add(d[w], d[w + 1]);
            }
        }
    }
    let mut den = (1_u64 << (n + 1)) - 2;
    let (_, mut remainder) = divide(d[h - 1], den);
    let mut g = den;
    while remainder != 0 {
        (g, remainder) = (remainder, g % remainder);
    }
    let (num, _) = divide(d[h - 1], g);
    den /= g;
    let magnitude = (u128::from(num[1]) << 64) | u128::from(num[0]);
    let num = i128::try_from(magnitude).ok()?;
    Some((if n % 4 == 0 { -num } else { num }, u128::from(den)))
}

fn add(a: [u64; 3], b: [u64; 3]) -> [u64; 3] {
    let mut result = [0; 3];
    let mut carry = 0;
    for i in 0..3 {
        let sum = u128::from(a[i]) + u128::from(b[i]) + carry;
        result[i] = u64::try_from(sum & u128::from(u64::MAX)).expect("low limb fits");
        carry = sum >> 64;
    }
    result
}

fn divide(a: [u64; 3], divisor: u64) -> ([u64; 3], u64) {
    let mut quotient = [0; 3];
    let mut remainder = 0_u128;
    for i in (0..3).rev() {
        let value = (remainder << 64) | u128::from(a[i]);
        quotient[i] = u64::try_from(value / u128::from(divisor)).expect("quotient limb fits");
        remainder = value % u128::from(divisor);
    }
    (quotient, u64::try_from(remainder).expect("remainder fits"))
}

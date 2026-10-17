use super::is_carmichael::prime_factors;

/// Return the number of quasi-Carmichael bases of `n`.
///
/// Port of `is_quasi_carmichael` and `is_quasi_base` in MPU 0.75
/// `util.c`. Divisors use the prime factor products of `factor.c`.
///
/// ```
/// assert_eq!(math_prime_util::is_quasi_carmichael(35), 1);
/// assert_eq!(math_prime_util::is_quasi_carmichael(7), 0);
/// ```
pub fn is_quasi_carmichael(n: u64) -> u64 {
    if n < 35 || [4, 9, 25, 49, 121, 169].iter().any(|p| n % p == 0) {
        return 0;
    }
    let factors = prime_factors(n);
    if factors.len() < 2 || factors.windows(2).any(|w| w[0] == w[1]) {
        return 0;
    }
    let spf = factors[0];
    let lpf = factors[factors.len() - 1];
    let two = factors.len() == 2;
    let target = if two {
        n / spf - 1
    } else {
        lpf * (n / lpf - 1)
    };
    let mut bases = 0;
    for d in divisors(target) {
        let k = if two {
            if d >= spf {
                break;
            }
            spf - d
        } else {
            let k = lpf.wrapping_sub(d);
            if lpf > d && k >= spf {
                continue;
            }
            k
        };
        if k != 0 && is_quasi_base(n, &factors, k) {
            bases += 1;
        }
    }
    bases
}

fn is_quasi_base(n: u64, factors: &[u64], b: u64) -> bool {
    let p = n.wrapping_sub(b);
    factors.iter().all(|f| {
        let d = f.wrapping_sub(b);
        d != 0 && p % d == 0
    })
}

fn divisors(n: u64) -> Vec<u64> {
    let factors = prime_factors(n);
    let mut divs = vec![1];
    let mut i = 0;
    while i < factors.len() {
        let p = factors[i];
        let len = divs.len();
        let mut power = 1;
        while i < factors.len() && factors[i] == p {
            power *= p;
            for j in 0..len {
                divs.push(divs[j] * power);
            }
            i += 1;
        }
    }
    divs.sort_unstable();
    divs
}

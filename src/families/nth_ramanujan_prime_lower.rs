#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
use crate::{logarithmic_integral, prime_count};

pub(super) const SMALL: [u64; 23] = [
    2, 11, 17, 29, 41, 47, 59, 67, 71, 97, 101, 107, 127, 149, 151, 167, 179, 181, 227, 229, 233,
    239, 241,
];
const MAX_INDEX: u64 = 425_656_284_035_217_743;
const MAX_PRIME: u64 = 18_446_744_073_709_551_557;

// MPU prime_counts.c, ramanujan_axler.
pub(super) fn axler(n: u64, c: f64, d: f64) -> u64 {
    let n = n as f64;
    let l = n.ln();
    let ll = l.ln();
    let b = 2.0_f64.ln();
    let c1 = 2.0 * b * b + b + c;
    let c2 = b * b * b + 2.0 * b * b + d;
    let u = (b * l * ll * ll - c1 * l * ll + c2 * l - b * b * ll + b * b * b + b * b)
        / (l * l * l * l + l * l * l * ll - l * l * l * b - l * l * b);
    (2.0 * n * (1.0 + b / l - (b * ll - b * b - b) / (l * l) + u)) as u64
}

// MPU prime_counts.c, prime_count_upper. Preserve its float table.
pub(super) fn count_upper(n: u64) -> u64 {
    if n < 33_000 {
        return prime_count(n);
    }
    let x = n as f64;
    let l = x.ln();
    let l2 = l * l;
    let upper = if n <= 821_800_000 {
        const TABLE: &[(u64, f32)] = &[
            (59000, 2.48),
            (355991, 2.54),
            (3550000, 2.51),
            (3560000, 2.49),
            (5000000, 2.48),
            (8000000, 2.47),
            (13000000, 2.46),
            (18000000, 2.45),
            (31000000, 2.44),
            (41000000, 2.43),
            (48000000, 2.42),
            (119000000, 2.41),
            (182000000, 2.40),
            (192000000, 2.395),
            (213000000, 2.390),
            (271000000, 2.385),
            (322000000, 2.380),
            (400000000, 2.375),
            (510000000, 2.370),
            (682000000, 2.367),
            (2953652287, 2.362),
        ];
        let a = TABLE
            .iter()
            .find(|&&(limit, _)| n < limit)
            .map_or(2.334, |&(_, a)| f64::from(a));
        x / l * (1.0 + 1.0 / l + a / l2)
    } else if n < 101_260_000_000 {
        let a = if n < 1_100_000_000 {
            0.032
        } else if n < 10_010_000_000 {
            0.027
        } else {
            0.021
        };
        logarithmic_integral(x) - a * l * x.sqrt() / (8.0 * std::f64::consts::PI)
    } else {
        logarithmic_integral(x)
    };
    upper.floor() as u64
}

// MPU prime_counts.c, prime_count_lower.
pub(super) fn count_lower(n: u64) -> u64 {
    if n < 33_000 {
        return prime_count(n);
    }
    let x = n as f64;
    let l = x.ln();
    let l2 = l * l;
    let lower = if n <= 300_070 {
        let a = match n {
            0..69720 => 905.0,
            69720..70120 => 961.0,
            70120..88800 => 918.2,
            88800..176000 => 887.7,
            176000..299270 => 839.46,
            _ => 846.66,
        };
        x / (l - 1.0 - 1.0 / l - 2.975666 / l2 - 13.024334 / (l * l2) + a / (l2 * l2))
    } else if n < 4_000_000_000 {
        let a = match n {
            0..88783 => 4.0,
            88783..300000 => -3.0,
            300000..303000 => 5.0,
            303000..1100000 => -7.0,
            1100000..4500000 => -37.0,
            4500000..10200000 => -70.0,
            10200000..36900000 => -53.0,
            36900000..38100000 => -29.0,
            _ => -84.0,
        };
        logarithmic_integral(x) - x.sqrt() / l * (1.94 + 2.50 / l + a / l2)
    } else {
        logarithmic_integral(x) - 2.0 * x.sqrt() / l
    };
    lower.ceil() as u64
}

// MPU prime_counts.c, nth_prime_lower / nth_prime_upper.
pub(super) fn prime_bound(n: u64, upper: bool) -> u64 {
    if n >= MAX_INDEX {
        return if n == MAX_INDEX { MAX_PRIME } else { 0 };
    }
    // The C primes_small table holds primes below 1000.
    if n < 168 {
        return crate::nth_prime(n).unwrap_or(0);
    }
    let x = n as f64;
    let l = x.ln();
    let ll = l.ln();
    if n < if upper { 15_360 } else { 2_000_000 } {
        let mut lo = (x * (l + ll - 1.0)) as u64;
        let mut hi = (x * (l + ll)) as u64 + 1;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            let count = if upper {
                count_lower(mid)
            } else {
                count_upper(mid)
            };
            if count < n {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        return lo;
    }
    if !upper {
        let b = if n < 56_000_000 {
            11.200
        } else {
            11.50800000002
        };
        return (x * (l + ll - 1.0 + (ll - 2.0) / l - (ll * ll - 6.0 * ll + b) / (2.0 * l * l)))
            .ceil() as u64;
    }
    let (c, d) = match n {
        46254381.. => (2.00, 10.667),
        8009824.. => (2.00, 10.273),
        5450000.. => (2.00, 10.1335),
        3906280.. => (1.67, 20.2675),
        2110840.. => (2.51, -5.5714),
        876700.. => (2.49, -4.5129),
        688382.. => (3.31, -26.3858),
        575750.. => (-0.79, 83.5215),
        467650.. => (0.93, 37.1597),
        382440.. => (2.92, -15.4768),
        301130.. => (5.92, -91.3415),
        138630.. => (2.01, 7.2842),
        85820.. => (2.07, 5.2103),
        39016.. => (2.76, -11.5918),
        31490.. => (1.49, 15.1821),
        25070.. => (11.89, -197.8951),
        15359.. => (4.80, -51.5928),
        _ => (3.92, -33.3994),
    };
    (x * (l + ll - 1.0 + (ll - c) / l - (ll * ll - 6.0 * ll + d) / (2.0 * l * l))).floor() as u64
}

/// Return a lower bound for the nth Ramanujan prime.
///
/// Port of MPU 0.75 `ramanujan_primes.c`, `nth_ramanujan_prime_lower`.
/// Return `None` for index zero.
///
/// ```
/// assert_eq!(math_prime_util::nth_ramanujan_prime_lower(10), Some(97));
/// assert_eq!(math_prime_util::nth_ramanujan_prime_lower(0), None);
/// ```
pub fn nth_ramanujan_prime_lower(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    if n <= 23 {
        return Some(SMALL[(n - 1) as usize]);
    }
    let (c, d) = if n < 34_816 {
        match n {
            0..189 => (4.2720, 0.340),
            189..1245 => (-0.2179, -6.179),
            1245..2984 => (0.1446, -4.8693),
            2984..14303 => (-0.3570, -5.1154),
            _ => (-1.5770, -7.5332),
        }
    } else {
        (
            1.472,
            match n {
                0..76400 => 0.0126,
                76400..280816 => 0.5132,
                280816..915887 => 0.9967,
                915887..4000001 => 1.5004,
                4000001..16000001 => 1.7184,
                16000001..64000001 => 1.9860,
                64000001..128000001 => 2.1352,
                128000001..256000001 => 2.1658,
                256000001..384000001 => 2.1999,
                384000001..512000001 => 2.2047,
                512000001..640000001 => 2.2324,
                640000001..=14888378285 => 2.2385,
                14888378286..=467037926604 => 2.29,
                467037926605..=2778491401197 => 2.31,
                2778491401198..=10656144781918 => 2.315,
                10656144781919..=63698770351741 => 2.317,
                _ => 2.319,
            },
        )
    };
    Some(prime_bound(axler(n, c, d), false))
}

use super::long_double::Ld;
use super::nth_prime;
use super::nth_prime_upper::{MAX_PRIME_IDX, NPRIMES_SMALL};
use crate::moebius;

/// Return an estimate of the `n`th prime, with the first prime as index 1.
///
/// Port of MPU 0.75 `prime_counts.c`, `nth_prime_approx`: below index 306 the
/// result is the exact prime, from there it is `inverse_R(n)` of `real.c`,
/// the ceiling of the `x` with `R(x) = n` for Riemann's R function, found by
/// Halley's method. As in MPU, `R` and `li` are evaluated in x87
/// `long double`, so that the result matches MPU to the last digit, and the
/// estimate saturates at `u64::MAX`.
///
/// Returns `None` for `n = 0`, where MPU returns `undef`, and for `n` above
/// `425_656_284_035_217_743`, the index of the largest 64-bit prime, where
/// the `n`th prime does not fit in a `u64` (MPU answers those with bigints).
///
/// # Examples
///
/// ```
/// use math_prime_util::nth_prime_approx;
/// assert_eq!(nth_prime_approx(1_000), Some(7_923)); // the prime is 7919
/// assert_eq!(nth_prime_approx(100), Some(541));
/// assert_eq!(nth_prime_approx(0), None);
/// ```
#[must_use]
pub fn nth_prime_approx(n: u64) -> Option<u64> {
    if n == 0 || n > MAX_PRIME_IDX {
        return None;
    }
    if n < NPRIMES_SMALL {
        return nth_prime(n);
    }
    Some(inverse_r(Ld::from_u64(n)).ceil_u64())
}

/// `euler_mascheroni` of MPU 0.75 `real.c`, as a `long double`.
const EULER: Ld = Ld::new(-1, 0x93c4_67e3_7db0_c7a5);
/// `li2` of MPU 0.75 `real.c`, `li(2)`, as a `long double`.
const LI2: Ld = Ld::new(0, 0x85c7_ed3f_48c0_3c2d);

/// `riemann_zeta_table` of MPU 0.75 `real.c`: `zeta(k) - 1` for `k` in
/// `2..=56`, its `long double` literals rounded to 64 bits as the C compiler
/// does.
const ZETA_MINUS_ONE: [Ld; 55] = [
    Ld::new(-1, 0xA51A_6625_307D_3231),
    Ld::new(-3, 0xCEE8_013C_0188_4E0E),
    Ld::new(-4, 0xA899_1563_EC24_1B60),
    Ld::new(-5, 0x9741_8ECA_7CCD_B7A2),
    Ld::new(-6, 0x8E13_0981_2489_1EFB),
    Ld::new(-7, 0x88CB_6853_3CE2_36A7),
    Ld::new(-8, 0x859B_57C3_1CB7_45F3),
    Ld::new(-9, 0x839F_3D81_6B57_0300),
    Ld::new(-10, 0x825C_674B_772F_C302),
    Ld::new(-11, 0x818C_6F92_2CCA_A238),
    Ld::new(-12, 0x8105_2D96_6982_0CDD),
    Ld::new(-13, 0x80AC_9D08_BBDE_B063),
    Ld::new(-14, 0x8072_5795_A70A_B0F1),
    Ld::new(-15, 0x804B_DE5F_88DF_6855),
    Ld::new(-16, 0x8032_66F5_9178_79D0),
    Ld::new(-17, 0x8021_839B_4334_069C),
    Ld::new(-18, 0x8016_4CA9_BA26_5C95),
    Ld::new(-19, 0x800E_D847_CDD2_4F22),
    Ld::new(-20, 0x8009_E2CA_2337_4C44),
    Ld::new(-21, 0x8006_95D5_9409_093B),
    Ld::new(-22, 0x8004_6336_763B_E5D5),
    Ld::new(-23, 0x8002_EC78_9C2C_62D3),
    Ld::new(-24, 0x8001_F2CF_FEF0_8FB5),
    Ld::new(-25, 0x8001_4C75_2AB1_9918),
    Ld::new(-26, 0x8000_DD98_B666_CC81),
    Ld::new(-27, 0x8000_93B5_C842_2540),
    Ld::new(-28, 0x8000_6276_82D7_15CB),
    Ld::new(-29, 0x8000_41A3_00D4_3560),
    Ld::new(-30, 0x8000_2BC1_5575_F0AB),
    Ld::new(-31, 0x8000_1D2B_38CD_CB37),
    Ld::new(-32, 0x8000_1371_FB22_7A6B),
    Ld::new(-33, 0x8000_0CF6_9210_0970),
    Ld::new(-34, 0x8000_08A4_56B2_9487),
    Ld::new(-35, 0x8000_05C2_DF20_9C0F),
    Ld::new(-36, 0x8000_03D7_3CBF_F65F),
    Ld::new(-37, 0x8000_028F_7C7F_CC21),
    Ld::new(-38, 0x8000_01B4_FCFF_CB3F),
    Ld::new(-39, 0x8000_0123_52FF_D590),
    Ld::new(-40, 0x8000_00C2_372A_8B78),
    Ld::new(-41, 0x8000_0081_7A07_067C),
    Ld::new(-42, 0x8000_0056_514F_5931),
    Ld::new(-43, 0x8000_0039_8B84_E5F1),
    Ld::new(-44, 0x8000_0026_5D00_9938),
    Ld::new(-45, 0x8000_0019_9354_661E),
    Ld::new(-46, 0x8000_0011_0CE2_4411),
    Ld::new(-47, 0x8000_000B_5DEB_D80A),
    Ld::new(-48, 0x8000_0007_93F2_655C),
    Ld::new(-49, 0x8000_0005_0D4C_2E3D),
    Ld::new(-50, 0x8000_0003_5E32_BED3),
    Ld::new(-51, 0x8000_0002_3ECC_79E2),
    Ld::new(-52, 0x8000_0001_7F32_F941),
    Ld::new(-53, 0x8000_0000_FF77_4F81),
    Ld::new(-54, 0x8000_0000_AA4F_89AB),
    Ld::new(-55, 0x8000_0000_718A_5B72),
    Ld::new(-56, 0x8000_0000_4BB1_9221),
];

/// MPU 0.75 `real.c` Neumaier summation (`SUM_INIT`, `SUM_ADD`, `SUM_FINAL`).
#[derive(Clone, Copy)]
struct Neumaier {
    sum: Ld,
    comp: Ld,
}

impl Neumaier {
    fn new() -> Self {
        Self {
            sum: Ld::ZERO,
            comp: Ld::ZERO,
        }
    }

    fn add(&mut self, term: Ld) {
        let t = self.sum + term;
        self.comp = if self.sum.abs() >= term.abs() {
            self.comp + ((self.sum - t) + term)
        } else {
            self.comp + ((term - t) + self.sum)
        };
        self.sum = t;
    }

    fn total(self) -> Ld {
        self.sum + self.comp
    }
}

/// MPU 0.75 `real.c`, `ld_inverse_R`, for `lx >= 306`: the `x` with
/// `R(x) = lx` by Halley's method. The first two starting rules of the C
/// (`lx <= 3.5`, `lx < 50`) are for smaller `lx`, which take the prime table.
fn inverse_r(lx: Ld) -> Ld {
    let two = Ld::from_u64(2);
    let mut t = lx * lx.ln();
    if lx < Ld::from_u64(1000) {
        t = t * Ld::from_f64(1.15);
    } else {
        // One step of inverse li for the first estimate.
        let dn = li(t) - lx;
        let term = dn * t.ln() / (Ld::ONE + dn / (two * t));
        t = t - term;
    }
    let mut old_term = Ld::ZERO;
    for i in 0..100 {
        let dn = riemann_r(t) - lx;
        let term = dn * t.ln() / (Ld::ONE + dn / (two * t));
        if i > 0 && term.abs() >= old_term.abs() {
            t = t - term / Ld::from_u64(4);
            break;
        }
        old_term = term;
        t = t - term;
    }
    t
}

/// MPU 0.75 `real.c`, `RiemannR(x, 1e-12)`, for `x >= 1`.
fn riemann_r(x: Ld) -> Ld {
    riemann_r_with_precision(x, Ld::from_f64(1e-12))
}

/// Port of MPU 0.75 `real.c`, `RiemannR`, for `x >= 1`.
pub(super) fn riemann_r_with_precision(x: Ld, eps: Ld) -> Ld {
    let mut sum = Neumaier::new();
    if x > Ld::from_f64(1e19) {
        // R(x) = sum of moebius(k) / k * li(x^(1/k)).
        sum.add(li(x));
        for k in 2..=100 {
            let mu = moebius(k);
            if mu == 0 {
                continue;
            }
            let ki = Ld::ONE / Ld::from_u64(k);
            let mu = if mu < 0 { -Ld::ONE } else { Ld::ONE };
            let term = mu * ki * li(x.pow(ki));
            let old_sum = sum.total();
            sum.add(term);
            if (sum.total() - old_sum).abs() <= eps {
                break;
            }
        }
        return sum.total();
    }
    // Gram's series: 1 + sum of (ln x)^k / (k! k zeta(k + 1)).
    sum.add(Ld::ONE);
    let flogx = x.ln();
    let mut part_term = Ld::ONE;
    for k in 1..=10_000 {
        let ki = match usize::try_from(k - 1) {
            Ok(i) if i < ZETA_MINUS_ONE.len() => ZETA_MINUS_ONE[i],
            _ => zeta_minus_one(k + 1),
        };
        let kf = Ld::from_u64(k);
        part_term = part_term * (flogx / kf);
        let term = part_term / (kf + kf * ki);
        let old_sum = sum.total();
        sum.add(term);
        if (sum.total() - old_sum).abs() <= eps {
            break;
        }
    }
    sum.total()
}

/// MPU 0.75 `real.c`, `ld_riemann_zeta(x)` (that is, `zeta(x) - 1`) for an
/// integer `x >= 57`, past the table. The C sums `i^-x` for `i = 2, 3, ...`
/// and returns once a term is below `LDBL_EPSILON` times the sum. For
/// `x >= 48` that happens by `i = 5`, as `(2/5)^x < 2^-63`, so the
/// Bernoulli-number tail of the C is never reached.
fn zeta_minus_one(x: u64) -> Ld {
    let minus_x = -Ld::from_u64(x);
    let mut s = Ld::ZERO;
    for i in 2..=5 {
        let b = Ld::from_u64(i).pow(minus_x);
        s = s + b;
        if b.abs() < (Ld::EPSILON * s).abs() {
            break;
        }
    }
    s
}

/// Port of MPU 0.75 `real.c`, `Li`: round the NV argument and result.
/// Perl's NV is binary64 in the oracle build; `_Li` uses long double.
pub(super) fn li_nv(x: Ld) -> Ld {
    li(x.round_binary64()).round_binary64()
}

/// MPU 0.75 `real.c`, `_Li(x)`, for `x > 1`: Ramanujan's series up to
/// `10^16`, `Ei(ln x)` above.
pub(crate) fn li(x: Ld) -> Ld {
    if x == Ld::from_u64(2) {
        return LI2;
    }
    if x > Ld::from_f64(1e16) {
        return ei(x.ln());
    }
    let logx = x.ln();
    let two = Ld::from_u64(2);
    let mut sum = Ld::ZERO;
    let mut factorial = Ld::ONE;
    let mut power2 = Ld::ONE;
    let mut p = -Ld::ONE;
    let mut inner = Neumaier::new();
    let mut k = 0;
    for n in 1..200 {
        factorial = factorial * Ld::from_u64(n);
        p = p * -logx;
        let q = factorial * power2;
        power2 = power2 * two;
        while k <= (n - 1) / 2 {
            inner.add(Ld::ONE / Ld::from_u64(2 * k + 1));
            k += 1;
        }
        let old_sum = sum;
        sum = sum + (p / q) * inner.total();
        if (sum - old_sum).abs() <= Ld::EPSILON {
            break;
        }
    }
    EULER + logx.ln() + x.sqrt() * sum
}

/// MPU 0.75 `real.c`, `_Ei(x)`, for `0 < x < 87`: `_ei_series_convergent`.
/// MPU takes this branch for `x < -2 ln(LDBL_EPSILON)`, which holds for every
/// `x = ln t` with `t` below `2^65`.
fn ei(x: Ld) -> Ld {
    let mut sum = Neumaier::new();
    let mut fact_n = x;
    for n in 2..=400 {
        let invn = Ld::ONE / Ld::from_u64(n);
        fact_n = fact_n * (x * invn);
        let term = fact_n * invn;
        sum.add(term);
        if term.abs() < Ld::EPSILON * sum.total().abs() {
            break;
        }
    }
    sum.add(EULER);
    sum.add(x.abs().ln());
    sum.add(x);
    sum.total()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_is_exact() {
        assert_eq!(nth_prime_approx(1), Some(2));
        assert_eq!(nth_prime_approx(100), Some(541));
        assert_eq!(nth_prime_approx(305), Some(2011));
        assert_eq!(nth_prime_approx(0), None);
    }

    #[test]
    fn mpu_values() {
        // MPU 0.75 values (C `nth_prime_approx` on x86-64), one per branch:
        // lx < 1000, Ramanujan li, Ei for li above 10^16, R above 10^19.
        for (n, want) in [
            (306, 2021),
            (999, 7914),
            (1000, 7923),
            (10_000_000_000_000, 323_780_512_411_510),
            (10_000_000_000_000_000, 394_906_913_798_224_976),
            (230_000_000_000_000_000, 9_822_516_865_247_720_342),
            (300_000_000_000_000_000, 12_893_644_917_464_346_068),
            (400_000_000_000_000_000, 17_309_402_788_975_827_888),
        ] {
            assert_eq!(nth_prime_approx(n), Some(want), "n = {n}");
        }
    }

    #[test]
    fn limits() {
        assert_eq!(nth_prime_approx(MAX_PRIME_IDX - 1), Some(u64::MAX));
        assert_eq!(nth_prime_approx(MAX_PRIME_IDX), Some(u64::MAX));
        assert_eq!(nth_prime_approx(MAX_PRIME_IDX + 1), None);
    }

    #[test]
    fn helpers() {
        assert_eq!(li(Ld::from_u64(2)), LI2);
        // zeta(57) - 1 is 2^-57 (1 + (2/3)^57 + 2^-57) to 64 bits.
        let z = zeta_minus_one(57);
        assert!(z > Ld::new(-57, 1 << 63) && z < Ld::new(-57, (1 << 63) + (1 << 31)));
        // R(10^6) is about 78527.4.
        let r = riemann_r(Ld::from_u64(1_000_000));
        assert_eq!(r.floor_u64(), 78_527);
        let mut s = Neumaier::new();
        s.add(Ld::ONE);
        s.add(Ld::new(80, 1 << 63));
        s.add(-Ld::new(80, 1 << 63));
        assert_eq!(s.total(), Ld::ONE);
    }
}

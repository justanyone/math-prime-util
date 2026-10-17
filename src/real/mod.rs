//! Real special functions from MPU 0.75.
#![allow(
    clippy::excessive_precision,
    clippy::unreadable_literal,
    clippy::float_cmp,
    clippy::approx_constant,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

const EULER: f64 = 0.577215664901532860606512090082402431;
const LI2: f64 = 1.045163780117492784844588889194613136;
const EPS: f64 = f64::EPSILON;

// Neumaier compensation, as in real.c SUM_ADD.
#[derive(Default)]
struct Sum {
    value: f64,
    correction: f64,
}
impl Sum {
    fn add(&mut self, x: f64) {
        let t = self.value + x;
        self.correction += if self.value.abs() >= x.abs() {
            (self.value - t) + x
        } else {
            (x - t) + self.value
        };
        self.value = t;
    }
    fn total(&self) -> f64 {
        self.value + self.correction
    }
}
fn poly(a: &[f64], x: f64) -> f64 {
    a.iter().rev().fold(0.0, |v, &a| a + x * v)
}
const C6P: [f64; 7] = [
    -148151.02102575750838086,
    150260.59476436982420737,
    89904.972007457256553251,
    15924.175980637303639884,
    2150.0672908092918123209,
    116.69552669734461083368,
    5.0196785185439843791020,
];
const C6Q: [f64; 7] = [
    256664.93484897117319268,
    184340.70063353677359298,
    52440.529172056355429883,
    8125.8035174768735759866,
    750.43163907103936624165,
    40.205465640027706061433,
    1.0000000000000000000000,
];
const P2: [f64; 10] = [
    1.75338801265465972390E02,
    -2.23127670777632409550E02,
    -1.81949664929868906455E01,
    -2.79798528624305389340E01,
    -7.63147701620253630855E00,
    -1.52856623636929636839E01,
    -7.06810977895029358836E00,
    -5.00006640413131002475E00,
    -3.00000000320981265753E00,
    1.00000000000000485503E00,
];
const Q2: [f64; 9] = [
    3.97845977167414720840E04,
    3.97277109100414518365E00,
    1.37790390235747998793E02,
    1.17179220502086455287E02,
    7.04831847180424675988E01,
    -1.20187763547154743238E01,
    -7.99243595776339741065E00,
    -2.99999894040324959612E00,
    1.99999999999048104167E00,
];
const RIEMANN_ZETA_TABLE: [f64; 55] = [
    0.6449340668482264364724151666460251892,
    0.2020569031595942853997381615114499908,
    0.0823232337111381915160036965411679028,
    0.0369277551433699263313654864570341681,
    0.0173430619844491397145179297909205279,
    0.0083492773819228268397975498497967596,
    0.0040773561979443393786852385086524653,
    0.0020083928260822144178527692324120605,
    0.0009945751278180853371459589003190170,
    0.0004941886041194645587022825264699365,
    0.0002460865533080482986379980477396710,
    0.0001227133475784891467518365263573957,
    0.0000612481350587048292585451051353337,
    0.0000305882363070204935517285106450626,
    0.0000152822594086518717325714876367220,
    0.0000076371976378997622736002935630292,
    0.0000038172932649998398564616446219397,
    0.0000019082127165539389256569577951013,
    0.0000009539620338727961131520386834493,
    0.0000004769329867878064631167196043730,
    0.0000002384505027277329900036481867530,
    0.0000001192199259653110730677887188823,
    0.0000000596081890512594796124402079358,
    0.0000000298035035146522801860637050694,
    0.0000000149015548283650412346585066307,
    0.0000000074507117898354294919810041706,
    0.0000000037253340247884570548192040184,
    0.0000000018626597235130490064039099454,
    0.0000000009313274324196681828717647350,
    0.0000000004656629065033784072989233251,
    0.0000000002328311833676505492001455976,
    0.0000000001164155017270051977592973835,
    0.0000000000582077208790270088924368599,
    0.0000000000291038504449709968692942523,
    0.0000000000145519218910419842359296322,
    0.0000000000072759598350574810145208690,
    0.0000000000036379795473786511902372363,
    0.0000000000018189896503070659475848321,
    0.0000000000009094947840263889282533118,
    0.0000000000004547473783042154026799112,
    0.0000000000002273736845824652515226821,
    0.0000000000001136868407680227849349105,
    0.0000000000000568434198762758560927718,
    0.0000000000000284217097688930185545507,
    0.0000000000000142108548280316067698343,
    0.00000000000000710542739521085271287735,
    0.00000000000000355271369133711367329847,
    0.00000000000000177635684357912032747335,
    0.000000000000000888178421093081590309609,
    0.000000000000000444089210314381336419777,
    0.000000000000000222044605079804198399932,
    0.000000000000000111022302514106613372055,
    0.0000000000000000555111512484548124372374,
    0.0000000000000000277555756213612417258163,
    0.0000000000000000138777878097252327628391,
];
const C8P: [f64; 9] = [
    1.287168121482446392809e10,
    1.375396932037025111825e10,
    5.106655918364406103683e09,
    8.561471002433314862469e08,
    7.483618124380232984824e07,
    4.860106585461882511535e06,
    2.739574990221406087728e05,
    4.631710843183427123061e03,
    5.787581004096660659109e01,
];
const C8Q: [f64; 9] = [
    2.574336242964846244667e10,
    5.938165648679590160003e09,
    9.006330373261233439089e08,
    8.042536634283289888587e07,
    5.609711759541920062814e06,
    2.247431202899137523543e05,
    7.574578909341537560115e03,
    -2.373835781373772623086e01,
    1.000000000000000000000,
];
const A: [f64; 13] = [
    12.0,
    -720.0,
    30240.0,
    -1209600.0,
    47900160.0,
    -1892437580.3183791606367583212735166425,
    74724249600.0,
    -2950130727918.1642244954382084600497650,
    116467828143500.67248729113000661089201,
    -4597978722407472.6105457273596737891656,
    181521054019435467.73425331153534235290,
    -7166165256175667011.3346447367083352775,
    282908877253042996618.18640556532523927,
];

fn lambert_approx(x: f64) -> f64 {
    let (k1, k2, k3): (f64, f64, f64);
    if x < -0.312 {
        k2 = 2.0 * (1.0 + 2.7182818284590452353603 * x);
        if k2 <= 0.0 {
            return -1.0 + EPS;
        }
        k1 = (k2).sqrt();
        -1.0 + (1.0
            + (-1.0 / 3.0
                + (11.0 / 72.0
                    + (-43.0 / 540.0
                        + (769.0 / 17280.0
                            + (-221.0 / 8505.0
                                + (680863.0 / 43545600.0
                                    + (-1963.0 / 204120.0 + 226287557.0 / 37623398400.0 * k1)
                                        * k1)
                                    * k1)
                                * k1)
                            * k1)
                        * k1)
                    * k1)
                * k1)
            * k1
    } else if x > -0.14 && x < 0.085 {
        (1.0 + (-1.0
            + (3.0 / 2.0
                + (-8.0 / 3.0
                    + (125.0 / 24.0
                        + (-54.0 / 5.0
                            + (16807.0 / 720.0
                                + (-16384.0 / 315.0 + 531441.0 / 4480.0 * x) * x)
                                * x)
                            * x)
                        * x)
                    * x)
                * x)
            * x)
            * x
    } else if x < 1.0 {
        k1 = (1.0 + 2.7182818284590452353603 * x).sqrt();
        k2 = 0.33333333333333333333333 + 0.70710678118654752440084 / k1
            - 0.058925565098878960366737 * k1
            + (x + 0.36787944117144)
                * (0.050248489761611 + (0.11138904851051 + 0.040744556245195 * x) * x)
                / (1.0 + (2.7090878606183 + (1.5510922597820 + 0.095477712183841 * x) * x) * x);
        -(k2 - 1.0) / k2
    } else if x < 40.0 {
        k1 = 1.0
            + (5.950065500550155
                + (13.96586471370701
                    + (10.52192021050505 + (3.065294254265870 + 0.1204576876518760 * x) * x) * x)
                    * x)
                * x;
        0.1600049638651493 * k1.ln()
    } else if x < 20000.0 {
        k1 = 1.0
            + (-3.16866642511229e11
                + (3.420439800038598e10
                    + (-1.501433652432257e9
                        + (3.44887729947585e7
                            + (-4.453783741137856e5
                                + (3257.926478908996
                                    + (-10.82545259305382
                                        + (0.6898058947898353e-1
                                            + 0.4703653406071575e-4 * x)
                                            * x)
                                        * x)
                                    * x)
                                * x)
                            * x)
                        * x)
                    * x)
                * x;
        0.9898045358731312e-1 * k1.ln()
    } else {
        k1 = 1.0 / (1.0 + (1.0 + x).ln());
        k2 = 1.0 / k1;
        k3 = k2.ln();
        k2 - 1.0 - k3
            + (1.0
                + k3
                + (-1.0 / 2.0
                    + (1.0 / 2.0) * k3 * k3
                    + (-1.0 / 6.0 + (-1.0 + (-1.0 / 2.0 + (1.0 / 3.0) * k3) * k3) * k3) * k1)
                    * k1)
                * k1
    }
}

/// Return the real exponential integral.
/// Port of `real.c::Ei` and `_Ei` in MPU 0.75.
/// Zero returns negative infinity, as in the Perl front end.
/// ```
/// assert!((math_prime_util::exponential_integral(1.0) - 1.89511781635594).abs() < 1e-12);
/// ```
pub fn exponential_integral(x: f64) -> f64 {
    if x == 0.0 {
        return f64::NEG_INFINITY;
    }
    if x.is_nan() {
        return x;
    }
    if x >= 12000.0 {
        return f64::INFINITY;
    }
    if x <= -12000.0 {
        return 0.0;
    }
    if x < 0.0 {
        if x >= -1.0 {
            return (-x).ln() - poly(&C6P, -x) / poly(&C6Q, -x);
        }
        let y = -x;
        let (mut lk, mut lk1) = (1.0, y + 1.0);
        let mut sum = Sum::default();
        sum.add(1.0 / lk1);
        // u/d = 1/(k+1) for v=1. Cancel before factorials overflow in f64.
        for k in 1..500 {
            let k = f64::from(k);
            let q = lk1 * (y + 2.0 * k + 1.0) / (k + 1.0) - lk * k / (k + 1.0);
            let r = 1.0 / ((k + 1.0) * q * lk1);
            sum.add(r);
            lk = lk1;
            lk1 = q;
            if r.abs() < 0.1 * EPS {
                break;
            }
        }
        return -sum.total() * (-y).exp();
    }
    if x < -2.0 * EPS.ln() {
        let mut fact = x;
        let mut sum = Sum::default();
        for n in 2..=400 {
            let n = f64::from(n);
            fact *= x / n;
            let term = fact / n;
            sum.add(term);
            if term.abs() < EPS * sum.total().abs() {
                break;
            }
        }
        sum.add(EULER);
        sum.add(x.ln());
        sum.add(x);
        return sum.total();
    }
    let inv = 1.0 / x;
    let mut frac = 0.0;
    for n in 0..9 {
        frac = Q2[n] / (P2[n] + x + frac);
    }
    frac += P2[9];
    if x < 700.0 {
        x.exp() * (inv + inv * inv * frac)
    } else {
        (x - x.ln()).exp() * (1.0 + inv * frac)
    }
}

/// Return the logarithmic integral.
/// Port of `real.c::Li` and `_Li` in MPU 0.75.
/// # Panics
/// Panics if `x` is negative.
/// ```
/// assert!((math_prime_util::logarithmic_integral(2.0) - 1.04516378011749).abs() < 1e-12);
/// ```
pub fn logarithmic_integral(x: f64) -> f64 {
    assert!(
        x >= 0.0 || x.is_nan(),
        "LogarithmicIntegral requires x >= 0"
    );
    if x == 0.0 {
        return 0.0;
    }
    if x == 1.0 {
        return f64::NEG_INFINITY;
    }
    if x == 2.0 {
        return LI2;
    }
    if x > 1e16 || x < 1.0 || x.is_nan() {
        return exponential_integral(x.ln());
    }
    let logx = x.ln();
    let (mut sum, mut factorial, mut power2, mut p) = (0.0, 1.0, 1.0, -1.0);
    let mut inner = Sum::default();
    let mut k = 0;
    for n in 1..200 {
        factorial *= f64::from(n);
        p *= -logx;
        let q = factorial * power2;
        power2 *= 2.0;
        while k <= (n - 1) / 2 {
            inner.add(1.0 / f64::from(2 * k + 1));
            k += 1;
        }
        let old = sum;
        sum += p / q * inner.total();
        if (sum - old).abs() <= EPS {
            break;
        }
    }
    EULER + logx.ln() + x.sqrt() * sum
}

/// Return zeta(x) minus one, as in MPU.
/// Port of `real.c::ld_riemann_zeta` in MPU 0.75.
/// # Panics
/// Panics if `x` is negative.
/// ```
/// assert!((math_prime_util::riemann_zeta(2.0) - 0.6449340668482264).abs() < 1e-15);
/// ```
pub fn riemann_zeta(x: f64) -> f64 {
    assert!(x >= 0.0 || x.is_nan(), "RiemannZeta requires x >= 0");
    if x == 1.0 {
        return f64::INFINITY;
    }
    if x >= 2.0 && x < 2.0 + RIEMANN_ZETA_TABLE.len() as f64 && x == x.floor() {
        return RIEMANN_ZETA_TABLE[x as usize - 2];
    }
    if (0.5..=5.0).contains(&x) {
        let num = poly(&C8P, x);
        let den = poly(&C8Q, x);
        return (num - (x - 1.0) * den) / ((x - 1.0) * den);
    }
    if x > 17000.0 {
        return 0.0;
    }
    let (mut s, mut b) = (0.0, 0.0);
    for i in 2..11 {
        b = f64::from(i).powf(-x);
        s += b;
        if b.abs() < (EPS * s).abs() {
            return s;
        }
    }
    s = s + b * 10.0 / (x - 1.0) - 0.5 * b;
    let mut a = 1.0;
    for (i, &den) in A.iter().enumerate() {
        let k = (2 * i) as f64;
        a *= x + k;
        b /= 10.0;
        let t = a * b / den;
        s += t;
        if t.abs() < (EPS * s).abs() {
            break;
        }
        a *= x + k + 1.0;
        b /= 10.0;
    }
    s
}

/// Return Riemann's prime-count estimate.
/// Port of `real.c::RiemannR` in MPU 0.75, with epsilon 1e-16.
/// # Panics
/// Panics if `x` is not positive.
/// ```
/// assert!((math_prime_util::riemann_r(100.0) - 25.6616332669242).abs() < 1e-12);
/// ```
pub fn riemann_r(x: f64) -> f64 {
    assert!(x > 0.0, "RiemannR requires x > 0");
    if x.is_infinite() {
        return x;
    }
    let mut sum = Sum::default();
    if x > 1e19 {
        sum.add(logarithmic_integral(x));
        for k in 2..=100_u32 {
            let mut n = k;
            let mut mu = 1;
            for p in 2..=k {
                if n % p != 0 {
                    continue;
                }
                n /= p;
                mu = -mu;
                if n % p == 0 {
                    mu = 0;
                    break;
                }
            }
            if mu == 0 {
                continue;
            }
            let ki = 1.0 / f64::from(k);
            let term = f64::from(mu) * ki * logarithmic_integral(x.powf(ki));
            let old = sum.total();
            sum.add(term);
            if (sum.total() - old).abs() <= EPS {
                break;
            }
        }
    } else {
        sum.add(1.0);
        let mut part = 1.0;
        for k in 1..=10000 {
            let k = f64::from(k);
            part *= x.ln() / k;
            let term = part / (k + k * riemann_zeta(k + 1.0));
            let old = sum.total();
            sum.add(term);
            if (sum.total() - old).abs() <= EPS {
                break;
            }
        }
    }
    sum.total()
}

/// Return the principal real Lambert W value.
/// Port of `real.c::lambertw` and `_lambertw_approx` in MPU 0.75.
/// # Panics
/// Panics below MPU's branch limit, -0.36787944117145.
/// ```
/// assert!((math_prime_util::lambert_w(1.0) - 0.567143290409784).abs() < 1e-14);
/// ```
pub fn lambert_w(x: f64) -> f64 {
    if x.is_nan() {
        return x;
    }
    assert!(x >= -0.36787944117145, "LambertW requires x >= -1/e");
    if x <= -1.0 / std::f64::consts::E {
        return -1.0;
    }
    if x == 0.0 || x.is_infinite() {
        return x;
    }
    let mut w = lambert_approx(x);
    if x < -0.36768 {
        return w;
    }
    for _ in 0..6 {
        if w == 0.0 {
            break;
        }
        let w1 = 1.0 + w;
        let zn = (x / w).ln() - w;
        let qn = 2.0 * w1 * (w1 + 2.0 / 3.0 * zn);
        let en = zn / w1 * (qn - zn) / (qn - 2.0 * zn);
        let wen = w * en;
        w += wen;
        if wen.abs() <= 64.0 * EPS {
            break;
        }
    }
    w
}

fn inverse_estimate(x: f64) -> (f64, f64) {
    let mut t = if x < 2.719 {
        2.0 + f64::from(x >= 1.668)
    } else {
        let l = x.ln();
        x * (l + l.ln())
    };
    let mut old: f64 = 0.0;
    for i in 0..4 {
        let dn = logarithmic_integral(t) - x;
        let term = dn * t.ln() / (1.0 + dn / (2.0 * t));
        if i > 0 && term.abs() >= old.abs() {
            break;
        }
        old = term;
        t -= term;
    }
    (t, old.abs() + 2.0)
}

/// Return the real inverse of li on x >= 0.
/// Port of `real.c::ld_inverse_li` and `XS.xs::inverse_li_nv` in MPU 0.75.
/// # Panics
/// Panics if `x` is negative or not finite, or the result overflows.
/// ```
/// assert!((math_prime_util::inverse_li_nv(1.0451637801174928) - 2.0).abs() < 1e-14);
/// ```
pub fn inverse_li_nv(x: f64) -> f64 {
    assert!(
        x >= 0.0 && x.is_finite(),
        "inverse_li_nv requires finite x >= 0"
    );
    let t = inverse_estimate(x).0;
    assert!(t.is_finite(), "inverse_li_nv result exceeds f64 range");
    t
}

/// Return the least integer whose li is at least x.
/// Port of `real.c::inverse_li` in MPU 0.75. Results saturate at `u64::MAX`.
/// ```
/// assert_eq!(math_prime_util::inverse_li(10), 21);
/// ```
pub fn inverse_li(x: u64) -> u64 {
    if x <= 2 {
        return x + u64::from(x > 0);
    }
    let lx = x as f64;
    let (t, error) = inverse_estimate(lx);
    let mut r = t.ceil() as u64;
    let mut i = error as u64;
    if r == u64::MAX {
        return r;
    }
    if logarithmic_integral((r - 1) as f64) >= lx {
        while r > i && logarithmic_integral((r - i) as f64) >= lx {
            r -= i;
        }
        i /= 2;
        while i > 0 {
            if r > i && logarithmic_integral((r - i) as f64) >= lx {
                r -= i;
            }
            i /= 2;
        }
    } else if logarithmic_integral(r as f64) < lx {
        while logarithmic_integral(r.saturating_add(i).saturating_sub(1) as f64) < lx {
            let next = r.saturating_add(i);
            if next == r {
                break;
            }
            r = next;
        }
        i /= 2;
        while i > 0 {
            if logarithmic_integral(r.saturating_add(i).saturating_sub(1) as f64) < lx {
                r = r.saturating_add(i);
            }
            i /= 2;
        }
    }
    r
}

/// Return pi rounded to the given number of significant digits.
/// Port of `util.c::pidigits` in MPU 0.75. Zero selects the native f64 value.
/// # Panics
/// Panics if `digits` exceeds the C limit of 306783370.
/// ```
/// assert_eq!(math_prime_util::pi(5), "3.1416");
/// ```
pub fn pi(digits: u32) -> String {
    use std::fmt::Write;
    assert!(digits <= 306_783_370, "too many pi digits");
    if digits == 0 {
        return std::f64::consts::PI.to_string();
    }
    if digits == 1 {
        return "3".into();
    }
    let digits = digits as usize;
    let mut c = 14 * ((digits + 1) / 4 + 2);
    let mut a = vec![2000_u64; c];
    let mut out = String::new();
    let mut d = 0;
    while out.len() < digits + 1 {
        c -= 14;
        d %= 10000;
        let e = d;
        for b in (1..c).rev() {
            let g = (2 * b - 1) as u64;
            d = d * b as u64 + 10000 * a[b];
            a[b] = d % g;
            d /= g;
        }
        let mut block = e + d / 10000;
        if block > 9999 {
            block -= 10000;
            let mut bytes = out.into_bytes();
            for byte in bytes.iter_mut().rev() {
                if *byte == b'9' {
                    *byte = b'0';
                } else {
                    *byte += 1;
                    break;
                }
            }
            out = String::from_utf8(bytes).expect("decimal digits");
        }
        write!(out, "{block:04}").expect("write to String");
    }
    let mut bytes = out.into_bytes();
    if bytes[digits] >= b'5' {
        for byte in bytes[..digits].iter_mut().rev() {
            if *byte == b'9' {
                *byte = b'0';
            } else {
                *byte += 1;
                break;
            }
        }
    }
    bytes.truncate(digits);
    bytes.insert(1, b'.');
    String::from_utf8(bytes).expect("decimal digits")
}

/// Return the Bernoulli number, with B1 = +1/2.
/// MPU has no C `bernreal`. This ports `PP.pm::_bernoulli_seidel`, used by
/// `Util.pm::bernreal`, with floating-point Seidel entries.
/// ```
/// assert!((math_prime_util::bernreal(4) + 1.0/30.0).abs() < 1e-15);
/// ```
pub fn bernreal(n: u32) -> f64 {
    if n == 0 {
        return 1.0;
    }
    if n == 1 {
        return 0.5;
    }
    if n % 2 == 1 {
        return 0.0;
    }
    // B260 already exceeds the f64 range.
    if n >= 260 {
        return if n % 4 == 0 {
            f64::NEG_INFINITY
        } else {
            f64::INFINITY
        };
    }
    let mut d = vec![0.0; n as usize / 2 + 2];
    d[1] = 1.0;
    let (mut h, mut w) = (1, 1);
    // Scale all entries by the final denominator to avoid intermediate overflow.
    d[1] /= 2.0_f64.powf(f64::from(n) + 1.0) - 2.0;
    for _ in 0..n {
        w ^= 1;
        if w != 0 {
            for j in 1..h {
                d[j] += d[j - 1];
            }
        } else {
            w = h;
            h += 1;
            while w > 1 {
                w -= 1;
                d[w] += d[w + 1];
            }
            w = 0;
        }
    }
    if n % 4 == 0 { -d[h - 1] } else { d[h - 1] }
}

fn harmonic_split(a: u64, b: u64) -> f64 {
    if b - a == 1 {
        return 1.0 / a as f64;
    }
    let m = a + (b - a) / 2;
    harmonic_split(a, m) + harmonic_split(m, b)
}

/// Return the nth harmonic number.
/// MPU has no C `harmreal`. This ports `PP.pm::harmreal` and
/// `_harmonic_split`, using f64 instead of rational or `BigFloat` values.
/// ```
/// assert!((math_prime_util::harmreal(3) - 11.0/6.0).abs() < 1e-15);
/// ```
pub fn harmreal(n: u64) -> f64 {
    if n == 0 {
        return 0.0;
    }
    if n <= 2002 {
        return harmonic_split(1, n + 1);
    }
    let n = n as f64;
    let n2 = n * n;
    let mut nt = n2;
    let mut h = 0.0;
    for d in [
        -12.0, 120.0, -252.0, 240.0, -132.0, 32760.0, -12.0, 8160.0, -14364.0, 6600.0, -276.0,
        65520.0, -12.0,
    ] {
        let term: f64 = 1.0 / (d * nt);
        if term.abs() < 1e-44 {
            break;
        }
        h += term;
        nt *= n2;
    }
    h + 1.0 / (2.0 * n) + EULER + n.ln()
}

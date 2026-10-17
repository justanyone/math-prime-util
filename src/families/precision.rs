//! Compensated arithmetic for MPU's long-double prime estimates.
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
#[derive(Clone, Copy)]
struct Wide(f64, f64);
impl Wide {
    fn add(self, b: Self) -> Self {
        let s = self.0 + b.0;
        let v = s - self.0;
        let e = (self.0 - (s - v)) + (b.0 - v) + self.1 + b.1;
        Self(s + e, e - ((s + e) - s))
    }
    fn mul(self, b: Self) -> Self {
        let p = self.0 * b.0;
        let e = self.0.mul_add(b.0, -p) + self.0 * b.1 + self.1 * b.0;
        Self(p + e, e - ((p + e) - p))
    }
    fn div(self, b: Self) -> Self {
        let q = self.0 / b.0;
        let r = self.add(b.mul(Self(-q, 0.0)));
        Self(q, 0.0).add(Self((r.0 + r.1) / b.0, 0.0))
    }
    fn integer(self, ceil: bool) -> u64 {
        let base = self.0 as u64;
        let fraction = self.add(Self(-(base as f64), 0.0));
        let f = fraction.0 + fraction.1;
        let delta = if ceil { f.ceil() } else { f.floor() } as i64;
        if delta < 0 {
            base.saturating_sub(delta.unsigned_abs())
        } else {
            base.saturating_add(delta.unsigned_abs())
        }
    }
}
fn logarithm(n: u64) -> Wide {
    let e = n.ilog2();
    let base = 1u64 << e;
    let m = Wide((n >> 32) as f64, 0.0)
        .mul(Wide(4294967296.0, 0.0))
        .add(Wide((n & 0xffff_ffff) as f64, 0.0))
        .div(Wide(base as f64, 0.0));
    let z = m.add(Wide(-1.0, 0.0)).div(m.add(Wide(1.0, 0.0)));
    let z2 = z.mul(z);
    let mut t = z;
    let mut s = z;
    for i in 1..80 {
        t = t.mul(z2);
        s = s.add(t.div(Wide(f64::from(2 * i + 1), 0.0)));
    }
    // ln(2) split into its nearest double and residual.
    Wide(std::f64::consts::LN_2, 2.319_046_813_846_299_6e-17)
        .mul(Wide(f64::from(e), 0.0))
        .add(s.mul(Wide(2.0, 0.0)))
}
// real.c: RiemannR, Gram series with the same zeta coefficients.
pub(super) fn prime_approx(n: u64) -> u64 {
    let l = logarithm(n);
    let mut term = Wide(1.0, 0.0);
    let mut sum = term;
    for k in 1..180 {
        term = term.mul(l).div(Wide(f64::from(k), 0.0));
        let z = if k < 13 {
            Wide(crate::riemann_zeta(f64::from(k + 1)), 0.0)
        } else {
            let mut z = Wide(1.0, 0.0);
            for j in (2..=1000).rev() {
                z = z.add(Wide(f64::from(j).powi(-(k + 1)), 0.0));
            }
            z
        };
        sum = sum.add(term.div(Wide(f64::from(k), 0.0).mul(z)));
    }
    sum.add(Wide(0.5, 0.0)).integer(false)
}
// real.c: Li via the convergent Ei series, evaluated with a residual.
pub(super) fn prime_bound(n: u64, upper: bool) -> u64 {
    let l = logarithm(n);
    let mut term = l;
    let mut sum = Wide(0.577_215_664_901_532_9 + (l.0 + l.1).ln(), 0.0).add(term);
    for k in 2..200 {
        term = term.mul(l).div(Wide(f64::from(k), 0.0));
        sum = sum.add(term.div(Wide(f64::from(k), 0.0)));
    }
    if upper {
        sum.integer(false)
    } else {
        sum.add(Wide(-2.0 * (n as f64).sqrt() / (l.0 + l.1), 0.0))
            .integer(true)
    }
}

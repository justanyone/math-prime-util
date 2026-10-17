#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
use super::fermat_factor::attempt;
use crate::{gcd, is_square_root, sqrtint};

/// Tries Shanks' square forms factorization with racing multipliers.
///
/// Port of MPU 0.75 `factor.c::squfof_factor` and `squfof_unit`, with the
/// `XS.xs` wrapper. `None` uses four million rounds. Inputs above 62 bits
/// retain their cofactor. The returned factors can be composite.
/// Zero returns `[0]`; one returns `[]`.
///
/// ```
/// use math_prime_util::squfof_factor;
/// assert_eq!(squfof_factor(91, None), vec![7, 13]);
/// ```
pub fn squfof_factor(n: u64, rounds: Option<u64>) -> Vec<u64> {
    attempt(n, |n| split(n, rounds.unwrap_or(4_000_000)))
}

const MULTIPLIERS: [u64; 38] = [
    3 * 5 * 7 * 11,
    3 * 5 * 7,
    3 * 5 * 7 * 11 * 13,
    3 * 5 * 7 * 13,
    3 * 5 * 7 * 11 * 17,
    3 * 5 * 11,
    3 * 5 * 7 * 17,
    3 * 5,
    3 * 5 * 7 * 11 * 19,
    3 * 5 * 11 * 13,
    3 * 5 * 7 * 19,
    3 * 5 * 7 * 13 * 17,
    3 * 5 * 13,
    3 * 7 * 11,
    3 * 7,
    5 * 7 * 11,
    3 * 7 * 13,
    5 * 7,
    3 * 5 * 17,
    5 * 7 * 13,
    3 * 5 * 19,
    3 * 11,
    3 * 7 * 17,
    3,
    3 * 11 * 13,
    5 * 11,
    3 * 7 * 19,
    3 * 13,
    5,
    5 * 11 * 13,
    5 * 7 * 19,
    5 * 13,
    7 * 11,
    7,
    3 * 17,
    7 * 13,
    11,
    1,
];

#[derive(Clone, Copy, Default)]
struct State {
    valid: i8,
    p: u32,
    bn: u32,
    qn: u32,
    q0: u32,
    b0: u32,
    it: u32,
    imax: u32,
}

impl State {
    fn step(&mut self) {
        let old_p = self.p;
        self.p = self.bn.wrapping_mul(self.qn).wrapping_sub(self.p);
        let old_qn = self.qn;
        self.qn = self
            .q0
            .wrapping_add(self.bn.wrapping_mul(old_p.wrapping_sub(self.p)));
        self.q0 = old_qn;
        self.bn = (self.b0 + self.p) / self.qn;
        self.it = self.it.wrapping_add(1);
    }

    fn unit(&mut self, n: u64) -> u64 {
        let limit = self.it.wrapping_add(self.imax);
        loop {
            if self.it & 1 != 0 {
                self.step();
            }
            let root = loop {
                if self.it >= limit {
                    return 0;
                }
                self.step();
                if let Some(root) = is_square_root(u64::from(self.qn)) {
                    break root as u32;
                }
                self.step();
            };
            let mut s = root;
            let mut ro = self.p + s * ((self.b0 - self.p) / s);
            let mut so = ((n - u64::from(ro) * u64::from(ro)) / u64::from(s)) as u32;
            let mut bbn = (self.b0 + ro) / so;
            let mut count = 0;
            loop {
                let old_ro = ro;
                ro = bbn.wrapping_mul(so).wrapping_sub(ro);
                let old_so = so;
                so = s.wrapping_add(bbn.wrapping_mul(old_ro.wrapping_sub(ro)));
                s = old_so;
                bbn = (self.b0 + ro) / so;
                if ro == old_ro {
                    break;
                }
                count += 1;
                // C checks this bound after each group of four iterations.
                if count == 8_000_008 {
                    self.valid = 0;
                    return 0;
                }
            }
            let f = gcd(u64::from(ro), n);
            if f > 1 {
                return f;
            }
        }
    }
}

fn split(n: u64, rounds: u64) -> u64 {
    const MAX: u64 = u64::MAX >> 2;
    if n > MAX {
        return n;
    }
    let mut states = [State {
        valid: -1,
        ..State::default()
    }; 38];
    let mut racing = 38;
    let mut done = 0;
    while racing > 0 && done < rounds {
        for (state, mult) in states.iter_mut().zip(MULTIPLIERS) {
            if done >= rounds {
                break;
            }
            if state.valid == 0 {
                continue;
            }
            if state.valid == -1 {
                if n > MAX / mult {
                    state.valid = 0;
                    racing -= 1;
                    continue;
                }
                let nn = n * mult;
                let root = sqrtint(nn);
                state.valid = 1;
                state.q0 = 1;
                state.b0 = root as u32;
                state.p = root as u32;
                state.qn = (nn - root * root) as u32;
                if state.qn == 0 {
                    return root;
                }
                state.bn = (2 * root / u64::from(state.qn)) as u32;
                state.imax = ((root as f64).sqrt() as u64 / 16).max(20).min(rounds) as u32;
            }
            if racing == 1 {
                state.imax = (rounds - done) as u32;
            }
            let f = state.unit(n * mult);
            if f > 1 {
                let reduced = f / gcd(f, mult);
                if reduced > 1 {
                    return reduced;
                }
                state.valid = 0;
            }
            if state.valid == 0 {
                racing -= 1;
            }
            done += u64::from(state.imax);
        }
    }
    n
}

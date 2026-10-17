//! MPU almost/omega oracle cases from the committed families fixture.
mod common;
use math_prime_util::{nth_almost_prime_upper, nth_omega_prime, omega_prime_count, omega_primes};

fn native(case: &common::Case) -> bool {
    // Signed k=-2 is outside the unsigned API in each of the four functions.
    // nth_almost_prime_upper(100,3), (63,2011), and (150,4557) need bigint results.
    case.args.iter().all(|a| a.parse::<u64>().is_ok())
        && case.result.as_ref().is_none_or(|r| r == "" || r.contains(' ') || r.parse::<u64>().is_ok())
}

#[test]
fn compat_nth_almost_prime_upper() {
    for case in common::load("families").iter().filter(|c| c.func=="nth_almost_prime_upper") {
        if native(case) { case.check(nth_almost_prime_upper(case.arg(0),case.arg(1))); }
    }
}
#[test]
fn compat_omega_prime_count() {
    for case in common::load("families").iter().filter(|c| c.func=="omega_prime_count") {
        if native(case) { case.check(Some(omega_prime_count(case.arg(0),case.arg(1)))); }
    }
}
#[test]
fn compat_nth_omega_prime() {
    for case in common::load("families").iter().filter(|c| c.func=="nth_omega_prime") {
        if native(case) { case.check(nth_omega_prime(case.arg(0),case.arg(1))); }
    }
}
#[test]
fn compat_omega_primes() {
    for case in common::load("families").iter().filter(|c| c.func=="omega_primes") {
        let (lo,hi)=if case.args.len()==2 { (1,case.arg(1)) } else { (case.arg(1),case.arg(2)) };
        let result=omega_primes(case.arg(0),lo,hi);
        let text=result.iter().map(u64::to_string).collect::<Vec<_>>().join(" ");
        case.check(Some(text));
    }
}

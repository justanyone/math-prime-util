//! Shared loader for the oracle fixtures in `tests/fixtures/*.tsv`.
//!
//! Each line is `<function>\t<args>\t<result>`; see `tools/gen_fixtures.pl`.
#![allow(dead_code, clippy::missing_panics_doc)]

use std::fmt::Display;

/// One oracle case.
#[derive(Debug)]
pub struct Case {
    pub func: String,
    pub args: Vec<String>,
    /// `None` for `undef`.
    pub result: Option<String>,
    /// `true` when MPU died on the call.
    pub error: bool,
    pub line: usize,
}

impl Case {
    /// Argument `i` parsed as `T`.
    pub fn arg<T: std::str::FromStr>(&self, i: usize) -> T
    where
        T::Err: std::fmt::Debug,
    {
        self.args[i].parse().unwrap_or_else(|e| {
            panic!(
                "fixture line {}: arg {i} {:?}: {e:?}",
                self.line, self.args[i]
            )
        })
    }

    /// Asserts a Rust result against the oracle, with `None` meaning `undef`.
    pub fn check<T: Display>(&self, got: Option<T>) {
        assert!(
            !self.error,
            "fixture line {}: oracle died, Rust must panic: {self:?}",
            self.line
        );
        let got = got.map(|g| g.to_string());
        assert_eq!(
            got,
            self.result,
            "fixture line {}: {} {}",
            self.line,
            self.func,
            self.args.join(" ")
        );
    }
}

/// Loads `tests/fixtures/<family>.tsv`.
pub fn load(family: &str) -> Vec<Case> {
    let path = format!("{}/tests/fixtures/{family}.tsv", env!("CARGO_MANIFEST_DIR"));
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("{path}: {e}; run `perl tools/gen_fixtures.pl {family}`"));
    let mut cases = Vec::new();
    for (i, line) in text.lines().enumerate() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut parts = line.split('\t');
        let func = parts.next().unwrap().to_string();
        let args = parts
            .next()
            .unwrap_or("")
            .split_whitespace()
            .map(str::to_string)
            .collect();
        let res = parts.next().unwrap_or("").to_string();
        let (result, error) = match res.as_str() {
            "undef" => (None, false),
            "ERROR" => (None, true),
            _ => (Some(res), false),
        };
        cases.push(Case {
            func,
            args,
            result,
            error,
            line: i + 1,
        });
    }
    assert!(!cases.is_empty(), "{path}: no cases");
    cases
}

/// Runs `f` for every case of `func` in the family and asserts the count is
/// nonzero, so a typo in the function name cannot pass silently.
pub fn each(family: &str, func: &str, mut f: impl FnMut(&Case)) {
    let mut n = 0;
    for c in load(family).iter().filter(|c| c.func == func) {
        f(c);
        n += 1;
    }
    assert!(n > 0, "no fixture cases for {func} in {family}");
}

/// Runs `f`, which may panic, for a case the oracle may have died on:
/// the Rust side must panic exactly when MPU died.
pub fn each_may_panic<T: Display>(family: &str, func: &str, f: impl Fn(&Case) -> Option<T>) {
    let mut n = 0;
    for c in load(family).iter().filter(|c| c.func == func) {
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(c)));
        match (c.error, r) {
            (true, Err(_)) => {}
            (true, Ok(v)) => panic!(
                "fixture line {}: oracle died but Rust returned {:?}",
                c.line,
                v.map(|x| x.to_string())
            ),
            (false, Err(_)) => panic!(
                "fixture line {}: Rust panicked, oracle gave {:?}",
                c.line, c.result
            ),
            (false, Ok(v)) => c.check(v),
        }
        n += 1;
    }
    assert!(n > 0, "no fixture cases for {func} in {family}");
}

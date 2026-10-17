# wx-mpu-real-g01 notes
- Plan: port ExponentialIntegral, LogarithmicIntegral, RiemannZeta from reference/mpu/real.c into src/real/ (one file each), add mod/pub use lines, compat + prop tests, fixture generator section for family real.
- Why: slice of MPU real.c port; siblings add other functions in parallel.
- Found on base: exponential_integral, logarithmic_integral, riemann_zeta already in src/real/mod.rs with doc examples, re-exported by lib.rs; tests/compat_real.rs and tests/prop_real.rs already cover them; fixture real.tsv present. No code change made.
- Ran: tools/test.sh tests/compat_real.rs (10 passed), tools/test.sh tests/prop_real.rs (7 passed).
- Not run: tools/gate.sh (blocked: requires approval), cargo fmt/clippy/MSRV/doc directly (blocked).
- STATUS.md: tools/status.py reports 0 changes; it ticks only snake_case `pub fn` names, while STATUS entries are CamelCase. Not hand-edited.
- Last line: stopped with PARTIAL; gate and the STATUS tick need the maintainer.

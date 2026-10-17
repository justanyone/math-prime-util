# fxc notes: resolve merge conflicts with origin/dev-001
- Plan: resolve conflict markers in benches/combinat.rs, src/combinat/mod.rs, tests/compat_combinat.rs, tests/prop_combinat.rs keeping both sides' intent.
- Then run focused tests with mw-test and commit.
- Resolved all 4 conflicts (union of modules, benches, tests). Next: run mw-test on combinat tests.
- tools/gate.sh needs approval here (denied); cargo fmt not on PATH in this shell. Last line: conflicts resolved, focused tests green, gate not run.

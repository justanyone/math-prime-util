# fxc notes: resolve merge conflict with origin/dev-001
- Plan: resolve conflict markers in benches/combinat.rs, src/combinat/mod.rs, tests/compat_combinat.rs, tests/prop_combinat.rs keeping both sides.
- Then run focused tests via mw-test, then gate if time allows.
- Resolved all 4 conflicts (union of modules, benches, tests). Next: mw-test focused.

# fxc-gf-mpu-rootmod-f02 merge resolution

Will resolve merge conflicts in `src/modular/mod.rs` and `tests/compat_modular.rs` against origin/dev-001, keeping both sides' intent, then fix every open reviewer finding with a regression test.

Why: the branch does not merge cleanly; the project gate requires a clean merge and passing modular tests.

Next: read PLAN.md §§5–8, conflict markers, and any open findings, then edit.
Resolved both conflict hunks: kept cornacchia/divmod (HEAD) and sqrtmod/allsqrtmod/rootmod/allrootmod plus pisano oracle (dev-001). `native_cases` is not in tests/common, so pisano uses `each` and skips moduli above u64. Next: focused compat_modular tests.
- 2026-10-18T03:04:25Z launcher: no commit for 61 s; 92 s elapsed. Files changed so far: README.md, STATUS.md, benches/arith.rs, benches/families.rs, benches/iter.rs, benches/modular.rs, benches/primality.rs, docs/worker-notes/fxc-gf-mpu-rootmod-f02-20261017-x-2202.md, src/arith/chinese.rs, src/arith/exp_mangoldt.rs, src/arith/kronecker.rs, src/arith/liouville.rs.
- 2026-10-18T03:06:57Z launcher: no commit for 61 s; 244 s elapsed. Files changed so far: README.md, STATUS.md, benches/arith.rs, benches/families.rs, benches/iter.rs, benches/modular.rs, benches/primality.rs, docs/worker-notes/fxc-gf-mpu-rootmod-f02-20261017-x-2202.md, src/arith/chinese.rs, src/arith/exp_mangoldt.rs, src/arith/kronecker.rs, src/arith/liouville.rs.
- 2026-10-18T03:07:57Z launcher: no commit for 60 s; 304 s elapsed. Files changed so far: README.md, STATUS.md, benches/arith.rs, benches/families.rs, benches/iter.rs, benches/modular.rs, benches/primality.rs, docs/worker-notes/fxc-gf-mpu-rootmod-f02-20261017-x-2202.md, src/arith/chinese.rs, src/arith/exp_mangoldt.rs, src/arith/kronecker.rs, src/arith/liouville.rs.

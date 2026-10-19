# MPU-GATE-BUILDS-EVERY-CI-FEATURE-SET

Plan: make tools/gate.sh clippy the same feature sets CI builds (default, bigint, gmp, all-features), each as its own step, and comment that the two lists must match.

Why: PR 137 passed the gate and failed the gmp CI job on a dead method that exists only in a narrower feature set.

Next: read PLAN.md sections 5–8, tools/gate.sh, and .github/workflows/ci.yml, then edit.

Edited tools/gate.sh (clippy-default, clippy-bigint, clippy-gmp, then clippy-all-features) and the matching comment in ci.yml.

Next: hand-check a gmp-only dead method `GateProbe::to_f64` in src/arith/sopf.rs. Expect tools/gate.sh to stop at `clippy-gmp`. Then remove the probe and run the green gate.

Hand check attempt 1 stopped at clippy-bigint on clippy::unused_self, not dead_code. The method body did not use `self`. Next: use `self` and run the gate again.

Hand check (probe removed after): tools/gate.sh passed fmt, clippy-default, and clippy-bigint, then stopped at step clippy-gmp. Errors: `struct GateProbe is never constructed` and `method to_f64 is never used` in src/arith/sopf.rs. Later steps did not run. Commit message should state this check.

Next: probe is gone. Run tools/gate.sh and expect `== gate: green`. About 5 minutes of budget may remain after that run starts.
- 2026-10-19T11:39:52Z launcher: no commit for 61 s; 91 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-gate-builds-every-ci-feature-set.md.
- 2026-10-19T11:44:23Z launcher: no commit for 60 s; 362 s elapsed. Files changed so far: .github/workflows/ci.yml, docs/worker-notes/wx-mpu-gate-builds-every-ci-feature-set.md, tools/gate.sh.
- 2026-10-19T11:45:25Z launcher: no commit for 61 s; 424 s elapsed. Files changed so far: .github/workflows/ci.yml, docs/worker-notes/wx-mpu-gate-builds-every-ci-feature-set.md, tools/gate.sh.
- 2026-10-19T11:46:25Z launcher: no commit for 60 s; 484 s elapsed. Files changed so far: .github/workflows/ci.yml, docs/worker-notes/wx-mpu-gate-builds-every-ci-feature-set.md, tools/gate.sh.
- 2026-10-19T11:47:25Z launcher: no commit for 60 s; 544 s elapsed. Files changed so far: .github/workflows/ci.yml, docs/worker-notes/wx-mpu-gate-builds-every-ci-feature-set.md, tools/gate.sh.
- 2026-10-19T11:48:26Z launcher: no commit for 61 s; 606 s elapsed. Files changed so far: .github/workflows/ci.yml, docs/worker-notes/wx-mpu-gate-builds-every-ci-feature-set.md, tools/gate.sh.
Green run 1 was killed during cargo test after fmt, all four clippy steps, and msrv had passed. Restarting tools/gate.sh.
- 2026-10-19T11:50:26Z launcher: no commit for 60 s; 726 s elapsed. Files changed so far: .github/workflows/ci.yml, docs/worker-notes/wx-mpu-gate-builds-every-ci-feature-set.md, tools/gate.sh.

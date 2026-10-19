# MPU-GATE-BUILDS-EVERY-CI-FEATURE-SET

Plan: make tools/gate.sh clippy the same feature sets CI builds (default, bigint, gmp, all-features), each as its own step, and comment that the two lists must match.

Why: PR 137 passed the gate and failed the gmp CI job on a dead method that exists only in a narrower feature set.

Next: read PLAN.md sections 5–8, tools/gate.sh, and .github/workflows/ci.yml, then edit.

Edited tools/gate.sh (clippy-default, clippy-bigint, clippy-gmp, then clippy-all-features) and the matching comment in ci.yml.

Next: hand-check a gmp-only dead method `GateProbe::to_f64` in src/arith/sopf.rs. Expect tools/gate.sh to stop at `clippy-gmp`. Then remove the probe and run the green gate.
- 2026-10-19T11:39:52Z launcher: no commit for 61 s; 91 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-gate-builds-every-ci-feature-set.md.

# MPU-GATE-BUILDS-EVERY-CI-FEATURE-SET

Plan: make tools/gate.sh clippy the same feature sets CI builds (default, bigint, gmp, all-features), each as its own step, and comment that the two lists must match.

Why: PR 137 passed the gate and failed the gmp CI job on a dead method that exists only in a narrower feature set.

Next: read PLAN.md sections 5–8, tools/gate.sh, and .github/workflows/ci.yml, then edit.

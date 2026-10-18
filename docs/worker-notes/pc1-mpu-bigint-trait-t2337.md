Continuation: inspect the existing bigint trait implementation and dependency lockfile.
Resolve the prior dependency blocker without adding dependencies or changing build tooling.
Run focused bigint tests, then the required full gate; record any environment blockers.
Keep changes inside this worktree and leave task readiness unchanged.
Focused `tools/test.sh --lib bigint` cannot compile: offline registry has no num-bigint entry and Cargo.lock omits it; trying dependency resolution needed for tests.

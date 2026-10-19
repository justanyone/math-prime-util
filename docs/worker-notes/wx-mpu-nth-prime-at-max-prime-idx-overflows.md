Task: fix native nth_prime(MAX_PRIME_IDX) u64 overflow in src/count/mod.rs.
Plan: clamp float upper bound to u64::MAX (or compute in u128); add tests for MAX_PRIME_IDX and MAX_PRIME_IDX+1.
Edited: clamp hi to MAX_PRIME-1 before +1; early return for n>MAX_PRIME_IDX and n==MAX_PRIME_IDX. Added 2 tests. Next: run focused tests.
- 2026-10-19T07:06:39Z launcher: no commit for 61 s; 91 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-nth-prime-at-max-prime-idx-overflows.md, src/count/mod.rs.
Gate (tools/gate.sh) and cargo fmt/clippy were blocked by the sandbox approval prompt; cargo not on PATH in this shell. Only focused lib tests ran (19 pass).
Remaining: run tools/gate.sh, fmt, clippy; add MAX_PRIME_IDX-1 test (not done: needs prime_count near 1.8e19, too slow for a default test).

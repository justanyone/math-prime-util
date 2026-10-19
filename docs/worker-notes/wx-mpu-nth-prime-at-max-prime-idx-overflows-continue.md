Continue the nth_prime(MAX_PRIME_IDX) overflow fix. The native upper bound `(fnn*(ln+lnln)) as u64` overflows for the largest index. Clamp or use u128 so nth_prime(MAX_PRIME_IDX) is MAX_PRIME and the next index is None.

Prior run claimed the clamp is in src/count/mod.rs and a MAX_PRIME_IDX test exists. Gate, fmt, and clippy did not run. A MAX_PRIME_IDX-1 test may still be missing.

Next: read the bound and the tests, add the missing case if a fast path exists, then run tools/test.sh and tools/gate.sh.
The clamp and the MAX_PRIME_IDX early return were already in src/count/mod.rs. The float bound at MAX_PRIME_IDX-1 is about 1.89e19, above u64::MAX.
Added a walk via prev_prime when that clamped bound is MAX_PRIME, plus a test that calls the index twice. Next: focused test, then gate.
Focused `tools/test.sh --lib max_index`: 4 passed, including nth_prime(MAX_PRIME_IDX), the index below it, and MAX_PRIME_IDX+1. Starting tools/gate.sh. About five minutes of budget may remain, so gate is the last step.
- 2026-10-19T07:09:21Z launcher: no commit for 61 s; 91 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-nth-prime-at-max-prime-idx-overflows-continue.md.
- 2026-10-19T07:10:21Z launcher: no commit for 60 s; 151 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-nth-prime-at-max-prime-idx-overflows-continue.md.
- 2026-10-19T07:11:21Z launcher: no commit for 60 s; 211 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-nth-prime-at-max-prime-idx-overflows-continue.md.
- 2026-10-19T07:14:23Z launcher: no commit for 60 s; 393 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-nth-prime-at-max-prime-idx-overflows-continue.md, src/count/mod.rs.
- 2026-10-19T07:15:24Z launcher: no commit for 61 s; 454 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-nth-prime-at-max-prime-idx-overflows-continue.md, src/count/mod.rs.
- 2026-10-19T07:16:24Z launcher: no commit for 60 s; 514 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-nth-prime-at-max-prime-idx-overflows-continue.md, src/count/mod.rs.
- 2026-10-19T07:17:24Z launcher: no commit for 60 s; 574 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-nth-prime-at-max-prime-idx-overflows-continue.md, src/count/mod.rs.

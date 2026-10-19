Continue the nth_prime(MAX_PRIME_IDX) overflow fix. The native upper bound `(fnn*(ln+lnln)) as u64` overflows for the largest index. Clamp or use u128 so nth_prime(MAX_PRIME_IDX) is MAX_PRIME and the next index is None.

Prior run claimed the clamp is in src/count/mod.rs and a MAX_PRIME_IDX test exists. Gate, fmt, and clippy did not run. A MAX_PRIME_IDX-1 test may still be missing.

Next: read the bound and the tests, add the missing case if a fast path exists, then run tools/test.sh and tools/gate.sh.
- 2026-10-19T07:09:21Z launcher: no commit for 61 s; 91 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-nth-prime-at-max-prime-idx-overflows-continue.md.

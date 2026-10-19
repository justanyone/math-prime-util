Add bigint sequence and product APIs without changing native APIs.
Read PLAN sections 5 to 8 and use the MPU reference algorithms.
Add a pure-Perl oracle family and native comparison tests.
This checkout runs on wowbagger; requested aragog or wonko is not available here.
Keep all work in this worktree. Do not commit or use the network.
- 2026-10-19T16:06:07Z launcher: no commit for 61 s; 92 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-bigint2-sequences-and-products.md.
Implemented all fourteen APIs, using signed Lucas results and Round Robin Frobenius tables. Next add offline oracle and native comparison tests.
The vendored MPU 0.75 loads offline. Added the bigint_sequences generator and replay test; generated fixtures via PERL5LIB=reference/mpu/lib to avoid the network bootstrap.
Added native comparison properties for all APIs, wide arithmetic identities, repeat/concurrent call checks, and invalid-input tests. Rust execution is blocked by the explicit aragog/wonko-only build rule on this wowbagger checkout.

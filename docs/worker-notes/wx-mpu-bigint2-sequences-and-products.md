Add bigint sequence and product APIs without changing native APIs.
Read PLAN sections 5 to 8 and use the MPU reference algorithms.
Add a pure-Perl oracle family and native comparison tests.
This checkout runs on wowbagger; requested aragog or wonko is not available here.
Keep all work in this worktree. Do not commit or use the network.
- 2026-10-19T16:06:07Z launcher: no commit for 61 s; 92 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-bigint2-sequences-and-products.md.
Implemented all fourteen APIs, using signed Lucas results and Round Robin Frobenius tables. Next add offline oracle and native comparison tests.
The vendored MPU 0.75 loads offline. Added the bigint_sequences generator and replay test; generated fixtures via PERL5LIB=reference/mpu/lib to avoid the network bootstrap.
Added native comparison properties for all APIs, wide arithmetic identities, repeat/concurrent call checks, and invalid-input tests. Rust execution is blocked by the explicit aragog/wonko-only build rule on this wowbagger checkout.
About five minutes remain. The oracle generated 272 rows without errors outside intended invalid inputs. Added full-u64 powersum and floor_sum properties. Formatting is complete; build, gate, and coverage require an allowed host.
Added above-2^64 Lucas indices for the practical P=1,Q=0 case; the fixture now has 275 rows. All fourteen names have implementations. Fibonacci/Lucas-number indices and prime-product bounds above 2^64 have impractical output sizes; small in <<cut 36 chars by kit>>
Validation: offline MPU fixture generation, rustfmt --check, and git diff --check pass. tools/test.sh, tools/gate.sh, and coverage were not run: this host is wowbagger and the task permits builds/tests only on aragog or wonko. No network, g <<cut 39 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 27 bytes)
Final review restored MPU's direct-summation powersum branch when k >= n, avoiding a large Stirling table for small n and large k. Added its 2^128+1 oracle case; 276 rows now replay. Formatting and diff checks pass. Rust gate and coverage r <<cut 23 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed -1 bytes)

Implement the four assigned Goldbach and Chen functions in separate family files.
Read PLAN sections 5 to 8 and preserve the C algorithms and bounds.
Use committed oracle fixtures; leave tools, fixtures, STATUS.md, and crate root unchanged.
Run focused offline tests and the repository gate; record any local blockers.
Confirmed Goldbach scalar/list fixture shapes and the double-sieve bounds; Chen lives in almost_primes.c, not util.c.
The factor module is empty, so Chen needs a private semiprime predicate following util.c and a local factor fallback.
Added four function files, module exports, oracle/property/concurrency tests, and Criterion cases; next run focused tests.
Goldbach uses two bounded boolean sieve segments; Chen retains C's search and native limit, with extended trial division because factor_one is not yet ported.
Focused offline compat/property tests are running; gate is queued behind the build lock. Formatting completed.
Focused tests passed: all four oracle tests and all three algebraic/concurrency tests. Gate passed fmt and pedantic clippy; MSRV build is running.
Changed Goldbach output ordering to the C reversal rather than sorting; removed unnecessary overflow branches using segment bounds.
Review found existing lucky-family tests and benchmarks had been overwritten. Restored their exact baseline content and appended the assigned cases.
The focused family unit tests passed, including native Chen limits and exhaustive semiprime checks. Added the C square shortcut and segment-overflow boundary tests.
The first offline gate printed == gate: green. Final rerun covers restored lucky tests and the last square/overflow tests; baseline diff has no whitespace errors.
Final offline gate printed == gate: green: fmt, pedantic clippy, MSRV 1.85 build, all tests, runnable docs, and documentation passed.
Finished all four requested APIs, tests, and benchmarks; preserved lucky-family coverage. One named bigint fixture is skipped by the native API; Chen's private semiprime fallback uses extended trial division pending factor_one. No dependenc <<cut 39 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 15 bytes)

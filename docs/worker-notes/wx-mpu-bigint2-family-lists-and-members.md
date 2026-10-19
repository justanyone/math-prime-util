Implement bigint family lists with existing primality and factor building blocks.
Keep native signatures and add the new bigint::families module.
Generate pure-Perl oracle rows offline and check native agreement.
Record impractical global-count families and any unavailable checks here.
- 2026-10-19T15:53:09Z launcher: no commit for 61 s; 91 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-bigint2-family-lists-and-members.md.
Implemented 16 bigint APIs; five global-scan families are impractical above 2^64. Host is wowbagger, so requested aragog/wonko validation is unavailable locally.
Added generator, fixture replay, native property checks, writer-error, invalid-offset, repeated-call and thread tests. Next: generate oracle offline and run focused checks.
Focused checks started; the vendored reference runs as an offline pure-Perl oracle without bootstrapping .perl-local. No dependency or tooling change.
The first focused compile found the native minimal_goldbach_pair returns only the smaller prime; corrected the bigint return type to match. Oracle generation still runs.
Native property checks pass (32 generated intervals plus edge/error/concurrency tests). Initial oracle run exceeded a practical wait and was stopped; enabled progress output and a bounded retry to identify the slow operation.
Oracle progress reaches minimal_goldbach_pair quickly; the following range operations are the likely slow stage. Gate identified test-style lint failures; fixed must-use and empty assertions.
The pure-Perl omega list completed; powerful_numbers now takes the long wait. The Rust short-range factor implementation remains practical. Preparing smaller oracle cases for that path.
Keep powerful_numbers native too: PP enumerates global power bases even for a 25-integer range at 2^64, and did not finish within the bounded oracle run. Removed that API and its tests rather than replace its enumeration algorithm with trial factoring.
Commit-message exceptions for launcher: goldbach_pairs scans half the input; lucky_numbers allocates a global lucky sieve; is_lucky builds a sieve near n/log(n); ramanujan_primes builds the initial sequence through prime_count(high)/2; is_ramanujan_prime calls that list; powerful_numbers enumerates global power bases. These six stay native.
Oracle has reached the 2^70 cases. Fixed the remaining fixture-helper ownership lint. Expanded random native intervals through 10000; all implemented APIs participate.
About five minutes remain. Fifteen APIs remain after the allowed practicality exclusions. Fixed an incomplete test removal, and restarted focused replay and the full gate. Coverage tooling is not on PATH.
Oracle generation completed both intervals. All 30 rows replay, and all four property/edge tests pass. Added an exact row-count check so an interrupted generation cannot pass with a partial fixture. STATUS already marks every task name complete, so it needs no change.
All four clippy feature configurations passed. Added deterministic 0..12 checks for prime-power and Goldbach boundaries and zero-depth sieve behavior. Gate is building MSRV.
All 30 oracle rows match; no wrong Perl values were found. Coverage binaries are absent, so 95 percent line coverage remains unmeasured. Full offline suite and gate are still running.
MSRV build passed. Gate and the required final tools/test.sh suite are executing; the focused suite already passed on the complete fixture.
MSRV is green. The final full suite is still compiling, and gate waits for its build lock. No further implementation changes are planned; coverage measurement and the requested remote-host run are unavailable.
Full compilation finished and the gate now runs compatibility tests. No oracle mismatches or regressions have been reported so far.
The full gate continues through existing count compatibility tests. All new family tests passed. The only unmeasured acceptance check is touched-module line coverage; no LLVM coverage tools are installed.

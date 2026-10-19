Implement bigint random APIs using the shared seeded random stream and existing bigint primality.
Read PLAN sections 5 through 8 before editing code. Preserve native signatures.
Add bigint_random oracle generation, fixture replay, and property checks. Run offline checks.
Large factor sampling may be impractical in pure Perl; record measured limits and blockers here.
- 2026-10-19T15:42:59Z launcher: no commit for 61 s; 91 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-bigint2-random-primes.md.

Read PP sampling and Gordon algorithms. STATUS already ticks all twelve native names; no status edits needed. Host is wowbagger, not requested aragog or wonko; only local offline checks are possible.
Implemented eleven bigint APIs, including Gordon strong primes and Kalai factored sampling. Local oracle bootstrap is absent; checking vendored PP without network next.

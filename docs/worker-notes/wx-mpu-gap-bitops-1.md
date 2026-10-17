Read PLAN.md sections 5 to 8 before implementation.
Port the five native u64 bit operations from MPU with source citations.
Add unit tests and algebraic property tests without dependencies or fixtures.
Run focused tests and the offline gate, then update the generated checklist.
The five names occur only in reference/mpu/TODO, not in C, Util.pm, or PP.pm; docs record this source gap.
Implemented direct u64 operators, edge tests, and five algebraic property tests; next run focused tests and the gate.
Focused bits_ops1 tests passed; the status generator changed exactly the five requested ticks.
Added Criterion benchmarks. Gate passed fmt, clippy, and MSRV and is running all tests offline.
State, concurrency, validation, and duplicate-launch risks do not apply to these pure total u64 operations; edge tests cover the full bit width.
All five unit tests and all five 4000-case property tests pass; the benchmark target compiles offline.
The generated checklist is current and git diff --check passes. Waiting for the gate's existing compatibility tests.
Gate remains active in the existing counting compatibility tests; no failure has been reported.
- 2026-10-17T09:42:31Z launcher: no commit for 61 s; 242 s elapsed. Files changed so far: STATUS.md, benches/bits_ops1.rs, docs/worker-notes/wx-mpu-gap-bitops-1.md, src/bits/bitand.rs, src/bits/bitandnot.rs, src/bits/bitnot.rs, src/bits/bitor.rs, src/bits/bitxor.rs, src/bits/mod.rs, src/lib.rs, tests/prop_bits_ops1.rs.
Existing nth_prime compatibility test passed; waiting for prime_count and the rest of the offline gate.

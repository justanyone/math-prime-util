Port the four arithmetic functions into separate module files.
Match the MPU C algorithms and reuse the existing fixture family.
Add boundary, algebraic, and repeated-call tests; run focused tests and the gate offline.
Keep STATUS.md, tools, fixtures, and crate exports unchanged as required by the task.
Existing implementations and oracle tests were present; identified missing C table fast paths, memory bounds, and prime-power detection. Splitting the functions and retaining shared sieve dependencies.
Ported the four files with C small tables, 32 KiB Moebius segments, prefix memory clamps, and prime-power detection. Added summatory difference, boundary, and concurrent repeated-call checks plus benchmarks; focused offline tests started.
Focused tests are still building; the gate has started offline. Reviewing the Mertens prefix storage against the C short-array and bounded cache before final validation.
Matched C storage more closely: signed 16-bit prefix entries and an eight-probe bounded cache with collision tests. Gate waits on the Cargo build lock; focused tests have not returned yet.
All requested implementations, docs, oracle coverage, properties, and benchmarks are written. Offline checks remain active, with Cargo compilation/locking delaying their output; no network or external repository changes were used.
- 2026-10-17T18:46:01Z launcher: no commit for 60 s; 307 s elapsed. Files changed so far: benches/arith.rs, docs/worker-notes/wx-mpu-totients-f02.md, src/arith/exp_mangoldt.rs, src/arith/liouville.rs, src/arith/mertens.rs, src/arith/mod.rs, src/arith/sumliouville.rs, tests/prop_arith.rs.

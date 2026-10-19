Port the missing bigint modular helpers with the MPU algorithms and existing bigint blocks.
Add a pure Perl oracle family and native agreement properties.
Keep native APIs and dependencies unchanged. Run focused offline tests and the gate.
The checkout is on wowbagger, not aragog or wonko; record host restrictions before tests.
- 2026-10-19T15:57:14Z launcher: no commit for 61 s; 91 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-bigint2-modular-helpers.md.
Implemented ten helpers using existing bigint operations; next implement all roots and oracle tests.
The local Perl environment is absent. Use the vendored MPU pure Perl module offline instead of the network bootstrap.
Added allrootmod with prime-power lifting and bounded output, and the eleven-name oracle generator.
Generated 51 fixture rows offline from vendored MPU; added replay, native properties, edge and root-lifting tests.
About five minutes remain. Verification on aragog/wonko is unavailable from this sandbox; formatter and gate availability are next.
Focused tests are compiling; gate waits on the build lock. is_qr now tests root existence without enumerating roots.
The first focused build found a factor API tuple mismatch and a closure lifetime error; both are fixed.
Offline verification is running locally on wowbagger because the named build hosts cannot be reached under this task's no-SSH rule. This does not satisfy the requested host check.
All eleven functions have docs and runnable examples. STATUS already checks every task name, so it needs no edit.
The focused test process still holds the build lock. No gate-green or coverage result is available yet.

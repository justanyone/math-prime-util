Read PLAN sections 5–8 and the required scripts before edits.
The four APIs already exist in src/iter/mod.rs; reassess their algorithms and tests.
Move only this task's APIs into separate files and preserve the existing public API.
Keep tools, fixtures, STATUS.md, and the crate root unchanged as explicitly required.
Run focused offline tests and the gate; record any environment blockers.
Verified XS permutation/derangement transitions, product snapshots, and PP multiset recursion. Moved the four existing ports into separate files and made fixture tests exercise closures and early exit.
Added independent multiset/product laws, derangement recurrence, clone/exhaustion, nested callbacks, unwind recovery, and concurrent traversal checks. Input validation remains enforced by typed Rust arguments; no Perl validation inputs can  <<cut 17 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed -7 bytes)
The iterator fixture and property commands are compiling offline. Added Criterion workloads for derangements, multiset permutations, and products; the permutation workload already existed. No dependencies or fixture changes were needed.
Confirmed the existing iter family has committed oracle cases and a generator section, so neither restricted file needs edits. Added explicit empty/singleton and duplicate-product boundary tests. The gate is waiting for the focused build lo <<cut 3 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed -20 bytes)
All implementation and test edits are complete. Offline verification is still compiling; queued a final focused run after the latest boundary tests. No test failure has been reported yet.
The first compile found fixture-test functions shadowing the four imported APIs. Qualified calls with math_prime_util to fix it. Property tests and benchmark targets compiled successfully; rerun focused tests and the gate next.
Focused verification passed: all 19 compatibility tests and all 20 iterator property/state tests. Criterion targets compiled. Gate formatting and clippy passed; the Rust 1.85 offline build is running.
The focused tests pass and git diff --check is clean. Waiting for the required MSRV/full-suite/documentation gate; no source changes remain pending.
The Rust 1.85 build passed offline. The gate is now compiling the full all-features test suite; formatting, pedantic clippy, fixture checks, and iterator properties have passed.

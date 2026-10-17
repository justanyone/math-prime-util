Port the four assigned Ramanujan functions from MPU 0.75 in separate files.
Reuse existing prime counting and sieve primitives, with the C bounds and tables.
Use the existing families oracle fixtures; leave tools, fixtures, and STATUS unchanged.
Run focused offline tests and the gate, and record any checkout limitations here.
The checkout lacks the prime-count and nth-prime bound APIs required by C; private supporting bounds are needed in this slice. The codegraph command exceeded the requested output limit; subsequent reads are bounded.
Implemented separate public files and private Axler/nth-prime bounds. The numerical helpers currently use f64; the u64 maximum oracle cases may expose long-double differences.
Focused fixture and property tests were launched. The first build exposed tuple delimiter mistakes in the bound tables; fixing syntax before numerical checks.
The initial write was truncated by the tool transport, including long table lines. Rewrote those tables with short lines and removed the injected truncation notice. Focused tests are rebuilding offline.
Exact-count oracle cases pass. Generation exposed a first-index offset, now corrected. Two u64::MAX floating-point cases differ from the C oracle; they need extended precision rather than fixture-specific constants.
The public APIs, compatibility tests, property tests, and Criterion source are written. No dependency, fixture, tools, STATUS, or git metadata was changed. Lint fixes and focused reruns are in progress; the large-input numerical mismatch re <<cut 11 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed -13 bytes)
All four property tests pass, including repeated and concurrent calls. Generation and exact-count fixture tests pass; lower and approximate counts each fail only at u64::MAX. A shared /tmp log name collided with another worker; the gate now <<cut 26 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 2 bytes)
Private bound edge tests and all 197 doc examples pass. Added more C bound thresholds to the unit tests and changed generation to invert the C lower nth bound for its upper index. Two active oracle failures at u64::MAX remain: lower 1418854 <<cut 96 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 72 bytes)
The gate now reaches test-only pedantic lint checks; fixing literal formatting and empty-vector assertions. The full offline suite was launched once. STATUS stays untouched as required by the task-specific read-only instruction.
All known pedantic lint findings in the new code have been fixed. Final gate and full-suite runs are still active. The remaining implementation work is extended-precision prime-count estimation and bounds for the maximum native input; no or <<cut 38 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 14 bytes)
Final gate has passed formatting, Clippy with warnings denied, and the Rust 1.85 build. It has reached the tests. This slice must remain PARTIAL until the two maximum-input oracle discrepancies are fixed and the gate reports green.

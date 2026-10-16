Implement the vector and sorted integer set APIs from the MPU reference.
Use the existing vec oracle fixtures without changing tools or fixture data.
Add compatibility cases, algebraic tests, and hot-function benchmarks.
Run focused offline tests and the required gate; record any blockers here.
Reference review: preserve reversed subset argument order and sorted merge set operations; skip unsupported bigint fixture inputs explicitly.
Read callback, extraction, frequency, and sort reference paths; implementing local state only, with explicit boundary and duplicate tests.
Implemented all public API bodies and direct MPU quicksort/radix sort paths. Preparing fixture dispatch and boundary checks next.
Implementation is now written. Initial generator was truncated by the command interface; replaced it with bounded source lines. Adding oracle tests.
About five minutes remain: focused oracle test started; added algebraic, concurrency, duplicate-call, callback, and validation regressions plus benchmarks.
Focused tests found mex validation and Perl scalar spelling differences. Fixed validation; named unrepresentable string skips. Property tests passed.
All 47 compatibility tests passed. All 7 property/regression tests passed. Fixed Clippy Copy and byte-mask findings; running gate again.
Gate cleared production Clippy checks. Fixed benchmark macro lint and focused test formatting findings; next gate run includes MSRV and all docs.
Final gate printed == gate: green (fmt, pedantic Clippy, MSRV 1.85, all tests, docs). Final tools/test.sh passed offline. All 47 APIs, fixtures tests, laws, and benchmarks are written; no tooling, fixture, STATUS, or git metadata edits.
Remaining limits are named fixture skips for bigint, signed unsigned-only inputs, and Perl string scalar spellings. Reference vector callbacks and helpers reside in XS.xs; docs cite those entry points rather than nonexistent util.c vector functions. Finished without network or git writes.

Read PLAN.md sections 5 to 8 and the named factor paths.
Add test-only cases for the open factor mutation survivors.
Record caught mutations and equivalent mutations with reasons in this PR text.
Use offline checks. The requested aragog run is unavailable without remote access.
Added direct prime_factors inputs and Lehman odd/square cases; host is wowbagger, not aragog. Next run focused tests and local mutations.
Focused tests are compiling offline. All named public functions are already checked in STATUS.md; no checklist edit is needed.
Listed the exact mutations. Started a local nice-19, two-job run with --re and offline library tests. Added p-1 replay cases.
Independent modular checks found a Lehman division witness. The addition mutation in its residue is equivalent modulo four; focused compilation is still running.
Corrected a Lehman expectation after the focused test failed. Stopped the mutation run to avoid counting that failure as a kill; p-1 replay test passes.
Found p-1 checkpoint witnesses with three prime factors. Two-factor inputs hid the gap because replay selected the same divisor. Added exact split assertions.
Focused library tests pass: 71 tests. The gate has passed fmt, clippy and the MSRV build and is running the full suite offline.
Found a HOLF shift witness in the multiplier-30 interval. Added a two-round private splitter assertion. Brent retry searches found no witness; those mutations remain open.

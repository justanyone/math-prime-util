Port the four assigned Ramanujan functions from MPU 0.75.
Keep each public function in its own file and preserve the C bounds and tables.
Use the existing families fixtures without changing tools or fixture data.
Run focused offline tests and the repository gate; record any blockers here.
Read the C bounds and Noe sieve; private dependencies are absent and will be included within assigned files.
Implemented the lower-bound tables and Noe range sieve. Next: interpolation and fixture tests.
All four functions and compat/property tests are written. Focused offline tests are running; zero, repeat and concurrent calls are covered.
Added threshold tests for the C parameter tables, prime-bound overflow sentinels, and count-bound regions.
First focused build found a u8 primality predicate in the test; corrected it. Clippy numeric-table formatting and checked indices are corrected.
All nth-function oracle cases pass. The max-u64 count case exposed f64 Li error; replacing that private path with compensated extended precision.

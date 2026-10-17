Port the four native random prime functions from MPU 0.75.
Keep each function in its own file and preserve the shared module exports.
Use the existing offline oracle fixtures and portable random source.
Run focused tests and the required gate; record unavailable checks here.
STATUS.md, tools, fixtures, and git metadata are read-only for this task.
Read the native MPU dispatch: strong primes start at 128 bits; Maurer and proven use random_nbit_prime through 64 bits. Implementing shared native sampling and native Small certificates.
Implemented all four APIs in separate files. Native generation keeps MPU bit bounds and index tables; certificates use the exact Small format. The fixture has no proven or Maurer rows, so test their native alias against random_nbit_prime rows.
Added oracle alias tests, certificate and primality properties, all native sizes, invalid-input stream preservation, repeat launches, thread isolation, and three benchmarks. Focused offline tests are running.
Focused offline random tests passed: 17 compatibility tests and 16 property/unit tests. The required gate is running; no fixture, tooling, dependency, or STATUS changes were made.
The required gate printed == gate: green (format, pedantic clippy, MSRV 1.85, tests, doctests, docs). Running the requested final tools/test.sh whole-suite pass offline.
Finished: focused tests, final offline whole-suite tools/test.sh, and tools/gate.sh all passed. All four native APIs, examples, compatibility/property tests, and benchmarks are complete; bigint support remains deferred as specified. No network or git writes were used.

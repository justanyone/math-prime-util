Implement the MPU ChaCha20 stream and seeded random APIs in src/random/.
Use the committed random fixture as the oracle; do not change tools or fixtures.
Check stream state, thread isolation, bounds, and repeated seeds with focused tests.
Use std OS input for entropy and report any platform or offline gate blockers.
Read seed expansion, ChaCha core, and rejection sampling; fixture includes mixed draws and buffer boundaries. Implementing thread-local streams and Unix OS entropy without dependencies.
Implemented the full native API, PCG seed expansion, 1024-byte ChaCha buffer, and rejection sampling. Next: fixture dispatcher, stream-law tests, and offline focused tests.
Focused compatibility and property tests are running offline. Native APIs skip only named signed/bigint fixture cases; adding cipher XOR vectors and mixed-call coverage next.
The first run found a test setup error: the BLAKE float case consumes seven bytes before irand. Corrected it; adding cipher vectors, mixed draws, and the C byte-count limit before the gate.
All 13 compat tests and 12 property/state tests pass offline, including all six cipher fixtures and mixed sequences. Running the required gate; signed and bigint inputs are explicitly skipped under the native API rule.
Gate first stopped on doc-markdown lint for ChaCha20. Fixed the documentation spelling and reran; reviewing offline MSRV and full-suite outcomes next.
Finished: tools/gate.sh printed == gate: green after formatting, pedantic clippy, MSRV 1.85, full offline tests, runnable examples, and docs. No protected files or git metadata were changed; launcher handles commits.
Remaining limits: signed/bigint fixture inputs are named skips; std-only entropy uses MPU's Unix device path and returns an I/O error on platforms without those devices. Windows entropy could not be tested on this Unix host.

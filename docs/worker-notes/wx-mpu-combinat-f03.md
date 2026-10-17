Port the four assigned combinat functions from the MPU C reference.
Keep each function in its own file and preserve the shared module exports.
Use the existing combinat fixtures without changing tools or fixture data.
Run focused offline tests and the required gate; record any blockers here.
Read PLAN sections 5–8 and the C algorithms. Preserve the integer-complexity cache and power-sum bounds; skip named signed factorial cases in the unsigned API.
Added four function files, fixture checks, algebraic properties, concurrency/reset checks, and criterion source. Existing fixtures contain bigint outputs, mapped to native overflow None.
Focused compatibility/property tests and cache unit tests are running offline. Started the required gate to catch formatting, lint, MSRV, and documentation issues.
Reviewed cache growth tests and used the known value complexity(3^10) = 30 across the growth boundary. The gate is waiting for the focused test build lock.
Focused tests found MPU exposes cache reset at zero as undef, so integer_complexity must return Option. Clippy also requests div_ceil. Correct both and rerun; the cache-growth expectation was corrected before its earlier build finished.
Matched the XS signed-native bound for integer_complexity and its zero-to-undef result. Updated property expectations for Option; rerunning focused checks after that signature change.
Cache unit tests pass, including growth and concurrent reset/repeat calls. Fixed benchmark lint findings; the focused fixtures/properties and the updated gate are running.
All four fixture tests and five property/boundary tests pass offline. Corrected the final checked-conversion lint; rerun the gate against this source.
Final source passes the focused oracle and property checks. The required gate and the single whole-suite tools/test.sh run are in progress; no fixture, tooling, dependency, root export, or STATUS changes were made.
Gate formatting, clippy, and MSRV build passed. The full suites reached existing sieve tests; waiting for those and the documentation steps to finish.
Finished: focused fixtures/properties, cache unit tests, and the single full tools/test.sh run passed offline. tools/gate.sh printed == gate: green (fmt, clippy, MSRV, tests, doctests, docs). Platform-matrix and coverage measurement remain  <<cut 39 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 15 bytes)

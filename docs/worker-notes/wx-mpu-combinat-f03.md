Port the four assigned combinat functions from the MPU C reference.
Keep each function in its own file and preserve the shared module exports.
Use the existing combinat fixtures without changing tools or fixture data.
Run focused offline tests and the required gate; record any blockers here.
Read PLAN sections 5–8 and the C algorithms. Preserve the integer-complexity cache and power-sum bounds; skip named signed factorial cases in the unsigned API.
Added four function files, fixture checks, algebraic properties, concurrency/reset checks, and criterion source. Existing fixtures contain bigint outputs, mapped to native overflow None.
Focused compatibility/property tests and cache unit tests are running offline. Started the required gate to catch formatting, lint, MSRV, and documentation issues.

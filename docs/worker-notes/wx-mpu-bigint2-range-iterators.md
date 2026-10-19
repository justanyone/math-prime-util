Plan: add bigint range iterators, follow existing bigint and fixture patterns, and document limits if the Perl oracle is unavailable.
First inspect MPU C routines and repository test conventions. Implement feature-gated BigUint APIs with docs and focused coverage.
Then add fixture generator/test/property files where the offline checkout permits.
Run focused tests and gate; record any environment blockers here.
Focused property and compat test commands returned successfully. The gate is blocked by concurrent cargo activity; next inspect gate output and fixture feasibility.
Final review: implementation artifacts exist, but oracle rows are not generated and compat replay intentionally cannot pass without them; the fixture generator API also needs confirmation. Gate/test processes repeatedly waited on the shared build lock, so compile status and coverage remain unverified.

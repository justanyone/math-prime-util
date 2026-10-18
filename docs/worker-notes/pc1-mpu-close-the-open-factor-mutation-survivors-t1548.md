Continue the partial factor mutation task without changing factory files or readiness status.
Read PLAN sections 5–8; inspect the prior tests, retry paths, dispatcher and HOLF.
Add deterministic regression tests for observable survivors and document equivalent mutants.
Run focused factor tests and the required gate, recording remaining validation limits.
- Inspection: three named survivors lack observable edge cases; no mutation outcome artifact is present in the checkout. Preparing short-budget HOLF and deterministic retry regressions.
- Added the missing HOLF 30-multiplier case and injectable private Brent attempts to test dispatcher budgets and retry sequencing deterministically; focused factor tests are running.
- HOLF case constructed algebraically: 60*n is a difference of squares, while the first 30*n iteration misses. This distinguishes the exact open cutoff mutation.
- Required gate started after formatting. Focused factor suite still running; mutation outcomes from the earlier factory run are not present locally.
- Recovered prior notes from Git history: no report identifies the unnamed survivors. Prior claimed HOLF witness was withdrawn, so this run will verify the actual shifted code rather than rely on the construction alone.
- Added explicit C-budget assertions at 45, 46 and 64 bits, plus four consecutive retry attempts. Independent modular calculation corrected the real fallback expectation to factor 31.
- Mutant listing confirms the remaining named operators now occur in dispatcher_with_brent (multiply to add), retry_brent (add-assign to subtract-assign), and HOLF cutoff (right shift to left shift). No mutation run is claimed yet.
- First focused run: 73 passed, one failed because its binary predated the corrected Brent factor expectation (31). Corrected source is on disk; rerunning the focused suite. The new HOLF and dispatcher tests passed.
- Validation continues: gate is compiling clippy dependencies; no lint errors reported. Public function checklist already covers these implementations, so STATUS.md remains unchanged.
- Started the single required tools/test.sh whole-suite run after finishing source edits. Gate and tests use Cargo locking; no mutated build shares this target directory.
- Gate fmt and clippy passed; gate advanced to the MSRV build. Waiting for focused and full-suite results before recording final validation status.
- Focused validation passed: tools/test.sh --lib factor reports 74 passed, 0 failed. Three new regressions cover HOLF round sensitivity, dispatcher budgets and retry sequencing; unnamed survivors remain unclassified without their report.

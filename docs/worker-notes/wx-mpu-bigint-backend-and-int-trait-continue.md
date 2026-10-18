# Continue: bigint backend and Int trait

Previous run left the bigint feature written but unbuilt: `num-bigint` was missing offline. This session checks the existing sources, tries an offline build and test, and fixes fmt, clippy, and coverage gaps that the compiler reports.

Plan: read PLAN.md sections 5–8 and the bigint module, run `tools/test.sh --features bigint --lib bigint`, then `tools/gate.sh` if the crate builds. No new dependencies beyond the named optional ones. Do not mark READY.

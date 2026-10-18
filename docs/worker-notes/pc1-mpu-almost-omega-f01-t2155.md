Continuation: inspect the existing almost/omega family implementation and prior changes.
Read PLAN.md sections 5–8 and preserve the current task scope and STATUS checklist.
Run focused family tests, then the required gate; repair any acceptance failures found.
Do not mark READY or write git metadata; record verification evidence and remaining blockers here.
The gate now executes: fmt and clippy passed; MSRV is waiting for the focused test build lock. Existing almost-prime APIs have C citations and examples; checking their tests before ticking completed entries.
Focused compat_families and prop_families passed. Ticked only the four implemented almost-prime APIs in STATUS.md. Gate passed MSRV and is running the full suite; no code changes have been necessary.
Required tools/gate.sh exited 0 and printed == gate: green: fmt, clippy, MSRV, full tests, 439 doctests, and documentation passed. Final tools/test.sh run is still in progress; no READY marker was added.

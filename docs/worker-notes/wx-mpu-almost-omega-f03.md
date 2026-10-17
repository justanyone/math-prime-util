Port the four assigned almost/omega functions in separate module files.
Read PLAN sections 5–8 and the named scripts before editing code.
Keep tooling, fixtures, STATUS.md, and sibling functions unchanged.
Use existing offline fixtures and property tests; finish with the gate.
Dependencies reviewed: almost upper needs private count bounds and interpolation; omega uses prime-power counts and recursive counts.
Implemented the four functions and private C helpers; next add fixture and algebraic tests before focused offline runs.
Focused fixture/property tests are running offline. Added hot-function benchmarks; no tools, fixture, or manifest edits.
Found pre-existing family test files in the diff; restored their contents and isolated the new tests in a nested module.
Reviewing native overflow and test integration while the first offline build runs; all added APIs have runnable examples.
Fixed overflow when locating the first sieve multiple near u64::MAX. The gate is queued behind the focused build lock.

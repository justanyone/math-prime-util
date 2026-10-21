Continuing the existing estimate mutation regression tests; no existing test files will be edited.
Check remaining mutants against public callers and MPU reference algorithms, distinguishing wrong bounds from equivalent or non-tight changes.
Add reachable regression cases in tests/mutants_estimates.rs, then run focused tests, targeted cargo-mutants, and the required gate.
Do not mark this task READY; record unresolved mutants and environment limitations explicitly.
Reviewed MPU lucky fits and public call domains: the n=7 simple_approx comparison is unreachable; harmonic tiny-term comparison appears f64-equivalent. Preparing targeted mutation baseline.
Added lucky fit boundary sandwich tests through 4e9 and independent fourth-almost-prime checks; focused tests and all 13 requested mutations are next.
Focused regression suite passes all 11 tests. cargo-mutants rejected --offline; restarted with CARGO_NET_OFFLINE=true. Required gate is running independently.
MPU inverse_interpolate.c::interpolate_with_approx confirms that interpolation is only a starting guess followed by exact count correction; comparing branch reachability before claiming equivalent mutants.
Formal branch argument: finite interpolation bounds were sampled only with count error >=20,000, so a bracket spanning both sides has width >=40,000. The span<=19,999 midpoint arm is unreachable; other guess changes require runtime/performance evidence, not a different exact-answer assertion.
Resolution detail (source references): lucky_numbers.c::_simple_lucky_count_approx is only reached from the Rust simple_lower when n>9000. Its n<7 versus n<=7 mutant cannot affect a public call; directly at 7 the mutant would incorrectly re <<cut 21 chars by kit>>
Resolution detail: PP.pm::harmreal tiny-term cutoff discards terms below 1e-44. For native n>2002, the accumulated correction starts at magnitude at least 1/(12*(u64::MAX as f64)^2), approximately 2.4e-40, with f64 spacing approximately 4e- <<cut 139 chars by kit>>
Resolution detail: inverse_interpolate.c::interpolate_with_approx recomputes exact_count at every guess and after exhausting 18 iterations. The final forward/backward scan restores the exact rank, so changed guesses can remain exact-answer  <<cut 139 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 225 bytes)

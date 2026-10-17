Port the four almost-prime functions from MPU 0.75 in separate module files.
Reuse existing prime and semiprime routines and retain the C bounds and tables.
Keep STATUS, tools, fixtures, and the crate root unchanged as required by the task.
Add fixture compatibility, algebraic tests, and benchmarks; run offline tests and gate.
Copied the C coefficient and maximum tables and implemented recursive counts and both estimates. Next: range construction and fixture checks.
Added range generation, the prime-power window sieve, four oracle tests, partition/list properties, concurrency checks, and benchmarks. Running focused offline tests next.
Focused tests are compiling offline. Range code now uses the C second-largest-factor bound and 262144-entry windows; coefficient and maximum tables came directly from the C source.
Added fit-boundary and high-window checks, including forced powers of two and window transitions. Guarded window index increments for 32-bit usize overflow.
The first offline build is still running; gate is queued on Cargo's build lock. Tests now cover C fit transitions, native maxima, small ranges, and two sieve windows.
No test result yet: tools/test.sh emits only the final log tail, so build progress is hidden. No network, fixture changes, or changes to readonly paths were needed.
Waiting for the initial offline dependency build. Replaced quotient rounding with Rust div_ceil and documented the inherited semiprime range panic.
Exact-count and range oracle tests pass. Approx/lower each fail on k=1,n=u64::MAX because existing prime_count_approx/lower use f64 instead of C long double. Gate found one literal-format lint, now fixed; unsigned oracle cases remain unskip <<cut 4 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed -19 bytes)
About five minutes remain. The remaining known mismatch is inherited long-double precision in the existing prime count dependencies. Keep the failing unsigned oracle checks visible; do not hard-code fixture values or edit sibling modules.
List/count, partition, and concurrent-call properties pass. Removed an invalid universal law: the C weighted estimate can round below its integer lower endpoint near u64::MAX. Replaced it with exact minimum-value checks for all k.
Fixed all reported clippy issues in added tests and benchmarks. Moved the new benchmark block into its own function to preserve the existing benchmark function length limit. Full offline suite was launched once.
The two k=1 maximum-input precision mismatches remain outside this row's editable module scope. All unsigned oracle assertions stay exact. Awaiting final gate and full-suite logs before recording the final status.
Minimum-value tests exposed a real overflow: reduction can reach k=0 before approximation shifts. Added the degenerate result before shifting. Registered the almost-prime benchmarks separately to meet clippy's function length limit.

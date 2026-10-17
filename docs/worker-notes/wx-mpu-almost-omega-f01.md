Port the four almost-prime functions from MPU 0.75 in separate module files.
Reuse existing prime and semiprime routines and retain the C bounds and tables.
Keep STATUS, tools, fixtures, and the crate root unchanged as required by the task.
Add fixture compatibility, algebraic tests, and benchmarks; run offline tests and gate.
Copied the C coefficient and maximum tables and implemented recursive counts and both estimates. Next: range construction and fixture checks.
Added range generation, the prime-power window sieve, four oracle tests, partition/list properties, concurrency checks, and benchmarks. Running focused offline tests next.
Focused tests are compiling offline. Range code now uses the C second-largest-factor bound and 262144-entry windows; coefficient and maximum tables came directly from the C source.

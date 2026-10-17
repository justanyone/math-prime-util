Port the four requested factor methods from MPU 0.75 into separate files.
Preserve the C algorithm bounds and the Perl public factor-list behavior.
Use the existing factor fixtures and add algebraic tests and benchmarks.
Keep tools, fixtures, STATUS.md, and the crate root unchanged.
Read the C routines and XS wrapper. The wrapper strips 2, 3, 5 and keeps an unfactored cofactor on budget exhaustion. Implementing explicit optional budgets and that behavior.
Four algorithm files, compatibility tests, product properties, repeat/concurrency checks, and benchmarks are now written. Running the focused offline tests next.
All four oracle tests passed. Product properties passed. Corrected a test that wrongly required SQUFOF to split a square in one racing round; the default budget does split it.
Focused compatibility and property rerun is complete. Fixed the pedantic lint findings in tests and benchmarks. The full offline gate is running; no tools or fixtures were changed.

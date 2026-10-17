Port the four requested factor methods from MPU 0.75 into separate files.
Preserve the C algorithm bounds and the Perl public factor-list behavior.
Use the existing factor fixtures and add algebraic tests and benchmarks.
Keep tools, fixtures, STATUS.md, and the crate root unchanged.
Read the C routines and XS wrapper. The wrapper strips 2, 3, 5 and keeps an unfactored cofactor on budget exhaustion. Implementing explicit optional budgets and that behavior.

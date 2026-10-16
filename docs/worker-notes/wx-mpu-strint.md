Implement the digits and checked integer helpers in src/digits/.
Use the existing digits oracle without changing fixtures or tools.
Check native C algorithms; this checkout has strops.c instead of strint.c.
Add compatibility, algebraic, boundary tests and hot function benchmarks.
Run the focused offline tests and the repository gate.
Confirmed floor, ceiling, truncating and Euclidean division differ. Shift counts may be negative. Implementing checked i128 arithmetic and native digit conversion now.
- 2026-10-16T23:09:55Z launcher: no commit for 60 s; 151 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-strint.md.

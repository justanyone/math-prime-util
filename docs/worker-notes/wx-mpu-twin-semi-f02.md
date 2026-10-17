Port the four twin and semiprime functions from the MPU C reference.
Keep each function in a separate families source file and reuse crate primitives.
Add fixture compatibility tests, algebraic tests, and criterion benchmarks.
Leave tools, fixtures, STATUS.md, and git metadata unchanged per task rules.
Run focused offline tests and the required gate; record any blockers here.
Reference methods and fixture input shapes reviewed; implement isolated source files and tests next.
Implemented semiprime marking, exact prime-count sum, and reference approximation tables. High narrow ranges resolve known prime factors early to avoid a full four-billion bound walk.

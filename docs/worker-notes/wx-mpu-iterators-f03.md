Scope: reassess the four existing iterator ports and their MPU reference algorithms.
Move only this task's functions into separate files under src/iter.
Check partition bounds, iterator state, closure breaks, and oracle cases offline.
Keep tools, fixtures, STATUS.md, and git metadata unchanged as required by the task.
Moved the existing four ports into their own files; checking native MPU bounds and iterator exhaustion next.
Reused factor.c divisor_list port; matched XS partition bound setup and early exit. Added state, bounds, callback restart, and algebraic regression tests.
Focused offline tests are compiling; added benchmarks for compositions and divisors and formatted the touched Rust files.
Composition state now has its own public iterator type; updated the existing fixture dispatcher for the distinct types. Gate is waiting for the focused build lock.

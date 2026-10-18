Continuing the partial integer trait task: inspect the existing implementation and finish BigUint support.
Preserve native arithmetic behavior and cite the MPU reference for ported algorithms.
Add focused BigUint tests, then run the required test wrapper and full gate.
Do not mark the task READY; leave all changes on disk for the launcher.
Inspected native trait implementations and confirmed num-bigint is absent from the standard cache; checking local packages before adding the feature.
Implementing an optional num-bigint backend with native-range delegation, extra-strong BPSW, modular powers, Euclidean gcd, and bounded Brent factoring.
Added the feature and BigUint implementation; next add native-agreement and >128-bit tests, then resolve local build availability.
Focused test could not resolve num-bigint offline. A worktree-local fetch also failed because index.crates.io DNS is unavailable; no external files were changed.

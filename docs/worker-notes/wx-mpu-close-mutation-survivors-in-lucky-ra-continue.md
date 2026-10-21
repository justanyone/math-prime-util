Continue the existing estimate mutation tests on this branch.
Review each reported survivor and distinguish bound validity from tightness.
Run the focused tests, the offline mutation sweep, and the repository gate.
Keep changes limited to the task paths and record blockers here.
Initial five tests passed. Gate found one narrowing cast; changed k to u8 and used u32::from. Added large lucky fit boundaries and the harmonic asymptotic branch.
Seven focused tests pass. The lucky <7 mutant is unreachable through public callers; harmonic termination is below f64 precision. Added Axler indices at the tail threshold and n=7140 (index 15360).

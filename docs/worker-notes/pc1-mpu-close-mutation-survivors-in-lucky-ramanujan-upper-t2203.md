Review the inherited estimate regression tests and equivalent-survivor evidence.
Check the reference algorithms and boundary semantics before changing tests or code.
Run focused tests, the thirteen targeted mutations on four files, and the gate.
Record equivalent mutations honestly; do not mark this task READY or claim impossible kills.
Reviewed inherited tests and reference boundaries; checking targeted mutant list before rerunning.
Focused tests compiling; first mutant regex matched nothing because mutation names put the operation before the function. Narrowing by current source locations.
Started targeted cargo-mutants on the four files and gate. Existing STATUS entries are already checked; no checklist changes needed.
33 focused tests passed. Mutation run with target copying raced against gate compilation; restarted without copying target and corrected moved interpolation locations (13 selected).
Added executable evidence that the harmonic cutoff and full finite expansion give identical f64 corrections; documented index 306 equivalence beside its existing boundary assertion.
The inherited refactor moved mutation locations; verifying both subtraction sites and both nth-prime threshold sites to avoid silently omitting the original survivor.
34 focused tests passed after adding the harmonic equivalence regression; mutation baselines are still compiling, gate has reached clippy-gmp.
Corrected prior interpretation: the task names nth_prime_upper line 146 (15360 threshold), whereas the inherited test only covers line 130 (306). Adding the missing true threshold case if mutation confirms the gap.

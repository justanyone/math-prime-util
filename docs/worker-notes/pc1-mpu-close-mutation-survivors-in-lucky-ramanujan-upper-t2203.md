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
Mutation baseline passed; harmonic < -> == confirmed MISSED. Reviewed the actual 15360 threshold and the C formula coefficients to add its missing boundary test.
Boundary review: lucky n=7 changes the helper from 3 to 2 under < -> <=, but callers use the helper only above their small exact branches; the direct private test kills this externally masked change.
Lucky division/comparison mutants change the approximation bands; a downward approximate lower bound may remain valid but become non-tight. The tests pin C compatibility as well as checking exact small bounds.
Axler tail tests pin C's strict threshold and negative constant; changing to the next coefficient earlier or making a coefficient positive changes the approximation. Exact Ramanujan values at the multi-billion threshold are not available in <<cut 76 chars by kit>>
Almost-prime tests assert bracket visits against monotone counts and the returned inverse. Final exact correction can mask bracket mutations; the trace verifies C's interpolation behavior rather than assuming every changed estimate breaks a <<cut 7 chars by kit>>
The 306 threshold was an inherited extra case; the actual task threshold is 15360. Added C-formula boundary values 168432 and 168444 at 15360 and 15361, with exact-prime upper-bound checks.

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 36 bytes)
Mutation build invokes all test targets even though execution is filtered; this is slower than expected. Final focused tests and full suite are queued behind the gate build lock.
Build command confirmed cargo test --no-run (all targets); execution filter does not restrict its compile phase. Waiting for current run rather than adding more competing builds.
First mutation run finished: 1 missed, 4 caught, 7 unviable. Restarted all thirteen exact mapped task mutations with -C --lib and one job, avoiding expensive integration-test compilation. The original almost-prime <= survivor maps to span <= 19999 (now 172), and minus to max - 19999 (now 175).

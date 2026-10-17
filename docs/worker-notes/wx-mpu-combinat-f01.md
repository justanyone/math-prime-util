Port binomial, catalan_number, stirling, and bell_number from MPU 0.75.
Keep each function in its own file and preserve the reference methods and bounds.
Use the existing combinat fixtures without changing tools, fixtures, or STATUS.md.
Add compatibility, algebraic, and boundary tests, then run offline focused tests and the gate.
Reference read: binomial uses gcd cancellation; Stirling uses binomial sums; Bell uses its triangle. Existing fixtures include big integers, so comparisons must normalize values outside the native range.
Implementation and fixture/property tests are written. Running focused offline tests now; no tools or fixtures were changed.
Focused tests are compiling offline. Formatting completed; the launcher is snapshotting the new files while checks run.
Focused checks found bigint binomial inputs to skip and Stirling cancellation beyond i128 despite a small final answer. The first gate found benchmark lint issues; fixing those and the Stirling fallback next.
Added the first-kind direct recurrence used by MPU PP.pm after C overflow, plus its second-kind near-diagonal shortcut. Focused fixtures now rerunning after the cancellation fix.
Near-diagonal review found fitting second-kind values beyond the C intermediate range. Added a capped positive recurrence fallback and regression cases; it stops on proven result overflow.
Near-diagonal and concurrent-repeat tests are complete. Corrected a hand-calculated regression constant; added a constant-time overflow check for deficit one to avoid billions of recurrence steps on u64::MAX.
All 12 focused tests passed. The final offline gate is running. The checkout has no origin/dev ref, so a base diff cannot be shown; git status is clean after launcher snapshots.
Finished: all 12 focused tests, the offline full suite, and the final gate passed; the gate printed == gate: green. No implementation work remains; cross-platform coverage remains for CI. Tools, fixtures, STATUS.md, and dependencies were unchanged.

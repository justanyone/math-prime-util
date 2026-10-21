Continue the estimate mutation task on the existing branch.
Read PLAN sections 5–8 and the four named helper locations.
Add direct helper checks through the new mutation test file where public APIs mask changes.
Run offline focused tests, targeted cargo-mutants, and the required gate.
Do not write git metadata; the launcher records changes.
- 2026-10-21T01:57:16Z launcher: no commit for 61 s; 91 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-close-mutation-survivors-i-restart-1792547524-continue.md.
Added direct private-helper tests in the new test file. Extracted the existing interpolation callback body, matching the C callback interface, to exercise rare bracket repairs without huge prime counts. Focused tests are running.
Focused tests exposed two incorrect hand-computed expectations; corrected them. The nth-prime bound at index 306 is exactly 2017, the same as the exact-prime branch, so its < to <= mutation is equivalent. Harmonic cutoff also appears equiva <<cut 105 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 80 bytes)
All 25 focused in-crate checks pass. Started cargo-mutants offline on exactly the 13 listed mutations across the four files, mapping interpolation lines to the extracted callback body. No public API change or dependency change.

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
Boundary assessment: lucky < to <= at 7 changes the private estimate 3 to 2, but public lucky bounds use the exact table there. Both lucky / to * mutants make their estimate zero and weaken the lower seed; the two range-comparison mutants c <<cut 195 chars by kit>>
Boundary assessment: nth_prime_upper < to <= at 306 selects the exact prime 2017 instead of the binary-search result 2017. Harmreal < to == only adds terms too small to change the accumulated f64 correction (below 1e-44 versus a correction  <<cut 87 chars by kit>>
Boundary assessment: deleting the minus on Axler -2.435 increases the upper bound (non-tight). Switching the strict threshold at 514808375201 uses the next coefficient at that endpoint; its private coefficient differs, but exact R at that e <<cut 219 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 427 bytes)
Gate found missing crate documentation after include! removed the test file inner docs. Changed the test hook to a path module with documented, reusable test macros. This keeps test bodies in the new file and avoids changing existing test f <<cut 5 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed -18 bytes)
Correction after inspecting mutation diffs: the requested Ramanujan endpoints are 3999654659 and nth index 15360, not the extra 514808375201 and 306 cases initially tested. The deleted minus is on -2.0884. Added the actual coefficient endpo <<cut 136 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 111 bytes)
About five minutes remain. First mutant run caught eight, missed three Ramanujan cases, and failed internally on disk quota before a reliable total. Correcting boundary coverage and rerunning without target copies; gate is running with the  <<cut 54 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 30 bytes)
Measured the requested nth-prime boundary: original upper(15360)=168432. Added that check and both sides of Axler 3999654659, plus the final negative coefficient. Started a second exact-13 offline mutant run with one worker, no target copy, <<cut 125 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 100 bytes)
The harmonic cutoff is an equivalent output mutation, so the literal all-13-killed criterion cannot honestly be claimed. Preserve C semantics rather than change correct numerical code just to alter a mutation score. A second full offline to <<cut 87 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 63 bytes)

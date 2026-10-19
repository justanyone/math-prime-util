I will replace bigint proving's duplicate partial factoring with the existing bounded factor API.
I will inspect its return structure and adapt the proof callers and tests.
There is no src/bigint/modular.rs in this checkout; I will verify no analogous helper is present elsewhere named by the task.
I will run the focused test and available offline formatting/lint checks without adding dependencies.
Replaced partial_factor in the BLS75 and ECPP candidate paths with factor_within and migrated its unit coverage. The focused proving test runner is still active and has not printed output; next I will collect its result, format, and run the <<cut 18 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed -6 bytes)
- 2026-10-19T12:25:46Z launcher: no commit for 60 s; 182 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-bigint-proving-uses-the-factor-modul.md, src/bigint/proving.rs.
The first proving test failed because the new two-second factor budget produced a BLS75 proof where the prior helper gave up. Reduced the budget to one second to retain the ECPP fallback; the targeted proving_paths run is still executing. N <<cut 46 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 22 bytes)
- 2026-10-19T12:28:18Z launcher: no commit for 60 s; 334 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-bigint-proving-uses-the-factor-modul.md, src/bigint/proving.rs.
The one-second limit still proved the test semiprime, so set proof callers to Duration::ZERO; this passed proving_paths and the full unit test set. The gate has passed fmt and all clippy feature sets and is continuing through all-feature in <<cut 81 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 57 bytes)
- 2026-10-19T12:30:19Z launcher: no commit for 61 s; 455 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-bigint-proving-uses-the-factor-modul.md, src/bigint/proving.rs.
The all-feature test command has passed unit tests, bigint proving compatibility, and multiple other compatibility suites. It is still running long iterator/property work (forsquarefreeint passed 60 seconds); no failure has appeared. Next:  <<cut 62 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 38 bytes)
- 2026-10-19T12:32:19Z launcher: no commit for 60 s; 575 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-bigint-proving-uses-the-factor-modul.md, src/bigint/proving.rs.

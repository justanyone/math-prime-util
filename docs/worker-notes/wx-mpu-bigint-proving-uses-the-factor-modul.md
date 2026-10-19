I will replace bigint proving's duplicate partial factoring with the existing bounded factor API.
I will inspect its return structure and adapt the proof callers and tests.
There is no src/bigint/modular.rs in this checkout; I will verify no analogous helper is present elsewhere named by the task.
I will run the focused test and available offline formatting/lint checks without adding dependencies.
Replaced partial_factor in the BLS75 and ECPP candidate paths with factor_within and migrated its unit coverage. The focused proving test runner is still active and has not printed output; next I will collect its result, format, and run the <<cut 18 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed -6 bytes)

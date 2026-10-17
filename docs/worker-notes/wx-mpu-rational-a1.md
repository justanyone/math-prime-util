# wx-mpu-rational-a1 notes
- Plan: port rational.c functions to src/rational/mod.rs, fixture-driven compat test, prop test, bench.
- Reason: task MPU-RATIONAL; keep diff minimal.
- Stopped at the 14 minute limit. compat_rational: 13 fail. Open: each_may_panic cannot skip (from_contfrac empty, next_farey q>n); 64/65 index overflow cases not skipped; farey/farey_len not ported; no prop_rational, no bench; gate not run. Not committed.

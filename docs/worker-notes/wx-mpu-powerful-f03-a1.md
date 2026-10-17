# MPU-POWERFUL-F03

Port `powerfree_sum`, `powerfree_part`, and `powerfree_part_sum` from MPU 0.75 `powerfree.c` into `src/families/`, each in its own file. Dependencies are on origin/dev-001. Compat tests use `tests/fixtures/families.tsv`. No new Cargo deps. Do not edit STATUS.md, lib.rs, tools, or fixtures.

- Minute 0: notes written. Next: read PLAN 5–8 headings, current `src/families`, and the C functions.
- Moved `powerfree_sum`, `powerfree_part`, and `powerfree_part_sum` into their own files. Compat oracles already passed on the shared `powerfree.rs` bodies. Next: fmt, focused tests, then the gate.
- Focused compat, property, and doc tests passed. Clippy on the powerfree targets is clean. Full clippy stops on pre-existing `float_cmp` asserts in `src/count/mod.rs` and `src/proving/is_aks_prime.rs`. Those files stay unchanged (other rows own them). Next: `tools/gate.sh`.
- 2026-10-17T19:05:18Z launcher: no commit for 61 s; 92 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-powerful-f03-a1.md.
- 2026-10-17T19:06:18Z launcher: no commit for 60 s; 152 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-powerful-f03-a1.md.
- 2026-10-17T19:07:18Z launcher: no commit for 60 s; 212 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-powerful-f03-a1.md.
- 2026-10-17T19:08:20Z launcher: no commit for 62 s; 274 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-powerful-f03-a1.md.
- 2026-10-17T19:09:20Z launcher: no commit for 60 s; 334 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-powerful-f03-a1.md.
- 2026-10-17T19:10:21Z launcher: no commit for 60 s; 395 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-powerful-f03-a1.md.
- 2026-10-17T19:11:23Z launcher: no commit for 62 s; 457 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-powerful-f03-a1.md.
- 2026-10-17T19:12:23Z launcher: no commit for 60 s; 517 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-powerful-f03-a1.md.
- 2026-10-17T19:13:23Z launcher: no commit for 60 s; 577 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-powerful-f03-a1.md.
- 2026-10-17T19:15:25Z launcher: no commit for 60 s; 699 s elapsed. Files changed so far: benches/families.rs, docs/worker-notes/wx-mpu-powerful-f03-a1.md, src/families/mod.rs, src/families/powerfree.rs, src/families/powerfree_part.rs, src/families/powerfree_part_sum.rs, src/families/powerfree_sum.rs, tests/prop_families.rs.
- 2026-10-17T19:16:25Z launcher: no commit for 60 s; 759 s elapsed. Files changed so far: benches/families.rs, docs/worker-notes/wx-mpu-powerful-f03-a1.md, src/families/mod.rs, src/families/powerfree.rs, src/families/powerfree_part.rs, src/families/powerfree_part_sum.rs, src/families/powerfree_sum.rs, tests/prop_families.rs.
- 2026-10-17T19:17:29Z launcher: no commit for 63 s; 823 s elapsed. Files changed so far: benches/families.rs, docs/worker-notes/wx-mpu-powerful-f03-a1.md, src/families/mod.rs, src/families/powerfree.rs, src/families/powerfree_part.rs, src/families/powerfree_part_sum.rs, src/families/powerfree_sum.rs, tests/prop_families.rs.
- 2026-10-17T19:19:29Z launcher: no commit for 60 s; 943 s elapsed. Files changed so far: benches/families.rs, docs/worker-notes/wx-mpu-powerful-f03-a1.md, src/families/mod.rs, src/families/powerfree.rs, src/families/powerfree_part.rs, src/families/powerfree_part_sum.rs, src/families/powerfree_sum.rs, tests/prop_families.rs.
- 2026-10-17T19:22:01Z launcher: no commit for 60 s; 1095 s elapsed. Files changed so far: benches/families.rs, docs/worker-notes/wx-mpu-powerful-f03-a1.md, src/count/mod.rs, src/families/mod.rs, src/families/powerfree.rs, src/families/powerfree_part.rs, src/families/powerfree_part_sum.rs, src/families/powerfree_sum.rs, src/proving/is_aks_prime.rs, tests/prop_families.rs.

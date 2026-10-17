# MPU-COMBINAT-F07

Port `permtonum`, `partitions`, and `partitionsq` from MPU 0.75 into `src/combinat/`. Each function gets its own file, one `mod` and one `pub use` in `mod.rs`, plus compat and property tests.

Why: sibling rows own the other combinat functions. These three are the slice. Overflow returns `None`. Cite the C file and function. No new dependencies. No edits outside this worktree.

Next: read PLAN.md sections 5–8, the combinat module, and the C sources for these three functions.
- 2026-10-17T06:59:52Z launcher: no commit for 61 s; 91 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-combinat-f07-a1.md.
- 2026-10-17T07:00:52Z launcher: no commit for 60 s; 151 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-combinat-f07-a1.md.
- 2026-10-17T07:01:52Z launcher: no commit for 60 s; 211 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-combinat-f07-a1.md.
- 2026-10-17T07:02:54Z launcher: no commit for 61 s; 273 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-combinat-f07-a1.md.
- 2026-10-17T07:03:54Z launcher: no commit for 60 s; 333 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-combinat-f07-a1.md.
- 2026-10-17T07:04:54Z launcher: no commit for 60 s; 393 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-combinat-f07-a1.md.
Tried: read util.c npartitions, npartitionsq, perm_to_num and the combinat fixtures. Next: fmt and focused tests.
- 2026-10-17T07:06:55Z launcher: no commit for 60 s; 514 s elapsed. Files changed so far: benches/combinat.rs, docs/worker-notes/wx-mpu-combinat-f07-a1.md, src/combinat/mod.rs, src/combinat/partitions.rs, src/combinat/partitionsq.rs, src/combinat/permtonum.rs, tests/compat_combinat.rs, tests/prop_combinat.rs.
Clippy is clean. Compat and property tests passed. Next: full gate (fmt, clippy, MSRV, tests, docs).

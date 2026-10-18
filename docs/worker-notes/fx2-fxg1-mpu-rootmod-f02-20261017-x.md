# fx2-fxg1-mpu-rootmod-f02-20261017-x

Will read the reviewer note (2 Open findings) and PLAN.md §§5–8, then fix each finding inside the branch's existing files and add a test that fails without the fix.

Why: the task is a fix pass on open review findings, not new API surface. Keep the diff minimal. Decline a finding only with a reason.

Next: open the review note and the relevant source once the findings name the functions.

Restored `util.c::cornacchia` prime Kronecker rejection plus `corn_all` for `p >= 2^53`. Put `znlog` / `factorialmod` / `binomialmod` oracles and the old property tests back beside the rootmod tests. Next: `mw-test` on the modular tests.

`mw-test --full` on compat_modular, prop_modular, and the cornacchia unit tests passed (compat pairs match the oracle). Clippy `-D warnings` is clean. Next: `tools/gate.sh`.

`tools/gate.sh` passed fmt, clippy, and the 1.85 build, then the full test run was killed by the session time limit while results were still `ok` (through `compat_families`). `cargo doc -D warnings` passed afterwards. About 5 minutes remain: one more gate attempt if the suite is still cached.
- 2026-10-18T03:26:10Z launcher: no commit for 61 s; 91 s elapsed. Files changed so far: docs/worker-notes/fx2-fxg1-mpu-rootmod-f02-20261017-x.md.
- 2026-10-18T03:27:10Z launcher: no commit for 60 s; 152 s elapsed. Files changed so far: docs/worker-notes/fx2-fxg1-mpu-rootmod-f02-20261017-x.md.
- 2026-10-18T03:28:10Z launcher: no commit for 60 s; 212 s elapsed. Files changed so far: docs/worker-notes/fx2-fxg1-mpu-rootmod-f02-20261017-x.md.
- 2026-10-18T03:29:12Z launcher: no commit for 61 s; 273 s elapsed. Files changed so far: docs/worker-notes/fx2-fxg1-mpu-rootmod-f02-20261017-x.md.
- 2026-10-18T03:30:12Z launcher: no commit for 60 s; 333 s elapsed. Files changed so far: docs/worker-notes/fx2-fxg1-mpu-rootmod-f02-20261017-x.md.
- 2026-10-18T03:31:12Z launcher: no commit for 60 s; 393 s elapsed. Files changed so far: docs/worker-notes/fx2-fxg1-mpu-rootmod-f02-20261017-x.md.
- 2026-10-18T03:32:14Z launcher: no commit for 61 s; 455 s elapsed. Files changed so far: docs/worker-notes/fx2-fxg1-mpu-rootmod-f02-20261017-x.md.
- 2026-10-18T03:33:14Z launcher: no commit for 60 s; 515 s elapsed. Files changed so far: docs/worker-notes/fx2-fxg1-mpu-rootmod-f02-20261017-x.md.
- 2026-10-18T03:34:14Z launcher: no commit for 60 s; 575 s elapsed. Files changed so far: docs/worker-notes/fx2-fxg1-mpu-rootmod-f02-20261017-x.md.
- 2026-10-18T03:35:16Z launcher: no commit for 62 s; 638 s elapsed. Files changed so far: docs/worker-notes/fx2-fxg1-mpu-rootmod-f02-20261017-x.md.
- 2026-10-18T03:37:49Z launcher: no commit for 62 s; 790 s elapsed. Files changed so far: docs/worker-notes/fx2-fxg1-mpu-rootmod-f02-20261017-x.md, src/modular/cornacchia.rs, tests/compat_modular.rs, tests/prop_modular.rs.
- 2026-10-18T03:38:49Z launcher: no commit for 60 s; 851 s elapsed. Files changed so far: docs/worker-notes/fx2-fxg1-mpu-rootmod-f02-20261017-x.md, src/modular/cornacchia.rs, tests/compat_modular.rs, tests/prop_modular.rs.
- 2026-10-18T03:41:51Z launcher: no commit for 60 s; 1033 s elapsed. Files changed so far: docs/worker-notes/fx2-fxg1-mpu-rootmod-f02-20261017-x.md, src/modular/cornacchia.rs, tests/compat_modular.rs, tests/prop_modular.rs.
- 2026-10-18T03:42:53Z launcher: no commit for 60 s; 1094 s elapsed. Files changed so far: docs/worker-notes/fx2-fxg1-mpu-rootmod-f02-20261017-x.md, src/modular/cornacchia.rs, tests/compat_modular.rs, tests/prop_modular.rs.
- 2026-10-18T03:43:56Z launcher: no commit for 63 s; 1158 s elapsed. Files changed so far: docs/worker-notes/fx2-fxg1-mpu-rootmod-f02-20261017-x.md, src/modular/cornacchia.rs, tests/compat_modular.rs, tests/prop_modular.rs.
- 2026-10-18T03:44:57Z launcher: no commit for 60 s; 1218 s elapsed. Files changed so far: docs/worker-notes/fx2-fxg1-mpu-rootmod-f02-20261017-x.md, src/modular/cornacchia.rs, tests/compat_modular.rs, tests/prop_modular.rs.
- 2026-10-18T03:45:57Z launcher: no commit for 60 s; 1279 s elapsed. Files changed so far: docs/worker-notes/fx2-fxg1-mpu-rootmod-f02-20261017-x.md, src/modular/cornacchia.rs, tests/compat_modular.rs, tests/prop_modular.rs.

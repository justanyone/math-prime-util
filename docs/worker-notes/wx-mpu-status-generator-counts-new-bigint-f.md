I will update tools/status.py so bigint public functions override stale no-form reason groups.
I will add a focused regression test for status output, warnings, and stale generated files.
The requested bigint coverage document is absent in this checkout, so I will inspect generator inputs and create it from the existing groups.
I will run only the focused offline checks available here and record any limits.
- 2026-10-19T15:43:27Z launcher: no commit for 60 s; 121 s elapsed. Files changed so far: docs/bigint-coverage.md, docs/worker-notes/wx-mpu-status-generator-counts-new-bigint-f.md, tools/status.py, tools/test_status.py.
- 2026-10-19T15:44:27Z launcher: no commit for 60 s; 181 s elapsed. Files changed so far: docs/bigint-coverage.md, docs/worker-notes/wx-mpu-status-generator-counts-new-bigint-f.md, tools/status.py, tools/test_status.py.
- 2026-10-19T15:45:28Z launcher: no commit for 61 s; 243 s elapsed. Files changed so far: docs/bigint-coverage.md, docs/worker-notes/wx-mpu-status-generator-counts-new-bigint-f.md, tools/status.py, tools/test_status.py.
- 2026-10-19T15:46:28Z launcher: no commit for 60 s; 303 s elapsed. Files changed so far: docs/bigint-coverage.md, docs/worker-notes/wx-mpu-status-generator-counts-new-bigint-f.md, tools/status.py, tools/test_status.py.
- 2026-10-19T15:47:28Z launcher: no commit for 60 s; 363 s elapsed. Files changed so far: docs/bigint-coverage.md, docs/worker-notes/wx-mpu-status-generator-counts-new-bigint-f.md, tools/status.py, tools/test_status.py.
- 2026-10-19T15:48:30Z launcher: no commit for 61 s; 424 s elapsed. Files changed so far: docs/bigint-coverage.md, docs/worker-notes/wx-mpu-status-generator-counts-new-bigint-f.md, tools/status.py, tools/test_status.py.
Focused Python unittest passed; status generation and --check passed.
tools/test.sh test_status ran the Cargo filter and completed; it does not invoke Python tests.
tools/gate.sh passed fmt, clippy, and MSRV build, then stalled in compat_iter::forsquarefreeint during the full suite.
Next: confirm the generated output stays current and leave this test limitation noted.
Final: implementation and regression test are on disk; generator check is current. The full gate remains limited by a long-running iterator compatibility test.

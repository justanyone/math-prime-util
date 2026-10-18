# Worker notes: close factor mutation survivors (continue run)

- Previous run added tests in is_carmichael.rs, lehman_factor.rs, pminus1_factor.rs (35 lines).
- Plan: check the remaining named survivors (holf_factor 34:33, factor.rs 108:32 and 161:11) and add a test or an equivalence reason for each.
- Focused tests only; no full-suite run unless time allows.
- 15:49 Survivors read: holf 34:33 is '>>' in the 60/30 multiplier band; factor.rs 108:32 is '*' in rounds; 161:11 is 'a += 2'. Running focused factor tests now.
- 2026-10-18T15:50:44Z launcher: no commit for 60 s; 123 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-close-the-open-factor-mutation-survi-continue.md.
- Focused run: tools/test.sh --lib is_carmichael: 4 passed.
- PR text, status by survivor (this run did not re-run cargo mutants):
  - Caught by tests added in the earlier run (not re-verified here): is_carmichael 52:14, 64:15; lehman_factor 49:52, 56:14; pminus1_factor 42:15, 56:35.
  - Open: holf_factor 34:33 (`>>` to `<<` in the 60/30 multiplier band; needs a golden case in the band MAX>>8 to MAX>>6 with a small round budget); factor.rs 108:32 (`*` to `+` in the rounds formula; needs a rounds-observable case); factor.rs 161:11 (`a += 2`; observable only if the first Brent constant fails).
  - Not classed as equivalent: none yet.
- Last line: stopped at the time limit. Remaining: the three open named survivors, the unnamed survivors, and the full gate run. Aragog cargo-mutants re-run not done.

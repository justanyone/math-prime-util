# Worker notes: close factor mutation survivors (continue run)

- Previous run added tests in is_carmichael.rs, lehman_factor.rs, pminus1_factor.rs (35 lines).
- Plan: check the remaining named survivors (holf_factor 34:33, factor.rs 108:32 and 161:11) and add a test or an equivalence reason for each.
- Focused tests only; no full-suite run unless time allows.
- 15:49 Survivors read: holf 34:33 is '>>' in the 60/30 multiplier band; factor.rs 108:32 is '*' in rounds; 161:11 is 'a += 2'. Running focused factor tests now.
- 2026-10-18T15:50:44Z launcher: no commit for 60 s; 123 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-close-the-open-factor-mutation-survi-continue.md.

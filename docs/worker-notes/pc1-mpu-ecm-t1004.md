# pc1-mpu-ecm-t1004

Continue PARTIAL ecm_factor: exact fixture parity and a green gate.
The Rust port follows factor.c tinyecm64; compat_factor special-cases
1819015037140 (49×1856137793 vs oracle 7×12992964551). Next: run the
oracle test, match Perl/XS output, then tools/gate.sh.

15:06:58 locating perl oracle and whether XS ecm matches the one mismatched fixture
- 2026-10-17T15:08:12Z launcher: no commit for 60 s; 216 s elapsed. Files changed so far: docs/worker-notes/pc1-mpu-ecm-t1004.md.
15:09:10 running tools/test.sh ecm_factor to see current parity
- 2026-10-17T15:10:17Z launcher: no commit for 62 s; 340 s elapsed. Files changed so far: docs/worker-notes/pc1-mpu-ecm-t1004.md.
15:11:21 probing which B1/curve count yields factor 7 of 90950751857

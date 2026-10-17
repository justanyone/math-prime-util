# pc1-mpu-ecm-t1004

Continue PARTIAL ecm_factor: exact fixture parity and a green gate.
The Rust port follows factor.c tinyecm64; compat_factor special-cases
1819015037140 (49×1856137793 vs oracle 7×12992964551). Next: run the
oracle test, match Perl/XS output, then tools/gate.sh.

15:06:58 locating perl oracle and whether XS ecm matches the one mismatched fixture

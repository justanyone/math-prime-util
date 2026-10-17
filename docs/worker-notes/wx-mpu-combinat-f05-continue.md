# wx-mpu-combinat-f05 continue notes
- Goal: finish congruent-number factor filters (is_congruent_number), verify 95% line coverage, run compat/prop tests.
- Plan: read current is_congruent_number.rs, compare with reference/mpu/congruent_numbers.c, port missing filters, run tools/test.sh combinat.
- Gate (tools/gate.sh) needs approval in this session; not run. fmt/clippy not run (cargo not on PATH outside tools/test.sh).
- Congruent-number filters 1 (rest), 2, 3 of congruent_numbers.c are still unported; the Rust port keeps the ACK check and Tunnell, which is exact. Left as is: porting ~400 lines of C in the remaining time without a gate run is unsafe.
- Final: no source edits in this session. Coverage at 95% not measured.

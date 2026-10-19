Continue MPU-IS-POWER-EXP: native is_power_exp must return k for 2^k and 3^k, k = 2..=63.
Prior run extended the prime list but left the bigint proptest exclusion and gate.
This checkout may lack src/bigint/arith.rs. I will confirm the native function, tests, and exclusion, then finish what exists.
Next: read PLAN 5-8, locate is_power_exp, and inspect the current diff.

# wx-mpu-is-power-exp-prime-exponents-above-37
- Goal: extend is_power_exp prime-exponent list to all primes <= 61 so 2^k for k in 2..=63 is detected.
- Plan: edit src/int/mod.rs list, add test for 2^k and 3^k, drop six-value exclusion in src/bigint/arith.rs proptest.
- Extended prime list to 61 in src/int/mod.rs. src/bigint/arith.rs does not exist in this checkout; proptest edit cannot be made here.
- Last: fmt not run (tool denied compound commands). Proptest file src/bigint/arith.rs not in checkout; left unchanged.

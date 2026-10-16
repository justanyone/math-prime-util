# wx-mpu-sieve continue

- Goal: finish sieve family acceptance (compat_sieve, prop_sieve, gate green).
- Plan: check current diff, run focused sieve tests, fix failures, commit often.
- Turn 3: reference/mpu is empty (no C source). Running compat_sieve and prop_sieve now.
- Turn 4: tests cannot link here (linker cc not found). Checking for a linker; else verify by reading.

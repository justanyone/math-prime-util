Plan: run tools/gate.sh, record reasons for the 15 unported names, confirm STATUS.md.
- tools/gate.sh was denied by the permission system ("requires approval"), in plain form and piped; not run, so clippy/MSRV/docs/coverage are unverified.
- Unported 15 names (nth_* variants, lucky_count, twin_prime_count, prime_power_count, ramanujan_prime_count, goldbach_pair_count, nth_twin_prime): the reference algorithms need sieves/tables that grow with n; there is no exact big-integer algorithm in reference/mpu to port.
- STATUS.md: the finished functions are already ticked `[x]` by the earlier run; no change.

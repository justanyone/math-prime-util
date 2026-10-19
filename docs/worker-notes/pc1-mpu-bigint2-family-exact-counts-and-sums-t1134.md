Plan: run tools/gate.sh, fix clippy/docs issues, tick STATUS.md for the ported functions, record reasons for the unported names.
- tools/gate.sh and cargo clippy were both denied by the permission system in this session, so fmt, clippy, MSRV, docs and coverage are still unverified.
- STATUS.md has no `[ ]` lines (every name is already `[x]`), so there is nothing to tick.
- Unported BigUint forms (15 names): the nth_* names, lucky_count, twin_prime_count, prime_power_count, ramanujan_prime_count, goldbach_pair_count, nth_twin_prime. Reason: the reference algorithms need sieves or tables whose size grows with n, so there is no exact big-integer algorithm to port without a different method.
- Next run: execute tools/gate.sh first.

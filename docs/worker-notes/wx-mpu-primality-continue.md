# wx-mpu-primality continue notes
- Goal: finish primality port: Perrin, Catalan, Chen, delicate-prime, mersenne checks; miller_rabin_random left as todo.
- Plan: read current src/primality/mod.rs, add missing fns from C primality.c, compat/prop tests, run tools/test.sh primality, gate.
- Perrin ported in src/primality/perrin.rs (compiles). Next: compat test for perrin, then Chen, Catalan, delicate.
- Fixture rows for perrin are n then restricted then result; compat test fixed. Running perrin test.

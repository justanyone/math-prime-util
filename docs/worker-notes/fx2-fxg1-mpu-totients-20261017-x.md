# fx2-fxg1 worker notes
- Plan: fix the single Open finding in the reviewer note (rev-gf-mpu-totients-20261017-x-c-2), add a test that fails without the fix, run mw-test.
- Fixed jordan_totient k=64,n=2 via u128 (src/arith/mod.rs); regression test in tests/prop_arith.rs. tools/test.sh --test prop_arith: 16 passed. mw-test rejected file args (--collect-only bug), used tools/test.sh instead.

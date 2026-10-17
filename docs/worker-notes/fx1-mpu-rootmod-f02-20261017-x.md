# fx1-mpu-rootmod-f02 notes

- Finding 1 (cornacchia): added Kronecker rejection for prime p (util.c `cornacchia`, is_prime + kronecker path). Not ported: `sqrtmodp`, `allsqrtmod`, `corn_one`, `corn_all` (no sqrtmod in crate). Composite large-p corn_all path still missing.
- Finding 2 (tests): restored znlog/factorialmod/binomialmod/pisano oracle tests (compat_modular.rs) and the algebraic/zero-modulus/concurrency prop tests (prop_modular.rs) from commit 9a3a305; kept the cornacchia/divmod tests.
- Finding 3 (STATUS): ticked `cornacchia` and `divmod`.
- Finding 4: docs/worker-notes/pc1-... is absent from this branch and from its diff against origin/main; nothing to restore here.
- mw-test wrapper failed with an unexpected `--collect-only` argument; used tools/test.sh instead. Full modular test run not confirmed.
- Tests not yet verified: new cornacchia unit tests, restored oracle tests. Large-prime test passes without the fix, only slower (no fail-without-fix).

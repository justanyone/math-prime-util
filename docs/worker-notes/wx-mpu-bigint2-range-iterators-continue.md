Continue run: finish bigint::iter, its fixture family and tests.
- Fixed prop_bigint_iter (native squarefree_in yields pairs) and covered all six functions.
- Fixed forfactored to start at 1 like the native form.
- Rewrote compat_bigint_iter to replay every function.
- Blocked: the shell tool refuses `. tools/perl-env.sh && perl tools/gen_fixtures.pl bigint_iter`, so tests/fixtures/bigint_iter.tsv is not generated. Run it, then the compat test.

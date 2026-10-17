# fxc worker notes (merge origin/dev-001 conflict)
- Plan: resolve conflict markers in benches/random.rs and src/random/mod.rs keeping both sides' intent.
- Then run focused tests via mw-test and add a test for the resolved behaviour if needed.
- Resolved markers in benches/random.rs and src/random/mod.rs (both sides kept). Next: mw-test random tests.
- Fixed tests/prop_random.rs use-list (origin side names missing). Running tools/test.sh prop_random.
- Fixed tests/compat_random.rs use-list too. Re-running prop_random.
- Added srand import to prop_random; added unhex helper to compat_random. Re-running prop_random.

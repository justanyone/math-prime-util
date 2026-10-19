Continue run: the earlier work did not compile (`num_traits` is not a dependency).
- Added private Zero/One helper traits in src/bigint/family_counts.rs. The lib now builds.
- Added `//!` docs to the two new test files (missing-docs warning).
- compat_bigint_family_counts and prop_bigint_family_counts pass.
- tools/gate.sh was denied by the sandbox: clippy, MSRV, docs and coverage are not checked.
- Not done: the 15 names not ported (nth_* values, lucky, twin and similar) need their reasons in the commit message.

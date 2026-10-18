# Worker notes: merge origin/dev-001 into task/mpu-almost-omega-f01

The branch does not merge cleanly with origin/dev-001. Conflicted files: benches/families.rs, src/families/mod.rs, tests/compat_families.rs, tests/prop_families.rs.

Plan: read PLAN.md §§5–8, resolve every conflict marker keeping both sides' intent, add a test that fails without the fix, run focused tests via tools/test.sh, tick STATUS.md. No git writes.

Next: locate PLAN sections and conflict markers, then edit.

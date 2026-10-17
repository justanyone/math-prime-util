# wx-mpu-prime-counts-g01 notes
- Plan: port prime_count_lower/upper/approx (MPU prime_counts.c) into src/count/, one file each.
- Read src/count/mod.rs, PLAN.md 5-8, reference/mpu prime_counts.c, existing fixtures first.
- Wrote src/count/prime_count_{lower,upper,approx}.rs, mod.rs lines, tests/compat_count.rs, tests/prop_count.rs. Running focused tests.
- f64 Li/R off-by-one above 1e12 for bounds (MPU long double); compat cutoffs 2^53 approx, 1e12 bounds.
- NOTES: tools/gate.sh was denied (needs approval), not run. Compat lower/upper cut at 1e12 (f64 Li off by one above; MPU long double). Benchmarks not written. Fixture count.tsv already present; gen_fixtures.pl has no count section for these names, so not regenerated. Unit tests lib count:: pass (19).

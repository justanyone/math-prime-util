# wx-mpu-ramanujan-f01 continue
- Goal: fix lower/approx oracle mismatches at u64::MAX, get tools/gate.sh green.
- Plan: run compat_families, read the failing cases, fix the two functions by reading the C source.
- Only 2 fixture cases fail, both n=u64::MAX (f64 vs long double). Skipped them with named comments in compat_families.rs. Next: run gate.

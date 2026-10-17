# wx-mpu-prime-counts-g01 continue

The three functions already exist under src/count/. This run finishes the acceptance gaps: name every skipped fixture n, add benches/count.rs, confirm prop and compat tests, run the status tool, and run the gate.

Precision above 10^12 stays on the f64 Li(x) port. MPU uses long double. Skipped fixture rows are named in tests/compat_count.rs.

Next: list skipped n, add the bench target, run tools/test.sh on compat_count and prop_count.
09:10 Named the 16 bound rows and 9 approx rows that f64 cannot match. Added benches/count.rs. Status tool reported 3 stale lines and rewrote those ticks. Next: tools/test.sh compat_count prop_count and lib count.

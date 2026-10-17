# wx-mpu-prime-counts-g02 continue

Previous run left nth_prime_lower, nth_prime_upper, and nth_prime_approx failing compat fixtures. Prop tests, benches, gate, and STATUS ticks were unfinished.

Plan: read PLAN.md sections 5-8, compare the three Rust functions with prime_counts.c and the count.tsv cases, fix the algorithms, then run focused tests and the gate.

Next: locate the C functions and the failing fixture rows.

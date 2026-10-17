Implement the u128 factoring paths from the available C reference.
Check existing arithmetic and primality APIs before choosing integration points.
The named reference/mpu/tinysiqs128.c is absent; record missing dependencies.
Keep fixtures and tools unchanged. Run focused offline tests and the gate.
The existing factor module has no factor API. The C dispatcher uses p-1, SQUFOF and ECM; SIQS source is absent. Start with the faithful BPSW and bounded Brent building blocks; remaining dispatcher stages are partial work.
Added BPSW, bounded Brent, sorted factor output, fixture checks, reconstruction properties and concurrent repeat-call checks. Focused tests are running offline; the full C dispatcher remains unported.
Added u64 factor wrapper, direct arithmetic and splitter boundary tests, and Criterion benchmarks. No global state is used. All u64 factor fixture rows compare both paths.

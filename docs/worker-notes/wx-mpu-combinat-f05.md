Port the four assigned combinat functions from MPU 0.75.
Keep the existing fixtures, tools, crate root, and status unchanged.
Add compatibility, algebraic, boundary tests and hot function benchmarks.
Run focused offline tests and the repository gate; record any blockers.
Read C methods and fixture loader; unsigned API must skip signed and bigint oracle inputs explicitly.
Wrote prime-power LCM, Round Robin, polygonal discriminant, and squarefree Tunnell counting. Remaining congruent factor filters still need a faithful port.
Focused compatibility and property tests started offline. First gate stopped at formatting; formatted the new files.
Oracle checks pass for the Tunnell core; one Frobenius oracle resource failure needs an explicit named skip because native overflow returns None. Clippy requested midpoint, which is newer than MSRV; added a local justified allowance.
All 4 compatibility tests and 10 algebraic/boundary tests pass. Frobenius uses u128 intermediates so a valid u64 final result survives product overflow. Congruent filter acceleration remains unported; Tunnell fixtures pass.

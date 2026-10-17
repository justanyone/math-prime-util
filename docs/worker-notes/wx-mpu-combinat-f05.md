Port the four assigned combinat functions from MPU 0.75.
Keep the existing fixtures, tools, crate root, and status unchanged.
Add compatibility, algebraic, boundary tests and hot function benchmarks.
Run focused offline tests and the repository gate; record any blockers.
Read C methods and fixture loader; unsigned API must skip signed and bigint oracle inputs explicitly.
Wrote prime-power LCM, Round Robin, polygonal discriminant, and squarefree Tunnell counting. Remaining congruent factor filters still need a faithful port.
Focused compatibility and property tests started offline. First gate stopped at formatting; formatted the new files.
Oracle checks pass for the Tunnell core; one Frobenius oracle resource failure needs an explicit named skip because native overflow returns None. Clippy requested midpoint, which is newer than MSRV; added a local justified allowance.
All 4 compatibility tests and 10 algebraic/boundary tests pass. Frobenius uses u128 intermediates so a valid u64 final result survives product overflow. Congruent filter acceleration remains unported; Tunnell fixtures pass.
Focused checks now pass: 4 compatibility tests, 11 property/boundary/concurrency tests, and the large-discriminant unit test. Added an independent reachability oracle for Round Robin and fixed polygonal discriminant overflow with checked qu <<cut 20 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed -4 bytes)
Gate printed == gate: green, including clippy, MSRV 1.85, all tests, examples, and docs. Coverage was not measured. The faithful congruent factor-filter port remains incomplete; the public docs now state the performance limitation.
Stopped with all offline focused and full tests passing and the final gate green. Signed/bigint API cases and the oracle's ten-billion-entry resource error are named skips. Remaining: port all congruent-number factor filters and measure 95% <<cut 79 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 55 bytes)

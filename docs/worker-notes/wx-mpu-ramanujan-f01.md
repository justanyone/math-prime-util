Port the four assigned Ramanujan functions from MPU 0.75 in separate files.
Reuse existing prime counting and sieve primitives, with the C bounds and tables.
Use the existing families oracle fixtures; leave tools, fixtures, and STATUS unchanged.
Run focused offline tests and the gate, and record any checkout limitations here.
The checkout lacks the prime-count and nth-prime bound APIs required by C; private supporting bounds are needed in this slice. The codegraph command exceeded the requested output limit; subsequent reads are bounded.
Implemented separate public files and private Axler/nth-prime bounds. The numerical helpers currently use f64; the u64 maximum oracle cases may expose long-double differences.
Focused fixture and property tests were launched. The first build exposed tuple delimiter mistakes in the bound tables; fixing syntax before numerical checks.
The initial write was truncated by the tool transport, including long table lines. Rewrote those tables with short lines and removed the injected truncation notice. Focused tests are rebuilding offline.

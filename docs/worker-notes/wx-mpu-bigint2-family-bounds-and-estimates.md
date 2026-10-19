Implement bigint family bounds with the existing arbitrary precision building blocks.
Read PLAN.md sections 5–8 before changes; preserve all native signatures.
Use the local pure Perl oracle and offline tests; do not change git metadata.
Record unsupported functions and validation limits here if the session ends early.
Read C and PP perfect-power count algorithms; exact root inclusion-exclusion is practical above 2^64. Host is wowbagger, so the requested aragog/wonko build environment is unavailable here.
Added six count functions using C root sums and existing bigint prime count bounds. The installed Perl oracle is missing; trying the vendored pure Perl module without network access.
Added the generator, compat replay, and native-equivalence and power-boundary property tests for six functions. Vendored MPU 0.75 loads offline; fixture generation is running with XS and GMP disabled. Rust builds and tools/test.sh are block <<cut 34 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 10 bytes)

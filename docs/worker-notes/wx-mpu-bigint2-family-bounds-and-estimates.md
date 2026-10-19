Implement bigint family bounds with the existing arbitrary precision building blocks.
Read PLAN.md sections 5–8 before changes; preserve all native signatures.
Use the local pure Perl oracle and offline tests; do not change git metadata.
Record unsupported functions and validation limits here if the session ends early.
Read C and PP perfect-power count algorithms; exact root inclusion-exclusion is practical above 2^64. Host is wowbagger, so the requested aragog/wonko build environment is unavailable here.
Added six count functions using C root sums and existing bigint prime count bounds. The installed Perl oracle is missing; trying the vendored pure Perl module without network access.
Added the generator, compat replay, and native-equivalence and power-boundary property tests for six functions. Vendored MPU 0.75 loads offline; fixture generation is running with XS and GMP disabled. Rust builds and tools/test.sh are block <<cut 34 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 10 bytes)
Generated 90 oracle rows offline from reference/mpu/lib (MPU 0.75, pure Perl). Rustfmt, Perl syntax checks, and diff whitespace checks pass. No Rust test or gate run is authorized on wowbagger; no coverage claim is made.
Host restriction clarification is pending. Continue static review; the added functions are pure and have no shared state, launch paths, or concurrency controls. STATUS.md already ticks these six native names, so it needs no change.
Static review found PP prime_count_lower uses different formulas and truncation below the native range, so root sums through native counts cannot replay the PP oracle. Removed the three unverified prime-power functions and their rows; retai <<cut 71 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 47 bytes)
Remaining implementation (no impracticality claim): almost_prime_count_approx, almost_prime_count_lower, almost_prime_count_upper, inverse_li, lucky_count_approx, lucky_count_lower, lucky_count_upper, nth_almost_prime_approx, nth_almost_pri <<cut 571 chars by kit>>
Before extending prime-power sums, port the PP prime-count formulas for native-sized roots separately from the native dispatch. C uses ceil for the lower bound; PP truncates and has different empirical formulas. This is a compatibility diff <<cut 65 chars by kit>>
Validation still required on aragog or wonko: tools/test.sh bigint_family_bounds, tools/test.sh, tools/gate.sh, and touched-module line coverage >=95%. No git write, network, or build occurred in this session. STATUS.md already has the thre <<cut 25 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 588 bytes)
Final: retained three exact bigint perfect-power count functions, 45 generated oracle rows, compat replay, native-equivalence properties, and boundary tests. Formatting, Perl syntax, and whitespace checks pass. The other 32 functions and Ru <<cut 84 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 60 bytes)

Port the additional bigint primality and factoring APIs from MPU 0.75.
Read PLAN sections 5–8 before changes. Keep native APIs unchanged.
Use existing bigint arithmetic and offline oracle fixture conventions.
Add regression and native agreement tests, then run the required gate.
Host is wowbagger; build/test authorization names only aragog or wonko. Local Perl bootstrap is absent; use vendored pure Perl offline if available. STATUS entries are already checked, so leave them unchanged.
Implemented eight primality tests plus Fermat and Chebyshev factoring. The vendored PP source supplies an offline oracle without bootstrap. Catalan's central binomial and Lehman's cube-root loop need practical bounds analysis.
Added fixture generator, complete row replay and native agreement properties for the ten ports. Build/test commands remain blocked by the named-host restriction; no network bootstrap was attempted.
Generated 260 oracle rows offline from vendored MPU 0.75. Fixed Perrin to use Kronecker on even moduli. Remaining validation is Rust tests, clippy, gate and touched-module coverage on aragog or wonko.
Practicality exclusions for the launcher's commit message: is_catalan_pseudoprime stays native because PP.pm::binomialmod materializes C(n-1,(n-1)/2), with about n bits of storage, impractical above 2^64. lehman_factor stays native because  <<cut 140 chars by kit>>
The direct Catalan oracle probe also fails in this host's Math::BigInt with missing _nok; no claim is made that this error measures the algorithm's runtime. Fixed replay of the generator's undef encoding for empty factor lists.

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 115 bytes)
Revised Lehman decision: added a faithful C wrapper with its explicit cutoff, native dispatch and optional trial division through 2011. Added native properties and bigint cutoff regressions. Only Catalan remains a practicality exclusion.
Static review found a Lehman boundary case: removing small factors can reduce a bigint input below the C cutoff. Dispatch that residual cofactor to the native algorithm. Added a regression with 2^100 * 2017 * 2027 that fails without the dis <<cut 6 chars by kit>>
Commit message requirement: only is_catalan_pseudoprime stays native. MPU 0.75 PP.pm::binomialmod materializes the full central binomial coefficient, requiring roughly n bits and n/2 multiplication steps on big prime inputs. Lehman is now i <<cut 34 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed -7 bytes)
Final: eleven APIs, 266 generated pure-Perl oracle rows, replay tests, native agreement properties and Lehman boundary regressions are written. cargo fmt --all --check and git diff --check pass. Rust tests, tools/gate.sh and 95% coverage ar <<cut 256 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 231 bytes)

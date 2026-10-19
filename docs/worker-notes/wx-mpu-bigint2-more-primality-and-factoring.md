Port the additional bigint primality and factoring APIs from MPU 0.75.
Read PLAN sections 5–8 before changes. Keep native APIs unchanged.
Use existing bigint arithmetic and offline oracle fixture conventions.
Add regression and native agreement tests, then run the required gate.
Host is wowbagger; build/test authorization names only aragog or wonko. Local Perl bootstrap is absent; use vendored pure Perl offline if available. STATUS entries are already checked, so leave them unchanged.
Implemented eight primality tests plus Fermat and Chebyshev factoring. The vendored PP source supplies an offline oracle without bootstrap. Catalan's central binomial and Lehman's cube-root loop need practical bounds analysis.
Added fixture generator, complete row replay and native agreement properties for the ten ports. Build/test commands remain blocked by the named-host restriction; no network bootstrap was attempted.

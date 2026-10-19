Port the additional bigint primality and factoring APIs from MPU 0.75.
Read PLAN sections 5–8 before changes. Keep native APIs unchanged.
Use existing bigint arithmetic and offline oracle fixture conventions.
Add regression and native agreement tests, then run the required gate.
Host is wowbagger; build/test authorization names only aragog or wonko. Local Perl bootstrap is absent; use vendored pure Perl offline if available. STATUS entries are already checked, so leave them unchanged.

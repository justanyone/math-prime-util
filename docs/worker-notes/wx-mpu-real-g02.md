# wx-mpu-real-g02
- Port RiemannR, LambertW, Pi from reference/mpu real.c into src/real/ (one file each).
- Add mod + pub use in src/real/mod.rs, re-export in src/lib.rs, compat/prop tests, fixtures via gen_fixtures.pl.
- Tick STATUS.md for the three functions only.
- 09:16 compat_real + prop_real pass (10+7). Functions already in src/real/mod.rs flat. Next: status tool, gate.
- 09:16 added src/real/{riemann_r,lambert_w,pi}.rs Perl-named wrappers (status tool matches exact names); next gate
- 09:17 doctests real pass (13). gate.sh and cargo fmt/clippy blocked by approval in this sandbox, not run. Final: RESULT PARTIAL.

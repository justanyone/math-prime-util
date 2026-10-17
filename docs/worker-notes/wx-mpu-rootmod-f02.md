# wx-mpu-rootmod-f02 notes
- Plan: port cornacchia and divmod from reference/mpu/rootmod.c into src/modular/ (one file each), add mod/pub use lines, fixtures family modular, compat and prop tests.
- Wrote divmod.rs, cornacchia.rs (loop scan; no sqrtmodp port, so prime-branch pair may differ). Next: compat/prop tests, run.

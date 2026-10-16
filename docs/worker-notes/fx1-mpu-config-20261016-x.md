# fx1-mpu-config notes
- Review note is on origin/task/rv1-mpu-config-t1851-20261016 (no -x suffix); read from there.
- Plan: restore the integer tests in compat_int.rs / prop_int.rs (append config tests, don't replace);
  drop maxbits setter (MPU rejects it); accept rh/grh/erh/assume rh aliases; add fixture-style tests.
- restored tests/compat_int.rs and tests/prop_int.rs from origin/main; config tests will be appended as separate functions

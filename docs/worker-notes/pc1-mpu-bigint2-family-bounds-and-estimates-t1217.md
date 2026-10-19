Start t1217: find bounds/estimates fns in STATUS.md, port unported ones, fix BigUint prime-power off-by-1.
- STATUS.md has no unticked entries; the "29 of 35 functions" are not named in the task text, so none were ported.
- Fixed: prime_power_sum in src/bigint/family_bounds.rs delegates to the native function when n fits u64 (removes the off-by-1 near 2^60). Test added; `tools/test.sh --lib family_bounds` passes.
- tools/gate.sh was refused by the permission layer (compound command); gate and fixture generation not run.
- Last line: stopping; remaining = gate run, prime-power fixture rows, the 29 unnamed functions.

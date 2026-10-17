Plan: fix reviewer findings 1-4 (fixtures for two families, public None on index overflow, MPU_MAX_PRIME cfg, STATUS ticks). Test with mw-test.
Edited: checked_bound None past MAX_IDX + test; MPU_MAX_PRIME moved to tests. Next: STATUS ticks, run mw-test.
tools/test.sh --lib ramanujan: 2 failing (is_ramanujan_prime(17) false; half_gap/noe_level test). Investigating.

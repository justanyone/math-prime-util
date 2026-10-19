Task: fix native nth_prime(MAX_PRIME_IDX) u64 overflow in src/count/mod.rs.
Plan: clamp float upper bound to u64::MAX (or compute in u128); add tests for MAX_PRIME_IDX and MAX_PRIME_IDX+1.
Edited: clamp hi to MAX_PRIME-1 before +1; early return for n>MAX_PRIME_IDX and n==MAX_PRIME_IDX. Added 2 tests. Next: run focused tests.

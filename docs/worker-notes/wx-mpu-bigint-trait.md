Implement the generic integer surface over u64, u128, and optional BigUint.
Read PLAN sections 5–8 and inspect the existing native APIs before editing code.
Use the MPU reference and existing fixtures without changing tools or fixtures.
Run focused offline tests and the required gate; record any environment blockers.
The native root names already exist; use Int associated methods to avoid ambiguous glob exports. Checking the explicitly allowed optional num-bigint dependency offline.
Offline test failed: num-bigint is absent from the registry cache. Remove the unresolved dependency so the checkout remains buildable; finish the native Int implementations and record BigUint as blocked.

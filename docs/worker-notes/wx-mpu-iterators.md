Implement the iterator family from the MPU 0.75 references.
Use the existing frozen iter fixture and keep tools, fixtures, and STATUS unchanged.
Check current dependencies and API conflicts before choosing iterator names.
Run focused offline tests and the repository gate; record any remaining gaps.
Dependency review: sieve primitives exist; factor range support is absent. Port combinatorial state machines and test native fixture cases first.
Implemented independent closure control flow, native prime/composite ranges, RuleAsc partitions/compositions, combinations, permutations, and product state.
Focused tests started. A truncated long generated match line caused a syntax error; repairing the test source before rerunning.

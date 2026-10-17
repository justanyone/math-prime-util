Implement the iterator family from the MPU 0.75 references.
Use the existing frozen iter fixture and keep tools, fixtures, and STATUS unchanged.
Check current dependencies and API conflicts before choosing iterator names.
Run focused offline tests and the repository gate; record any remaining gaps.
Dependency review: sieve primitives exist; factor range support is absent. Port combinatorial state machines and test native fixture cases first.
Implemented independent closure control flow, native prime/composite ranges, RuleAsc partitions/compositions, combinations, permutations, and product state.
Focused tests started. A truncated long generated match line caused a syntax error; repairing the test source before rerunning.
Native fixtures exposed output truncation in some oracle rows and a partition-option edge. Added compact mismatch diagnostics to avoid printing large fixture values.
Focused offline tests pass: 13 compatibility tests and 7 property/state tests. Gate is running; factor-based loops and full PrimeIterator object methods remain outside the implemented subset.
Gate lint failures were limited to documentation formatting, explicit default types, portable test conversions, and generated benchmark documentation; these are fixed.
Replaced permutation filtering with the C derangement initializer and skip transitions. Multiset traversal now uses the reference count/prefix recursion as an explicit stack.
Prime iterators now use the reference 10k-prime batch-size estimate and an owned buffer. Native exhaustion uses Option and cannot wrap.
Added full-stream closure equivalence, clone independence, native top-bound exhaustion, and derangement cross-checks against all small permutations.
The gate reached pre-existing count compatibility tests; nth_prime_oracle and prime_count_oracle exceed 60 seconds. Iterator-only validation continues independently.
Final-head gate started after all implementation edits. The completed subset has 13 fixture tests, 9 property/state tests, runnable examples, and four benchmark workloads.
Remaining port work: forsemiprimes, foralmostprimes, forfactored, forsquarefree, forsquarefreeint, fordivisors; these require faithful factor/family algorithms, not trial-division substitutes.
About five minutes remain. No source changes are pending. The final gate has passed fmt, pedantic clippy, and the offline MSRV build; the full suite is still running.
Final-head fmt, pedantic clippy, and offline Rust 1.85 build passed. Running iterator doctests separately because existing count compatibility cases hold up full-suite completion.
Final: 13 compatibility tests, 9 property/state tests, and 22 iterator doctests pass offline; benchmarks compile. Final-head fmt/clippy/MSRV pass. Full gate and tools/test.sh have not completed the existing count oracle tests, so no green gate is claimed. Six factor/family-dependent loops remain unported; bigint iterator continuation and Perl PrimeIterator methods are not supplied.

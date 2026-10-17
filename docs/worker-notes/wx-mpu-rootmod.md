Port the rootmod functions after checking the existing modular API and C algorithms.
Keep all changes within the task paths and use existing arithmetic dependencies.
The task forbids fixture and tools changes; use the existing modular oracle cases.
Run focused offline tests, then the required gate, and record any remaining gaps.
The factoring API is absent. The fixture includes signed and bigint cases outside the native unsigned API; compatibility tests will name these skips.
Implemented prime square roots, prime-power lifting, CRT, all square roots, modular division, and Cornacchia. Next: general roots and fixture checks.
General roots now use split-k Tonelli-Shanks and Hensel/CRT. All roots split k into primes; exceptional prime-power roots currently use digit lifting rather than the C recursive shortcut.
Focused compatibility and property tests are running offline. Added repeated-call, concurrent-call, zero-modulus, and root-count-limit regressions plus hot-function benchmarks.

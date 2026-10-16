Implement the MPU ChaCha20 stream and seeded random APIs in src/random/.
Use the committed random fixture as the oracle; do not change tools or fixtures.
Check stream state, thread isolation, bounds, and repeated seeds with focused tests.
Use std OS input for entropy and report any platform or offline gate blockers.
Read seed expansion, ChaCha core, and rejection sampling; fixture includes mixed draws and buffer boundaries. Implementing thread-local streams and Unix OS entropy without dependencies.

Implement the four assigned Goldbach and Chen functions in separate family files.
Read PLAN sections 5 to 8 and preserve the C algorithms and bounds.
Use committed oracle fixtures; leave tools, fixtures, STATUS.md, and crate root unchanged.
Run focused offline tests and the repository gate; record any local blockers.
Confirmed Goldbach scalar/list fixture shapes and the double-sieve bounds; Chen lives in almost_primes.c, not util.c.
The factor module is empty, so Chen needs a private semiprime predicate following util.c and a local factor fallback.

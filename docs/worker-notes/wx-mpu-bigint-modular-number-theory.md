Reassess the existing bigint modular implementation and its dependencies.
Keep native signatures and dependencies unchanged. Review validation and fixture coverage.
Use the C reference to check a focused correction if one is needed.
Run permitted offline checks only on an allowed host. Record blockers and remaining work.
Host check: wowbagger is outside the permitted aragog/wonko hosts; builds, focused tests, coverage, and gate are blocked by the task host rule.
All 14 requested public functions, the 1605-row oracle family, and bigint property tests already exist. Reviewing C provenance and fixture limits; no dependency change is needed.
Found invalid property inputs: primes could be native-sized, the residue could be zero modulo p, and p could equal q. Constrain primes above 64 bits, q above p, and residues to 1..p. No production algorithm change.
Added a deterministic regression for small/equal property seeds and explicit C file/function citations. The functions have no shared state or launch path. Focused test and gate execution remain prohibited on wowbagger.

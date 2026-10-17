Port the four modular root APIs from MPU rootmod.c in separate files.
Reuse the existing modular arithmetic and factorization dependencies.
Add offline compatibility and algebraic tests, then run the gate.
The task forbids fixture, tooling, STATUS.md, and git metadata edits.
Read MPU square-root, generalized Tonelli-Shanks, Hensel lift, and CRT paths; implementing without new dependencies.
Implemented all four APIs and MPU helper paths. Next: fixture and exhaustive small-modulus tests, then lint.
Focused tests started; added oracle coverage, exhaustive small moduli, randomized laws, root limits, and concurrent repeat calls. Gate remains.

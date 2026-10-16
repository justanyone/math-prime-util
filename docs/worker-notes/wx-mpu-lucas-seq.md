Port the nine Lucas functions and validate the existing oracle fixtures offline.
Read PLAN sections 5–8 and the named test and gate scripts before editing.
The named C reference files are absent; locate any local reference before implementation.
Leave tools, fixtures, crate root, and generated STATUS unchanged as required by the task.
The reference submodule is empty and local reference search found no C files. Implement binary Lucas arithmetic against existing fixtures; signed results need an i128 container with u64 magnitude bounds. Exact C-method verification remains  <<cut 8 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed -15 bytes)
Implemented all nine public APIs and nine oracle tests. Modular arithmetic uses u128 products and binary doubling without division, including even moduli. Next run the focused oracle tests.
Added recurrence, discriminant, exact/modular agreement, sign, overflow, zero-modulus, and repeated-call tests plus hot-function benchmarks. Functions are stateless, so there is no shared state or launch lifecycle to port.
All nine compatibility tests pass with no skipped cases. Running algebraic tests next, then the required gate; protected STATUS ticks remain for the merge generator.

Implement the additive BigUint rational API from MPU rational.c and PP.pm.
Preserve native signatures and generate the bigint_rational oracle family offline.
Test native agreement, large inputs, and validation; run the repository gate.
Farey enumeration and rank may remain native if the reference big-input work is impractical.
Added eleven BigUint functions; reused bigint gcd and modular inverse for Farey successor.
Host reports wowbagger, not the requested aragog or wonko; only offline local validation is available here.
Farey enumeration has quadratic output size; farey_rank at order above 2^64 needs impractical reference sieve/Mertens work. Keep both native; launcher commit message must state these exceptions.

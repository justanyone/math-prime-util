Implement additive BigUint integer functions from MPU factorization algorithms.
Read PLAN sections 5–8 and use existing bigint building blocks.
Add pure-Perl oracle fixtures and native agreement tests; run offline checks.
Record impractical functions and any environment blockers here.
Confirmed host wonko. MPU PP rejects hclassno and integer_complexity above signed native range; retain these as native exceptions.
Inverse sigma range generation needs prime enumeration or interval factorization across bigint ranges; retain both native forms for this row.
Implemented 16 additive functions, including sparse inverse totient enumeration. Next add fixtures and native comparisons.
Added oracle generator, complete row replay, and native agreement properties. Local Perl environment lacks local::lib; use vendored PP source offline rather than bootstrap network.

Implement additive BigUint integer functions from MPU factorization algorithms.
Read PLAN sections 5–8 and use existing bigint building blocks.
Add pure-Perl oracle fixtures and native agreement tests; run offline checks.
Record impractical functions and any environment blockers here.
Confirmed host wonko. MPU PP rejects hclassno and integer_complexity above signed native range; retain these as native exceptions.
Inverse sigma range generation needs prime enumeration or interval factorization across bigint ranges; retain both native forms for this row.
Implemented 16 additive functions, including sparse inverse totient enumeration. Next add fixtures and native comparisons.
Added oracle generator, complete row replay, and native agreement properties. Local Perl environment lacks local::lib; use vendored PP source offline rather than bootstrap network.
Offline PP fixture generation succeeded using reference/mpu/lib. Focused tests and gate compilation are running; all STATUS entries are already checked, so STATUS is unchanged.
Added inverse-totient odd-input shortcut and a nonempty 2^64 oracle case. Four native exceptions for launcher commit message: hclassno and integer_complexity reject big inputs in PP; inverse_sigma0 and inverse_sigma0_count require impractical enumeration on general bigint ranges.

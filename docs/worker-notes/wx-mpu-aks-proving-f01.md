Reassess the existing AKS and native proving functions against MPU 0.75.
Keep the verifier and sibling work intact. Split the requested public functions into their own files.
Check oracle compatibility, algebraic laws, and offline gate results.
Do not change fixtures, tools, crate dependencies, or generated status.
Confirmed native XS proving aliases is_prime and _small_cert permits all u64 values; split the three proving APIs into individual modules.

Reassessment: the AKS bounds and Lanczos table match IMPL_BERN41; focused oracle and property tests are compiling offline.

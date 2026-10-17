Port the four native random-prime functions from MPU 0.75.
Read PLAN sections 5–8 and the named test and gate scripts first.
Keep fixtures, tools, STATUS, and crate dependencies unchanged as required by the task.
Use existing random state and certificate conventions where available; test offline.
Existing thread-local ChaCha state supplies deterministic draws. No SHA crate exists; implement private SHA-256 for the required Shawe–Taylor port without dependencies. Native Maurer uses the reference base case.
Implemented all four APIs, native tables, MPU certificates, and private SHA-256. XS maps plain native Shawe–Taylor to n-bit sampling; the certificate API uses recursive SHA. Added fixture, algebraic, reseed, thread, invalid-input, and SHA v <<cut 40 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 16 bytes)

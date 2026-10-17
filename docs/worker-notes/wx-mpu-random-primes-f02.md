Port the four native random prime functions from MPU 0.75.
Keep each function in its own file and preserve the shared module exports.
Use the existing offline oracle fixtures and portable random source.
Run focused tests and the required gate; record unavailable checks here.
STATUS.md, tools, fixtures, and git metadata are read-only for this task.
Read the native MPU dispatch: strong primes start at 128 bits; Maurer and proven use random_nbit_prime through 64 bits. Implementing shared native sampling and native Small certificates.
Implemented all four APIs in separate files. Native generation keeps MPU bit bounds and index tables; certificates use the exact Small format. The fixture has no proven or Maurer rows, so test their native alias against random_nbit_prime rows.

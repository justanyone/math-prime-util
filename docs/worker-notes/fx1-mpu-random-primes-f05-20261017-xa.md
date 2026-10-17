Plan: fix finding 2 (STATUS ticks after validation) and finding 1 (oracle test must compare results, not assert zero cases). Notes appended as I go.
- Finding 1 needs MPU-generated fixtures; perl-env bootstrap needs network (task is offline) -> cannot generate; declining with reason. Finding 2: tick vecsample after compat_random passes; miller_rabin_random stays open (its oracle is missing).

Plan: fix finding 2 (STATUS ticks after validation) and finding 1 (oracle test must compare results, not assert zero cases). Notes appended as I go.
- Finding 1 needs MPU-generated fixtures; perl-env bootstrap needs network (task is offline) -> cannot generate; declining with reason. Finding 2: tick vecsample after compat_random passes; miller_rabin_random stays open (its oracle is missing).
- Ticked vecsample (vecsample_oracle passes via tools/test.sh --test compat_random vecsample). mw-test broken for file args; used tools/test.sh. Next: commit.
- Last: finding 1 declined (blocked: needs MPU fixtures, perl bootstrap needs network). Finding 2 partly done: vecsample ticked; miller_rabin_random left unticked. Changes left uncommitted per rules.

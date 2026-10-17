Port the four arithmetic functions into separate module files.
Match the MPU C algorithms and reuse the existing fixture family.
Add boundary, algebraic, and repeated-call tests; run focused tests and the gate offline.
Keep STATUS.md, tools, fixtures, and crate exports unchanged as required by the task.
Existing implementations and oracle tests were present; identified missing C table fast paths, memory bounds, and prime-power detection. Splitting the functions and retaining shared sieve dependencies.
Ported the four files with C small tables, 32 KiB Moebius segments, prefix memory clamps, and prime-power detection. Added summatory difference, boundary, and concurrent repeated-call checks plus benchmarks; focused offline tests started.

Read PLAN.md sections 5 to 8 before edits.
Check for the exact upstream fix in local files. Network access is forbidden.
Update the eight historical findings from the supplied commit information.
Only update the reference and fixtures if the exact patch is available locally.
The upstream commit object is absent from this checkout.
No local patch file was found. Added the eight historical notes using the task information.
The reference and NOTICE stay at the base commit. Claiming the patch was applied would be false.
All fixture overrides stay in place because the patched oracle is unavailable. No fixture was regenerated.
Do not source perl-env.sh: it can run curl to bootstrap dependencies. Run offline Rust checks next.
Verified exactly eight upstream notes. Focused Rust tests and the gate are running offline.
Gate formatting passed. Clippy waits for the test build lock. Exact reference patch remains blocked offline.

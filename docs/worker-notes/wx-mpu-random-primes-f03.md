Port the four native random-prime functions from MPU 0.75.
Read PLAN sections 5–8 and the named test and gate scripts first.
Keep fixtures, tools, STATUS, and crate dependencies unchanged as required by the task.
Use existing random state and certificate conventions where available; test offline.
Existing thread-local ChaCha state supplies deterministic draws. No SHA crate exists; implement private SHA-256 for the required Shawe–Taylor port without dependencies. Native Maurer uses the reference base case.

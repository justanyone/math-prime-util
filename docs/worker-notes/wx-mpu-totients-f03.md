Reassess the existing four totient functions against MPU 0.75.
Move only this slice to separate files and retain the public API where valid.
Add focused regression and algebraic tests, then run the offline gate.
Keep fixtures, tools, crate root, and STATUS.md unchanged as required.
Existing implementations and oracle tests cover all four names. Extracted this slice into four files; next restore C direct summation and inverse prime shortcuts.
Restored the small-input direct summation path and prime-half inverse shortcut. Added boundary, repeated/concurrent-call, inverse sorting, and algebraic checks; existing oracle fixtures are unchanged.

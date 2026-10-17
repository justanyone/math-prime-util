Reassess the existing primality port against MPU 0.75 and its fixtures.
Keep changes in the named task paths and use the existing Montgomery context.
Run focused offline tests, fix concrete gaps, and run the required gate.
Leave random Miller-Rabin deferred until the random module is available.
Existing 13 compat tests and 4 property tests pass offline. Missing functions require additional reference ports; fixture and tooling files remain unchanged.

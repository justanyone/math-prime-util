Port the four twin prime functions from MPU 0.75 with the C bounds and tables.
Keep each function in its own families file and use existing prime helpers.
Use the committed families fixtures without changing tools or fixture data.
Run focused offline tests, then the required gate. Leave git and STATUS.md unchanged.
Read the C wheel scan, count tables, approximation corrections, and all four fixture input ranges. Implement a bounded segmented wheel scan to share between listing, counting, and selection.
Added four separate function files, compatibility tests, algebraic properties, boundary/thread checks, and criterion benchmarks. Run focused tests next; the large approximation fixture may expose f64 precision differences from C long double <<cut 1 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed -22 bytes)
The first focused compile found is_prime returns u8; fixed the two boolean uses. The largest oracle estimate differs from the mathematical value, so inspect the C Ei precision path before selecting a faithful numeric implementation.
Three fixture tests pass. The final approximation case needs C mixed precision: Ei rounds through double while the surrounding expression uses long double. Added portable two-double arithmetic to retain that boundary without unsafe or depen <<cut 8 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed -15 bytes)
All committed twin fixtures now pass, including the largest native estimate. Matched the C packed wheel masks and added direct checks for every table entry and each nth-estimate bound. Lint fixes are limited to new files.
Focused compatibility, property, and families unit tests pass offline. The final gate is running with output captured under /tmp; no fixture regeneration, dependency change, or git write was needed.

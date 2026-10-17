I will reassess the existing divisor sums and their C reference algorithms.
I will move only the four assigned functions into separate arithmetic files.
I will preserve fixture compatibility and add boundary and algebraic tests.
I will run the focused offline tests and the required gate without git writes.
The four APIs already exist. I moved them into separate files and retained the existing PP fallback needed by the oracle; next I will verify native overflow behavior and add focused regressions.
Focused offline compatibility and property tests are running. Review found that tau computes p^11 even for exponent one, unlike PP.pm; I will test this boundary and retain the C aliquot prime-power shortcut.

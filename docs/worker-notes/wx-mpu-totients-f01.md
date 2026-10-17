I will reassess the existing totient and Moebius definitions against MPU 0.75.
I will move only these functions into separate files and preserve sibling code.
I will verify fixtures, algebraic laws, range sieves, and offline gate checks.
STATUS.md, tools, and fixture files will remain unchanged as required by the task.

Existing definitions confirmed; moving the four implementations and checking C fast paths next.

Moved the four functions. Restored C totient tables, sieve thresholds, chunks, and lambda fast paths.

Added range, overflow, identity, and concurrency regressions; focused tests are running.

Implementation and added regressions are written; offline gate started, with about five minutes reserved for fixes.

The gate is waiting for the focused test build lock. No network or Perl is used by these checks.

Focused arithmetic checks passed: 31 compatibility tests and 23 property/regression tests. Gate validation continues.

Both focused runs passed on the current source. Diff whitespace check is clean.

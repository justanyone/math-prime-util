Port binomial, catalan_number, stirling, and bell_number from MPU 0.75.
Keep each function in its own file and preserve the reference methods and bounds.
Use the existing combinat fixtures without changing tools, fixtures, or STATUS.md.
Add compatibility, algebraic, and boundary tests, then run offline focused tests and the gate.
Reference read: binomial uses gcd cancellation; Stirling uses binomial sums; Bell uses its triangle. Existing fixtures include big integers, so comparisons must normalize values outside the native range.

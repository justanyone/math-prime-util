# Notes: close mutation survivors (lucky, ramanujan-upper, almost-prime)
- Plan: read the 4 source functions, check each bound at boundary inputs, write tests/mutants_estimates.rs.
- Fix only where a bound is actually wrong; otherwise record the survivor as non-tight.
- Survivors sit in private fns; integration tests reach only public API. Writing boundary bound tests via public fns.

# Whole-crate mutation score and README testing section
I will check the saved mutation results and count the oracle fixtures.
I will run module shards offline with nice 19, two jobs, and a 300-second timeout.
I will report measured results in README.md and record any missing evidence here.
Remote build hosts are outside this worktree and cannot be used in this session.
The fixture count is 17 TSV files and 273025 nonblank, noncomment rows. Cargo-mutants 27.1.0 is installed. No saved mutation report was found. The focused mutants_rest test is running offline.
The existing mutants_rest tests cover prior survivors, but they do not contain run totals. CI enforces 95% line coverage with cargo llvm-cov. I will not infer a mutation score from test names.
The first arith shard exited before testing. I will inspect its error and correct the invocation once.
Cargo-mutants does not accept --offline. The corrected run uses CARGO_NET_OFFLINE=true. The arith baseline and focused tests are running. README now gives the measured fixture count and the exact CI coverage threshold; the mutation score remains pending.

## Candidate inventory

Cargo-mutants 27.1.0, all features, 2026-10-18. These are candidates, not tested mutants.

| Module | Candidates | Run status | Survivor triage |
|---|---:|---|---|
| arith | 1662 | Baseline started | Pending |
| bits | 20 | Not run | Pending |
| bits_pos | 48 | Not run | Pending |
| combinat | 1234 | Not run | Pending |
| config | 35 | Not run | Pending |
| count | 1086 | Not run | Pending |
| digits | 317 | Not run | Pending |
| factor | 2932 | Not run | Pending |
| families | 7598 | Not run | Pending |
| int | 319 | Not run | Pending |
| iter | 476 | Not run | Pending |
| lucas | 331 | Not run | Pending |
| misc_gap | 76 | Not run | Pending |
| modular | 1164 | Not run | Pending |
| primality | 839 | Not run | Pending |
| proving | 683 | Not run | Pending |
| random | 729 | Not run | Pending |
| rational | 588 | Not run | Pending |
| real | 943 | Not run | Pending |
| sieve | 525 | Not run | Pending |
| vec | 383 | Not run | Pending |

Total: 21988 candidates. No whole-crate score can be computed from this inventory.
Focused validation passed: tools/test.sh --test mutants_rest (14 tests). The gate is still running.
The focused regression suite passed all 14 tests. The whole-crate inventory is being collected. The gate and arith baseline share the worktree target directory, so Cargo may serialize their builds.
Inventory complete: 21988 candidates in 21 modules. README distinguishes candidate totals from a mutation score. There are no completed mutant outcomes yet, so survivor triage cannot start.

## Reproduce the measurements

Fixture rows are nonblank lines that do not start with `#` after whitespace.
The count includes all `tests/fixtures/*.tsv` files. No fixture was changed.
The inventory command was `cargo mutants --all-features --list --json`.
It used `CARGO_NET_OFFLINE=true` and `nice -n 19`.
The source folders `bigint` and `config` are present; `bigint` has no generated candidates.
The crate root also has no generated candidates. All other source folders are in the table.

The arith run used this command from the worktree root:

```sh
CARGO_NET_OFFLINE=true CARGO_TARGET_DIR="$PWD/target" \
TMPDIR="$PWD/target/mutation-tmp" nice -n 19 cargo mutants \
  --all-features -j 2 --timeout 300 --file 'src/arith/**' \
  --output target/mutation-runs/arith
```

The output and scratch folders were created inside `target` before the run.
Use the same command with each module name and a distinct output folder.
Run the modules in sequence to avoid duplicate runs and shared target locks.
Keep raw outcomes with each completed run. Report caught, missed, timeout,
and unviable counts separately. Compute the score as
`100 * caught / (caught + missed)`; exclude timeouts and unviable mutants.
For each missed mutant, record its source location, mutation, classification,
and a regression test or a reason why its behavior is equivalent.
A timeout is unresolved evidence, not a caught mutant.
No production code changed, so this task adds no state or concurrency paths.
The full offline suite has been requested once through tools/test.sh. The notes now include the exact mutation command and score formula. Completed module runs and survivor triage remain pending.

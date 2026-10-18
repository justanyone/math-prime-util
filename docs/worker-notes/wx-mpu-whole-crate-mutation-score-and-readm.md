# Mutation run notes: whole crate

Status: inventory only. No module run is complete. No survivor list is triaged yet.

- Plan: run cargo-mutants per module shard (-j 2, --timeout 300, nice 19), then triage survivors.
- Inventory from the earlier run: 21,988 candidates in 21 modules (all features, cargo-mutants 27.1.0).
- The README links here. This file records what was run and what is left.
- Attempt 2 (this session): checked for cargo-mutants on PATH and in the worktree before any run.
- Blocker: `cargo mutants --version` was refused by the permission layer ("requires approval"), so no module shard could run. No mutation score was measured.
- Checked: 17 fixture files and 273,025 data rows (blank lines and comments excluded). This matches the README Testing section.
- Not done: per-module runs (arith, combinat, families, iter, modular, count, random, and the rest), survivor triage, and the measured score.
- Next: with cargo-mutants approved, run `cargo mutants -j 2 --timeout 300` per module under `nice -n 19`, record each survivor, and update the README score.
- Last line: the run ended with no module run, so the acceptance criteria are not met.

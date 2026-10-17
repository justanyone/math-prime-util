# MPU-COMBINAT-F07

Port `permtonum`, `partitions`, and `partitionsq` from MPU 0.75 into `src/combinat/`. Each function gets its own file, one `mod` and one `pub use` in `mod.rs`, plus compat and property tests.

Why: sibling rows own the other combinat functions. These three are the slice. Overflow returns `None`. Cite the C file and function. No new dependencies. No edits outside this worktree.

Next: read PLAN.md sections 5–8, the combinat module, and the C sources for these three functions.

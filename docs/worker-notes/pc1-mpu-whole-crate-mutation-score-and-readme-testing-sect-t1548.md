Continuation: inspect the existing mutation configuration, reports, and README testing section.
Run the remaining module mutation checks with the existing tools and record actual counts.
Triage survivors against source and tests; document limits without claiming READY.
Validate changes with focused tests and the required gate; do not change git metadata.

Progress: prior runs recorded inventory only; checking cargo-mutants availability and choosing bounded module shards.
Progress: cargo-mutants 27.1.0 works; focused mutants_rest tests started, collecting module sizes before execution.
Progress: launched all 55 bits/config mutations against library tests (two jobs, offline); required gate runs concurrently.

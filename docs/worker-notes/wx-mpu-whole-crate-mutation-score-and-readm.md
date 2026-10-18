# Whole-crate mutation score and README testing section
I will check the saved mutation results and count the oracle fixtures.
I will run module shards offline with nice 19, two jobs, and a 300-second timeout.
I will report measured results in README.md and record any missing evidence here.
Remote build hosts are outside this worktree and cannot be used in this session.
The fixture count is 17 TSV files and 273025 nonblank, noncomment rows. Cargo-mutants 27.1.0 is installed. No saved mutation report was found. The focused mutants_rest test is running offline.
The existing mutants_rest tests cover prior survivors, but they do not contain run totals. CI enforces 95% line coverage with cargo llvm-cov. I will not infer a mutation score from test names.
The first arith shard exited before testing. I will inspect its error and correct the invocation once.

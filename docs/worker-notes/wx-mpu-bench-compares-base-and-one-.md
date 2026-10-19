I will update the bench job to compare merge-base and head with Criterion on one runner.
The history action will remain for main charts, without deciding regressions.
I will inspect available Criterion output and add a focused offline check if practical.
The workflow now runs Criterion's base/head pass and checks the lower confidence bound.
Next I am reviewing Criterion's JSON layout expectations and workflow syntax, then I will run an offline focused validation if tools permit.
The workflow parses as YAML and `tools/test.sh --lib sieve` passed offline (12 tests).
The runner lacks GitHub event refs here, so the same-host workflow behavior cannot be exercised locally; next I will run the repository gate and document its outcome.
`tools/gate.sh` passed fmt and initial clippy stages; it is still compiling/running its remaining checks after the first wait.

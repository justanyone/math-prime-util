Implement the six u64 bit-position operations in separate src/bits_pos files.
Read PLAN.md sections 5–8 before edits; preserve MPU bounds and scan semantics.
Add unit and property tests, runnable docs, and benchmarks without dependencies.
Run focused offline tests, the status generator, and the gate; record blockers here.
Reference inspection: all six names occur only in reference/mpu/TODO, not Util.pm, PP.pm, or C. Implement a checked u64 API and record the missing oracle as a verification blocker.

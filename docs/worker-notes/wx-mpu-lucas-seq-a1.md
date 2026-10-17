# wx-mpu-lucas-seq-a1 notes
- Plan: port reference/mpu lucas_seq.c into src/lucas/mod.rs with i64 P/Q, Option for u64 overflow.
- Add tests/compat_lucas.rs (fixture lucas.tsv), tests/prop_lucas.rs, benches/lucas.rs; gen fixture via tools/gen_fixtures.pl if generator exists.
- 19:02 wrote src/lucas/mod.rs (all 9 fns). next: compat/prop tests, gate.

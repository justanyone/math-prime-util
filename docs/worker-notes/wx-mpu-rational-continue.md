# wx-mpu-rational continue notes

- Plan: finish rational port (Farey methods vs rational.c), compat tests, STATUS ticks, gate.
- Reason: prior run ended PARTIAL; remaining item is comparing Farey and bounds with reference/mpu rational.c.
- 21:38 tools/test.sh fails here: linker cc not found (build scripts). Cannot compile; verify by reading.
- 21:39 Python model of farey_rank/next_farey matches all 592 fixture rows. Next: STATUS ticks, compat coverage.

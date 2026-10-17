Port the four factor predicates and smooth count from MPU 0.75.
Keep each public function in its own file and reuse available primitives.
Use the committed factor fixtures and algebraic properties for offline checks.
Leave tools, fixtures, STATUS.md, and git metadata unchanged.
The named predicates are in util.c, not factor.c; factor support is absent in this checkout. Use the actual C locations in citations and private factoring support where needed.

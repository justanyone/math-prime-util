Reassess the four existing functions against MPU 0.75 and their dependencies.
Move each owned public function to its own module file, preserving sibling functions.
Use the existing families oracle and add regression tests for any defects found.
Run focused offline tests and the gate; record unavailable checks here.
The existing oracle and property suites cover all four functions. Split their definitions into owned files; retain shared sieve helpers for sibling sum functions.

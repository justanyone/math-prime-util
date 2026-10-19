I will replace the all-primes allocation in squarefree segments with the segmented prime iterator.
For short ranges near u64::MAX, I will use the existing squarefree factor test per value.
I will add edge-focused property coverage and update only completed checklist items.
I will run the focused offline test and gate if time permits.
The edge property passes. I added a bounded segmented prime sieve because `primes_in` walks primes one by one; next I am checking the fixture test and existing squarefree property.

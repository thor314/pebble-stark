# Air implementation
Referencing the [starkware C++ implementation](https://github.com/starkware-libs/stone-prover/tree/main/src/starkware/air).

Initial migration:
- [ ] air.h
- [ ] compile_time_optional - maybe C++ fonkery, perhaps necessary in Rust
- [ ] test_utils
- [ ] trace.h
- [ ] trace_context.h
- [ ] trace_test.cc

## Air notes
- translating `Air` to a trait, this seems reasonable as `Air` is mostly an ABC
- can't represent the C++ constructor check in trait, which would require modifying struct state (trace_length)
- `CompositionPolynomial` in C++ implementation simply stores a unit struct pointer, I suspect this should be `composition_polynomial::CompositionPolynomial` 


##
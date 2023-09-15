# Air implementation
Referencing the [starkware C++ implementation](https://github.com/starkware-libs/stone-prover/tree/main/src/starkware/air).

Initial migration:
- 9/10 air.h
- 0/10 compile_time_optional* - maybe C++ fonkery, perhaps necessary in Rust
- 2/10 test_utils*
- 9/10 trace.h - ask a human 
- 10/10 trace_context.h
- 0/10 trace_test.cc


## Air notes
- translating `Air` to a trait, this seems reasonable as `Air` is mostly an ABC
- can't represent the C++ constructor check in trait, which would require modifying struct state (trace_length)
- `CompositionPolynomial` in C++ implementation simply stores a unit struct pointer, I suspect this should be `composition_polynomial::CompositionPolynomial` 


### Compile Time Optional notes
this looks like optimization territory

### test_utils
we'll get back to this when implementing tests for the module and stub it out for now

### Trace notes
`Trace` seems like a struct, no virtual methods. Custom copy constructor, default move constructor, default destructor, default {copy,move} assignment. Weird logic around copy, might be a bug on theirs. This file has a lot of odd boilerplate logic that I might want to talk to a human about.
- mapping `Allocate` to `new`
- Trace constructor - mapping to `From<Vec<Vec<F>>>`
- `CopyFrom` - ths looks to have a minor inefficiency, calling `values.size()` instead of `values.Length()`; 

#### Trace context
this looks like a pretty useless trait, candidate for erasure

#### Trace test
come back later


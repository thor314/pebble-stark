use std::marker::PhantomData;

// Define a struct representing a prime field element.
// The NBits and Index are generic parameters.
/// use this instead: https://docs.rs/ark-ff/latest/ark_ff/fields/trait.PrimeField.html
/// usage:
/// ```
/// // where before: 
/// // PrimeFieldElement<256, 0>
/// // now:
/// // ark_test_curves::
/// ```
pub struct PrimeFieldElement<NBits, Index> {
  _nbits: PhantomData<NBits>,
  _index: PhantomData<Index>,
}

impl<NBits, Index> From<usize> for PrimeFieldElement<NBits, Index> {
  fn from(val: usize) -> Self {
    // Self{val}
    todo!()
  }
}

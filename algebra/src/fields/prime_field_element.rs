use std::marker::PhantomData;

// use super::big_prime_constants::BigPrimeConstants;

// Define a struct representing a prime field element.
// The NBits and Index are generic parameters.
// todo: probably get rid of this and use Arkworks PrimeFieldElement
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

// impl<NBits, Index> PrimeFieldElement<NBits, Index> {
//   // ... other methods as needed
// }

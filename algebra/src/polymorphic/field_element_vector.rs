use std::ops::{Deref, Index, IndexMut};

use ark_ff::Field;

/// A class representing a vector of FieldElement-s of a common field.
/// To create an instance call FieldElementVector::make().
#[derive(Default, Clone)]
pub struct FieldElementVector<F: Field>(pub Vec<F>);

impl<F: Field> From<Vec<F>> for FieldElementVector<F> {
  fn from(value: Vec<F>) -> Self { Self(value) }
}

impl<F: Field> Deref for FieldElementVector<F> {
  type Target = Vec<F>;

  fn deref(&self) -> &Self::Target { &self.0 }
}

impl<F: Field> Index<usize> for FieldElementVector<F> {
  type Output = F;

  fn index(&self, index: usize) -> &Self::Output { &self.0[index] }
}

impl<F: Field> IndexMut<usize> for FieldElementVector<F> {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output { &mut self.0[index] }
}

impl<F: Field> From<FieldElementVector<F>> for Vec<F> {
  fn from(value: FieldElementVector<F>) -> Self { value.0 }
}

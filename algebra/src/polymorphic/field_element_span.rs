//! https://github.com/starkware-libs/stone-prover/blob/a78ff37c1402dc9c3e3050a1090cd98b7ff123b3/src/starkware/algebra/polymorphic/field_element_span.h
use ark_ff::Field;

pub struct FieldElementVector;

pub struct ConstFieldElementSpan<F: Field> {
  pub(crate) elements: Vec<F>,
}

pub struct FieldElementSpanImpl;

pub struct FieldElementSpan<F: Field> {
  pub(crate) elements: Vec<F>,
  // pub(crate) offset: usize,
  // pub(crate) length: usize,
}

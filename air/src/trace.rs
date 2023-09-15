//! https://github.com/starkware-libs/stone-prover/blob/a78ff37c1402dc9c3e3050a1090cd98b7ff123b3/src/starkware/air/trace.h

use algebra::FieldElementVector;
use ark_ff::Field;
use composition_polynomial::TempGslSpan;

// doc(tk)
#[derive(Clone, Default)]
pub struct Trace<F: Field> {
  values:     Vec<FieldElementVector<F>>,
  /// length of the Trace
  pub length: usize,
  /// width of the Trace
  pub width:  usize,
}

impl<F: Field> Trace<F> {
  pub fn new(n_columns: usize, trace_length: usize) -> Self {
    // todo(tk): check that this is equivalent: FieldElementT::UninitializedVector(trace_length))
    let values = (0..n_columns).map(|_| Vec::with_capacity(trace_length).into()).collect();
    Self { values, length: trace_length, width: n_columns }
  }

  pub(crate) fn set_trace_element_for_testing(
    &mut self,
    column: usize,
    index: usize,
    field_element: &F,
  ) {
    self.values[column][index] = *field_element;
  }
}

impl<F: Field> From<Vec<Vec<F>>> for Trace<F> {
  fn from(values: Vec<Vec<F>>) -> Self {
    if values.is_empty() {
      Trace::default()
    } else {
      let width = values.len();
      let length = values[0].len();
      let values: Vec<FieldElementVector<F>> = values.into_iter().map(|v| v.into()).collect();
      Trace { width, length, values }
    }
  }
}

impl<F: Field> From<Vec<FieldElementVector<F>>> for Trace<F> {
  fn from(values: Vec<FieldElementVector<F>>) -> Self {
    if values.is_empty() {
      Trace::default()
    } else {
      let width = values.len();
      let length = values[0].0.len();
      let values: Vec<FieldElementVector<F>> = values.into_iter().collect();
      Trace { width, length, values }
    }
  }
}

impl<F: Field> From<Trace<F>> for Vec<TempGslSpan<F>> {
  fn from(val: Trace<F>) -> Self { val.values.into_iter().map(|v| v.into()).collect() }
}

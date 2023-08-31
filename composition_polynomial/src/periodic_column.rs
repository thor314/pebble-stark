use ark_ff::Field;

use crate::{TempGslSpan, TempLDEManager};

pub struct PeriodicColumn<F: Field> {
  values:      Vec<F>,
  index_mask:  usize,
  lde_manager: TempLDEManager,
}
impl<F: Field> PeriodicColumn<F> {
  /// Constructs a PeriodicColumn whose evaluations on the coset offset*<group_generator> is
  /// composed of repetitions of the given values.
  pub fn new(
    values: TempGslSpan<F>,
    group_generator: &F,
    offset: &F,
    coset_size: usize,
    column_step: usize,
  ) -> Self {
    todo!()
  }

  pub fn eval_at_point(&self, x: &F) -> F { todo!() }

  /// Returns the actual degree of the interpolant.
  pub fn get_actual_degree(&self) -> usize {
    // self.lde_manager.get_evaluation_degree(0)
    todo!()
  }

  // ...
}

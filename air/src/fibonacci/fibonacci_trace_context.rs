use ark_ff::Field;
use utils::assert_on_release;

use super::FibonacciAir;
use crate::{trace::Trace, trace_context::TraceContext};

pub struct FibonacciTraceContext<F: Field> {
  // todo(tk)
  // air: MaybeOwnedPtr,
  witness:               F,
  fibonacci_claim_index: usize,
}

impl<F: Field> TraceContext<F> for FibonacciTraceContext<F> {
  fn get_interaction_trace(&self) -> Trace<F> {
    assert_on_release(
      false,
      "Calling GetInteractionTrace from fibonacci air that doesn't have interaction",
    );
    unimplemented!()
  }
}

impl<F: Field> FibonacciTraceContext<F> {
  pub fn new(
    // air: MaybeOwnedPtr<>,
    witness: F,
    fibonacci_claim_index: usize,
  ) -> Self {
    Self { witness, fibonacci_claim_index }
  }

  // todo(tk)
  // pub fn get_trace(&self) {
  //   FibonacciAir::get_trace(
  //     &self.witness,
  //     // air.trace_length(),
  //     self.fibonacci_claim_index,
  //   )
  // }
}

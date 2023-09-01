//! https://github.com/starkware-libs/stone-prover/blob/a78ff37c1402dc9c3e3050a1090cd98b7ff123b3/src/starkware/air/trace_context.h
use algebra::FieldElementVector;
use ark_ff::Field;

use crate::{assert_on_release, trace::Trace, Air};

/// Initializes parameters for trace generation. The motivation is initializing the parameters
/// relevant to the trace when creating the statement, before generating the trace itself in
/// ProveStark method.
pub trait TraceContext<F: Field> {
  /// Updates AIR with given interaction elements if this AIR has an interaction. If not - throws an
  /// error
  fn set_interaction_elements(&mut self, _interaction_elms: &FieldElementVector) {
    assert_on_release(false, "Calling set_interaction_elements in an AIR with no interaction.")
  }

  /// Create an interaction trace. In case the AIR does not have interaction, throw error or panic.
  // todo(tk): update type sig: throw error or panic.
  fn get_interaction_trace(&self) -> Trace<F>;

  fn get_air(&self) -> &dyn Air<F> {
    assert_on_release(false, "GetAir is not implemented.");
    unimplemented!();
  }
}

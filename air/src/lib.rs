// https://github.com/starkware-libs/stone-prover/blob/a78ff37c1402dc9c3e3050a1090cd98b7ff123b3/src/starkware/air/air.h
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod boundary;
mod boundary_constraints;
mod components;
mod cpu;
mod fibonacci;
mod trace;
mod trace_context;

use std::{collections::HashMap, option::Option, vec::Vec};

use algebra::{ConstFieldElementSpan, FieldElementVector};
use ark_ff::Field;
use composition_polynomial::CompositionPolynomial;

/// Temporary type while I figure out what to do with gsl::span
pub type TempGslSpan<T> = Vec<T>;

// todo(tk): import this from composition_polynomial
// pub trait TmpCompositionPolynomial<F: Field> {}

// doc(tk)
///
pub trait Air<F: Field> {
  /// Creates a CompositionPolynomial object based on the given (verifier-chosen) coefficients.
  fn create_composition_polynomial(
    &self,
    trace_generator: &F,
    random_coefficients: &ConstFieldElementSpan<F>,
  ) -> Box<CompositionPolynomial<F>>;

  fn trace_length(&self) -> usize;

  /// Default to zero
  fn get_composition_polynomial_degree_bound(&self) -> usize;

  /// Number of random coefficients that are chosen by the verifier and affect the constraint.
  /// They are the coefficients of the linear combination of the constraints and must be random in
  /// order to maintain soundness.
  fn num_random_coefficients(&self) -> usize;

  // refactor(tk): redundant?
  fn get_num_constraints(&self) -> usize { self.num_random_coefficients() }

  // get mask

  /// helper for `get_n_columns_first`
  fn num_columns(&self) -> usize;

  /// When the AIR has interaction (i.e. for debugging and testing), clones the AIR and updates its
  /// interaction elements. Returns the cloned AIR. Otherwise, this function shouldn't be used.
  // todo: unsure how this is used, ignore for now
  // refactor(tk): non-idiomatic rust
  fn with_interaction_elements(
    &self,
    interaction_elms: &algebra::FieldElementVector<F>,
  ) -> Box<dyn Air<F>> {
    assert_on_release(false, "Calling WithInteractionElements in an Air with no interaction.");
    unreachable!()
  }

  /// Returns the interaction parameters.
  /// If there is no interaction, returns None.
  fn get_interaction_params(&self) -> Option<InteractionParams>;

  /// If air has interaction, returns the number of columns in the first trace;
  /// otherwise, returns the total number of columns.
  fn get_n_columns_first(&self) -> usize {
    match self.get_interaction_params() {
      Some(params) => params.n_columns_first,
      None => self.num_columns(),
    }
  }

  // todo(tk): verify ok to drop ths
  //  protected:
  //   uint64_t trace_length_;
}

/// Helper NewType for the `get_mask` of the Air.
// refactor(tk): newtype->alias?
pub struct AirMask(pub Vec<(i64, u64)>);

/// Stores data relevant to the interaction.
pub struct InteractionParams {
  // Number of columns in first and second trace.
  pub n_columns_first:        usize,
  pub n_columns_second:       usize,
  // Number of interaction random elements.
  pub n_interaction_elements: usize,
}

pub fn assert_on_release(condition: bool, msg: &str) {
  #[cfg(not(debug_assertions))]
  {
    if !condition {
      panic!("{}", msg);
    }
  }
}

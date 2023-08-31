//! https://github.com/starkware-libs/stone-prover/blob/a78ff37c1402dc9c3e3050a1090cd98b7ff123b3/src/starkware/air/boundary/boundary_air.h
// todo(tk): may need to write CompositionPolynomial first to implement:
//  public:
// using FieldElementT_ = FieldElementT;
// using Builder = typename CompositionPolynomialImpl<BoundaryAir<FieldElementT>>::Builder;
//
// something like:
// type Builder = CompositionPolynomial<BoundaryAir<F::FieldExt>>;
use std::collections::{HashMap, VecDeque};

use ark_ff::Field;

use crate::Air;

/// A simple AIR the describes the contraints:
/// (column_i(x) - y0_i) / (x - x0_i).
///
/// Parameters:
/// - trace_length, size of trace.
/// - n_columns, number of columns in trace.
/// - boundary_conditions, list of tuples (column, x, y) indicating the constraint that column(x)=y.
pub struct BoundaryAir<F: Field> {
  trace_length: usize,
  n_columns:    usize,
  // todo(tk): check usage of VecDeq
  constraints:  VecDeque<ConstraintData<F>>,
  /// The mask touches each column once at the current row.
  mask:         Vec<(usize, usize)>,
}

impl<F: Field> BoundaryAir<F> {
  pub fn new(
    trace_length: usize,
    n_columns: usize,
    boundary_conditions: &[BoundaryCondition<F>],
    // todo(tk): Starkware constructor also returns trace_length and n_columns, curiously
  ) -> Self {
    panic_on_release_if(trace_length.is_power_of_two(), "trace length must be power of 2");

    let mut constraints: VecDeque<ConstraintData<F>> =
      VecDeque::with_capacity(boundary_conditions.len());

    // Group constraints by the point_x, store them in constraints
    for (coeff_idx, BoundaryCondition { column_idx, point_x, point_y }) in
      boundary_conditions.iter().enumerate()
    {
      // Insert the current boundary condition next to one with the same x
      // or at the end of the list.
      let pos = constraints.iter().position(|c| &c.point_x == point_x);
      let constraint_data = ConstraintData {
        coeff_idx,
        column_index: *column_idx,
        point_x: *point_x,
        point_y: *point_y,
      };

      match pos {
        Some(pos) => constraints.insert(pos, constraint_data),
        None => constraints.push_back(constraint_data),
      }
    }

    let mask = (0..n_columns).map(|i| (0, i)).collect();

    Self { trace_length, n_columns, constraints, mask }
  }
}

impl<F: Field> Air<F> for BoundaryAir<F> {
  fn create_composition_polynomial(
    &self,
    trace_generator: &F,
    random_coefficients: &algebra::ConstFieldElementSpan<F>,
    // refactor(tk): dyn bad
  ) -> Box<dyn crate::CompositionPolynomial<F>> {
    todo!()
  }

  fn trace_length(&self) -> usize { todo!() }

  fn get_composition_polynomial_degree_bound(&self) -> usize { todo!() }

  fn num_random_coefficients(&self) -> usize { todo!() }

  fn get_mask() -> crate::AirMask { todo!() }

  fn num_columns(&self) -> usize { todo!() }

  fn get_interaction_params(&self) -> Option<crate::InteractionParams> { todo!() }

  fn with_interaction_elements(&self, interaction_elms: &algebra::FieldElementVector) {
    panic_on_release_if(true, "Calling WithInteractionElements in an air with no interaction.");
    todo!()
  }
}

/// List of tuples (column, x, y) indicating the constraint that column(x)=y.
// todo(tk): does this match sig: `gsl::span<const std::tuple<size_t, FieldElement,
// FieldElement>> boundary_conditions`?
pub struct BoundaryCondition<F: Field> {
  pub column_idx: usize,
  pub point_x:    F,
  pub point_y:    F,
}

// doc(tk)
struct ConstraintData<F: Field> {
  coeff_idx:    usize,
  column_index: usize,
  point_x:      F,
  point_y:      F,
}

pub fn panic_on_release_if(condition: bool, msg: &str) {
  #[cfg(not(debug_assertions))]
  {
    if condition {
      panic!("{}", msg);
    }
  }
}

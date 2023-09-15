//! https://github.com/starkware-libs/stone-prover/blob/a78ff37c1402dc9c3e3050a1090cd98b7ff123b3/src/starkware/air/boundary/boundary_air.h
// todo(tk): may need to write CompositionPolynomial first to implement:
//  public:
// using FieldElementT_ = FieldElementT;
// using Builder = typename CompositionPolynomialImpl<BoundaryAir<FieldElementT>>::Builder;
//
// something like:
// type Builder = CompositionPolynomial<BoundaryAir<F::FieldExt>>;
use std::collections::{HashMap, VecDeque};

use algebra::ConstFieldElementSpan;
use ark_ff::Field;
use composition_polynomial::CompositionPolynomial;

use crate::{assert_on_release, Air};

/// A simple AIR the describes the contraints:
/// (column_i(x) - y0_i) / (x - x0_i).
///
/// Parameters:
/// - trace_length, size of trace.
/// - n_columns, number of columns in trace.
/// - boundary_conditions, list of tuples (column, x, y) indicating the constraint that column(x)=y.
pub struct BoundaryAir<F: Field> {
  pub trace_length: usize,
  pub num_columns:  usize,
  // todo(tk): check usage of VecDeq
  constraints:      VecDeque<ConstraintData<F>>,
  /// Mask is the list of pairs (relative_row, col) that define the neighbors needed for the
  /// constraint.
  /// The mask touches each column once at the current row.
  pub mask:         Vec<(usize, usize)>,
}

impl<F: Field> BoundaryAir<F> {
  pub fn new(
    trace_length: usize,
    n_columns: usize,
    boundary_conditions: &[BoundaryCondition<F>],
  ) -> Self {
    assert_on_release(trace_length.is_power_of_two(), "trace length must be power of 2");

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

    Self { trace_length, num_columns: n_columns, constraints, mask }
  }

  // todo(tk): find replacement for gsl::span
  // pub fn precompute_domain_evals_on_coset(
  //   point: &F,
  //   generator: &F,
  //   point_exponents: gsl::span<usize>,
  //   shifts: gsl::span<F>,
  // ) { // do nothing
  // }

  // pub fn constraints_eval(&self, ){
  //       gsl::span<const FieldElementT> neighbors,
  //     gsl::span<const FieldElementT> /* periodic_columns */,
  //     gsl::span<const FieldElementT> random_coefficients,
  //     gsl::span<const FieldElementT> point_powers, gsl::span<const FieldElementT> /*shifts*/,
  //     gsl::span<const FieldElementT> /* precomp_domains */) const {
  //   ASSERT_DEBUG(neighbors.size() == n_columns_, "Wrong number of neighbors");
  //   ASSERT_DEBUG(
  //       random_coefficients.size() == constraints_.size(), "Wrong number of random
  // coefficients");

  //   const FieldElementT& point = point_powers[0];

  //   FractionFieldElement<FieldElementT> outer_sum(FieldElementT::Zero());
  //   FieldElementT inner_sum(FieldElementT::Zero());

  //   FieldElementT prev_x = constraints_[0].point_x;

  //   for (const ConstraintData& constraint : constraints_) {
  //     const FieldElementT constraint_value =
  //         random_coefficients[constraint.coeff_idx] *
  //         (neighbors[constraint.column_index] - constraint.point_y);
  //     if (prev_x == constraint.point_x) {
  //       // All constraints with the same constraint.point_x are summed into inner_sum.
  //       inner_sum += constraint_value;
  //     } else {
  //       // New constraint.point_x, add the old (inner_sum/prev_x) to the outer_sum
  //       // and start a new inner_sum.
  //       outer_sum += FractionFieldElement<FieldElementT>(inner_sum, point - prev_x);
  //       inner_sum = constraint_value;
  //       prev_x = constraint.point_x;
  //     }
  //   }
  //   outer_sum += FractionFieldElement<FieldElementT>(inner_sum, point - prev_x);

  //   return outer_sum;
  // }}

  //   std::vector<FieldElementT> DomainEvalsAtPoint(
  //     gsl::span<const FieldElementT> /* point_powers */,
  //     gsl::span<const FieldElementT> /* shifts */) const {
  //   return {};
  // }
}

impl<F: Field> Air<F> for BoundaryAir<F> {
  fn create_composition_polynomial(
    &self,
    trace_generator: &F,
    random_coefficients: &ConstFieldElementSpan<F>,
  ) -> Box<CompositionPolynomial<F>> {
    // todo(tk): builder is a bad name for a builder of CompositionPolynomial, commenting until
    // farther into refactor
    //
    // let builder = Builder::new(0);
    // builder.build_unique_ptr(use_owned(&self), trace_generator, self.trace_length,
    // random_coefficients, (), ())
    todo!()
  }

  fn get_composition_polynomial_degree_bound(&self) -> usize { self.trace_length }

  fn num_random_coefficients(&self) -> usize { self.constraints.len() }

  fn get_interaction_params(&self) -> Option<crate::InteractionParams> { None }

  fn num_columns(&self) -> usize { self.num_columns }

  fn trace_length(&self) -> usize { todo!() }
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

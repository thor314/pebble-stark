//! https://github.com/starkware-libs/stone-prover/tree/main/src/starkware/composition_polynomial
//! Note(tk): eliminating the C++-idiomatic indirection of `CompositionPolynomialImpl`;
//! CompositionPolynomial is now just a struct
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

/// Temporary type while I figure out what to do with gsl::span
pub type TempGslSpan<T> = Vec<T>;
pub type TempLDEManager = ();

mod multiplicative_neighbors;
mod periodic_column;

use algebra::{ConstFieldElementSpan, FieldElementSpan};
use ark_ff::Field;
use multiplicative_neighbors::MultiplicativeNeighbors;

pub use crate::periodic_column::PeriodicColumn;

/// Represents a polynomial of the form:
///
/// F(x, y_1, y_2, ... , y_k) =
/// \sum_i c_i * f_i(x, y_0, y_1, ... , y_k, p_0, ..., p_m) *
/// P_i(x)/Q_i(x).
///
/// Where:
///
/// - The sequence (p_0, ..., p_m) consists of the evaluations of the periodic public columns.
/// - The term f_i(y_0, y_1, ... , y_k, p_0, ..., p_m) represents a constraint.
/// - The term P_i(x)/Q_i(x) is rational function such that Q_i(x)/P(i) is a polynomial with only
/// simple roots, and it's roots are exactly the locations the constraint f_i has to be satisfied
/// on.
///
/// Parameters deduction:
///
/// - (c_0, c_1, ... ) are the 'coefficients'.
/// - The functions (f_0, f_1,...) are induced by air.ConstraintsEval().
/// - The mask (for evaluation on entire cosets) is obtained from air.GetMask().
///
/// This class is used both to evaluate F( x, y_0, y_1, ...) on a single point, and on entire cosets
/// using optimizations improving the (amortized) computation time for each point in the coset.
pub struct CompositionPolynomial<F: Field> {
  // air: MaybeOwnedPtr<'a, AirT>,
  trace_generator:  F,
  coset_size:       usize,
  periodic_columns: Vec<Option<PeriodicColumn<F>>>,
  coefficients:     Vec<F>,
  point_exponents:  Vec<usize>,
  shifts:           Vec<F>,
}

impl<F: Field> CompositionPolynomial<F> {
  /// Users should use the Builder class to build an instance of this class.
  fn new(
    // air: MaybeOwnedPtr<'a, AirT>,
    trace_generator: F,
    coset_size: u64,
    periodic_columns: Vec<Option<PeriodicColumn<F>>>,
    coefficients: Vec<F>,
    point_exponents: Vec<u64>,
    shifts: Vec<F>,
  ) -> Self {
    todo!()
  }

  /// Returns the inverse of all denominators needed for evaluation over the coset
  /// coset_offset*<group_generator>, which is expected to be of size group_size. n_points
  /// is number of points on which the function operates. Technically, it return a vector of length
  /// group_size*n_constraints where n_constraints is the number of constraints and the denominator
  /// of the i-th constraint needed for the point coset_offset*group_generator^j is stored at
  /// index n_constraints*j + i in the result.
  fn compute_denominators_inv(offset: &F, n_points: usize, denominators_inv: TempGslSpan<F>) {
    todo!()
  }

  /// Evaluates the polynomial on a single point. The neighbors are the values obtained from the
  /// trace low degree extension, using the AIR's mask.
  pub fn eval_at_point(&self, point: &F, neighbors: &ConstFieldElementSpan<F>) -> F { todo!() }

  pub fn eval_at_span(&self, point: &F, neighbors: &TempGslSpan<F>) -> F { todo!() }

  /// Evaluates composition_poly on the coset coset_offset*<group_genertor>, which must be of size
  /// coset_size. The evaluation is split to different tasks of size task_size each. The evaluation
  /// is written to 'out_evaluation', in bit reversed order: out_evaluation[i] contains the
  /// evaluation on the point
  ///   coset_offset*(group_genertor^{bit_reverse(i)}).
  pub fn eval_on_coset_bit_reversed_output(
    &self,
    coset_offset: &F,
    trace_lde: TempGslSpan<ConstFieldElementSpan<F>>,
    out_evaluation: &FieldElementSpan<F>,
    task_size: usize,
  ) {
    todo!()
  }

  pub fn eval_on_coset_bit_reversed_output_span(
    &self,
    coset_offset: &F,
    multiplicative_neighbors: &MultiplicativeNeighbors<F>,
    // note(tk): why shouldn't this be same type as above?
    out_evaluation: TempGslSpan<F>,
    task_size: usize,
  ) {
    todo!()
  }

  // todo: air trait method, not sure, would cause circular dependency if used
  // fn get_degree_bound(&self) -> usize { self.get_composition_polynomial_degree_bound() }
}

pub struct CompositionPolynomialBuilder<F: Field> {
  constraint_degrees: Vec<u64>,
  periodic_columns:   Vec<Option<PeriodicColumn<F>>>,
}

impl<F: Field> CompositionPolynomialBuilder<F> {
  pub fn new(num_periodic_columns: u64) -> Self {
    todo!()
    // Self {
    //   constraint_degrees: Vec::new(),
    //   periodic_columns:   vec![None; num_periodic_columns as usize],
    // }
  }

  pub fn add_periodic_column(&mut self, column: PeriodicColumn<F>, periodic_column_index: usize) {
    self.periodic_columns[periodic_column_index] = Some(column);
  }

  // todo(tk) types and pointer analogs
  /// Builds an instance of CompositionPolynomial.
  /// Note that once Build or BuildUniquePtr are used, the constraints that were added
  /// previously are consumed and the Builder goes back to a clean slate state.
  pub fn build(
    self,
    //   air: MaybeOwnedPtr<'a, AirT>,
    trace_generator: F,
    coset_size: u64,
    random_coefficients: &[F],
    point_exponents: &[u64],
    shifts: &[F],
  ) -> CompositionPolynomial<F> {
    // { CompositionPolynomial{ air, trace_generator,
    //   coset_size, periodic_columns: self.periodic_columns, coefficients:
    //   random_coefficients.to_vec(), point_exponents: point_exponents.to_vec(), shifts:
    //   shifts.to_vec(), }
    todo!()
  }
}

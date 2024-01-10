//! https://github.com/starkware-libs/stone-prover/blob/a78ff37c1402dc9c3e3050a1090cd98b7ff123b3/src/starkware/air/boundary/boundary_air.h
//! Note: Using `stdVecDeque` in place of C++ `std::vector`, for efficient insertion and removal
//! from both ends.
// todo(tk): may need to write CompositionPolynomial first to implement:
//  public:
// using FieldElementT_ = FieldElementT;
// using Builder = typename CompositionPolynomialImpl<BoundaryAir<FieldElementT>>::Builder;
//
// something like:
// type Builder = CompositionPolynomial<BoundaryAir<F::FieldExt>>;
use std::{
  collections::{HashMap, VecDeque},
  vec,
};

use algebra::{ConstFieldElementSpan, FractionFieldElement};
use ark_ff::Field;
use composition_polynomial::CompositionPolynomial;
use utils::assert_on_release;

use crate::Air;

// todo: lift to crate root;
// todo: verify that VecDeque is optimal replacement
/// Annotation of type mirroring of `gsl::span` type in Stone Prover codebase.
pub type GslSpan<T> = VecDeque<T>;
// TODO(TK 2024-01-09): placeholder type

/// A simple AIR the describes the contraints:
/// (column_i(x) - y0_i) / (x - x0_i).
///
/// Parameters:
/// - trace_length, size of trace.
/// - n_columns, number of columns in trace.
/// - boundary_conditions, list of tuples (column, x, y) indicating the constraint that column(x)=y.
// todo: this method is difficult to read and likely implemented incorrectly
pub struct BoundaryAir<F: Field> {
  pub trace_length: u64,
  pub num_columns:  usize,
  // todo(tk): check usage of VecDeq
  constraints:      VecDeque<ConstraintData<F>>,
  /// Mask is the list of pairs (relative_row, col) that define the neighbors needed for the
  /// constraint.
  /// The mask touches each column once at the current row.
  pub mask:         Vec<(isize, usize)>,
}

impl<F: Field> BoundaryAir<F> {
  pub fn new(
    trace_length: u64,
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

  // fn create_composition_polynomial in Air trait impl.

  // TODO(TK 2024-01-09): should this be in Air trait?
  pub fn precompute_domain_evals_on_coset(
    point: &F,
    generator: &F,
    point_exponents: GslSpan<u64>,
    shifts: GslSpan<F>,
  ) { // do nothing
  }

  // evaluate the set of constraints for the AIR
  // TODO(TK 2024-01-09): almost certainly should be in Air trait
  pub fn constraints_eval(
    &self,
    neighbors: GslSpan<F>,
    periodic_columns: GslSpan<F>,
    random_coefficients: GslSpan<F>,
    point_powers: GslSpan<F>,
    shifts: GslSpan<F>,
    precomp_domains: GslSpan<F>,
  ) -> FractionFieldElement<F> /*outer_sum*/ {
    assert_on_release(neighbors.len() == self.num_columns, "Wrong number of neighbors");
    assert_on_release(
      random_coefficients.len() == self.constraints.len(),
      "Wrong number of coefficients",
    );

    let point = &point_powers[0];

    let mut outer_sum = FractionFieldElement::<F>::zero();
    let mut inner_sum = F::zero();
    let mut prev_x = self.constraints.front().expect("Constraints cannot be empty").point_x;

    for constraint in &self.constraints {
      let constraint_value = random_coefficients[constraint.coeff_idx]
        * (neighbors[constraint.column_index] - constraint.point_y);

      if prev_x == constraint.point_x {
        // All constraints with the same constraint.point_x are summed into inner_sum.
        inner_sum += constraint_value;
      } else {
        // New constraint.point_x, add the old (inner_sum/prev_x) to the outer_sum
        // and start a new inner_sum.
        outer_sum += FractionFieldElement::new(inner_sum, *point - prev_x);
        inner_sum = constraint_value;
        prev_x = constraint.point_x;
      }
    }
    outer_sum += FractionFieldElement::new(inner_sum, *point - prev_x);

    outer_sum
  }

  fn domain_evals_at_point(point_powers: GslSpan<F>, shifts: GslSpan<F>) -> VecDeque<F> {
    Default::default() // intentionally left blank
  }
}

impl<F: Field> Air<F> for BoundaryAir<F> {
  fn create_composition_polynomial(
    &self,
    trace_generator: &F,
    random_coefficients: &ConstFieldElementSpan<F>,
  ) -> Box<CompositionPolynomial<F>> {
    Box::new(CompositionPolynomial::new(
      // TODO(TK 2024-01-09): placeholder impl
      &F::one(),
      *trace_generator,
      self.trace_length,
      vec![],
      vec![],
      // random_coefficients,
      vec![],
      vec![],
    ))
  }

  fn get_composition_polynomial_degree_bound(&self) -> u64 { self.trace_length }

  fn num_random_coefficients(&self) -> usize { self.constraints.len() }

  fn get_interaction_params(&self) -> Option<crate::InteractionParams> { None }

  fn num_columns(&self) -> usize { self.num_columns }

  fn trace_length(&self) -> u64 { self.trace_length }
}

/// List of tuples (column, x, y) indicating the constraint that column(x)=y.
#[derive(Clone, Debug)]
pub struct BoundaryCondition<F: Field> {
  pub column_idx: usize,
  pub point_x:    F,
  pub point_y:    F,
}

impl<F: Field> BoundaryCondition<F> {
  pub fn new(column_idx: usize, point_x: F, point_y: F) -> Self {
    Self { column_idx, point_x, point_y }
  }
}

// doc(tk)
#[derive(Clone, Debug)]
struct ConstraintData<F: Field> {
  coeff_idx:    usize,
  column_index: usize,
  point_x:      F,
  point_y:      F,
}

#[cfg(test)]
mod test {
  // mirror: https://github.com/starkware-libs/stone-prover/blob/main/src/starkware/air/boundary/boundary_air_test.cc
  // test for soundness and correctness of the boundary constraint.
  use std::{iter::repeat_with, vec::Vec};

  use ark_ff::{BigInteger, Field, PrimeField, UniformRand};
  use ark_std::{test_rng, One, Zero};
  use ark_test_curves::fp128::Fq;
  use rand::Rng;

  use super::*;
  use crate::test_utils;

  //   use crate::{
  //     boundary_air::BoundaryAir,
  //     utils::{FieldElementVector, Prng},
  //   };

  const N_COLUMNS: usize = 10;
  const N_CONDITIONS: usize = 20;
  const TRACE_LENGTH: u64 = 1024;

  // This test builds a random trace. It then generate random points to sample on random
  // columns, and creates boundary constraints for them. It then tests that the resulting
  // composition polynomial is indeed of expected low degree., which means the constraints
  // hold.
  #[test]
  fn correctness() {
    let mut rng = test_rng();
    let trace: Vec<Vec<_>> = repeat_with(|| {
      repeat_with(|| <Fq as UniformRand>::rand(&mut rng)).take(N_COLUMNS).collect::<Vec<_>>()
    })
    .take(TRACE_LENGTH as usize)
    .collect();

    // Compute correct boundary conditions.
    let boundary_conditions: Vec<BoundaryCondition<Fq>> = repeat_with(|| {
      let column_index = rng.gen_range(0..N_COLUMNS) as usize;
      let point_x = <Fq as UniformRand>::rand(&mut rng);
      let point_y = <Fq as UniformRand>::rand(&mut rng);

      BoundaryCondition::new(column_index, point_x, point_y)
    })
    .take(N_CONDITIONS)
    .collect();

    let air = BoundaryAir::new(TRACE_LENGTH, N_COLUMNS, &boundary_conditions);
    let random_coefficients = repeat_with(|| <Fq as UniformRand>::rand(&mut rng))
      .take(air.num_random_coefficients())
      .collect::<Vec<_>>();
    // let actual_degree =
    //   test_utils::compute_composition_degree(&air, trace, &random_coefficients);

    // // Degree is expected to be trace_length - 2.
    // assert_eq!(TRACE_LENGTH - 2, actual_degree);
    // assert_eq!(air.get_composition_polynomial_degree_bound() - 2, actual_degree);
  }

  #[test]
  fn soundness() {
    //     let mut prng = Prng::new();
    //     let mut trace = Vec::new();

    //     // Generate random trace.
    //     for _ in 0..n_columns {
    //       trace.push(prng.random_field_element_vector(trace_length));
    //     }

    //     // Compute incorrect boundary conditions.
    //     let mut boundary_conditions = Vec::new();
    //     for _ in 0..n_conditions {
    //       let column_index = prng.uniform_int(0, n_columns - 1);
    //       let point_x = FieldElement::random(&mut prng);
    //       let point_y = FieldElement::random(&mut prng);

    //       boundary_conditions.push((column_index, point_x, point_y));
    //     }

    //     let air = BoundaryAir::new(trace_length, n_columns, &boundary_conditions);

    //     let random_coefficients =
    // prng.random_field_element_vector(air.num_random_coefficients());

    //     let num_of_cosets = 2;
    //     let actual_degree =
    //       compute_composition_degree(&air, trace, &random_coefficients, num_of_cosets);

    //     assert_eq!(num_of_cosets * air.get_composition_polynomial_degree_bound() - 1,
    // actual_degree);
  }
}

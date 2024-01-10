use std::{
  collections::{BTreeSet, HashSet},
  convert::AsRef,
  fmt::Debug,
  mem::take,
  ops::Range,
};

// use rand_core::RngCore;
use ark_ff::Field;

use crate::{
  // air::{Air, InteractionParams},
  // composition_polynomial::CompositionPolynomial,
  // field::Field,
  // math::{batch_pow, pow},
  trace::Trace,
  Air,
};

pub struct DummyAir<F>
where F: Field {
  _field: F,
  // Same fields as original DummyAir
}

impl<F> Air<F> for DummyAir<F>
where F: Field + Debug
{
  fn create_composition_polynomial(
    &self,
    trace_generator: &F,
    random_coefficients: &algebra::ConstFieldElementSpan<F>,
  ) -> Box<composition_polynomial::CompositionPolynomial<F>> {
    todo!()
  }

  fn trace_length(&self) -> u64 { todo!() }

  fn get_composition_polynomial_degree_bound(&self) -> u64 { todo!() }

  fn num_random_coefficients(&self) -> usize { todo!() }

  fn num_columns(&self) -> usize { todo!() }

  fn get_interaction_params(&self) -> Option<crate::InteractionParams> { todo!() }
}

pub fn compute_composition_degree<F: Field>(
  air: impl Air<F>,
  trace: &Trace<F>,
  coefficients: &[F],
  num_cosets: usize,
) -> i64 {
  assert!(!trace.is_empty(), "Nothing to do with empty trace.");

  // Evaluation domain specifications.
  let coset_size = trace[0].len();
  // TODO(TK 2024-01-09): bugcheck. maybe off (too low) by one
  let evaluation_domain_size =
    1 << usize::ilog2(air.get_composition_polynomial_degree_bound() as usize * num_cosets);
  let n_cosets = usize::checked_div(evaluation_domain_size, coset_size).expect("divided by zero");
  // TODO(TK 2024-01-09): implement ListOfCosets
  // let domain = ListOfCosets::<F>::make_list_of_cosets(coset_size, n_cosets);
  // let cosets = domain.cosets_offsets();

  // // Initialize LDE manager and storage for trace LDE evaluations.
  // let mut lde_manager = make_lde_manager(domain.bases());
  // let mut trace_lde = Vec::with_capacity(trace.width());
  // for column in trace.columns() {
  //     lde_manager.add_evaluation(column);
  //     trace_lde.push(FieldElementVector::make_uninitialized(field, coset_size));
  // }

  // // Construct and evaluate composition polynomial.
  // let composition_poly = air.create_composition_polynomial(domain.trace_generator(),
  // coefficients); let mut evaluation = FieldElementVector::make_uninitialized(field,
  // evaluation_domain_size); for (i, coset_offset) in
  // cosets_offsets.iter().enumerate().take(n_cosets) {     let bit_reversed_i = bit_reverse(i,
  // safe_log2(n_cosets));     lde_manager.eval_on_coset(&coset_offset, &trace_lde);

  //     const TASK_SIZE: u64 = 256;
  //     composition_poly.eval_on_coset_bit_reversed_output(
  //         &coset_offset,
  //         &trace_lde,
  //         &mut evaluation.as_span().sub_span(bit_reversed_i * coset_size, coset_size),
  //         TASK_SIZE
  //     );
  // }

  // // Compute degree of the evaluation.
  // let group = MultiplicativeGroup::make_group(evaluation_domain_size, field);
  // let mut lde_manager_eval = make_bit_reversed_order_lde_manager(&group,
  // group.get_field().one()); lde_manager_eval.add_evaluation(&evaluation);

  // lde_manager_eval.get_evaluation_degree(0)
  todo!()
}

// pub fn test_one_constraint<F>(
//   air: &dyn Air,
//   trace: &Trace,
//   constraint_id: usize,
//   rng: &mut impl RngCore,
// ) -> bool { // Implementation
// }

// pub fn get_failing_constraints<F>(
//   air: &dyn Air,
//   trace: &Trace,
//   rng: &mut impl RngCore,
// ) -> HashSet<usize> { // Implementation
// }

// pub fn get_failing_constraints_binary_search<F>(
//   air: &dyn Air,
//   trace: &Trace,
//   rng: &mut impl RngCore,
// ) -> HashSet<usize> { // Implementation
// }

// pub fn draw_random_trace(width: usize, height: usize, field: &F, rng: &mut impl RngCore) -> Trace
// where F: Field + Debug + Rand {
//   // Implementation
// }

// pub fn test_air_constraint<F>(
//   air: &dyn Air,
//   field: &F,
//   trace_len: usize,
//   constraint_idx: usize,
//   domain_indices: Vec<usize>,
//   trace_manipulator: impl Fn(&mut [F], &mut [F], bool),
//   rng: &mut impl RngCore,
// ) where F: Field + Debug,
// {
//   // Implementation
// }

// pub fn domain_predicate_to_list(
//   predicate: impl Fn(usize) -> bool,
//   num_elements: usize,
// ) -> Vec<usize> { // Implementation
// }

// pub fn merge_traces<F>(traces: &[Trace]) -> Trace
// where F: Field {
//   // Implementation
// }

// pub struct TestTraceContext {
//   trace: Option<Trace>,
//   // Other fields
// }

// impl TraceContext for TestTraceContext {
//   // Implement methods
// }

// pub fn compute_composition_degree<F: Field>(
//   air: &dyn Air,
//   trace: &Trace,
//   coefficients: &[F],
//   num_cosets: usize,
// ) -> i64 { // Implementation
// }

// pub fn draw_random_trace<F>(
//   width: usize,
//   height: usize,
//   field: &F,
//   rng: &mut impl RngCore,
// ) -> Trace
// where
//   F: Field + Debug + Rand,
// {
//   // Implementation
// }

// pub fn test_one_constraint<F: Field>(
//   air: &dyn Air,
//   trace: &Trace,
//   constraint_id: usize,
//   rng: &mut impl RngCore,
// ) -> bool { // Implementation
// }

// pub fn test_constraint_range<F: Field>(
//   air: &dyn Air,
//   trace: &Trace,
//   start: usize,
//   end: usize,
//   rng: &mut impl RngCore,
// ) -> bool { // Implementation
// }

// pub fn get_failing_constraints<F: Field>(
//   air: &dyn Air,
//   trace: &Trace,
//   rng: &mut impl RngCore,
// ) -> HashSet<usize> { // Implementation
// }

// pub fn get_failing_constraints_binary_search<F: Field>(
//   air: &dyn Air,
//   trace: &Trace,
//   rng: &mut impl RngCore,
// ) -> HashSet<usize> { // Implementation
// }

// pub fn get_trace_row<F: Field>(trace: &Trace, row_idx: usize, dst: &mut [F]) {
//   // Implementation
// }

// pub fn set_trace_row<F: Field>(trace: &mut Trace, row_idx: usize, src: &[F]) {
//   // Implementation
// }

// pub fn apply_manipulation<F: Field>(
//   trace: &mut Trace,
//   row_idx: usize,
//   manipulator: impl FnMut(&mut [F], &mut [F]),
// ) { // Implementation
// }

// pub fn test_air_constraint<F: Field>(
//   air: &dyn Air,
//   field: &F,
//   trace_len: usize,
//   constraint_idx: usize,
//   domain_indices: Vec<usize>,
//   manipulator: impl FnMut(&mut [F], &mut [F], bool),
//   rng: &mut impl RngCore,
// ) { // Implementation
// }

// pub fn domain_predicate_to_list(
//   predicate: impl Fn(usize) -> bool,
//   num_elements: usize,
// ) -> Vec<usize> { // Implementation
// }

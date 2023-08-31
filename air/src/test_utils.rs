use std::{
  collections::{BTreeSet, HashSet},
  convert::AsRef,
  fmt::Debug,
  func::{Fn, FnMut},
  mem::take,
  ops::Range,
};

use rand_core::RngCore;

use crate::{
  air::{Air, InteractionParams},
  composition_polynomial::CompositionPolynomial,
  field::Field,
  math::{batch_pow, pow},
  trace::Trace,
};

pub struct DummyAir<F>
where F: Field {
  // Same fields as original DummyAir
}

impl<F> Air for DummyAir<F>
where F: Field + Debug
{
  // Implement Air traits
}

pub fn compute_composition_degree(
  air: &dyn Air,
  trace: &Trace,
  coefficients: &[F],
  num_cosets: usize,
) -> i64 {
  // Implementation
}

pub fn test_one_constraint<F>(
  air: &dyn Air,
  trace: &Trace,
  constraint_id: usize,
  rng: &mut impl RngCore,
) -> bool {
  // Implementation
}

pub fn get_failing_constraints<F>(
  air: &dyn Air,
  trace: &Trace,
  rng: &mut impl RngCore,
) -> HashSet<usize> {
  // Implementation
}

pub fn get_failing_constraints_binary_search<F>(
  air: &dyn Air,
  trace: &Trace,
  rng: &mut impl RngCore,
) -> HashSet<usize> {
  // Implementation
}

pub fn draw_random_trace(width: usize, height: usize, field: &F, rng: &mut impl RngCore) -> Trace
where F: Field + Debug + Rand {
  // Implementation
}

pub fn test_air_constraint<F>(
  air: &dyn Air,
  field: &F,
  trace_len: usize,
  constraint_idx: usize,
  domain_indices: Vec<usize>,
  trace_manipulator: impl Fn(&mut [F], &mut [F], bool),
  rng: &mut impl RngCore,
) where
  F: Field + Debug,
{
  // Implementation
}

pub fn domain_predicate_to_list(
  predicate: impl Fn(usize) -> bool,
  num_elements: usize,
) -> Vec<usize> {
  // Implementation
}

pub fn merge_traces<F>(traces: &[Trace]) -> Trace
where F: Field {
  // Implementation
}

pub struct TestTraceContext {
  trace: Option<Trace>,
  // Other fields
}

impl TraceContext for TestTraceContext {
  // Implement methods
}

pub fn compute_composition_degree<F: Field>(
  air: &dyn Air,
  trace: &Trace,
  coefficients: &[F],
  num_cosets: usize,
) -> i64 {
  // Implementation
}

pub fn draw_random_trace<F>(
  width: usize,
  height: usize,
  field: &F,
  rng: &mut impl RngCore,
) -> Trace
where
  F: Field + Debug + Rand,
{
  // Implementation
}

pub fn test_one_constraint<F: Field>(
  air: &dyn Air,
  trace: &Trace,
  constraint_id: usize,
  rng: &mut impl RngCore,
) -> bool {
  // Implementation
}

pub fn test_constraint_range<F: Field>(
  air: &dyn Air,
  trace: &Trace,
  start: usize,
  end: usize,
  rng: &mut impl RngCore,
) -> bool {
  // Implementation
}

pub fn get_failing_constraints<F: Field>(
  air: &dyn Air,
  trace: &Trace,
  rng: &mut impl RngCore,
) -> HashSet<usize> {
  // Implementation
}

pub fn get_failing_constraints_binary_search<F: Field>(
  air: &dyn Air,
  trace: &Trace,
  rng: &mut impl RngCore,
) -> HashSet<usize> {
  // Implementation
}

pub fn get_trace_row<F: Field>(trace: &Trace, row_idx: usize, dst: &mut [F]) {
  // Implementation
}

pub fn set_trace_row<F: Field>(trace: &mut Trace, row_idx: usize, src: &[F]) {
  // Implementation
}

pub fn apply_manipulation<F: Field>(
  trace: &mut Trace,
  row_idx: usize,
  manipulator: impl FnMut(&mut [F], &mut [F]),
) {
  // Implementation
}

pub fn test_air_constraint<F: Field>(
  air: &dyn Air,
  field: &F,
  trace_len: usize,
  constraint_idx: usize,
  domain_indices: Vec<usize>,
  manipulator: impl FnMut(&mut [F], &mut [F], bool),
  rng: &mut impl RngCore,
) {
  // Implementation
}

pub fn domain_predicate_to_list(
  predicate: impl Fn(usize) -> bool,
  num_elements: usize,
) -> Vec<usize> {
  // Implementation
}

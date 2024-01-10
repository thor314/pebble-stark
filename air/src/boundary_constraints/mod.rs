//! https://github.com/starkware-libs/stone-prover/tree/main/src/starkware/air/boundary_constraints
use ark_ff::Field;
use composition_polynomial::PeriodicColumn;

use crate::GslSpan; // Using ark-ff crate for field generics

/// Creates a periodic column
/// which satisfies:
///   periodic_column(rows[i]) = values[i] * c[i]
/// where c[i] is invertible (!= 0) and depends on rows but not on values.
///
/// In particular, one may take one periodic column with (rows, values) and another with
/// (rows, ones) (where ones is a vector of 1's. See create_base_boundary_periodic_column()) and
/// obtain:
///   periodic_column(rows[i]) / ones_periodic_column(rows[i]) = values[i].
///
///  The two periodic columns may be used to enforce boundary constraints on the trace. Take
/// (rows, values) to be the boundary constraints, and add the following constraint to the AIR:
///   mask_item * ones_periodic_column - periodic_column
/// on the domain given by rows (prod_i(x - g^rows[i])).
///
/// # Arguments
/// - `rows`: span of row indices.
/// - `values`: span of field element values.
pub fn create_boundary_periodic_column<F: Field>(
  rows: GslSpan<usize>,
  values: GslSpan<F>,
  trace_length: usize,
  trace_generator: F,
  trace_offset: F,
) -> PeriodicColumn<F> {
  todo!()
}

/// Creates a periodic column with y-values all set to 1.
pub fn create_base_boundary_periodic_column<F: Field>(
  rows: GslSpan<usize>,
  trace_length: u64,
  trace_generator: F,
  trace_offset: F,
) -> PeriodicColumn<F> {
  todo!()
}

/// Creates a periodic column that is zero at the specified rows and invertible elsewhere.
pub fn create_vanishing_periodic_column<F: Field>(
  rows: GslSpan<usize>,
  trace_length: u64,
  trace_generator: F,
  trace_offset: F,
) -> PeriodicColumn<F> {
  todo!()
}

/// Creates a periodic column that is zero on rows {0, step, 2*step, ...} except for the given rows.
pub fn create_complement_vanishing_periodic_column<F: Field>(
  rows: GslSpan<usize>,
  step: u64,
  trace_length: u64,
  trace_generator: F,
  trace_offset: F,
) -> PeriodicColumn<F> {
  todo!()
}

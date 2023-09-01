mod fibonacci_trace_context;

use ark_ff::Field;

use crate::{assert_on_release, trace::Trace};

pub struct FibonacciAir<F: Field> {
  _phantom: std::marker::PhantomData<F>,
}

impl<F: Field> FibonacciAir<F> {
  pub fn fibonacci_air(witness: &F, trace_length: usize, fibonacci_claim_index: usize) -> Trace<F> {
    assert_on_release(trace_length.is_power_of_two(), "trace_length must be a power of 2.");
    assert_on_release(
      fibonacci_claim_index < trace_length,
      "fibonacci_claim_index must be less than trace_length.",
    );

    let mut x = F::ONE;
    let mut y = *witness;
    let trace_values: Vec<Vec<F>> = std::iter::repeat_with(|| {
      let out = vec![x, y];
      std::mem::swap(&mut x, &mut y);
      y += x;

      out
    })
    .take(trace_length)
    .collect();
    Trace::from(trace_values)
  }

  pub fn public_input_from_private_input(witness: &F, fibonacci_claim_index: usize) -> F {
    let mut x = F::ONE;
    let mut y = *witness;

    for _ in 0..fibonacci_claim_index {
      std::mem::swap(&mut x, &mut y);
      y += x;
    }

    x
  }
}

mod fibonacci_trace_context;

use ark_ff::Field;

use crate::{assert_on_release, trace::Trace};

// pub type FibonacciAirInvokedLayoutTypes = InvokedTypes<integral_constant<int, 0>>;

pub struct FibonacciAir<F: Field> {
  _phantom: std::marker::PhantomData<F>,
  // trace_length: usize, // tk(todo) maybe, are we implementing Air?
}

impl<F: Field> FibonacciAir<F> {
  pub fn get_trace(
    &self,
    witness: &F,
    trace_length: usize,
    fibonacci_claim_index: usize,
  ) -> Trace<F> {
    assert_on_release(trace_length.is_power_of_two(), "trace_length must be a power of 2.");
    assert_on_release(
      fibonacci_claim_index < trace_length,
      "fibonacci_claim_index must be less than trace_length.",
    );

    let mut trace_values = (Vec::with_capacity(trace_length), Vec::with_capacity(trace_length));
    let mut x = F::one();
    let mut y = *witness;

    for _ in 0..trace_length {
      trace_values.0.push(x);
      trace_values.1.push(y);

      std::mem::swap(&mut x, &mut y);
      y += &x;
    }

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

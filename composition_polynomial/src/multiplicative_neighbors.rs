use ark_ff::Field;

// todo(tk)
pub struct MultiplicativeNeighbors<F: Field> {
  pub neighbors: Vec<Vec<F>>,
}

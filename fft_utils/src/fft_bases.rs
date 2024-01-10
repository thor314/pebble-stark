//! mirror: https://github.com/starkware-libs/stone-prover/blob/00b274b55c82077184be4c0758f7bed18950eaba/src/starkware/fft_utils/fft_bases.h

use ark_ff::Field;
use utils::assert_on_release;

/// Contains information about FFT/FRI layers. For a domain of size 2^N, there are N layers.
/// Layer i reduces a domain of size 2^(N-i) to a domain of size  2^(N-i-1). This means we have N+1
/// domains (Layer i transform from domain i to domain i+1). The last domain is of size 1, with an
/// empty basis.
#[derive(Debug, Clone)]
// TODO(TK 2024-01-09): maybe Field -> FftField
// avoid the <X>Impl C++ type pattern.
// intentionally unimplemented: get_field()
pub struct FftBases<F: Field> {
  pub num_layers: usize,
  pub bases:      Vec<DomainT<F>>,
}

impl<F: Field> FftBases<F> {
  pub fn new(bases: Vec<DomainT<F>>) -> Self {
    assert_on_release(!bases.is_empty(), "basis must not be empty");
    // TODO(TK 2024-01-09): fix lata
    // assert_on_release(!bases.last().basis_size() == 0, "basis must end in an empty domain");

    Self { num_layers: bases.len(), bases }
  }

  pub fn get_shifted_bases(&self, offset: &F) -> &FftBases<F> { todo!() }

  pub fn split_to_cosets(&self, n_log_cosets: usize) -> (&FftBases<F>, Vec<F>) { todo!() }

  pub fn apply_basis_transform(point: &F, layer_index: usize) -> F { todo!() }
}

// impl<F: Field> std::ops::Index<usize> for FftBases<F> {
//   type Output = FftBase<F>;

//   fn index(&self, idx: usize) -> &Self::Output { &self.bases[idx] }
// }

#[derive(Debug, Clone)]
pub struct DomainT<F: Field> {
  pub domain: FftDomainBase<F>,
}

#[derive(Debug, Clone)]
pub struct FftDomainBase<F: Field> {
  pub coset_size:           F,
  pub coset_offset:         usize,
  pub coset_index:          usize,
  pub coset_size_log:       usize,
  pub coset_offset_log:     usize,
  pub coset_index_log:      usize,
  pub coset_size_inv:       usize,
  pub coset_offset_inv:     usize,
  pub coset_index_inv:      usize,
  pub coset_size_inv_log:   usize,
  pub coset_offset_inv_log: usize,
  pub coset_index_inv_log:  usize,
}

// todo: temp
#[derive(Debug, Clone)]
pub struct FftBase<F: Field>(F);

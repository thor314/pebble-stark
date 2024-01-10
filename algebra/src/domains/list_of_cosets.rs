//! mirror: https://github.com/starkware-libs/stone-prover/blob/00b274b55c82077184be4c0758f7bed18950eaba/src/starkware/algebra/domains/list_of_cosets.h
//! mirror: https://github.com/starkware-libs/stone-prover/blob/00b274b55c82077184be4c0758f7bed18950eaba/src/starkware/algebra/domains/list_of_cosets.cc

use ark_ff::FftField;
use fft_utils::{Base, FftBases};

/// ListOfCosets is a union of cosets of a group. Let G be a multiplicative subgroup of the
/// field of size coset_size, then the instance represents a set which is a union of the
/// cosets s_0 # G, s_1 # G , ... , s_{n_cosets-1} # G (where # is the group opperation).
#[derive(Clone, Debug)]
pub struct ListOfCosets<F: FftField, B: Base> {
  pub fft_bases:       Box<FftBases<F, B>>,
  pub cosets_offsets:  Vec<F>,
  pub trace_generator: F,
}

// intentionally unimplemented: group(), get_field(), trace_generator(), cosets_offsets(), bases
impl<F: FftField, B: Base> ListOfCosets<F, B> {
  /// Constructs an instance with a group of size coset_size and the number of cosets is n_cosets.
  /// The cosets offsets are as follows:
  ///
  /// In both cases the offsets are s_0 = c, s_1 = ch, s_2 = c(h^2), ... , s_i = c(h^i).
  /// Where c is a generator of the field's multiplicative group (in both cases).
  /// In the multiplicative case h is a generator of a group H such that G is a subgroup of H, and
  /// |H| is the minimal power of two not smaller than |G|*n_cosets.
  ///
  /// In particular: G is disjoint to all cosets, and in case G is multiplicative and |G|*n_cosets
  /// is a valid size of a subgroup H, the union of the cosets is a coset of H.
  ///
  /// The order parameter affects the order of Bases(), which represents the domain within each
  /// coset. The cosest offsets remain unchanged, only the order within each coset.
  pub fn new() -> Self { todo!() }

  pub fn num_cosets(&self) -> usize { self.cosets_offsets.len() }

  // pub fn size(&self) -> usize { self.fft_bases.bases.len() * self.cosets_offsets.len() }

  // pub fn element_by_index(&self, coset_index: usize, group_index: usize) -> F{todo!()}
  // pub fn vanishing_polynomial(&self, eval_point: &F) -> F{todo!()}
}

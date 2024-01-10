//! mirror: https://github.com/starkware-libs/stone-prover/blob/00b274b55c82077184be4c0758f7bed18950eaba/src/starkware/fft_utils/fft_bases.h
//! todo: need to find differences between DomainT = FftDomain<GroupT>, and my types,
//! ark_ff::FftField

use std::marker::PhantomData;

use ark_ff::{FftField, Field};
use utils::assert_on_release;

// TODO(TK 2024-01-09):
// placeholder trait until I find out what Base needs to be
pub trait Base: Clone + std::fmt::Debug {}
pub type DomainT<F> = F;

/// Contains information about FFT/FRI layers. For a domain of size 2^N, there are N layers.
/// Layer i reduces a domain of size 2^(N-i) to a domain of size  2^(N-i-1). This means we have N+1
/// domains (Layer i transform from domain i to domain i+1). The last domain is of size 1, with an
/// empty basis.
// TODO(TK 2024-01-09): maybe Field -> FftField
// TODO(TK 2024-01-09): C++ implementation is generic over BasesT and GroupT, may need diff api
// avoid the <X>Impl C++ type pattern. We will need an FFT domain over the group, and a Fieldelement
// from the group, which seems... hmm unsure intentionally unimplemented: get_field(), num_layers
#[derive(Debug, Clone)]
pub struct FftBases<F: FftField, B: Base> {
  pub num_layers: usize,
  pub bases:      Vec<B>,
  _todo_f:        PhantomData<F>,
}

impl<F: FftField, B: Base> FftBases<F, B> {
  pub fn new(bases: Vec<DomainT<B>>) -> Self {
    assert_on_release(!bases.is_empty(), "basis must not be empty");
    // TODO(TK 2024-01-09): fix later
    // assert_on_release(!bases.last().basis_size() == 0, "basis must end in an empty domain");

    Self { num_layers: bases.len(), bases, _todo_f: PhantomData }
  }

  /// Returns an FftBasesImpl instance that is derived from the original FftBasesImpl
  /// by changing the offsets in all the layers.
  /// The offset at layer i is obtained from the offset at layer i-1 using a 2 to 1 mapping.
  /// The result is independent of the offset in the original FftBasesImpl instance.
  pub fn get_shifted_bases(&self, offset: &F) -> &FftBases<F, B> {
    // B(self[0].get_shifted_domain(offset))
    todo!()
  }

  // todo:
  // template <typename BasesT, typename GroupT>
  // std::tuple<std::unique_ptr<FftBases>, std::vector<FieldElement>>
  // FftBasesCommonImpl<BasesT, GroupT>::SplitToCosets(size_t n_log_cosets) const {
  // https://github.com/starkware-libs/stone-prover/blob/00b274b55c82077184be4c0758f7bed18950eaba/src/starkware/fft_utils/fft_bases.inl#L27C1-L29C79
  pub fn split_to_cosets(&self, n_log_cosets: usize) -> (FftBases<F, B>, Vec<F>) {
    assert_on_release(!self.bases.is_empty(), "bases must not be empty");
    let domain = self[0].clone();
    // let basis: Vec<F> = domain.basis();
    // assert_on_release(basis.len() >= n_log_cosets, "too many cosets requested");

    // let (cosets_bases, offsets_domain) = B::split_domain(domain, n_log_cosets);
    // let offsets: Vec<F> = offsets_domain.into_iter().collect();
    // // prior:  std::vector<FieldElement> offsets({offsets_domain.begin(), offsets_domain.end()});
    // ASSERT_RELEASE(offsets.len() == usize::pow2(n_log_cosets), "Wrong number of offsets");

    // (coset_bases, offsets)
    todo!()
  }

  pub fn apply_basis_transform(&self, point: &F, layer_index: usize) -> F {
    // assert_on_release(layer_index < self.num_layers(), "layer index out of range");
    // B::apply_basis_transform_tmpl(point, layer_index)
    todo!()
  }

  // TODO(TK 2024-01-09): unergonomic, move to From trait for Base
  pub fn base_from_layer(&self, idx: usize) -> B {
    assert_on_release(idx < self.bases.len(), "index out of bounds");
    // B(self.bases[idx].clone())
    todo!()
  }

  // TODO(TK 2024-01-09): unergonomic, move to From trait for Base
  // pub fn from_layer_as_unique(&self, idx: usize) -> Self { Self::new(self.base_from_layer(idx)) }
}

impl<F: FftField, B: Base> std::ops::Index<usize> for FftBases<F, B> {
  type Output = B;

  fn index(&self, idx: usize) -> &Self::Output { &self.bases[idx] }
}

/// Creates a series of FFT domains.
/// The i'th domain is start_offset^i*<g^i>.
/// The domains are ordered according to the Order paramater.
// TODO(TK 2024-01-09): clarify what this C++ type sigil is
// C++ note: template <
// typename FieldElementT,
// MultiplicativeGroupOrdering Order = MultiplicativeGroupOrdering::kBitReversedOrder>
// friend class FftBasesCommonImpl<
//  MultiplicativeFftBases<FieldElementT, Order>, FftMultiplicativeGroup<FieldElementT>>;
//   using GroupT = FftMultiplicativeGroup<FieldElementT>;
// using DomainT = FftDomain<GroupT>;
// static const MultiplicativeGroupOrdering kOrder = Order;
#[derive(Debug, Clone)]
pub struct MultiplicativeFftBases<F: Field> {
  _field: F,
}
// https://github.com/starkware-libs/stone-prover/blob/00b274b55c82077184be4c0758f7bed18950eaba/src/starkware/fft_utils/fft_bases.h#L165 stopping here until further need

impl<F: Field> MultiplicativeFftBases<F> {
  pub fn new(generator: &F, log_n: usize, start_offset: &F) -> Self {
    {
      todo!()
      // Self {}
    }
  }

  // todo
  // pub fn is_natural_order() -> bool { self.order == MultiplicativeGroupOrdering::kNaturalOrder}

  // TODO(TK 2024-01-09): domain stuff giving me hell
  pub fn is_natural_order(domain: &F) -> bool {
    todo!()
    // domain.coset_offset == 1 && domain.coset_index == 0
  }
}

// todo: https://github.com/starkware-libs/stone-prover/blob/00b274b55c82077184be4c0758f7bed18950eaba/src/starkware/fft_utils/fft_bases.inl#L42
// impl From<Domain> for MultiplicativeFftBases<F> {
//   fn from(domain: Domain) -> Self { Self::from(&domain) }
// }

// mirror: https://github.com/starkware-libs/stone-prover/blob/00b274b55c82077184be4c0758f7bed18950eaba/src/starkware/fft_utils/fft_bases.inl#L21
pub fn apply_dim_one_vanishing_polynomial<F: Field>(point: &F, basis_element: &F) -> F {
  *point * (*point + *basis_element)
}

// https://github.com/starkware-libs/stone-prover/blob/00b274b55c82077184be4c0758f7bed18950eaba/src/starkware/fft_utils/fft_bases.inl#L62
pub fn invoke_bases_template_version<F: FftField, B: Base, H, I, O>(
  func: H,
  bases: &FftBases<F, B>,
) where
  H: Fn(I) -> O,
{
  todo!()
}

#[cfg(test)]
mod test {
  // mirror: https://github.com/starkware-libs/stone-prover/blob/00b274b55c82077184be4c0758f7bed18950eaba/src/starkware/fft_utils/fft_bases_test.cc
  // TODO(TK 2024-01-09): complete tests
  use super::*;

  // TODO(TK 2024-01-09): temp typeholder
  #[derive(Debug, Clone)]
  struct TestFieldElement {
    elem: usize,
    // _f: PhantomData<F>
  }

  // Tests that the domains in Reversered order bases were derived correctly from the domain
  // start_offset*<g> where g is an element of order 8.
  fn test_reversed_order_multiplicative_bases() {
    todo!();
  }

  // Tests that the domains in Natural Order bases were derived correctly from the domain
  // start_offset*<g> where g is an element of order 8.
  fn test_natural_order_multiplicative_bases(
    g: &TestFieldElement,
    start_offset: &TestFieldElement,
    bases: &impl Base,
  ) {
    todo!()
  }

  #[test]
  fn fft_bases_test_multiplicative() {
    // let mut rng = ark_std::test_rng();
  }

  #[allow(clippy::extra_unused_type_parameters)]
  fn test_get_shifted_bases<F>(log_n: usize) {
    // let _a = F::one();
    // let mut rng = ark_std::test_rng();
    todo!()
  }

  #[test]
  fn fft_bases_get_shifted() {
    // test_get_shifted_bases::<TestFieldElement>(4)
  }

  #[test]
  fn test_fft_bases_from_layer() {
    // let mut rng = ark_std::test_rng();
  }

  // C++ testing with several types:
  //   using BasesTypes = ::testing::Types<
  //     MultiplicativeFftBases<TestFieldElement, MultiplicativeGroupOrdering::kBitReversedOrder>,
  //     MultiplicativeFftBases<TestFieldElement, MultiplicativeGroupOrdering::kNaturalOrder>>;

  // template <typename>
  // class FftBasesTyped : public ::testing::Test {};

  // TYPED_TEST_CASE(FftBasesTyped, BasesTypes);

  // https://github.com/starkware-libs/stone-prover/blob/00b274b55c82077184be4c0758f7bed18950eaba/src/starkware/fft_utils/fft_bases_test.cc#L161
  // fn test_split<F: Field, B: Base>(domain: B::Domain, n: usize){}

  // a couple other things
}

//! mirror: https://github.com/starkware-libs/stone-prover/blob/00b274b55c82077184be4c0758f7bed18950eaba/src/starkware/fft_utils/fft_bases.h

use ark_ff::Field;

/// Contains information about FFT/FRI layers. For a domain of size 2^N, there are N layers.
/// Layer i reduces a domain of size 2^(N-i) to a domain of size  2^(N-i-1). This means we have N+1
/// domains (Layer i transform from domain i to domain i+1). The last domain is of size 1, with an
/// empty basis.
#[derive(Debug, Clone)]
// TODO(TK 2024-01-09): maybe Field -> FftField
pub struct FftBases<F: Field> {
  pub bases: Vec<FftBase<F>>,
}

// todo: temp
#[derive(Debug, Clone)]
pub struct FftBase<F: Field>(F);

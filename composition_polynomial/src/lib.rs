//! https://github.com/starkware-libs/stone-prover/tree/main/src/starkware/composition_polynomial
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

/// Temporary type while I figure out what to do with gsl::span
pub type TempGslSpan<T> = Vec<T>;
pub type TempLDEManager = ();

mod periodic_column;

pub use periodic_column::PeriodicColumn;

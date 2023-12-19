use std::str::FromStr;

use lazy_static::lazy_static;
use num_bigint::BigInt;

pub trait BigPrimeConstants {
  const NBITS: u32;
  const INDEX: u32;
  type ValueType: Sized;

  // Define associated functions instead of constants for complex types
  fn modulus() -> Self::ValueType;
  fn montgomery_r() -> Self::ValueType;
  fn montgomery_r_squared() -> Self::ValueType;
  fn montgomery_r_cubed() -> Self::ValueType;
  fn factors() -> &'static [Self::ValueType];
  fn montgomery_m_prime() -> Self::ValueType;
  fn generator() -> Self::ValueType;
  fn max_divisible() -> Self::ValueType;
}

struct BigPrimeConstants252_0;

lazy_static! {
  static ref FACTORS_252_0: [BigInt; 5] = [
    BigInt::from_str("3").unwrap(),
    BigInt::from_str("5").unwrap(),
    BigInt::from_str("7").unwrap(),
    BigInt::from_str("5e2430d").unwrap(),
    BigInt::from_str("9f1e667").unwrap(),
  ];
}

impl BigPrimeConstants for BigPrimeConstants252_0 {
  type ValueType = BigInt;

  const INDEX: u32 = 0;
  const NBITS: u32 = 252;

  fn modulus() -> Self::ValueType {
    BigInt::from_str("800000000000011000000000000000000000000000000000000000000000001").unwrap()
  }

  fn montgomery_r() -> Self::ValueType {
    BigInt::from_str("7fffffffffffdf0ffffffffffffffffffffffffffffffffffffffffffffffe1").unwrap()
  }

  fn montgomery_r_squared() -> Self::ValueType {
    BigInt::from_str("7ffd4ab5e008810ffffffffff6f800000000001330ffffffffffd737e000401").unwrap()
  }

  fn montgomery_r_cubed() -> Self::ValueType {
    BigInt::from_str("38e5f79873c0a6df47d84f8363000187545706677ffcc06cc7177d1406df18e").unwrap()
  }

  fn factors() -> &'static [Self::ValueType] { &*FACTORS_252_0 }

  fn montgomery_m_prime() -> Self::ValueType {
    BigInt::from(u64::MAX) // Equivalent to ~uint64_t(0)
  }

  fn generator() -> Self::ValueType { BigInt::from(3) }

  fn max_divisible() -> Self::ValueType {
    BigInt::from_str("f80000000000020f00000000000000000000000000000000000000000000001f").unwrap()
  }
}

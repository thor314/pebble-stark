#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
pub fn assert_on_release(condition: bool, msg: &str) {
  #[cfg(not(debug_assertions))]
  {
    if !condition {
      panic!("{}", msg);
    }
  }
}

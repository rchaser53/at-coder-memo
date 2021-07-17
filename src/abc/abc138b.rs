/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    n:usize,
    vals:[f64;n]
  }

  let mut total = 0f64;
  for v in vals {
    total += 1f64 / v;
  }
  println!("{}", 1f64 / total);
}

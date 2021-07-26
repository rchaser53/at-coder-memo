/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

const MOD:usize = 1_000_000_007;
pub fn main(
) {
  input! {
    n:usize,
    vals:[(f64, String);n]
  }

  let mut total = 0f64;
  let mut bc = 0f64;

  for (v, t) in vals {
    if t == String::from("JPY") {
      total += v;
    } else {
      bc += v;
    }
  }

  total += bc * 380000f64;
  println!("{}", total);
}

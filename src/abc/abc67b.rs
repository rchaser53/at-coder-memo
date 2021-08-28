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
    k:usize,
    mut vals:[usize;n]
  }
  vals.sort();
  vals.reverse();
  let mut result = 0;
  for i in 0..k {
    result += vals[i];
  }
  println!("{}", result);
}

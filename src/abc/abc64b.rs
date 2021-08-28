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
    mut vals:[usize;n]
  }

  vals.sort();
  println!("{}", vals[n-1] - vals[0]);
}

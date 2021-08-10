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
    mut vals:[usize;3],
    k:usize,
  }

  vals.sort();
  for _ in 0..k {
    vals[2] *= 2;
  }

  println!("{}", vals.iter().sum::<usize>());
}
